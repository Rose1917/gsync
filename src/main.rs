use argh::FromArgs;
use toml::{de::Error, Value as TomlVal};

use std::fs;
use std::path::Path;
use std::process::{Command, Stdio};

use crate::utils::*;

const VERSION: &str = "0.1.0";
const CONFIG_PATH: &str = ".git-sync.toml";

mod repos;
mod server;
mod utils;

#[derive(FromArgs, PartialEq, Debug)]
/// gsync is a fast, free, open source sync tool based on git.
struct Args {
    /// show the gsync version
    #[argh(switch, short = 'v')]
    version: bool,
    #[argh(subcommand)]
    nested: Option<Subcommands>,
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand)]
enum Subcommands {
    List(ListArgs),
    Track(TrackArgs),
    Untrack(UntrackArgs),
    Daemon(DaemonArgs),
    Server(ServerArgs),
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "list")]
/// list the tracked folders and folders in the server
struct ListArgs {
    #[argh(switch)]
    ///show the remote folders that have not been tracked
    available: bool,

    #[argh(switch)]
    ///show the tracked folders
    tracked: bool,
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "track")]
/// append a new folder to tracked folders set
struct TrackArgs {
    #[argh(switch)]
    /// download a folder from server and track it
    online: bool,

    #[argh(switch)]
    /// upload a local folder to the server and track it
    local: bool,

    #[argh(option)]
    ///the path to the folder, only directory name is needed if --online is set
    path: String,

    #[argh(option)]
    ///the sync trigger, you can use "on_change"/"on_closed"/"on_minutes"
    ///if you use on_minuts, you can set the number of minutes to sync the file
    sync_type: Option<String>,

    #[argh(option, default = "1")]
    ///when you use on_minuts, the minuts to track the file change
    minutes: u32,
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "untrack")]
/// remove a folder from tracking directory set
struct UntrackArgs {
    #[argh(positional)]
    /// the path to where it is stored
    path: String,
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "daemon")]
/// manage the git-sync daemon status
struct DaemonArgs {
    #[argh(switch)]
    /// show the gsync daemon status
    status: bool,

    #[argh(switch)]
    /// start the gsync daemon
    start: bool,

    #[argh(switch)]
    /// stop the gsync daemon
    stop: bool,

    #[argh(switch)]
    /// start the gsync daemon when the system boots
    enable: bool,

    #[argh(switch)]
    /// disable the gsync daemon
    disable: bool,
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "server")]
/// show and set the server configuration
struct ServerArgs {
    #[argh(switch)]
    /// show the server configuration
    show: bool,

    #[argh(switch, short = 'n')]
    /// reset the server
    reset: bool,
}

// static mut REPO_INFO: Vec<TomlVal> = Vec::new();
fn main() {
    let args: Args = argh::from_env();

    if args.version {
        println!("v{}", VERSION);
        std::process::exit(0);
    }

    if args.nested.is_none() {
        println!();
        println!(r#"    welcome to use gsync."#);
        println!(r#"  ____ ______   ___   _  ____ "#);
        println!(r#" / ___/ ___\ \ / / \ | |/ ___|"#);
        println!(r#"| |  _\___ \\ V /|  \| | |    "#);
        println!(r#"| |_| |___) || | | |\  | |___ "#);
        println!(r#" \____|____/ |_| |_| \_|\____|"#);
        println!();
    }

    // trying to find configuration file in ~/.git-sync.toml
    let conf_path = std::env::home_dir()
        .unwrap()
        .join(CONFIG_PATH)
        .as_path()
        .to_owned();

    if !conf_path.exists() {
        println!(
            ".git-sync.toml can not be found under {}",
            std::env::home_dir().unwrap().to_str().unwrap()
        );
        println!("you can try to reinstall gsync");
        std::process::exit(1);
    }

    if !if_git_exists() {
        println!("We cannot find `git`.");
        println!("Try running `git --version` to diagnose your problem.");
        std::process::exit(1);
    }

    let toml_str = fs::read_to_string(conf_path.to_str().unwrap());
}

fn parse_repos() -> TomlVal {
    // if !if_file_exists(CONFIG_PATH) {
    //     eprintln!("can not find the configuration file in {}", CONFIG_PATH);
    // }

    let content =
        fs::read_to_string(CONFIG_PATH).expect("an error occurred while parsing the repo file");
    toml::from_str(&content).unwrap()
}

fn init() {}

#[cfg(test)]
mod test {

    use super::utils::*;
    #[test]
    fn test_if_cmd_exists() {
        assert_eq!(true, if_git_exists());
    }
}
