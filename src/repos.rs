use crate::Subcommands;
use crate::{Config, Repo, ServerConfig, ServerRepo, Status};
use crate::{DaemonArgs, ListArgs, ServerArgs, TrackArgs, UntrackArgs};

use crate::CONFIG_NAME;

use chrono::Local;
use std::fs;
use std::path::Path;

fn if_tracked(repo_name: &str, toml_conf: &Config) -> bool {
    let repos = &toml_conf.repos;
    repos.iter().any(|repo| repo.repo_name.eq(repo_name))
}

fn if_path_trcked(path: &str, toml_conf: &Config) -> bool {
    let repos = &toml_conf.repos;
    repos.iter().any(|repo| repo.local_path.eq(path))
}

// fn get_conf_path() -> std::path::Path {}

pub fn show_repos(subargs: &ListArgs, toml_conf: &Config, server_toml_conf: &ServerConfig) {
    //show the available repos in the server
    if subargs.online {
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
    if toml_conf.repos.is_empty() {
        println!("you have not tracked any directory");
        println!("use `gsync track <path/to/directory> to track one");
        std::process::exit(0);
    }

    // print repos
    // TODO: use a function to do the following print.
    for (i, repo) in toml_conf.repos.iter().enumerate() {
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

fn show_locals(subargs: &ListArgs, toml_conf: &Config, server_toml_conf: &ServerConfig) {}

pub fn track_repos(
    subargs: &TrackArgs,
    toml_conf: &mut Config,
    server_toml_conf: &mut ServerConfig,
) {
    // check if the path is a valid directory
    let full_path = std::path::Path::new(&subargs.path);
    if !full_path.is_dir() {
        println!("we can not locate the directory {}", &subargs.path);
        std::process::exit(0);
    }

    // check if the path is already in track
    if if_path_trcked(&subargs.path, toml_conf) {
        println!("the path {} is already in track", &subargs.path);
        std::process::exit(0);
    }

    let repo_name = full_path.file_name().unwrap().to_str().unwrap();
    let new_repo = Repo {
        repo_name: repo_name.to_string(),
        local_path: full_path.to_str().unwrap().to_string(),
        sync_type: subargs
            .sync_type
            .as_ref()
            .unwrap_or(&"on_change".to_string())
            .clone(),
        sync_freq: subargs.minutes,
        if_owner: true,
        if_activated: true,
        added_time: Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
        synced_time: Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
    };

    // add this repo to the vector
    let repo_list = &mut toml_conf.repos;
    repo_list.push(new_repo);

    // write to file
    let toml_str = toml::to_string(&toml_conf).unwrap();
    let toml_path = dirs::home_dir()
        .unwrap()
        .join(CONFIG_NAME)
        .as_path()
        .to_owned();
    fs::write(toml_path, toml_str).expect("write the toml file error");
}

fn parse_repos() {}
