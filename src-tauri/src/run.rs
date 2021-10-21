use std::error::Error;
// use std::path::{Path, PathBuf};
use std::process::{self, Command};

use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use tauri::api::path;

use crate::{code_path, load_fn, logseq_path};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

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
  Output = 1,
}

#[derive(Debug, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum ActionType {
  Open = 0,
  GitPull = 1,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ActionSpecificActions {
  command: String,
  exit_code: Await,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Action {
  name: String,
  icon: String,
  types: ActionType,
  props: ActionSpecificActions,
  run: bool,
}

impl Action {
  fn run(&self) -> Result<()> {
    //   doesn't matter for now lol
    let mut args = self.props.command.split_whitespace();

    // TODO too lazy to think of a way to replace unwrap rn
    let mut command = Command::new(args.next().unwrap());
    let args = args.into_iter().collect::<Vec<_>>();
    command.args(&args);

    match self.props.exit_code {
      Await::Spawn => {
        command.spawn()?;
      }
      Await::Output => {
        command.output()?;
      }
    }

    Ok(())
  }
}

pub fn run_fn() -> Result<()> {
    let actions = load_fn()?;
    run_actions(&actions)
}

fn run_actions(actions: &[Action]) -> Result<()> {
  actions.iter().filter(|a| a.run).try_for_each(|a| a.run())
}

// fn main() -> Result<()> {
//     let home_dir = path::home_dir().expect("home dir not set");
//     let git_dir = {
//         let mut t = home_dir.clone();
//         t.push("git");
//         t
//     };

//     let ffpath = crate::ff_path()?;

//     let crafting_interpreters_dir = {
//         let mut t = git_dir.clone();
//         t.push("craftinginterpreters");
//         t.push("site");
//         t.push("contents.html");
//         t
//     };

//     // Start new firefox window, open crafting interpreters file
//     println!("opening crafting interpreters");
//     Command::new(&ffpath)
//         .args(&[
//             "--new-window",
//             &crafting_interpreters_dir
//                 .to_str()
//                 .expect("crafting interpreters dir emtpy")
//                 .to_string(),
//         ])
//         .spawn()?;

//     // Start new firefox window, open dj okawari
//     println!("opening spotify");
//     Command::new(&ffpath)
//         .args(&[
//             "--new-window",
//             // should be changed probably
//             "https://open.spotify.com/artist/34QbYbTlUCLkZsQ8QmacV9/discography/album"
//         ])
//         .spawn()?;

//     let notes_dir = {
//         let mut t = git_dir.clone();
//         t.push("notes");
//         t.push(".git");
//         t
//     };

//     // update logseq
//     println!("updating logseq");
//     let output = Command::new("git")
//         .args(&[
//             &format!(
//                 "--git-dir={}",
//                 notes_dir.to_str().expect("nenia dir does not exist")
//             ),
//             "pull",
//             "--rebase",
//         ])
//         .output()?;
//     if output.status.success() {
//         println!("{}", String::from_utf8(output.stdout)?);
//     } else {
//         eprintln!("{}", String::from_utf8(output.stderr)?);
//     }

//     // open logseq
//     println!("opening logseq");
//     Command::new(logseq_path()?).spawn()?;

//     // open alarms
//     println!("opening alarms");
//     Command::new("explorer")
//         .arg("shell:appsFolder\\microsoft.windowsalarms_8wekyb3d8bbwe!App")
//         .spawn()?;

//     // update nenia
//     let nenia_dir = {
//         let mut t = git_dir.clone();
//         t.push("nenia");
//         t
//     };

//     let nenia_git_dir = {
//         let mut t = nenia_dir.clone();
//         t.push(".git");
//         t
//     };

//     println!("updating nenia");
//     let output = Command::new("git")
//         .args(&[
//             &format!(
//                 "--git-dir={}",
//                 nenia_git_dir.to_str().expect("nenia dir does not exist")
//             ),
//             "pull",
//             "--rebase",
//         ])
//         .output()?;
//     if output.status.success() {
//         println!("{}", String::from_utf8(output.stdout)?);
//     } else {
//         eprintln!("{}", String::from_utf8(output.stderr)?);
//     }

//     // then open it with code
//     println!("opening nenia dir with code");
//     Command::new(code_path()?)
//         .args(&[
//             "-r",
//             nenia_dir.to_str().expect("nenia dir does not exist"),
//         ])
//         .spawn()?;

//     Ok(())
// }
