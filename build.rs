const COMMANDS: &[&str] = &[
    "request_permissions",
    "permissions_status",
    "start",
    "start_scan",
    "connect",
    "disconnect",
];

fn main() {
    tauri_plugin::Builder::new(COMMANDS)
        .android_path("android")
        .build();
}
