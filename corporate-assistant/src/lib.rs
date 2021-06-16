pub mod interpreter {

    use std::collections::HashMap;
    use std::rc::Rc;
    use tts::*;

    pub trait CorporateAction {
        fn Run(&self, tts: &mut tts::TTS) -> ();
    }

    pub struct Intents {
        intents: HashMap<String, Rc<CorporateAction>>,
    }

    // Register new action Trait
    impl Intents {
        pub fn register_action(
            &mut self,
            keys: Vec<String>,
            action: Rc<dyn CorporateAction>,
        ) -> Result<(), String> {
            for key in keys {
                self.intents.insert(String::from(key), action.clone());
            }
            Ok(())
        }

        pub fn get_action(&self, key: &str) -> Result<Rc<dyn CorporateAction>, String> {
            if self.intents.contains_key(key) {
                Ok(self.intents[key].clone())
            } else {
                Err(format!("Action \"{}\" not recognized", key) as String)
            }
        }

        pub fn get_commands(&self) -> Vec<&str> {
            let mut commands : Vec<&str> = Vec::new();

            for command in self.intents.keys() {
                commands.push(command);
            }

            commands
        }

        pub fn new() -> Self {
            Intents {
                intents: HashMap::new(),
            }
        }
    }
}
