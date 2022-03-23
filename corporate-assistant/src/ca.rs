pub mod actions {
    use corporate_assistant::interpreter::CorporateAction;
    pub use custom_action::{action_creator, action_executor};
    //use config::configuration;
    use crate::config::configuration;
    use record::recorder::Recorder;
    use std::cell::RefCell;
    use std::rc::Rc;

    pub struct CreateCustomAction {
        m: Rc<RefCell<deepspeech::Model>>,
        rec: Rc<Recorder>,
    }

    impl CorporateAction for CreateCustomAction {
        fn run(&self, tts: &mut tts::TTS) -> () {
            tts.speak("Please edit custom action script", true)
                .expect("Problem with utterance");
            // Get user script name and pass it to be executed
            let custom_action_config_file =
                configuration::CAConfig::new().get_custom_action_config();
            action_creator(custom_action_config_file, self.m.clone(), self.rec.clone());
        }
    }

    impl CreateCustomAction {
        pub fn new(m: Rc<RefCell<deepspeech::Model>>, rec: Rc<Recorder>) -> Self {
            Self { m, rec }
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
                    ca.phrase.clone(),
                    Rc::new(ExecuteCustomAction::new(ca.script.clone())),
                )
                .expect(&format!("Unable to register: {}", ca.phrase))
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use deepspeech::Model;
        use std::path::Path;
        use tts::*;

        #[test]
        #[ignore]
        fn test_custom_action() -> Result<(), String> {
            let mut tts = TTS::default().expect("Problem starting TTS engine");

            let rec = Rc::new(Recorder::new());
            // local model . Change to yours if needed
            let model_path = Path::new("/home/jczaja/DEEPSPEECH/jacek-04-02-2021.pbmm");
            let m = Rc::new(RefCell::new(Model::load_from_files(&model_path).unwrap()));
            m.borrow_mut()
                .enable_external_scorer(Path::new(
                    "/home/jczaja/DEEPSPEECH/deepspeech-0.9.3-models.scorer",
                ))
                .unwrap();

            CreateCustomAction::new(m, rec.clone()).run(&mut tts);
            Ok(())
        }
    }
}
