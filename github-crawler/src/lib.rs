use chrono;
use err_handling::ResultExt;
use graphql_client::*;
use std::collections::HashMap;
use std::env;
use std::path::PathBuf;
use std::str::FromStr;

pub mod config_parser;
pub use crate::config_parser::parse_config;

type DateTime = String;
#[derive(graphql_client::GraphQLQuery)]
#[graphql(
    schema_path = "src/github_schema.graphql",
    query_path = "src/github_prs_query.graphql",
    response_derives = "Debug"
)]

struct UserPrView;

#[derive(graphql_client::GraphQLQuery)]
#[graphql(
    schema_path = "src/github_schema.graphql",
    query_path = "src/github_prs_query.graphql",
    response_derives = "Debug"
)]

struct UserPrViewNext;

fn parse_repo_name(repo_name: &str) -> Result<(&str, &str), &str> {
    let mut parts = repo_name.split('/');
    match (parts.next(), parts.next()) {
        (Some(owner), Some(name)) => Ok((owner, name)),
        _ => Err("wrong format for the repository name param (we expect something like facebook/graphql)")
    }
}

fn behind_proxy() -> Result<bool, String> {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        Ok(false)
    } else {
        let arg1 = &args[1];
        if arg1 == &String::from("use-intel-proxy") {
            Ok(true)
        } else {
            Err(String::from(
                "Incorrect proxy argument. If you are behind Intel proxy, use use-intel-proxy.",
            ))
        }
    }
}

fn build_client(proxies: &Option<Vec<String>>, token: &str) -> reqwest::blocking::Client {
    let proxy = if let Some(proxies) = proxies {
        Some(proxies[0].clone())
    } else {
        None
    };

    reqwest::blocking::Client::builder()
        .user_agent("request")
        .default_headers(
            std::iter::once((
                reqwest::header::AUTHORIZATION,
                reqwest::header::HeaderValue::from_str(&format!("Bearer {}", token)).unwrap(),
            ))
            .collect(),
        )
        .proxy(reqwest::Proxy::custom(move |_| match &proxy {
            Some(proxy) => Some(reqwest::Url::parse(&proxy).unwrap()),
            None => None,
        }))
        .build()
        .expect("Couldn't build a client")
}

#[derive(Debug)]
pub struct Contrib {
    pub merge_date: chrono::DateTime<chrono::Utc>,
    pub id: i64,
    pub title: String,
}

pub type Contribs = Vec<Contrib>;
pub type RepoContribs = HashMap<String, Contribs>;

fn get_repo_contribs(
    response_data: &user_pr_view::ResponseData,
    repos: &Vec<String>,
    from_time: &chrono::DateTime<chrono::Utc>,
    to_time: &chrono::DateTime<chrono::Utc>,
) -> (Option<String>, RepoContribs) {
    let user = match &response_data.user {
        Some(user) => user,
        None => panic!("missing repository"),
    };

    let mut end_cursor: Option<String> = None;
    let mut repo_contribs = HashMap::new();

    for contrs_by_repo in &user
        .contributions_collection
        .pull_request_contributions_by_repository
    {
        let repo = &contrs_by_repo.repository.name;
        if repos.contains(repo) {
            if end_cursor.is_none() {
                if contrs_by_repo.contributions.page_info.has_next_page {
                    end_cursor = contrs_by_repo.contributions.page_info.end_cursor.clone();
                }
            }

            for contr in contrs_by_repo
                .contributions
                .nodes
                .as_ref()
                .expect("No nodes in contributions")
            {
                let pr = &contr.as_ref().expect("Empty contribution").pull_request;
                let state = &pr.state;

                if let user_pr_view::PullRequestState::MERGED = state {
                    let merge_date =
                        chrono::DateTime::<chrono::Utc>::from_str(pr.merged_at.as_ref().unwrap())
                            .unwrap();

                    if merge_date >= *from_time && merge_date <= *to_time {
                        repo_contribs.entry(repo.clone()).or_insert(Vec::new());
                        repo_contribs.get_mut(repo).unwrap().push(Contrib {
                            id: pr.number,
                            title: pr.title.clone(),
                            merge_date: merge_date,
                        });
                    }
                }
            }
        }
    }

    return (end_cursor, repo_contribs);
}

