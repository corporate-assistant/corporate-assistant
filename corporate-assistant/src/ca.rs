pub mod actions {
    use corporate_assistant::interpreter::CorporateAction;
    pub use custom_action::{action_creator, action_executor};
    //use config::configuration;
    use crate::config::configuration;

    pub struct CreateCustomAction {}

    impl CorporateAction for CreateCustomAction {
        fn run(&self, tts: &mut tts::TTS) -> () {
            tts.speak("Please edit custom action script", true)
                .expect("Problem with utterance");
            // Get user script name and pass it to be executed
            let script_name = configuration::CAConfig::new().get_custom_action_script();
            action_creator(script_name);
        }
    }

    impl CreateCustomAction {
        pub fn new() -> Self {
            Self {}
        }
    }

    // Execute Custom action

    pub struct ExecuteCustomAction {}

    impl CorporateAction for ExecuteCustomAction {
        fn run(&self, tts: &mut tts::TTS) -> () {
            tts.speak("Executing custom action", true)
                .expect("Problem with utterance");
            // Get user script name and pass it to be executed
            let script_name = configuration::CAConfig::new().get_custom_action_script();
            action_executor(script_name);
        }
    }

    impl ExecuteCustomAction {
        pub fn new() -> Self {
            Self {}
        }
    }
}
