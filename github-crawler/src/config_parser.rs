use serde::Deserialize;
use std::io::prelude::*;
use std::path::PathBuf;
use toml;

#[derive(Deserialize, Debug)]
pub struct GithubConfig {
    pub user: String,
    pub token: String,
    pub repos: Vec<String>,
    pub url: String,
}

#[derive(Deserialize, Debug)]
pub struct JIRAConfig {
    pub user: String,
    pub project: String,
    pub url: String,
}

#[derive(Deserialize)]
struct RepoConfig {
    github: Option<GithubConfig>,
    jira: Option<JIRAConfig>,
}

pub fn parse_config(path: PathBuf) -> (Option<GithubConfig>, Option<JIRAConfig>) {
    let file = std::fs::File::open(path);
    let mut reader = std::io::BufReader::new(file.expect("Cannot open file"));

    let mut c: String = "".to_string();

    reader.read_to_string(&mut c);

    let repo_config: RepoConfig = toml::from_str(&c).unwrap();

    (repo_config.github,repo_config.jira)
}
