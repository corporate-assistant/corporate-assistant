extern crate audrey;
extern crate clap;
extern crate deepspeech;

use clap::{App, Arg};

use std::fs::create_dir_all;
use std::fs::File;
use std::fs::OpenOptions;
use std::io;
use std::path::Path;
use std::rc::Rc;
use std::{thread, time};

use audrey::read::Reader;
use deepspeech::Model;
use tts::*;

pub use record::recorder::Recorder;
mod msr; // Need this to know there is separate module in this project
mod labeling_assistant; // Need this to know there is separate module in this project
use corporate_assistant::interpreter::CorporateAction; // Trait need to be visible in scope to be used

// The model has been trained on this specific
// sample rate.
const SAMPLE_RATE: u32 = 16_000;

fn read_audio_buffer<T>(reader: &mut Reader<T>) -> Result<Vec<i16>, String>
where
    T: std::io::Read + std::io::Seek,
{
    let desc = reader.description();

    if desc.channel_count() != 1 {
        Err(String::from(
            "The channel count is required to be one, at least for now",
        ))
    } else if desc.sample_rate() != SAMPLE_RATE {
        let msg: String = "Incorrect sample rate. ".to_owned() + &SAMPLE_RATE.to_string();
        Err(msg)
    } else {
        Ok(reader.samples().map(|s| s.unwrap()).collect())
    }
}

fn main() {
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
            Arg::with_name("scorer")
                .long("scorer")
                .help("Sets model scorer")
                .value_name("FILE")
                .takes_value(true),
        )
        .get_matches();

    let model_file_path = matches.value_of("model").expect("Set model file");
    let scorer_file_path = matches.value_of("scorer");

    let model_path = Path::new(&model_file_path);
    // Load DS model to memory
    let mut m = Model::load_from_files(&model_path).unwrap();
    match scorer_file_path {
        Some(s) => {
            let scorer_path = Path::new(&s);
            m.enable_external_scorer(&scorer_path).unwrap();
        }
        None => (),
    }

    // either read audio from file or from mic
    let result: String;
    // Record audio to mimic arecord e.g.
    // arecord -r 16000 -f S16_LE $filename.wav
    // if only utterance is completed
    let mut tts = TTS::default().expect("Problem starting TTS engine");
    let rec = Recorder::new();
    tts.speak("I'm listening", true)
        .expect("Problem with utterance");
    // TTS finished talking e.g. sending data to speaker
    while tts.is_speaking().unwrap() == true {}
    //.. yet we need to wait so that recorder did not pick up still hearable sound
    let canceling_pause = time::Duration::from_millis(400);
    thread::sleep(canceling_pause);
    let (recorded_vec, channels, freq) = rec.record().expect("Problem with recording audio");
    result = m.speech_to_text(&recorded_vec).unwrap();
    // Output the result
    eprintln!("Transcription:");
    println!("{}", result);

    // Registration of actions
    let mut intents = corporate_assistant::interpreter::Intents::new();
    intents
        .register_action(
            vec![
                "compose my monthly status report".to_string(),
                "compose monthly status report".to_string(),
                "create my monthly status report".to_string(),
                "create monthly status report".to_string(),
            ],
            Rc::new(msr::actions::MSR::new(4)),
        )
        .expect("Registration failed");
    intents
        .register_action(
            vec![
                "compose my weekly status report".to_string(),
                "compose weekly status report".to_string(),
                "create my weekly status report".to_string(),
                "create weekly status report".to_string(),
            ],
            Rc::new(msr::actions::MSR::new(1)),
        )
        .expect("Registration failed");

    // Get requested action
    let action = intents.get_action(&result);

    // Let's match e.g. If phrase not recognized then execute dumping
    // if proper Action object returned then execute it
    match action {
        Ok(action) => action.Run(&mut tts),
        Err(action) => {
            println!("No action found for : {}", action);
            // Announce that request is unknown
            tts.speak("I do not understand", true)
                .expect("Error: Problem with utterance");
            // Make a GUI for helping user to label recording
            let result = labeling_assistant::labeling_assistant::run(intents.get_commands(),&result);
            // Create directory for unrecognized requests if needed
            let unrecognized_dir = "unrecognized_content";
            create_dir_all(unrecognized_dir)
                .expect("Error: unable to create directory: unrecognized");
            // Save unrecognized audio into directory
            rec.store(&recorded_vec, channels, freq, unrecognized_dir, &result)
                .expect("Saving unrecognized command failed!");
        }
    }
}
