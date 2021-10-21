#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use app::{load_fn, run::{Action, run_fn}, save_fn};

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

#[tauri::command]
fn run() -> u32 {
   if let Err(e) = run_fn() {
    eprintln!("{}", e);
  } 
  // 0 for success
  // 1 for failure
  0
}

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![save, load, run])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
