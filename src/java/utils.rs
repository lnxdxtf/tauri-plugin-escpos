#![cfg(target_os = "android")]

use std::cell::RefCell;
use std::sync::atomic::{AtomicUsize, Ordering};

use jni::objects::GlobalRef;
use jni::{AttachGuard, JNIEnv};
use tokio::sync::OnceCell;

static CLASS_LOADER: OnceCell<GlobalRef> = OnceCell::const_new();
pub(crate) static JAVAVM: OnceCell<jni::JavaVM> = OnceCell::const_new();
pub(crate) static RUNTIME: OnceCell<tokio::runtime::Runtime> = OnceCell::const_new();

pub(crate) fn get_java_thread<'a>(
    jni_env: &'a JNIEnv<'a>,
) -> Result<jni::objects::JObject<'a>, jni::errors::Error> {
    Ok(jni_env
        .call_static_method(
            "java/lang/Thread",
            "currentThread",
            "()Ljava/lang/Thread;",
            &[],
        )?
        .l()?)
}

#[derive(thiserror::Error, Debug)]
pub(crate) enum SetupClassloaderError {
    #[error(transparent)]
    Jni(#[from] jni::errors::Error),
    #[error("Failed to set class loader")]
    ClassLoaderAlreadySet,
}

pub(crate) fn setup_class_loader(env: &JNIEnv) -> Result<(), SetupClassloaderError> {
    let thread = get_java_thread(env).expect("Failed to get current java thread");
    let class_loader = env
        .call_method(
            thread,
            "getContextClassLoader",
            "()Ljava/lang/ClassLoader;",
            &[],
        )?
        .l()?;

    CLASS_LOADER.set(env.new_global_ref(class_loader)?).map_err(|_| SetupClassloaderError::ClassLoaderAlreadySet)?;

    Ok(())
}

#[derive(thiserror::Error, Debug)]
pub(crate) enum LoadBtleplugContextError {
    #[error(transparent)]
    Jni(#[from] jni::errors::Error),
    #[error("Failed to get current java thread")]
    FailedToGetJavaThread,
}

pub(crate) fn load_btleplug_context() -> Result<(), LoadBtleplugContextError> {
    let vm = JAVAVM.get().ok_or(LoadBtleplugContextError::FailedToGetJavaThread)?;
    let env = vm.attach_current_thread()?;

    let thread = get_java_thread(&env)?;
    env.call_method(
        thread,
        "setContextClassLoader",
        "(Ljava/lang/ClassLoader;)V",
        &[CLASS_LOADER.get().unwrap().as_obj().into()],
    )?;

    Ok(())
}

#[derive(thiserror::Error, Debug)]
pub(crate) enum SetupRuntimeError {
    #[error("Failed to create jni attached runtime")]
    RuntimeCreation(#[source] std::io::Error),
    #[error("Failed to set runtime")]
    RuntimeAlreadySet,
}

std::thread_local! {
    static JNI_ENV: RefCell<Option<AttachGuard<'static>>> = RefCell::new(None);
}

pub(crate) fn setup_runtime() -> Result<(), SetupRuntimeError>{
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .thread_name_fn(|| {
            static ATOMIC_ID: AtomicUsize = AtomicUsize::new(0);
            let id = ATOMIC_ID.fetch_add(1, Ordering::SeqCst);
            format!("tauri-plugin-btleplug-{}", id)
        })
        .on_thread_stop(|| {
            JNI_ENV.with(|f| *f.borrow_mut() = None);
        })
        .on_thread_start(move || {
            let vm = JAVAVM.get().expect("JavaVM not set");
            let env = vm.attach_current_thread().expect("Failed to attach to JVM");

            load_btleplug_context().expect("Failed to load btleplug context");

            JNI_ENV.with(|f| *f.borrow_mut() = Some(env));
        })
        .build()
        .map_err(SetupRuntimeError::RuntimeCreation)?;

    RUNTIME.set(runtime).map_err(|_| SetupRuntimeError::RuntimeAlreadySet)?;

    Ok(())
}
