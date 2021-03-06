use anyhow::{anyhow, Result};
use chrono::{DateTime, Utc};
use log::debug;
use reqwest::{blocking::Client, header};
use serde::Deserialize;
use serde_json::{json, Value};
use std::time::Duration;

static APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));
const BASE_URL: &str = "https://api.github.com/";

/// Represents a GH repo.
#[derive(Clone, Debug, Deserialize)]
pub(crate) struct Repository {
    pub(crate) name: String,
    pub(crate) full_name: String,
    pub(crate) pushed_at: DateTime<Utc>,
    pub(crate) archived: bool,
}

/// Struct to encapsulate GH interactions.
pub(crate) struct GitHub {
    client: Client,
    username: String,
}

impl GitHub {
    /// Create a new struct using the GH token.
    ///
    /// As part of the struct creation, the GH API is utilized
    /// to retrieve the username of the account with the token.
    /// If this API call fails, the token cannot be used, so an
    /// error is returned.
    pub(crate) fn new(token: &str) -> Result<Self> {
        let mut headers = header::HeaderMap::new();
        headers.insert(
            header::AUTHORIZATION,
            header::HeaderValue::from_str(&format!("token {}", token))?,
        );
        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .user_agent(APP_USER_AGENT)
            .default_headers(headers)
            .build()?;
        let username = match get_username(&client) {
            Ok(u) => u,
            Err(_) => return Err(anyhow!("Could not authenticate - likely incorrect token")),
        };
        Ok(GitHub { client, username })
    }

    /// Retrieve a list of repos from the user.
    pub(crate) fn get_repos(&self) -> Result<Vec<Repository>> {
        let resp = self
            .client
            .get(&format!("{}users/{}/repos", BASE_URL, self.username))
            .send()?;
        debug!("Repos get call response status: {}", resp.status());
        let data: Vec<Repository> = resp.json()?;
        let not_archived = data.iter().filter(|&r| !r.archived).cloned().collect();
        Ok(not_archived)
    }

    /// Archive a single repo.
    pub(crate) fn archive_repo(&self, repo_name: &str) -> Result<()> {
        debug!("Debugging {}", repo_name);
        let resp = self
            .client
            .patch(&format!(
                "{}repos/{}/{}",
                BASE_URL, self.username, repo_name
            ))
            .json(&json!({
                "archived": true,
            }))
            .send()?;
        debug!("Repo patch call response status: {}", resp.status());
        if !resp.status().is_success() {
            return Err(anyhow!(format!("Status code {}", resp.status())));
        }
        Ok(())
    }
}

/// Get the username for an authenticated HTTP client.
fn get_username(client: &Client) -> Result<String> {
    let resp = client.get(&format!("{}user", BASE_URL)).send()?;
    debug!("Get username response status: {}", resp.status());
    if !resp.status().is_success() {
        return Err(anyhow!(format!("Status code {}", resp.status())));
    }
    let data: Value = resp.json()?;
    match data["login"].as_str() {
        Some(u) => Ok(u.to_owned()),
        None => Err(anyhow!("Could not get 'login' field from response body")),
    }
}
