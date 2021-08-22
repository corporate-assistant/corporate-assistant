pub mod actions {
    use corporate_assistant::interpreter::CorporateAction;
    use webbrowser;

    impl<'a> CorporateAction for OpenWebsites<'a> {
        fn Run(&self, tts: &mut tts::TTS) -> () {
            tts.speak(self.feedback.clone(), true)
                .expect("Problem with utterance");
            self.urls.iter().for_each(|x| {
                webbrowser::open(x).is_ok();
                ()
            });
        }
    }

    pub struct OpenWebsites<'a> {
        urls: Vec<&'a str>,
        feedback: String,
    }

    impl<'a> OpenWebsites<'a> {
        pub fn new(urls: Vec<&'a str>, feedback: String) -> Self {
            OpenWebsites {
                urls: urls,
                feedback: feedback,
            }
        }
    }
}
