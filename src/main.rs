use log::debug;
use rpassword::read_password_from_tty;
use std::{env, process::exit};
use structopt::StructOpt;

mod github;

/// A CLI program for interactively archiving your own GitHub repos
#[derive(Debug, StructOpt)]
struct Options {
    #[structopt(short, long)]
    debug: bool,
    #[structopt(short, long)]
    token: Option<String>,
}

/// Prompts the user to enter a GH token.
///
/// Does not echo the token to the terminal.
fn prompt_for_token() -> String {
    let token = match read_password_from_tty(Some("GitHub token: ")) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("Error reading token: {}", e);
            exit(1);
        }
    };
    if token.is_empty() {
        eprintln!("No token supplied");
        exit(1);
    }
    token
}

/// Entry point.
fn main() {
    let options = Options::from_args();
    if env::var("RUST_LOG").is_err() {
        if options.debug {
            env::set_var("RUST_LOG", "archive_my_stuff=debug");
        } else {
            env::set_var("RUST_LOG", "archive_my_stuff=info");
        }
    }
    pretty_env_logger::init();

    debug!("Determining token");
    let token = match options.token {
        Some(t) => t,
        None => prompt_for_token(),
    };

    debug!("Setting up GitHub struct");
    let github = match github::GitHub::new(&token) {
        Ok(g) => g,
        Err(e) => {
            eprintln!("Could not access GitHub: {}", e);
            exit(1);
        }
    };

    debug!("Getting repos");
    let repos = match github.get_repos() {
        Ok(r) => r,
        Err(e) => {
            eprintln!("Could not get your repos: {}", e);
            exit(1);
        }
    };

    // TODO
}