fn get_repo_contribs_next(
    response_data: &user_pr_view_next::ResponseData,
    repos: &Vec<String>,
    from_time: &chrono::DateTime<chrono::Utc>,
    to_time: &chrono::DateTime<chrono::Utc>,
) -> (Option<String>, RepoContribs) {
    let user = match &response_data.user {
        Some(user) => user,
        None => panic!("missing repository"),
    };

    let mut end_cursor: Option<String> = None;
    let mut repo_contribs = HashMap::new();

    for contrs_by_repo in &user
        .contributions_collection
        .pull_request_contributions_by_repository
    {
        let repo = &contrs_by_repo.repository.name;
        if repos.contains(repo) {
            if end_cursor.is_none() {
                if contrs_by_repo.contributions.page_info.has_next_page {
                    end_cursor = contrs_by_repo.contributions.page_info.end_cursor.clone();
                }
            }

            for contr in contrs_by_repo
                .contributions
                .nodes
                .as_ref()
                .expect("No nodes in contributions")
            {
                let pr = &contr.as_ref().expect("Empty contribution").pull_request;
                let state = &pr.state;

                if let user_pr_view_next::PullRequestState::MERGED = state {
                    let merge_date =
                        chrono::DateTime::<chrono::Utc>::from_str(pr.merged_at.as_ref().unwrap())
                            .unwrap();

                    if merge_date >= *from_time && merge_date <= *to_time {
                        repo_contribs.entry(repo.clone()).or_insert(Vec::new());
                        repo_contribs.get_mut(repo).unwrap().push(Contrib {
                            id: pr.number,
                            title: pr.title.clone(),
                            merge_date: merge_date,
                        });
                    }
                }
            }
        }
    }

    return (end_cursor, repo_contribs);
}

pub struct Conf {
    pub from_date: chrono::DateTime<chrono::Utc>,
    pub to_date: chrono::DateTime<chrono::Utc>,
    pub proxies: Option<Vec<String>>,
    pub config_file: PathBuf,
}

pub fn get_contributions(conf: Conf, config: config_parser::GithubConfig) -> RepoContribs {
    let github_user = &config.user;

    let github_url = &config.url;
    let github_api_token = &config.token;
    let proxies = &conf.proxies;

    let q = UserPrView::build_query(user_pr_view::Variables {
        login: github_user.to_string(),
    });

    let client = build_client(&proxies, &github_api_token);

    let mut res = client
        .post(github_url)
        .json(&q)
        .send()
        .expect_and_log("Sender error");

    let response_body: Response<user_pr_view::ResponseData> =
        res.json().expect_and_log("Response error");

    let (mut end_cursor, mut contribs) = get_repo_contribs(
        &response_body.data.expect("missing response data"),
        &config.repos,
        &conf.from_date,
        &conf.to_date,
    );

    while let Some(ec) = end_cursor {
        let q = UserPrViewNext::build_query(user_pr_view_next::Variables {
            login: github_user.to_string(),
            after_cursor: ec,
        });

        let mut res = client
            .post(github_url)
            .json(&q)
            .send()
            .expect_and_log("Yet another sending error");

        let response_body: Response<user_pr_view_next::ResponseData> =
            res.json().expect_and_log("Yet another response error");
        let (after_cursor, next_contribs) = get_repo_contribs_next(
            &response_body.data.expect("missing response data"),
            &config.repos,
            &conf.from_date,
            &conf.to_date,
        );

        end_cursor = after_cursor;

        for (k, mut v) in next_contribs {
            contribs.entry(k).or_insert(Vec::new()).append(&mut v);
        }
    }
    contribs
}
