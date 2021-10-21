mod run;

use std::{error::Error, path::PathBuf};
use tauri::api::path;

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

