#![cfg(target_os = "android")]

use crate::java::utils::{setup_class_loader, setup_runtime, JAVAVM};
use jni::objects::JClass;
use jni::JNIEnv;

#[no_mangle]
pub extern "system" fn Java_com_plugin_escpos_EscposPlugin_init(env: JNIEnv, _class: JClass) {
    eco_print::escpos::btleplug::platform::init(&env)
        .expect("Failed to initialize btleplug platform");

    setup_class_loader(&env).expect("Failed to set class loader");

    if let Err(_) = JAVAVM.set(env.get_java_vm().expect("Failed to get JavaVM")) {
        panic!("Failed to set JavaVM for btleplug plugin");
    }

    setup_runtime().expect("Failed to setup tokio runtime");
}
