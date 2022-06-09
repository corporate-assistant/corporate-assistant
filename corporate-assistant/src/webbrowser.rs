pub mod actions {
    use corporate_assistant::interpreter::CorporateAction;
    use corporate_assistant::speaker::Speaker;
    use webbrowser;

    impl CorporateAction for OpenWebsites {
        fn run(&self, speaker: &mut Speaker) -> () {
            speaker
                .speak(&self.feedback.clone(), true)
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
