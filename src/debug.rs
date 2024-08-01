use log::LevelFilter;

#[cfg(target_os = "android")]
pub fn debug_init() {
    use android_logger::{init_once, Config, FilterBuilder};
    init_once(
        Config::default()
            .with_max_level(LevelFilter::Trace)
            .with_tag("TauriPluginEscpos"),
    );
    log::info!("Android logger initialized");
}
