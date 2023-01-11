extern crate audrey;
extern crate clap;
extern crate deepspeech;

use clap::{App, Arg};

use crate::config::configuration;
use corporate_assistant::speaker::Speaker;
use deepspeech::Model;
use err_handling::{init_logging_infrastructure, ResultExt};
use github_crawler::parse_config;
use std::cell::RefCell;
use std::fs::create_dir_all;
use std::path::Path;
use std::rc::Rc;
use std::{thread, time};

pub use record::recorder::Recorder;
mod ca;
mod config;
mod jira;
mod labeling_assistant;
mod msr; // Need this to know there is separate module in this project // Need this to know there is separate module in this project
mod nlu;
mod skm;
mod webbrowser;

fn main() {
    init_logging_infrastructure();
    log::info!("Corporate-assistant started!");

    let matches = App::new("Corporate Assistant")
        .arg(
            Arg::with_name("model")
                .long("model")
                .help("Sets deepspeech model file")
                .value_name("FILE")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("project")
                .long("project")
                .help("Name of project configuration file")
                .value_name("FILE")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("organization")
                .long("organization")
                .help("Name of organization configuration file")
                .value_name("FILE")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("scorer")
                .long("scorer")
                .help("Sets model scorer")
                .value_name("FILE")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("disable-tts")
                .long("disable-tts")
                .help("Disables text-to-speech.")
                .takes_value(false)
                .required(false),
        )
        .get_matches();

    let model_file_path = matches.value_of("model").expect_and_log("Set model file");
    let scorer_file_path = matches.value_of("scorer");

    let model_path = Path::new(&model_file_path);
    // Load DS model to memory
    let m = Rc::new(RefCell::new(
        Model::load_from_files(&model_path)
            .expect_and_log(&format!("Error loading model: {}", model_file_path)),
    ));
    match scorer_file_path {
        Some(s) => {
            let scorer_path = Path::new(&s);
            m.borrow_mut()
                .enable_external_scorer(&scorer_path)
                .expect_and_log(&format!("Error loading scorer: {}", s));
            log::info!("Scorer {} loaded", s);
        }
        None => (),
    }

    // either read audio from file or from mic
    let result: String;
    // Record audio to mimic arecord e.g.
    // arecord -r 16000 -f S16_LE $filename.wav
    // if only utterance is completed
    let mut speaker = if matches.is_present("disable-tts") {
        Speaker::none().expect("Problem creating empty TTS engine")
    } else {
        Speaker::new().expect("Problem starting TTS engine")
    };

    let rec = Rc::new(Recorder::new());

    speaker
        .speak("I'm listening", true)
        .expect("Problem with utterance");

    // TTS finished talking e.g. sending data to speaker
    while speaker.is_speaking().unwrap() == true {}
    //.. yet we need to wait so that recorder did not pick up still hearable sound
    let canceling_pause = time::Duration::from_millis(400);
    thread::sleep(canceling_pause);
    let (recorded_vec, channels, freq) =
        rec.record().expect_and_log("Problem with recording audio");

    result = m
        .borrow_mut()
        .speech_to_text(&recorded_vec)
        .expect_and_log("Speech to text failed");

    // Output the result
    log::info!("Transcription: {}", result);

    // Origanization/site info
    let path = matches
        .value_of("organization")
        .expect_and_log("Please set organization config file");

    let organization_config_file = configuration::CAConfig::new().get_organization_config(path);

    let org_info = configuration::parse_organization_config(&organization_config_file);
    // Project info
    let project_config_file = matches
        .value_of("project")
        .expect_and_log("Please set project config file");

    let config_file = configuration::CAConfig::new().get_repos_config(project_config_file);
    let (_, jira_config) = parse_config(config_file);

    //TODO(jczaja): Make it an async function to register all of this
    // Registration of actions
    let mut intents = corporate_assistant::interpreter::Intents::new();
    // Register custom actions
    ca::actions::register_custom_actions(&mut intents);

    // JIRA Helper
    match jira_config {
        Some(jira) => {
            intents
                .register_action(
                    nlu::nlu::normalize_phrase("file an issue"),
                    Rc::new(jira::jira::JIRA::new(
                        jira.user,
                        jira.url,
                        org_info.proxy.clone(),
                        jira.project,
                        jira.epics,
                    )),
                )
                .expect_and_log("Registration of JIRA module failed");
            log::info!("JIRA module registered");
        }
        None => (),
    }

    match org_info.email {
        Some(email_config) => {
            intents
                .register_action(
                    nlu::nlu::normalize_phrase("create monthly status report"),
                    Rc::new(msr::actions::MSR::new(
                        &org_info.proxy,
                        project_config_file,
                        4,
                        &email_config,
                    )),
                )
                .expect_and_log("Registration of MSR module failed");
            log::info!("MSR module registered");
            intents
                .register_action(
                    nlu::nlu::normalize_phrase("create weekly status report"),
                    Rc::new(msr::actions::MSR::new(
                        &org_info.proxy,
                        project_config_file,
                        1,
                        &email_config,
                    )),
                )
                .expect_and_log("Registration of MSR module failed");
            log::info!("MSR module registered");
        }
        None => (),
    }
    intents
        .register_action(
            nlu::nlu::normalize_phrase("create custom action"),
            Rc::new(ca::actions::CreateCustomAction::new(m, rec.clone())),
        )
        .expect_and_log("Registration of CCA module failed");
    log::info!("CCA module registered");

    let mut to_from: Vec<String>;
    match org_info.home_work_train_stations {
        Some(i) => {
            intents
                .register_action(
                    nlu::nlu::normalize_phrase("when is the next train to work"),
                    Rc::new(skm::skm::SKM::new(
                        "https://skm.trojmiasto.pl/".to_string(),
                        org_info.proxy.clone(),
                        i.clone(),
                    )),
                )
                .expect_and_log("Registration of SKM module failed");
            // for travelling back we need to swap start and destination
            to_from = i;
            to_from.reverse();
            intents
                .register_action(
                    nlu::nlu::normalize_phrase("when is the next train home"),
                    Rc::new(skm::skm::SKM::new(
                        "https://skm.trojmiasto.pl/".to_string(),
                        org_info.proxy.clone(),
                        to_from,
                    )),
                )
                .expect_and_log("Registration of SKM module failed");
            log::info!("Next Train module registered");
        }
        None => (),
    }
    match org_info.restaurants {
        Some(i) => {
            intents
                .register_action(
                    nlu::nlu::normalize_phrase("open lunch menus"),
                    Rc::new(webbrowser::actions::OpenWebsites::new(
                        i,
                        "Opening lunch menus".to_string(),
                    )),
                )
                .expect_and_log("Registration of Lunch Menus module failed");
            log::info!("Lunch menu module registered");
        }
        None => (),
    }
    match org_info.holidays {
        Some(i) => {
            intents
                .register_action(
                    nlu::nlu::normalize_phrase("give me holidays"),
                    Rc::new(webbrowser::actions::OpenWebsites::new(
                        i,
                        "Opening the holdiday request form".to_string(),
                    )),
                )
                .expect_and_log("Registration of holidays module failed");
            log::info!("holidays module registered");
        }
        None => (),
    }
    match org_info.recognition {
        Some(i) => {
            intents
                .register_action(
                    nlu::nlu::normalize_phrase("give recognition"),
                    Rc::new(webbrowser::actions::OpenWebsites::new(
                        i,
                        "Opening the recognition system".to_string(),
                    )),
                )
                .expect_and_log("Registration of Recognition module failed");
            log::info!("Recognition module registered");
        }
        None => (),
    }

    // Get requested action
    let action = intents.get_action(&nlu::nlu::normalize_phrase(&result));

    // Let's match e.g. If phrase not recognized then execute dumping
    // if proper Action object returned then execute it
    match action {
        Ok(action) => action.run(&mut speaker),
        Err(action) => {
            log::error!("No action found for : {}", action);
            // Announce that request is unknown
            speaker
                .speak("I do not understand", true)
                .expect("Error: Problem with utterance");
            // Make a GUI for helping user to label recording
            let result = labeling_assistant::labeling_assistant::run(
                &intents,
                &mut speaker,
                &rec,
                recorded_vec.to_vec(),
                intents.get_commands(),
                &result,
            );
            // If user decided to store recording then proceed
            match result {
                Some(result) => {
                    // Create directory for unrecognized requests if needed
                    let unrecognized_dir = "unrecognized_content";
                    create_dir_all(unrecognized_dir)
                        .expect_and_log("Error: unable to create directory: unrecognized");
                    // Save unrecognized audio into directory
                    rec.store(&recorded_vec, channels, freq, unrecognized_dir, &result)
                        .expect_and_log("Saving unrecognized command failed!");
                    speaker
                        .speak("Recording stored", true)
                        .expect("Problem with utterance");
                }
                None => (),
            };
        }
    }
}
