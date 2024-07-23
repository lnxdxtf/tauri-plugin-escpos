const COMMANDS: &[&str] = &["request_permissions", "permissions_ok"];

fn main() {
    tauri_plugin::Builder::new(COMMANDS)
        .android_path("android")
        .ios_path("ios")
        .build();
}
