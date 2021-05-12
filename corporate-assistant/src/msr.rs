pub mod actions {
  use corporate_assistant::interpreter::CorporateAction;
  pub use github_crawler::{get_contributions,RepoContribs,Contrib,Conf, parse_config};
  use tts::*;

  impl CorporateAction for MSR {


    fn Run(&self, tts : &mut tts::TTS) -> () {
      tts.speak("Composing monthly status report", true).expect("Problem with utterance");

      // Make some test of github crawler
      let conf = Conf {
          from_date: chrono::Utc::now() - chrono::Duration::weeks(4),
          to_date: chrono::Utc::now(),
          //TODO: Make it read from HOME
          config_file: String::from("/home/jczaja/DEEPSPEECH/corporate-assistant/corporate-assistant/config.toml"),
          behind_proxy: true,
      };

      let config_file = conf.config_file.clone();
      let config = parse_config(config_file);
      let contribs = get_contributions(conf, config );
      MSR::print_text(&contribs);
    }
  }

  pub struct MSR {

  }
 
  impl MSR {

    pub fn new() -> Self {
        MSR {
        }
    }

    fn print_text(repo_contribs: &RepoContribs) -> () {
      let repos = repo_contribs.keys();

      let mut text = "".to_string();
      for repo in repos {
          text += &("* ".to_owned() + &repo + "\n");

          for c in repo_contribs.get(repo).unwrap() {
              let line = "\t - ".to_owned()
                  + &c.merge_date.format("%Y-%m-%d").to_string()
                  + ": "
                  + &c.id.to_string()
                  + ", "
                  + &c.title
                  + "\n";
              text += &line;
          }
      }

      println!("{}", text);
      ()
    }
  }
}
