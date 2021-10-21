pub mod run;

use run::Action;
use std::fs;
use std::{error::Error, path::PathBuf};
use tauri::api::path;

pub fn load_fn() -> Result<Vec<Action>, Box<dyn std::error::Error>> {
  // then upon verifying, we'll write actions to a rando file
  let path = path::config_dir();
  if let Some(mut path) = path {
    path.push("startup");
    path.push("config.json");
    let config_string = fs::read_to_string(path)?;
    let obj: Vec<Action> = serde_json::from_str(&config_string)?;
    Ok(obj)
  } else {
    // write an empty config
    let vec: Vec<Action> = Vec::new();
    let config_string = serde_json::to_string(&vec)?;
    save_fn(&config_string)?;
    Ok(vec)
  }
}

pub fn save_fn(actions: &str) -> Result<(), Box<dyn std::error::Error>> {
  let _: Vec<Action> = serde_json::from_str(actions)?;
  // then upon verifying, we'll write actions to a rando file
  let path = path::config_dir();
  if let Some(mut path) = path {
    path.push("startup");
    fs::create_dir_all(&path)?;
    path.push("config.json");
    fs::write(path, actions)?;
  }

  Ok(())
}

pub fn ff_path() -> Result<PathBuf, Box<dyn Error>> {
  // just return c directory for now lol
  let path = [
    "C:\\",
    "Program Files",
    "Firefox Developer Edition",
    "firefox.exe",
  ]
  .iter()
  .collect();
  dbg!(&path);
  Ok(path)
}

pub fn logseq_path() -> Result<PathBuf, Box<dyn Error>> {
  // just return c directory for now lol
  let appdata_local = path::cache_dir().expect("cache dir not found");
  let logseq_path = {
    let mut t = appdata_local;
    t.push("Logseq");
    t.push("Logseq.exe");
    t
  };
  Ok(logseq_path)
}

pub fn code_path() -> Result<PathBuf, Box<dyn Error>> {
  // just return c directory for now lol
  let appdata_local = path::cache_dir().expect("cache dir not found");
  let code_path = {
    let mut t = appdata_local;
    t.push("Programs");
    t.push("Microsoft VS Code");
    t.push("Code.exe");
    t
  };
  Ok(code_path)
}
