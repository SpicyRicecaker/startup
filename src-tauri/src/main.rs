#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use app::{load_fn, run::Action, save_fn};

#[tauri::command]
/// Error handling on this func is bad lol
fn save(actions: String) {
  if let Err(e) = save_fn(&actions) {
    eprintln!("{}", e);
  }
}

#[tauri::command]
fn load() -> Vec<Action> {
  match load_fn() {
    Ok(o) => o,
    Err(e) => {
      eprintln!("{}", e);
      panic!();
    }
  }
}

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![save, load])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
