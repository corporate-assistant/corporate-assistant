pub mod actions {
    use crate::config::configuration;
    use corporate_assistant::interpreter::CorporateAction;
    pub use github_crawler::{get_contributions, parse_config, Conf, Contrib, RepoContribs};
    pub use mailer::{Client, Email};
    use serde::Deserialize;
    use std::io::prelude::*;
    use toml;

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
                config_file: configuration::CAConfig::new().get_repos_config(),
                behind_proxy: true,
            };

            let config_file = conf.config_file.clone();
            let config = parse_config(config_file);
            let contribs = get_contributions(conf, config);
            //            MSR::print_text(&contribs);
            MSR::send_msr_email(&contribs);
        }
    }

    pub struct MSR {
        time_frame: u8,
    }

    #[derive(Deserialize, Debug)]
    struct EmailConfig {
        login: String,
        password: String,
        server: String,
        port: u16,
        from: String,
        to: String,
    }

    #[derive(Deserialize)]
    struct EmailConfigOpt {
        email: Option<EmailConfig>,
    }

    impl MSR {
        pub fn new(time_frame: u8) -> Self {
            MSR {
                time_frame: time_frame,
            }
        }

        fn compose_contrib_text(repo_contribs: &RepoContribs) -> String {
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

            text
        }

        fn print_text(repo_contribs: &RepoContribs) -> () {
            let text = MSR::compose_contrib_text(repo_contribs);
            println!("{}", text);
            ()
        }

        fn parse_config_file(email_config_file: String) -> EmailConfig {
            let path = std::path::PathBuf::from(email_config_file);
            let file = std::fs::File::open(path);
            let mut reader = std::io::BufReader::new(file.expect("Cannot open file"));

            let mut c: String = "".to_string();
            reader.read_to_string(&mut c);

            println!("{}", c);
            let email_config: EmailConfigOpt = toml::from_str(&c).unwrap();

            email_config.email.unwrap()
        }

        fn send_email(config: &EmailConfig, subject_line: &String, text: &String) {
            let client = Client::new(&config.login, &config.password, &config.server, config.port);

            let email = Email::new(&config.from, &config.to, &subject_line, &text);

            email.send(&client);
        }

        fn send_msr_email(repo_contribs: &RepoContribs) -> () {
            use chrono::Datelike;

            let email_config = Self::parse_config_file("C:\\Users\\tpatejko\\projects\\corporate-assistant\\corporate-assistant\\corporate-assistant\\email_client.toml".to_string());

            let month = chrono::Utc::now().date().month();
            let year = chrono::Utc::now().date().year();

            let subject_line =
                "Test MSR ".to_owned() + &month.to_string() + " " + &year.to_string();
            let text = MSR::compose_contrib_text(repo_contribs);

            MSR::send_email(&email_config, &subject_line, &text);
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use std::rc::Rc;
        #[test]
        fn test_register() -> Result<(), String> {
            let mut intents = corporate_assistant::interpreter::Intents::new();
            intents
                .register_action(
                    vec![
                        "compose my monthly status report".to_string(),
                        "compose monthly status report".to_string(),
                        "create my monthly status report".to_string(),
                        "create monthly status report".to_string(),
                    ],
                    Rc::new(MSR::new(4)),
                )
                .expect("Registration failed");
            // Get registered action
            let action = intents.get_action("compose my monthly status report");
            match action {
                Ok(action) => Ok(()),
                Err(action) => Err(String::from("Error getting an action")),
            }
        }

        #[test]
        fn test_parse_empty_email_config() {
            let path = "C:\\Users\\tpatejko\\projects\\corporate-assistant\\corporate-assistant\\corporate-assistant\\email_client_empty.toml";
            let email_config = MSR::parse_config_file(path.to_string());
            assert_eq!(email_config.login, "");
            assert_eq!(email_config.port, 0);
        }

        #[test]
        fn test_parse_email_config() {
            let path = "C:\\Users\\tpatejko\\projects\\corporate-assistant\\corporate-assistant\\corporate-assistant\\email_client.toml";
            let email_config = MSR::parse_config_file(path.to_string());
            assert_eq!(email_config.login, "tpatejko");
            assert_eq!(email_config.port, 587);
            assert_eq!(email_config.server, "smtpauth.intel.com");
        }

        #[test]
        fn test_send_email() {
            let email_config = EmailConfig {
                login: "tpatejko".to_string(),
                password: "l4Hm:Ng)9".to_string(),
                server: "smtpauth.intel.com".to_string(),
                port: 587,
                from: "tomasz.patejko@gmail.com".to_string(),
                to: "tomasz.patejko@intel.com".to_string(),
            };

            MSR::send_email(&email_config, &"Test".to_string(), &"Content".to_string());
        }
    }
}
