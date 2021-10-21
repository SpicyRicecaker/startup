#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use app::run::Action;

#[tauri::command]
fn save(actions: String) {
  let rando: Vec<Action> = match serde_json::from_str(&actions) {
    Ok(o) => o,
    Err(e) => {
      eprintln!("{}", e);
      panic!();
    },
  };
  dbg!(rando);
}

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![save])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
