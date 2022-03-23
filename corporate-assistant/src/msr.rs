pub mod actions {
    use crate::configuration;
    use chrono::Datelike;
    use configuration::EmailConfig;
    use corporate_assistant::interpreter::CorporateAction;
    use err_handling::ResultExt;
    use fltk::{app, button::Button, input::Input, input::SecretInput, prelude::*, window::Window};
    pub use github_crawler::{get_contributions, parse_config, Conf, Contrib, RepoContribs};
    pub use mailer::{Client, Email};
    use num_traits::FromPrimitive;
    use serde::Deserialize;
    use std::io::prelude::*;
    use toml;

    #[derive(Clone)]
    enum LoginResult {
        Done,
        Exit,
    }

    fn render_login_window() -> Option<(String, String)> {
        let app = app::App::default();

        let input_size_x = 300;
        let input_size_y = 25;

        let button_size_x = 100;
        let button_size_y = 50;

        let internal_spacing = 10;
        let external_spacing_y = 5;

        let external_spacing_x = 100;
        let wind_size_x = external_spacing_x + input_size_x + external_spacing_x;

        let wind_size_y = external_spacing_y
            + input_size_y
            + internal_spacing
            + input_size_y
            + internal_spacing
            + button_size_y
            + external_spacing_y;

        let mut wind = Window::default()
            .with_size(wind_size_x, wind_size_y)
            .center_screen()
            .with_label("Email credentials");

        let login_input_pos_x = external_spacing_x;
        let login_input_pos_y = external_spacing_y;
        let login_input = Input::default()
            .with_pos(login_input_pos_x, login_input_pos_y)
            .with_size(input_size_x, input_size_y)
            .with_label("Login");

        let password_input_pos_x = login_input_pos_x;
        let password_input_pos_y = login_input_pos_y + input_size_y + internal_spacing;
        let password_input = SecretInput::default()
            .with_pos(password_input_pos_x, password_input_pos_y)
            .with_size(input_size_x, input_size_y)
            .with_label("Password");

        let submit_but_pos_x = (wind_size_x - button_size_x) / 2;
        let submit_but_pos_y = password_input_pos_y + input_size_y + internal_spacing;
        let mut submit_but = Button::default()
            .with_pos(submit_but_pos_x, submit_but_pos_y)
            .with_size(button_size_x, button_size_y)
            .with_label("Submit");

        wind.end();
        wind.show();

        let (s, r) = app::channel::<LoginResult>();
        submit_but.emit(s.clone(), LoginResult::Done);
        wind.emit(s.clone(), LoginResult::Exit);

        while app.wait() {
            let msg = r.recv();

            match &msg {
                Some(msg) => match msg {
                    LoginResult::Done => {
                        return Some((login_input.value(), password_input.value()));
                    }
                    LoginResult::Exit => {
                        return None;
                    }
                },
                _ => {}
            }
        }

        app.run().unwrap();
        None
    }

    impl CorporateAction for MSR {
        fn run(&self, tts: &mut tts::TTS) -> () {
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
                config_file: configuration::CAConfig::new()
                    .get_repos_config(&self.project_config_file),
                proxies: self.proxies.clone(),
            };

            let config_file = conf.config_file.clone();
            let (config, _) = parse_config(config_file);
            println!("{:?}", config);
            let contribs = get_contributions(conf, config.unwrap());
            MSR::send_msr_email(&contribs, &self.server, self.port, &self.from, &self.to);
        }
    }

    pub struct MSR {
        proxies: Option<Vec<String>>,
        project_config_file: String,
        time_frame: u8,
        server: String,
        port: u16,
        from: String,
        to: String,
    }

    impl MSR {
        pub fn new(
            proxies: &Option<Vec<String>>,
            project_config_file: &str,
            time_frame: u8,
            email_config: &EmailConfig,
        ) -> Self {
            MSR {
                proxies: proxies.clone(),
                project_config_file: project_config_file.to_string(),
                time_frame: time_frame,
                server: email_config.server.clone(),
                port: email_config.port,
                from: email_config.from.clone(),
                to: email_config.to.clone(),
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

        fn print_text(repo_contribs: &RepoContribs) {
            let text = MSR::compose_contrib_text(repo_contribs);
            println!("{}", text);
            ()
        }

        fn send_email(
            server: &str,
            port: u16,
            subject_line: &str,
            text: &str,
            from: &str,
            to: &str,
        ) {
            if let Some(r) = render_login_window() {
                let (login, password) = r;
                let client = Client::new(&login, &password, &server, port);

                let email = Email::new(&from, &to, &subject_line, &text);

                email.send(&client);
            } else {
                ()
            }
        }

        fn send_msr_email(
            repo_contribs: &RepoContribs,
            server: &str,
            port: u16,
            from: &str,
            to: &str,
        ) -> () {
            let month = chrono::Utc::now().date().month();
            let year = chrono::Utc::now().date().year();

            let subject_line = format!(
                "Draft of MSR {} {}",
                chrono::Month::from_u32(month).unwrap().name(),
                year
            );

            let text = MSR::compose_contrib_text(repo_contribs);

            MSR::send_email(&server, port, &subject_line, &text, &from, &to);
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::nlu::*;
        use std::rc::Rc;
        use tts::*;

        #[test]
        fn test_register() -> Result<(), String> {
            let mut intents = corporate_assistant::interpreter::Intents::new();
            let dummy_config = EmailConfig {
                server: String::new(),
                port: 0,
                from: String::new(),
                to: String::new(),
            };

            intents
                .register_action(
                    nlu::normalize_phrase("create my monthly status report"),
                    Rc::new(MSR::new(&None, "dummy.toml", 4, &dummy_config)),
                )
                .expect("Registration failed");
            // Get registered action
            let action =
                intents.get_action(&nlu::normalize_phrase("create my monthly status report"));
            match action {
                Ok(action) => Ok(()),
                Err(action) => Err(String::from("Error getting an action")),
            }
        }

        #[test]
        #[ignore]
        fn test_msr() -> Result<(), String> {
            let mut tts = TTS::default().expect("Problem starting TTS engine");
            let dummy_config = EmailConfig {
                server: String::new(),
                port: 0,
                from: String::new(),
                to: String::new(),
            };

            let msr = MSR::new(&None, "paddle.toml", 4, &dummy_config);
            msr.run(&mut tts);
            Ok(())
        }
    }
}
