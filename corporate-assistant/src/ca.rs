pub mod actions {
    use corporate_assistant::interpreter::CorporateAction;
    pub use custom_action::{action_creator, action_executor};

    pub struct CreateCustomAction {}

    impl CorporateAction for CreateCustomAction {
        fn Run(&self, tts: &mut tts::TTS) -> () {
            todo!();
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
        fn Run(&self, tts: &mut tts::TTS) -> () {
            tts.speak("Executing custom action", true)
                .expect("Problem with utterance");
            // Get user script name and pass it to be executed
            let mut script_name = std::env::current_exe().unwrap();
            script_name.pop();
            script_name.pop();
            script_name.pop();
            script_name.push("custom_action");
            script_name.push("scripts");
            script_name.push("custom_script.sh");
            action_executor(script_name);
        }
    }

    impl ExecuteCustomAction {
        pub fn new() -> Self {
            Self {}
        }
    }
}
