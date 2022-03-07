use crate::Subcommands;
use crate::{Config, Repo, Status};
use crate::{DaemonArgs, ListArgs, ServerArgs, TrackArgs, UntrackArgs};

pub fn show_repos(subargs: &ListArgs, toml_conf: &Config) {
    ///show the available repos in the server
    if subargs.available {
        ///TODO:add online repos list
        if subargs.verbose {
            ///print detailedly
            ()
        } else {
            ///print generally
            ()
        }
        std::process::exit(0);
    }

    /// show tracked repos
    /// even if the user does not provide the --tracked, eg. it is default
    let repos = toml_conf.repos.as_ref().unwrap_or_else(|| {
        println!("you have not tracked any directory");
        println!("use `gsync track <path/to/directory> to track one");
        std::process::exit(0);
    });

    /// print repos
    /// TODO: use a function to do the following print.
    for (i, repo) in repos.iter().enumerate() {
        if !repo.if_activated {
            continue;
        }

        println!("# {}:", i + 1);

        println!("repo name: {}", repo.repo_name);
        println!("directory name: {}", repo.local_path);
        println!("sync_type: {}", repo.sync_type);
        if repo.sync_type == "on_minutes" {
            println!("sync_freq:{}", repo.sync_freq);
        }
        println!("if owner: {}", repo.if_owner);
        println!("added time: {}", repo.added_time);
        println!("synced_time: {}", repo.synced_time);
        println!("");
    }
}

fn show_locals() {}

fn add_repos() {}

fn parse_repos() {}
