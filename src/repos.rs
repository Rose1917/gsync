use crate::Subcommands;
use crate::{Config, Repo, ServerConfig, ServerRepo, Status};
use crate::{DaemonArgs, ListArgs, ServerArgs, TrackArgs, UntrackArgs};

fn if_tracked(repo_name: &str, toml_conf: &Config) -> bool {
    if let None = toml_conf.repos {
        return false;
    } else {
        let repos = toml_conf.repos.as_ref().unwrap();
        repos.iter().any(|repo| repo.repo_name.eq(repo_name))
    }
}

pub fn show_repos(subargs: &ListArgs, toml_conf: &Config, server_toml_conf: &ServerConfig) {
    //show the available repos in the server
    if subargs.available {
        //show  online repos list
        let repos = server_toml_conf.repos.as_ref().unwrap_or_else(|| {
            println!("there are not repos available in the server");
            std::process::exit(0);
        });

        for (i, repo) in repos.iter().enumerate() {
            println!("#{}", i + 1);
            println!("repo name:{}", repo.repo_name);
            if subargs.verbose {
                println!("added time:{}", repo.added_time);
                println!("if tracked:{}", if_tracked(&repo.repo_name, toml_conf));
            }
        }

        std::process::exit(0);
    }

    // show tracked repos
    // even if the user does not provide the --tracked, eg. it is default
    let repos = toml_conf.repos.as_ref().unwrap_or_else(|| {
        println!("you have not tracked any directory");
        println!("use `gsync track <path/to/directory> to track one");
        std::process::exit(0);
    });

    // print repos
    // TODO: use a function to do the following print.
    for (i, repo) in repos.iter().enumerate() {
        if !repo.if_activated {
            continue;
        }

        println!("# {}:", i + 1);

        println!("repo name: {}", repo.repo_name);
        println!("directory name: {}", repo.local_path);
        if subargs.verbose {
            println!("sync_type: {}", repo.sync_type);
            if repo.sync_type == "on_minutes" {
                println!("sync_freq:{}", repo.sync_freq);
            }
            println!("if owner: {}", repo.if_owner);
            println!("added time: {}", repo.added_time);
            println!("synced_time: {}", repo.synced_time);
        }
        println!("");
    }
}

fn show_locals() {}

fn add_repos() {}

fn parse_repos() {}
