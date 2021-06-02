pub mod actions {
    use corporate_assistant::interpreter::CorporateAction;
    pub use github_crawler::{get_contributions, parse_config, Conf, Contrib, RepoContribs};
    use tts::*;

    impl CorporateAction for MSR {
        fn Run(&self, tts: &mut tts::TTS) -> () {
            let time_frame = match self.time_frame {
                1 => "weekly",
                4 => "monthly",
                _ => panic!("Error: Unsupported time frame value {}", self.time_frame),
            };
            let feedback = "Composing ".to_string() + time_frame + " status report";
            tts.speak(feedback, true).expect("Problem with utterance");

            // Make some test of github crawler
            let conf = Conf {
                from_date: chrono::Utc::now() - chrono::Duration::weeks(self.time_frame as i64),
                to_date: chrono::Utc::now(),
                //TODO: Make it read from HOME
                config_file: String::from(
                    "/home/jczaja/corporate-assistant/corporate-assistant/config.toml",
                ),
                behind_proxy: true,
            };

            let config_file = conf.config_file.clone();
            let config = parse_config(config_file);
            let contribs = get_contributions(conf, config);
            MSR::print_text(&contribs);
        }
    }

    pub struct MSR {
        time_frame: u8,
    }

    impl MSR {
        pub fn new(time_frame: u8) -> Self {
            MSR {
                time_frame: time_frame,
            }
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
    }
}
