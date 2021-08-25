pub mod actions {
    use corporate_assistant::interpreter::CorporateAction;
    use webbrowser;

    impl CorporateAction for OpenWebsites {
        fn Run(&self, tts: &mut tts::TTS) -> () {
            tts.speak(self.feedback.clone(), true)
                .expect("Problem with utterance");
            self.urls.iter().for_each(|x| {
                webbrowser::open(x).is_ok();
                ()
            });
        }
    }

    pub struct OpenWebsites {
        urls: Vec<String>,
        feedback: String,
    }

    impl OpenWebsites {
        pub fn new(urls: Vec<String>, feedback: String) -> Self {
            OpenWebsites {
                urls: urls,
                feedback: feedback,
            }
        }
    }
}
