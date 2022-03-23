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
        // recognition
        let mut mapping: HashMap<String, String> = HashMap::new();
        mapping.insert("recognize".to_string(), "give recognition".to_string());
        mapping.insert(
            "recognize colleague".to_string(),
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
            "what should i eat".to_string(),
            "open lunch menus".to_string(),
        );
        mapping.insert(
            "what should i have for lunch".to_string(),
            "open lunch menus".to_string(),
        );
        // skm to work
        mapping.insert(
            "when is train work".to_string(),
            "when is the next train to work".to_string(),
        );
        mapping.insert(
            "when is train to work".to_string(),
            "when is the next train to work".to_string(),
        );
        mapping.insert(
            "when is next train work".to_string(),
            "when is the next train to work".to_string(),
        );
        mapping.insert(
            "when is next train to work".to_string(),
            "when is the next train to work".to_string(),
        );
        mapping.insert(
            "when can i go to work".to_string(),
            "when is the next train to work".to_string(),
        );
        mapping.insert(
            "when does train to work departs".to_string(),
            "when is the next train to work".to_string(),
        );
        // skm home
        mapping.insert(
            "when is next train home".to_string(),
            "when is the next train home".to_string(),
        );
        mapping.insert(
            "when is train home".to_string(),
            "when is the next train home".to_string(),
        );
        mapping.insert(
            "when can i return home".to_string(),
            "when is the next train home".to_string(),
        );
        mapping.insert(
            "when does train home departs".to_string(),
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
        mapping.insert("add an issue".to_string(), "file na issue".to_string());

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
        #[test]
        fn test_nlu_msr() -> Result<(), String> {
            assert_eq!(
                normalize_phrase("create my monthly status report"),
                "create monthly status report"
            );
            assert_eq!(
                normalize_phrase("compose my monthly status report"),
                "create monthly status report"
            );
            assert_eq!(
                normalize_phrase("compose monthly status report"),
                "create monthly status report"
            );

            Ok(())
        }
        #[test]
        fn test_nlu_wsr() -> Result<(), String> {
            assert_eq!(
                normalize_phrase("create my weekly status report"),
                "create weekly status report"
            );
            assert_eq!(
                normalize_phrase("compose my weekly status report"),
                "create weekly status report"
            );
            assert_eq!(
                normalize_phrase("compose weekly status report"),
                "create weekly status report"
            );
            Ok(())
        }
        #[test]
        fn test_nlu_skm() -> Result<(), String> {
            // Train to work
            assert_eq!(
                normalize_phrase("when is the train work"),
                "when is the next train to work"
            );
            assert_eq!(
                normalize_phrase("when is the next train work"),
                "when is the next train to work"
            );
            assert_eq!(
                normalize_phrase("when is the next train to work"),
                "when is the next train to work"
            );
            assert_eq!(
                normalize_phrase("when is the train to work"),
                "when is the next train to work"
            );
            assert_eq!(
                normalize_phrase("when can i go to work"),
                "when is the next train to work"
            );
            assert_eq!(
                normalize_phrase("when does the train to work departs"),
                "when is the next train to work"
            );

            // Train home
            assert_eq!(
                normalize_phrase("when is the train home"),
                "when is the next train home"
            );
            assert_eq!(
                normalize_phrase("when is the next train home"),
                "when is the next train home"
            );
            assert_eq!(
                normalize_phrase("when can i return home"),
                "when is the next train home"
            );
            assert_eq!(
                normalize_phrase("when does the train home departs"),
                "when is the next train home"
            );

            Ok(())
        }
        #[test]
        fn test_nlu_jira() -> Result<(), String> {
            assert_eq!(normalize_phrase("create an issue"), "file na issue");
            assert_eq!(normalize_phrase("submit an issue"), "file na issue");
            assert_eq!(normalize_phrase("add an issue"), "file na issue");
            Ok(())
        }
        #[test]
        fn test_nlu_restaurants() -> Result<(), String> {
            assert_eq!(normalize_phrase("open the lunch menu"), "open lunch menus");
            assert_eq!(normalize_phrase("open lunch menus"), "open lunch menus");
            assert_eq!(normalize_phrase("what should i eat"), "open lunch menus");
            assert_eq!(
                normalize_phrase("what should i have for lunch"),
                "open lunch menus"
            );

            Ok(())
        }

        #[test]
        fn test_nlu_recognition() -> Result<(), String> {
            assert_eq!(normalize_phrase("i want to recognize"), "give recognition");
            assert_eq!(
                normalize_phrase("recognize my colleague"),
                "give recognition"
            );
            assert_eq!(normalize_phrase("recognize someone"), "give recognition");
            assert_eq!(normalize_phrase("give recognition"), "give recognition");

            Ok(())
        }
    }
}
