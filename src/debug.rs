use log::LevelFilter;

pub fn debug_init() {
    #[cfg(target_os = "android")]
    {
        use android_logger::{init_once, Config};
        init_once(
            Config::default()
                .with_max_level(LevelFilter::Trace)
                .with_tag("TauriPluginEscpos"),
        );
        info!("Android logger initialized");
    }

    // TODO: Add more platforms
}
