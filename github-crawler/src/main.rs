use chrono;
use prettytable::*;
use std::env;
use std::str::FromStr;

pub use github_crawler::{get_contributions, parse_config, Conf, Contrib, RepoContribs};

type DateTime = String;

fn parse_date(s: &str) -> Option<chrono::DateTime<chrono::Utc>> {
    let values: Vec<_> = s.split(".").collect();

    if values.len() == 3 {
        let d = values[0].parse().unwrap();
        let m = values[1].parse().unwrap();
        let y = values[2].parse().unwrap();

        Some(chrono::DateTime::<chrono::Utc>::from_utc(
            chrono::NaiveDate::from_ymd(y, m, d).and_hms(0, 0, 0),
            chrono::Utc,
        ))
    } else {
        None
    }
}

fn parse_cmdline() -> Conf {
    let mut from_date: Option<chrono::DateTime<chrono::Utc>> = None;
    let mut to_date: Option<chrono::DateTime<chrono::Utc>> = None;
    let mut config_file: Option<String> = None;

    let mut behind_proxy = false;

    for arg in env::args() {
        if arg.starts_with("--") {
            println!("{:?}", arg);
            let trimmed = arg.trim_start_matches("--");
            let split: Vec<_> = trimmed.split("=").collect();
            let key = split[0];

            match key {
                "config-file" => config_file = Some(String::from(split[1])),
                "behind-proxy" => behind_proxy = true,
                "from" => from_date = parse_date(split[1]),
                "to" => to_date = parse_date(split[1]),
                _ => (),
            }
        }
    }

    Conf {
        from_date: from_date.expect("No starting date"),
        to_date: to_date.expect("No ending date"),
        config_file: config_file.expect("No configuration file"),
        behind_proxy: behind_proxy,
    }
}

fn print_table(repo_contribs: &RepoContribs) -> () {
    let mut pr_table = prettytable::Table::new();
    pr_table.add_row(row!(b => "repo", "number", "title", "merging date"));

    let repos = repo_contribs.keys();

    for repo in repos {
        for c in repo_contribs.get(repo).unwrap() {
            pr_table.add_row(row!(repo, c.id, c.title, c.merge_date));
        }
    }

    pr_table.printstd();
}

fn print_text(repo_contribs: &RepoContribs) -> () {
    let repos = repo_contribs.keys();

    let mut text = "".to_string();
    for repo in repos {
        text += &("* ".to_owned() + &repo + "\n");

        for c in repo_contribs.get(repo).unwrap() {
            let line = "\t - ".to_owned()
                + &c.merge_date.format("%Y-%m-%d").to_string()
                + ": "
                + &c.id.to_string()
                + ", "
                + &c.title
                + "\n";
            text += &line;
        }
    }

    println!("{}", text);
    ()
}

fn main() -> Result<(), anyhow::Error> {
    let conf = parse_cmdline();
    let config_file = conf.config_file.clone();
    let config = parse_config(config_file);

    let contribs = get_contributions(conf, config);

    print_table(&contribs);
    print_text(&contribs);

    Ok(())
}
