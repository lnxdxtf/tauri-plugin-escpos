const COMMANDS: &[&str] = &[
    "request_permissions",
    "permissions_ok",
    "start",
    "check_store_state",
];

fn main() {
    tauri_plugin::Builder::new(COMMANDS)
        .android_path("android")
        // .ios_path("ios")
        .build();
}
