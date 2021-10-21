use std::error::Error;
// use std::path::{Path, PathBuf};
use std::process::{self, Command};

use tauri::api::path;
use serde::{Serialize, Deserialize};
use serde_repr::{Serialize_repr, Deserialize_repr};

use crate::{code_path, logseq_path};

// firefox --new-window
// Start new firefox window, open spotify / soundcloud - dj okawari

// Open code, open nenia

// Git pull logseq

// Open logseq

// Start alarm

// Move firefox spotify

// and close alarm

#[derive(Debug, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum Await {
    Spawn = 0,
    Output = 1
}

#[derive(Debug, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum ActionType {
    Open = 0,
    GitPull = 1
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ActionSpecificActions {
    command: String,
    exit_code: Await
}


#[derive(Debug, Serialize, Deserialize)]
pub struct Action {
    name: String,
    icon: String,
    types: ActionType,
    props: ActionSpecificActions,
    run: bool
}

fn main() -> Result<(), Box<dyn Error>> {
    let home_dir = path::home_dir().expect("home dir not set");
    let git_dir = {
        let mut t = home_dir.clone();
        t.push("git");
        t
    };

    let ffpath = crate::ff_path()?;

    let crafting_interpreters_dir = {
        let mut t = git_dir.clone();
        t.push("craftinginterpreters");
        t.push("site");
        t.push("contents.html");
        t
    };

    // Start new firefox window, open crafting interpreters file
    println!("opening crafting interpreters");
    Command::new(&ffpath)
        .args(&[
            "--new-window",
            &crafting_interpreters_dir
                .to_str()
                .expect("crafting interpreters dir emtpy")
                .to_string(),
        ])
        .spawn()?;

    // Start new firefox window, open dj okawari
    println!("opening spotify");
    Command::new(&ffpath)
        .args(&[
            "--new-window",
            // should be changed probably
            "https://open.spotify.com/artist/34QbYbTlUCLkZsQ8QmacV9/discography/album"
        ])
        .spawn()?;

    let notes_dir = {
        let mut t = git_dir.clone();
        t.push("notes");
        t.push(".git");
        t
    };

    // update logseq
    println!("updating logseq");
    let output = Command::new("git")
        .args(&[
            &format!(
                "--git-dir={}",
                notes_dir.to_str().expect("nenia dir does not exist")
            ),
            "pull",
            "--rebase",
        ])
        .output()?;
    if output.status.success() {
        println!("{}", String::from_utf8(output.stdout)?);
    } else {
        eprintln!("{}", String::from_utf8(output.stderr)?);
    }

    // open logseq
    println!("opening logseq");
    Command::new(logseq_path()?).spawn()?;

    // open alarms
    println!("opening alarms");
    Command::new("explorer")
        .arg("shell:appsFolder\\microsoft.windowsalarms_8wekyb3d8bbwe!App")
        .spawn()?;

    // update nenia
    let nenia_dir = {
        let mut t = git_dir.clone();
        t.push("nenia");
        t
    };

    let nenia_git_dir = {
        let mut t = nenia_dir.clone();
        t.push(".git");
        t
    };

    println!("updating nenia");
    let output = Command::new("git")
        .args(&[
            &format!(
                "--git-dir={}",
                nenia_git_dir.to_str().expect("nenia dir does not exist")
            ),
            "pull",
            "--rebase",
        ])
        .output()?;
    if output.status.success() {
        println!("{}", String::from_utf8(output.stdout)?);
    } else {
        eprintln!("{}", String::from_utf8(output.stderr)?);
    }

    // then open it with code
    println!("opening nenia dir with code");
    Command::new(code_path()?)
        .args(&[
            "-r",
            nenia_dir.to_str().expect("nenia dir does not exist"),
        ])
        .spawn()?;

    Ok(())
}
