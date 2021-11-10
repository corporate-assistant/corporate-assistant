pub mod actions {
    use corporate_assistant::interpreter::CorporateAction;
    pub use custom_action::{action_creator, action_executor};
    //use config::configuration;
    use crate::config::configuration;
    use std::rc::Rc;

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

    pub struct ExecuteCustomAction {
        script_: String,
    }

    impl CorporateAction for ExecuteCustomAction {
        fn run(&self, tts: &mut tts::TTS) -> () {
            tts.speak("Executing custom action", true)
                .expect("Problem with utterance");
            action_executor(&self.script_);
        }
    }

    impl ExecuteCustomAction {
        pub fn new(script: String) -> Self {
            Self { script_: script }
        }
    }

    // Load and register custom actions
    pub fn register_custom_actions(intents: &mut corporate_assistant::interpreter::Intents) {
        let custom_action_config_file = configuration::CAConfig::new().get_custom_action_config();
        let custom_actions = custom_action::parse_config(custom_action_config_file);

        //iterate through actions and register each of them
        for ca in custom_actions.custom_actions.iter() {
            println!("Registring phrase: {}", ca.phrase);
            println!("  with script: {}", ca.script);
            intents
                .register_action(
                    vec![ca.phrase.clone()],
                    Rc::new(ExecuteCustomAction::new(ca.script.clone())),
                )
                .expect(&format!("Unable to register: {}", ca.phrase))
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use tts::*;

        #[test]
        #[ignore]
        fn test_custom_action() -> Result<(), String> {
            let mut tts = TTS::default().expect("Problem starting TTS engine");

            CreateCustomAction::new().run(&mut tts);
            Ok(())
        }
    }
}
