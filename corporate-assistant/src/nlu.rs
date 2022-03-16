pub mod nlu {
    use std::collections::HashMap;

    pub fn normalize_phrase(phrase: &str) -> String {
        log::info!("NLU input: {}", phrase);
        // 1. Strip some unimportant elements e.g. "i want to " , "the" "", "my "
        let processed_phrase = phrase
            .replace("i want to ", "")
            .replace("i want ", "")
            .replace("the ", "")
            .replace("my ", "");

        // 2. Check if there is a mapping for given phrase
        // roecognition
        let mut mapping: HashMap<String, String> = HashMap::new();
        mapping.insert("recognize".to_string(), "give recognition".to_string());
        mapping.insert(
            "recognize my colleague".to_string(),
            "give recognition".to_string(),
        );
        mapping.insert(
            "recognize someone".to_string(),
            "give recognition".to_string(),
        );
        // holidays mapping
        mapping.insert("holidays".to_string(), "give me holidays".to_string());
        mapping.insert("book holidays".to_string(), "give me holidays".to_string());
        mapping.insert("my holidays".to_string(), "give me holidays".to_string());
        mapping.insert(
            "request holidays".to_string(),
            "give me holidays".to_string(),
        );
        mapping.insert(
            "request vacations".to_string(),
            "give me holidays".to_string(),
        );
        //restuarants
        mapping.insert(
            "open lunch menu".to_string(),
            "open lunch menus".to_string(),
        );
        mapping.insert(
            "open the lunch menu".to_string(),
            "open lunch menus".to_string(),
        );
        mapping.insert(
            "open the lunch menus".to_string(),
            "open lunch menus".to_string(),
        );
        mapping.insert(
            "what should i eat".to_string(),
            "open lunch menus".to_string(),
        );
        mapping.insert(
            "what should i have for lunch".to_string(),
            "open lunch menus".to_string(),
        );
        mapping.insert(
            "open the lunch menus".to_string(),
            "open lunch menus".to_string(),
        );
        // skm to work
        mapping.insert(
            "when is the train work".to_string(),
            "when is the train to work".to_string(),
        );
        mapping.insert(
            "when is the next train work".to_string(),
            "when is the train to work".to_string(),
        );
        mapping.insert(
            "when is the next train to work".to_string(),
            "when is the train to work".to_string(),
        );
        mapping.insert(
            "when can i go to work".to_string(),
            "when is the train to work".to_string(),
        );
        mapping.insert(
            "when does the train to work departs".to_string(),
            "when is the train to work".to_string(),
        );
        // skm home
        mapping.insert(
            "when is the train home".to_string(),
            "when is the next train home".to_string(),
        );
        mapping.insert(
            "when can i return home".to_string(),
            "when is the next train home".to_string(),
        );
        mapping.insert(
            "when does the train home departs".to_string(),
            "when is the next train home".to_string(),
        );
        // Create Custom action
        mapping.insert(
            "compose custom action".to_string(),
            "create custom action".to_string(),
        );
        // MSR
        mapping.insert(
            "create my monthly status report".to_string(),
            "create monthly status report".to_string(),
        );
        mapping.insert(
            "compose my monthly status report".to_string(),
            "create monthly status report".to_string(),
        );
        mapping.insert(
            "compose monthly status report".to_string(),
            "create monthly status report".to_string(),
        );
        mapping.insert(
            "create my weekly status report".to_string(),
            "create weekly status report".to_string(),
        );
        mapping.insert(
            "compose my weekly status report".to_string(),
            "create weekly status report".to_string(),
        );
        mapping.insert(
            "compose weekly status report".to_string(),
            "create weekly status report".to_string(),
        );
        // JIRA
        mapping.insert("create an issue".to_string(), "file na issue".to_string());
        mapping.insert("submit an issue".to_string(), "file na issue".to_string());

        //
        let possible_outcome = mapping.get(&processed_phrase);
        let outcome = match possible_outcome {
            None => &processed_phrase,
            Some(i) => i,
        };
        log::info!("NLU output: {}", outcome);

        outcome.to_string()
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use tts::*;

        #[test]
        fn test_nlu_custom_action() -> Result<(), String> {
            assert_eq!(
                normalize_phrase("create custom action"),
                "create custom action"
            );
            assert_eq!(
                normalize_phrase("compose custom action"),
                "create custom action"
            );

            assert_eq!(normalize_phrase("i want holidays"), "give me holidays");
            assert_eq!(normalize_phrase("give me my holidays"), "give me holidays");

            Ok(())
        }

        #[test]
        fn test_nlu_holidays() -> Result<(), String> {
            assert_eq!(normalize_phrase("i want holidays"), "give me holidays");
            assert_eq!(normalize_phrase("give me my holidays"), "give me holidays");

            Ok(())
        }
    }
}
//TODO: more UT
// TODO: Remove RC
// TODO: Doc on this module
