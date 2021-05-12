use serde::Deserialize;
use std::io::prelude::*;
use toml;

#[derive(Deserialize, Debug)]
pub struct GithubConfig {
    pub user: String,
    pub token: String,
    pub repos: Vec<String>,
    pub url: String,
}

#[derive(Deserialize)]
struct RepoConfig {
    github: Option<GithubConfig>,
}

pub fn parse_config(config_file: String) -> GithubConfig {
    let path = std::path::PathBuf::from(config_file);
    let file = std::fs::File::open(path);
    let mut reader = std::io::BufReader::new(file.expect("Cannot open file"));

    let mut c: String = "".to_string();

    reader.read_to_string(&mut c);

    let repo_config: RepoConfig = toml::from_str(&c).unwrap();

    let github_config = repo_config.github.unwrap();

    github_config
}
