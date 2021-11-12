pub mod jira {
    use corporate_assistant::interpreter::CorporateAction;
    use fltk::{
        app, button::Button, frame::Frame, input::SecretInput, menu::Choice, prelude::*,
        text::TextBuffer, text::TextEditor, window::Window,
    };
    use futures::executor::block_on;
    use serde::{Deserialize, Serialize};
    use std::collections::HashMap;

    impl CorporateAction for JIRA {
        fn run(&self, tts: &mut tts::TTS) -> () {
            let feedback = "Creating JIRA issue. Please type your password and edit JIRA issue ";
            tts.speak(feedback, true).expect("Problem with utterance");

            if let Some((pass, title, desc, epic)) = self.get_jira_input() {
                tts.speak("Input send to JIRA", true)
                    .expect("Problem with utterance");
                block_on(self.submit(tts, &self.user, &pass, &title, &desc, epic));
            } else {
                tts.speak("Invalid password", true)
                    .expect("Invalid password");
            }
        }
    }

    #[derive(Debug, Deserialize, Serialize)]
    struct BoardSpec {
        id: u32,
        #[serde(rename = "self")]
        self_: String,
        name: String,
        #[serde(rename = "type")]
        type_: String,
    }

    #[derive(Debug, Deserialize, Serialize)]
    struct CustomFieldDesc {
        id: String,
        name: String,
    }

    #[derive(Debug, Deserialize, Serialize)]
    struct SprintSpec {
        id: u32,
        #[serde(rename = "self")]
        self_: String,
        state: String,
        name: String,
        #[serde(rename = "startDate")]
        start_date: String,
        #[serde(rename = "endDate")]
        end_date: String,
        #[serde(rename = "originBoardId")]
        origin_board_id: u32,
    }

    #[derive(Debug, Deserialize, Serialize)]
    struct JIRAResponse<T> {
        #[serde(rename = "maxResults")]
        max_results: u32,
        #[serde(rename = "startAt")]
        start_at: u32,
        #[serde(rename = "isLast")]
        is_last: bool,
        values: Vec<T>,
    }

    #[derive(Debug, Deserialize, Serialize)]
    struct JIRATaskSubmitted {
        id: String,
        key: String,
        #[serde(rename = "self")]
        self_: String,
    }

    // TODO: How to unify WithinEpic and without epic?

    #[derive(Debug, Deserialize, Serialize)]
    struct JIRATaskWithinEpicProjectDesc {
        project: HashMap<String, String>,
        summary: String,
        description: String,
        issuetype: HashMap<String, String>,
        customfield_11900: String, // TODO(jczaja): Make it generic based name
    }

    impl JIRATaskWithinEpicProjectDesc {
        fn new(project: &str, title: &str, desc: &str, epic: &str) -> Self {
            let mut my_issuetype = HashMap::new();
            my_issuetype.insert("name".to_string(), "Task".to_string());
            let mut my_project = HashMap::new();
            my_project.insert("key".to_string(), project.to_string());
            JIRATaskWithinEpicProjectDesc {
                project: my_project,
                summary: title.to_string(),
                description: desc.to_string(),
                issuetype: my_issuetype,
                customfield_11900: epic.to_string(),
            }
        }
    }

    #[derive(Debug, Deserialize, Serialize)]
    struct JIRATaskProjectDesc {
        project: HashMap<String, String>,
        summary: String,
        description: String,
        issuetype: HashMap<String, String>,
    }

    impl JIRATaskProjectDesc {
        fn new(project: &str, title: &str, desc: &str) -> Self {
            let mut my_issuetype = HashMap::new();
            my_issuetype.insert("name".to_string(), "Task".to_string());
            let mut my_project = HashMap::new();
            my_project.insert("key".to_string(), project.to_string());
            JIRATaskProjectDesc {
                project: my_project,
                summary: title.to_string(),
                description: desc.to_string(),
                issuetype: my_issuetype,
            }
        }
    }

    struct Sprint {
        name: String,
        url: String,
    }

    pub struct JIRA {
        user: String,
        jira_url: String,
        proxy: Option<Vec<String>>,
        project: String,
    }

    impl JIRA {
        pub fn new(
            user: String,
            jira_url: String,
            proxy: Option<Vec<String>>,
            project: String,
        ) -> Self {
            JIRA {
                user: user,
                jira_url: jira_url,
                proxy: proxy,
                project: project,
            }
        }

        // Get password, issue title and desc from user
        fn get_jira_input(&self) -> Option<(String, String, String, Option<String>)> {
            let app = app::App::default();
            let mut wind = Window::default()
                .with_size(480, 640)
                .center_screen()
                .with_label("JIRA helper");
            let vpack = fltk::group::Pack::default()
                .with_size(400, 600)
                .center_of(&wind);
            let _frame = Frame::default().with_size(0, 50).with_label("Issue Title:");

            let mut tb = TextBuffer::default();
            tb.set_text("<Title of Issue>");
            let mut te = TextEditor::default().with_size(0, 50);
            te.set_buffer(Some(tb));
            te.set_insert_mode(true);

            // Make a dropdown list with epics to link to
            let _frame = Frame::default()
                .with_size(0, 50)
                .with_label("Epics to link to:");
            let epics = vec![
                "PADDLEQ-1575",
                "PADDLEQ-1249",
                "PADDLEQ-1259",
                "PADDLEQ-1254",
            ]; // TODO(do something with it)

            let mut epics_list = Choice::new(0, 0, 0, 30, "");
            epics_list.add_choice(epics[0]); //fp32
            epics_list.add_choice(epics[1]); //bf16
            epics_list.add_choice(epics[2]); //int8
            epics_list.add_choice(epics[3]); //validation

            // Issue description
            let _frame = Frame::default()
                .with_size(0, 50)
                .with_label("Issue Description:");

            let mut db = TextBuffer::default();
            db.set_text("<Description of Issue>");
            let mut de = TextEditor::default().with_size(0, 200);
            de.set_buffer(Some(db));
            de.set_insert_mode(true);
            // JIRA passoword
            let _frame = Frame::default()
                .with_size(0, 50)
                .with_label("JIRA password:");
            let si = SecretInput::default().with_size(0, 50);
            let mut approve_but = Button::default().with_size(0, 50).with_label("Submit");
            vpack.end();
            wind.end();
            wind.show();
            let (s, r) = app::channel::<String>();
            approve_but.emit(s.clone(), String::from("done"));
            wind.emit(s.clone(), String::from("exit"));

            while app.wait() {
                let msg = r.recv();
                match &msg {
                    Some(msg) => {
                        if (msg == "done") && (si.value().len() > 0) {
                            return Some((
                                si.value(),
                                te.buffer().unwrap().text(),
                                de.buffer().unwrap().text(),
                                match epics_list.value() {
                                    -1 => None,
                                    _ => {
                                        println!("chosen IDX: {}", epics_list.value());
                                        Some(epics[epics_list.value() as usize].to_string())
                                    }
                                },
                            ));
                        } else if msg == "exit" {
                            return None;
                        }
                    }
                    _ => {}
                }
            }

            app.run().unwrap();
            None
        }

        async fn submit(
            &self,
            tts: &mut tts::TTS,
            login: &str,
            pass: &str,
            title: &str,
            desc: &str,
            epic: Option<String>,
        ) {
            // If there is proxy then pick first URL
            let client = match &self.proxy {
                Some(org_proxies) => reqwest::Client::builder()
                    .proxy(reqwest::Proxy::http(&org_proxies[0]).expect("Error setting HTTP proxy"))
                    .proxy(
                        reqwest::Proxy::https(&org_proxies[0]).expect("Error setting HTTPS proxy"),
                    )
                    .build()
                    .expect("Could not create REST API client"),
                None => reqwest::Client::builder()
                    .build()
                    .expect("Could not create REST API client"),
            };

            // Get Current Sprint if any
            let curr_sprint = self.fetch_sprint(&client, login, pass);

            // Get Epic Link custom field
            let epic_custom_link = self.get_epics_custom_link(&client, login, pass);

            // Send an issue to JIRA
            self.submit_issue(
                tts,
                &client,
                login,
                pass,
                epic_custom_link,
                curr_sprint,
                title,
                desc,
                epic,
            );
        }

        fn get_epics_custom_link(
            &self,
            client: &reqwest::Client,
            login: &str,
            pass: &str,
        ) -> Option<String> {
            // Get custom_field  id for "Epic Link"
            let body = client
                .get(&(self.jira_url.clone() + "/rest/api/2/field"))
                .basic_auth(&login, Some(&pass)) // Get password
                .send();
            let mut actual_body = body.expect("GET to get JIRA board failed");
            if actual_body.status().is_success() == false {
                eprintln!("Error getting Response from JIRA");
                panic!();
            }

            // println!("Fields: {}", actual_body.text().unwrap());
            let custom_fields = actual_body
                .json::<Vec<CustomFieldDesc>>()
                .expect("Error converting response to JSON");
            //println!("fields = {:#?}", custom_fields);

            let mut epic_custom_link_iter = custom_fields.iter().filter(|x| x.name == "Epic Link");

            //println!("epic_custom_link = {:#?}", epic_custom_link_iter.next());
            let epic_custom_link = epic_custom_link_iter.next();
            match epic_custom_link {
                Some(i) => Some(i.id.clone()),
                None => None,
            }
        }

        fn fetch_sprint(
            &self,
            client: &reqwest::Client,
            login: &str,
            pass: &str,
        ) -> Option<Sprint> {
            let body = client
                .get(
                    &(self.jira_url.clone()
                        + "/rest/agile/1.0/board?projectKeyOrId="
                        + &self.project),
                )
                .basic_auth(&login, Some(&pass)) // Get password
                .send();
            let mut actual_body = body.expect("GET to get JIRA board failed");
            if actual_body.status().is_success() == false {
                eprintln!("Error getting Response from JIRA");
                panic!();
            }

            // This to lib or rest_api
            let sprints: JIRAResponse<SprintSpec>;
            let boards = actual_body
                .json::<JIRAResponse<BoardSpec>>()
                .expect("Error converting response to JSON");
            println!("body = {:#?}", boards);
            // Pick first returned value as most recent board
            if boards.values.len() > 0 {
                //result=`curl -u jczaja:$pass -H Content-Type: application/json -X GET $board_url/sprint?state=active`
                let body = client
                    .get(&((&boards.values[0].self_).to_string() + "/sprint?state=active"))
                    .basic_auth(&login, Some(&pass)) // Get password
                    .send();
                let mut actual_body = body.expect("GET to get JIRA current sprint failed");
                if actual_body.status().is_success() {
                    sprints = actual_body
                        .json::<JIRAResponse<SprintSpec>>()
                        .expect("Error converting response to JSON");
                    println!("body of sprints = {:#?}", sprints);
                    Some(Sprint {
                        name: sprints.values[0].name.clone(),
                        url: sprints.values[0].self_.clone(),
                    })
                } else {
                    eprintln!("Error getting Response from JIRA for current sprint getting");
                    None
                }
            } else {
                None
            }
        }

        fn submit_issue(
            &self,
            tts: &mut tts::TTS,
            client: &reqwest::Client,
            login: &str,
            pass: &str,
            epic_custom_link: Option<String>, // TODO: Use it somewhere
            curr_sprint: Option<Sprint>,
            title: &str,
            desc: &str,
            epic: Option<String>,
        ) {
            // Here is submitting task to JIRA
            let res = match epic {
                None => {
                    let mut my_jira_task = HashMap::new();
                    let issue_to_submit = JIRATaskProjectDesc::new(&self.project, title, desc);
                    my_jira_task.insert("fields".to_string(), issue_to_submit);
                    let res = client
                        .post(&(self.jira_url.clone() + "/rest/api/2/issue/"))
                        .basic_auth(&login, Some(&pass))
                        .json(&my_jira_task)
                        .send();
                    res
                }
                Some(epic_name) => {
                    let mut my_jira_task = HashMap::new();
                    let issue_to_submit =
                        JIRATaskWithinEpicProjectDesc::new(&self.project, title, desc, &epic_name);
                    my_jira_task.insert("fields".to_string(), issue_to_submit);
                    let res = client
                        .post(&(self.jira_url.clone() + "/rest/api/2/issue/"))
                        .basic_auth(&login, Some(&pass))
                        .json(&my_jira_task)
                        .send();
                    res
                }
            };

            let mut actual_response = res.expect("Error sending JIRA request");
            if actual_response.status().is_success() {
                let added_task = actual_response
                    .json::<JIRATaskSubmitted>()
                    .expect("Error converting response to JSON");
                println!("JIRA task submitted: {:#?}", added_task);
                // Send added JIRA task to sprint
                match curr_sprint {
                    Some(sprint) => {
                        let mut sprint_issues = HashMap::new();
                        sprint_issues.insert("issues".to_string(), vec![added_task.key]);
                        let res = client
                            .post(&(String::from(sprint.url) + "/issue"))
                            .basic_auth(login, Some(pass))
                            .json(&sprint_issues)
                            .send();
                        let mut actual_response =
                            res.expect("Error sending JIRA request to add task to Sprint");
                        if actual_response.status().is_success() {
                            let feedback = format!(
                                "{} submitted to Sprint {} in JIRA",
                                sprint_issues["issues"][0], sprint.name
                            );
                            tts.speak(feedback, true).expect("Problem with utterance");
                        } else {
                            eprintln!(
                                "Error {} adding {} to Sprint {}.",
                                actual_response.status(),
                                sprint_issues["issues"][0],
                                sprint.name
                            );
                            let error_body = actual_response
                                .text()
                                .expect("Error converting response to Text");
                            eprintln!("error_body = {:#?}", error_body);
                        }
                    }
                    None => {
                        println!("Not adding to sprint")
                    }
                }
            } else {
                eprintln!(
                    "Error submitting JIRA issue. Error: {:?}",
                    actual_response.status()
                );
                let error_body = actual_response
                    .text()
                    .expect("Error converting response to Text");
                eprintln!("error_body = {:#?}", error_body);
                todo!();
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::config::configuration;
        use github_crawler::parse_config;
        use tts::*;

        #[test]
        #[ignore]
        fn test_jira() -> Result<(), String> {
            let mut tts = TTS::default().expect("Problem starting TTS engine");
            let organization_config_file =
                configuration::CAConfig::new().get_organization_config("itp.toml");
            let org_info = configuration::parse_organization_config(&organization_config_file);
            let config_file = configuration::CAConfig::new().get_repos_config("paddle.toml");
            let (_, jira_config) = parse_config(config_file);
            let actual_config = jira_config.unwrap();
            let jira = JIRA::new(
                actual_config.user,
                actual_config.url,
                org_info.proxy,
                actual_config.project,
            )
            .run(&mut tts);
            Ok(())
        }
    }
}
