const COMMANDS: &[&str] = &["check_network_status", "wait_for_network"];

fn main() {
  tauri_plugin::Builder::new(COMMANDS)
    .android_path("android")
    .ios_path("ios")
    .build();
}
