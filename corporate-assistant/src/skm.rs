pub mod skm {
    use chrono::prelude::*;
    use corporate_assistant::interpreter::CorporateAction;
    use corporate_assistant::speaker::Speaker;
    use err_handling::ResultExt;
    use fltk::{
        app, button::Button, frame::Frame, input::SecretInput, menu::Choice, prelude::*,
        text::TextBuffer, text::TextEditor, window::Window,
    };
    use futures::executor::block_on;
    use regex::Regex;
    use serde::{Deserialize, Serialize};
    use std::collections::HashMap;

    type ReqwestClient = reqwest::blocking::Client;

    impl CorporateAction for SKM {
        fn run(&self, speaker: &mut Speaker) -> () {
            block_on(self.submit(speaker));
        }
    }

    pub struct SKM {
        skm_url: String,
        proxy: Option<Vec<String>>,
        from_to: Vec<String>,
    }

    impl SKM {
        pub fn new(skm_url: String, proxy: Option<Vec<String>>, from_to: Vec<String>) -> Self {
            SKM {
                skm_url: skm_url,
                proxy: proxy,
                from_to: from_to,
            }
        }

        async fn get_station_id<'a>(&self, body: &'a str, station: &str) -> &'a str {
            // Replace white characters with commas
            let re = Regex::new(r"\s+").unwrap();
            let t = re.replace_all(station, ",").to_lowercase();

            // We connstruct search pattern. for example:
            // "data-keywords="gdansk,wrzeszcz" value=\""
            let search_phrase = "data-keywords=\"".to_string() + &t + "\" value=";
            let id_offset_start = body
                .find(&search_phrase)
                .expect_and_log(&format!("Pattern: {}", search_phrase))
                + search_phrase.len()
                + 1;
            let pattern_slice = &body[id_offset_start..];
            let id_offset_end = pattern_slice
                .find('"')
                .expect_and_log("Id pattern not found");
            // SO I need to extract: "<number>" and the parse <number> to get value
            &pattern_slice[0..id_offset_end]
        }

        async fn get_message(&self, body: &str, station: &str) -> String {
            // We connstruct search pattern to get remaining time. for example:
            // Najbliższa kolejka za</p>
            //<h3 class="no-print">28 min</h3>
            let search_phrase = "Najbl".to_string();
            let start_offset = body
                .find(&search_phrase)
                .expect_and_log(&format!("Pattern: {}", search_phrase));
            let pattern_slice = &body[start_offset..start_offset + 100]; // 100 characters should be enough
                                                                         // find first two "dd min"
            let re = Regex::new(r"[0-9]+\s[m][i][n]").unwrap();

            let next_train_minutes = match re.find(pattern_slice) {
                Some(hit) => hit.as_str(),
                None => panic!(),
            };

            "Next train from station ".to_string()
                + station
                + " departs in "
                + next_train_minutes
                + "utes"
        }

        async fn submit(&self, speaker: &mut Speaker) {
            // If there is proxy then pick first URL
            let client = match &self.proxy {
                Some(org_proxies) => ReqwestClient::builder()
                    .proxy(
                        reqwest::Proxy::http(&org_proxies[0])
                            .expect_and_log("Error setting HTTP proxy"),
                    )
                    .proxy(
                        reqwest::Proxy::https(&org_proxies[0])
                            .expect_and_log("Error setting HTTPS proxy"),
                    )
                    .build()
                    .expect_and_log("Could not create HTTP(SKM) client"),
                None => ReqwestClient::builder()
                    .build()
                    .expect_and_log("Could not create HTTP(SKM) client"),
            };

            // TODO: make it more async

            log::info!("SKM: sending request: {}", &self.skm_url);
            // Get IDs of stations e.g. Gdansk Wrzeszcz : 7534
            let res = client
                .get(&(self.skm_url.clone()))
                .send()
                .expect_and_log("Error sending SKM request")
                .text();

            let actual_response = res.expect_and_log("Error: unwrapping SKM response");
            let from = self.get_station_id(&actual_response, &self.from_to[0]);
            let to = self.get_station_id(&actual_response, &self.from_to[1]);
            // Get Data

            let from_id = from.await;
            let to_id = to.await;

            // Lets get current data and time
            let date = chrono::Local::now().format("%Y-%m-%d").to_string();
            let hour = chrono::Local::now().format("%H").to_string();
            let minutes = chrono::Local::now().format("%M").to_string();

            // Send a request to SKM web page
            let request = "".to_string()
                + &self.skm_url
                + "/rozklad/?from="
                + from_id
                + "&to="
                + to_id
                + "&date="
                + &date
                + "&hour="
                + &hour
                + "%3A"
                + &minutes;

            log::info!("SKM: sending request: {}", &request);

            // Get actual times for our chosen destination
            let res = client
                .get(&request)
                .send()
                .expect_and_log("Error sending SKM request")
                .text();
            let actual_response = res.expect_and_log("Error: unwrapping SKM response");
            let message = self.get_message(&actual_response, &self.from_to[0]).await;
            log::info!("SKM: uttered message: {}", &message);
            speaker
                .speak(&message, true)
                .expect("Problem with utterance");
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::config::configuration;
        use err_handling::{init_logging_infrastructure, ResultExt};
        use tts::*;

        #[test]
        fn test_skm() -> Result<(), String> {
            init_logging_infrastructure();
            let mut speaker = Speaker::none().unwrap();
            // TODO: Polish characters support
            let skm = SKM::new(
                "https://skm.trojmiasto.pl/".to_string(),
                None,
                vec![
                    "Gdansk Wrzeszcz".to_string(),
                    "Gdansk Port Lotniczy".to_string(),
                ],
            )
            .run(&mut speaker);
            Ok(())
        }
    }
}
