use anyhow::Result;
use dialoguer::{theme::ColorfulTheme, Confirm};
use log::debug;
use rpassword::read_password_from_tty;
use std::{io::Write, process::exit};
use structopt::StructOpt;

mod github;
use github::{GitHub, Repository};

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

/// Prompts the user for whether they want to archive the repo.
fn prompt_to_archive(github: &GitHub, repo: &Repository) -> Result<()> {
    let theme = ColorfulTheme::default();
    let mut prompt = Confirm::with_theme(&theme);
    prompt.with_prompt(&format!(
        "Do you want to archive {}? Last update: {}",
        repo.name, repo.pushed_at
    ));
    prompt.default(false);
    if !prompt.interact().unwrap() {
        return Ok(());
    }
    github.archive_repo(&repo.name)?;
    Ok(())
}

/// Entry point.
fn main() {
    let options = Options::from_args();

    env_logger::Builder::new()
        .format(|f, record| writeln!(f, "{}", record.args()))
        .filter_level(if options.debug {
            log::LevelFilter::Debug
        } else {
            log::LevelFilter::Info
        })
        .init();

    debug!("Determining token");
    let token = match options.token {
        Some(t) => t,
        None => prompt_for_token(),
    };

    debug!("Setting up GitHub struct");
    let github = match GitHub::new(&token) {
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

    debug!("Prompting for archiving");
    for repo in repos {
        if let Err(e) = prompt_to_archive(&github, &repo) {
            eprintln!("Error: {}", e);
        };
    }
}
