use std::fs;
use std::path::Path;
use std::process::{Command, Stdio};

use toml::{de::Error, Value as TomlVal};

const VERSION: &str = "0.1.0";
const CONFIG_PATH: &str = ".gh-sync.toml";

mod repos;
mod server;

static mut REPO_INFO: Vec<TomlVal> = Vec::new();
fn main() {}

fn if_cmd_exists(cmd_name: &str) -> bool {
    Command::new(cmd_name)
        .args(&["--version"])
        .stdout(Stdio::null())
        .spawn()
        .and_then(|mut child| child.wait())
        .map(|status| status.success())
        .unwrap_or(false)
}

fn if_file_exists(file_path: &str) -> bool {
    Path::new(file_path).exists()
}

fn parse_repos() -> TomlVal {
    if !if_file_exists(CONFIG_PATH) {
        eprintln!("can not find the configuration file in {}", CONFIG_PATH);
    }

    let content =
        fs::read_to_string(CONFIG_PATH).expect("an error occurred while parsing the repo file");
    toml::from_str(&content).unwrap()
}

fn init() {}

#[cfg(test)]
mod test {

    use super::*;
    #[test]
    fn test_if_cmd_exists() {
        assert_eq!(true, if_cmd_exists("pwd"));
        assert_eq!(true, if_cmd_exists("cp"));
        assert_eq!(true, if_cmd_exists("mv"));

        assert_eq!(false, if_cmd_exists("linux"));
        assert_eq!(false, if_cmd_exists(""));
    }

    #[test]
    fn test_if_file_exists() {
        assert_eq!(true, if_file_exists("/etc/passwd"));
        assert_eq!(false, if_file_exists("/etc/password"));
        assert_eq!(true, if_file_exists(".gh-sync.toml"));
    }

    #[test]
    fn test_parse_repos() {
        let v = parse_repos();
        assert_eq!(Some("Rose1917"), v["github"]["username"].as_str());
        //        assert_eq!(Some("********"), v["github"]["password"].as_str());
    }
}
