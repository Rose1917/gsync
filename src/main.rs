use argh::FromArgs;
use dirs;
use serde::Deserialize;
use serde::Serialize;
use toml::value::Datetime;

use std::fs;

use crate::repos::*;
use crate::utils::*;

const VERSION: &str = "0.1.0";
const CONFIG_NAME: &str = ".git-sync.toml";
#[allow(dead_code)]
mod repos;
#[allow(dead_code)]
mod server;
#[allow(dead_code)]
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
pub struct ListArgs {
    #[argh(switch)]
    ///show the remote folders that have not been tracked
    available: bool,

    #[argh(switch)]
    ///show the tracked folders
    tracked: bool,

    #[argh(switch)]
    ///show the verbose information
    verbose: bool,
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "track")]
/// append a new folder to tracked folders set
pub struct TrackArgs {
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
pub struct UntrackArgs {
    #[argh(positional)]
    /// the path to where it is stored
    path: String,
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "daemon")]
/// manage the git-sync daemon status
pub struct DaemonArgs {
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

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    status: Status,
    remote: Remote,
    repos: Option<Vec<Repo>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Status {
    enabled: bool,
    local_base: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Remote {
    host: String,
    port: i32,
    user_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Repo {
    repo_name: String,
    local_path: String,
    sync_type: String,
    sync_freq: i32,
    if_owner: bool,
    if_activated: bool,
    added_time: Datetime,
    synced_time: Datetime,
}

// static mut REPO_INFO: Vec<TomlVal> = Vec::new();
fn main() {
    let args: Args = argh::from_env();

    // handle the `gsync --version`
    if args.version {
        println!("v{}", VERSION);
        std::process::exit(0);
    }

    // handle the `gsync`
    if args.nested.is_none() {
        println!();
        println!(r#"    welcome to use gsync."#);
        println!(r#"    use gsync --help to see all the subcommands"#);
        println!(r#"  ____ ______   ___   _  ____ "#);
        println!(r#" / ___/ ___\ \ / / \ | |/ ___|"#);
        println!(r#"| |  _\___ \\ V /|  \| | |    "#);
        println!(r#"| |_| |___) || | | |\  | |___ "#);
        println!(r#" \____|____/ |_| |_| \_|\____|"#);
        println!();
        std::process::exit(0);
    }

    // check the configuration file in ~/.git-sync.toml
    let conf_path = dirs::home_dir()
        .unwrap()
        .join(CONFIG_NAME)
        .as_path()
        .to_owned();

    if !conf_path.exists() {
        println!(
            ".git-sync.toml can not be found under {}",
            dirs::home_dir().unwrap().to_str().unwrap()
        );
        println!("you can try to reinstall gsync");
        std::process::exit(1);
    }

    // check the git prerequisites
    if !if_git_exists() {
        println!("We cannot find `git`.");
        println!("Try running `git --version` to diagnose your problem.");
        std::process::exit(1);
    }

    // test if the daemon is running
    if !if_daemon_running("gsyncd-fake") {
        println!("the gsync daemon is not running, please run gsync start to start it");
        std::process::exit(1);
    }

    // parse the ~/.git-sync.toml
    let toml_str = &fs::read_to_string(conf_path.to_str().unwrap()).unwrap();
    let mut toml_conf: Config = toml::from_str(toml_str).unwrap_or_else(|e| {
        println!("{}", e);
        println!("an error occurred while paring the gsync configuration file");
        std::process::exit(1);
    });

    // get the subcommands
    let command = args.nested.unwrap();
    match command {
        Subcommands::List(subargs) => show_repos(&subargs, &toml_conf),
        Subcommands::Track(subargs) => (),
        Subcommands::Untrack(subargs) => (),
        Subcommands::Daemon(subargs) => (),
        Subcommands::Server(subargs) => (),
    }
}

#[allow(dead_code)]
fn init() {}

#[cfg(test)]
#[allow(dead_code)]
mod test {

    use super::utils::*;
    #[test]
    fn test_if_cmd_exists() {
        assert_eq!(true, if_git_exists());
    }
}
