#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

// use app::hello_world;

#[tauri::command]
fn hello_world() {
    println!("123");
}

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![hello_world])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
