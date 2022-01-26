pub mod skm {
    use corporate_assistant::interpreter::CorporateAction;
    use err_handling::ResultExt;
    use fltk::{
        app, button::Button, frame::Frame, input::SecretInput, menu::Choice, prelude::*,
        text::TextBuffer, text::TextEditor, window::Window,
    };
    use regex::Regex;
    use futures::executor::block_on;
    use serde::{Deserialize, Serialize};
    use std::collections::HashMap;

    impl CorporateAction for SKM {
        fn run(&self, tts: &mut tts::TTS) -> () {
            block_on(self.submit(tts));
        }
    }

    pub struct SKM {
        skm_url: String,
        proxy: Option<Vec<String>>,
        from: String,
        to: String,
    }

    impl SKM {
        pub fn new(
            skm_url: String,
            proxy: Option<Vec<String>>,
            from: String,
            to: String,
        ) -> Self {
            SKM {
                skm_url: skm_url,
                proxy: proxy,
                from: from,
                to: to,
            }
        }

        async fn get_station_id(body : &str, station : &str) -> String {
            // Replace white characters with commas
            let re = Regex::new(r"\s+").unwrap();
            let t = re.replace_all(station, ",").to_lowercase();

            // We connstruct search pattern. for example:
            // "data-keywords="gdansk,wrzeszcz" value=\""
            let search_phrase = "data-keywords=\"".to_string()+&t+"\" value=";//+
            // SO I need to extract: "<number>" and the parse <number> to get value
            "".to_string()
        }

        async fn submit(
            &self,
            tts: &mut tts::TTS,
        ) {
            // If there is proxy then pick first URL
            let client = match &self.proxy {
                Some(org_proxies) => reqwest::Client::builder()
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
                None => reqwest::Client::builder()
                    .build()
                    .expect_and_log("Could not create HTTP(SKM) client"),
            };

            // Get IDs of stations e.g. Gdansk Wrzeszcz : 7534
            let res = client
                .get(&(self.skm_url.clone()))
                .send()
                .expect_and_log("Error sending SKM request")
                .text();

            let mut actual_response = res.expect_and_log("Error: unwrapping SKM response");
                println!("actual response: {:#?}", actual_response);
            let from = self.get_station_id(actual_response,"gdansk wrzeszcz");
            let to = self.get_station_id(actual_response,"gdansk port lotniczy");
            // Get Data 
            

            // Send a request to SKM web page
        }

    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::config::configuration;
        use tts::*;

        #[test]
        fn test_skm() -> Result<(), String> {
            let mut tts = TTS::default().expect("Problem starting TTS engine");
            let organization_config_file =
                configuration::CAConfig::new().get_organization_config("itp.toml");
            let org_info = configuration::parse_organization_config(&organization_config_file);

            // TODO: Polish characters support
            let skm = SKM::new(
                "https://skm.trojmiasto.pl/".to_string(),
                org_info.proxy,
                "Gdansk Wrzeszcz".to_string(),
                "Gdansk Port Lotniczy".to_string(),
            )
            .run(&mut tts);
            Ok(())
        }
    }
}
