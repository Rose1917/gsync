//use crate::utils
// use psutil::process as ps_process;
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

pub fn if_daemon_running(daemon_str: &str) -> bool {
    // println!("{:#?}", ps_process::processes().unwrap());
    // let ps_collect = ps_process::processes().unwrap();
    // ps_collect.iter().
    let output = Command::new("ps")
        .args(&["aux"])
        // Tell the OS to record the command's output
        .stdout(Stdio::piped())
        // execute the command, wait for it to complete, then capture the output
        .output()
        // Blow up if the OS was unable to start the program
        .unwrap();

    // extract the raw bytes that we captured and interpret them as a string
    let output = String::from_utf8(output.stdout).unwrap();
    output.contains(daemon_str)
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_if_daemon_running() {
        assert_eq!(true, if_daemon_running("ssh"));
    }
}
