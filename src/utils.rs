//use crate::utils
use std::process::Command;
use std::process::Stdio;

pub fn if_git_exists() -> bool {
    Command::new("git")
        .args(&["--version"])
        .stdout(Stdio::null())
        .spawn()
        .and_then(|mut child| child.wait())
        .map(|status| status.success())
        .unwrap_or(false)
}
