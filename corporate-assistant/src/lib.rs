pub mod speaker;

pub mod interpreter {
    use crate::speaker::Speaker;
    use std::collections::HashMap;
    use std::rc::Rc;
    pub trait CorporateAction {
        fn run(&self, speaker: &mut Speaker) -> ();
    }

    pub struct Intents {
        intents: HashMap<String, Rc<dyn CorporateAction>>,
    }

    // Register new action Trait
    impl Intents {
        pub fn register_action(
            &mut self,
            key: String,
            action: Rc<dyn CorporateAction>,
        ) -> Result<(), String> {
            self.intents.insert(String::from(key), action.clone());
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
            let mut commands: Vec<&str> = Vec::new();

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
