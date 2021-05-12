extern crate wav;

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use rand::{distributions::Alphanumeric, Rng};
use chrono::{DateTime, Utc};
use structopt::StructOpt;
use std::fs::OpenOptions;
use std::fs;


pub use record::recorder::Recorder; 

fn save_wav(file_name : &str, channels : u16, freq : u32, recorded_vec: &Vec<i16>) -> Result<(), String> {

    println!("Saving data to {}", file_name);
    let mut out_file = File::create(Path::new(file_name)).expect("Unable to open file");
    let header = wav::Header::new(1, channels, freq, 16); 
    let bit_depth = wav::BitDepth::Sixteen(recorded_vec.clone());
    wav::write(header, &bit_depth, &mut out_file).expect("Problem writting data");
    
    Ok(())
}

/// Returns a percent value
fn calculate_average_volume(recorded_vec: &[i16]) -> f32 {
    let sum: i64 = recorded_vec.iter().map(|&x| (x as i64).abs()).sum();
    (sum as f32) / (recorded_vec.len() as f32) / (i16::MAX as f32) * 100.0
}

/// Returns a percent value
fn calculate_max_volume(recorded_vec: &[i16]) -> f32 {
    let max: i64 = recorded_vec
        .iter()
        .map(|&x| (x as i64).abs())
        .max()
        .expect("expected at least one value in recorded_vec");
    (max as f32) / (i16::MAX as f32) * 100.0
}

// Implements: filename=`cat /dev/urandom | tr -cd 'a-f0-9' | head -c 12``date -I`
fn generate_file_name() -> String {
    
    // 1. Get Random string
    let s: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(12)
            .map(char::from)
            .collect();

    // 2. Get Date 
     let now: DateTime<Utc> = Utc::now();
     
    // 3. Make two of them plus ".wav" a proper string
    s+&(now.format("-%Y-%m-%d").to_string())+".wav"
}

/// Search for a pattern in a file and display the lines that contain it.
#[derive(StructOpt)]
struct Cli {
    /// Text of utterance to be spoken
    #[structopt(short = "t", long = "text")]
    text: String,
    /// name of database file
    #[structopt(short = "db", long = "database", default_value="all")]
    database: std::path::PathBuf,
}

/// Create database file if it does not exist or open existing one and append wav metadata
/// Put given metadat of wav into database
fn add_wav_to_database(database_name : &std::path::PathBuf, wav_name : &str, text : &str) -> std::io::Result<()>
{
  let mut db_file = OpenOptions::new()
            .read(false)
            .write(true)
            .append(true)
            .create(true)
            .open(database_name).unwrap();

  // If database empty then append invocation
  let db_attr = fs::metadata(database_name)?;
  if db_attr.len() == 0 {
    let invocation = "wav_filename,wav_filesize,transcript\n";
    db_file.write_all(invocation.as_bytes())?;
  }
  db_file.write_all(wav_name.as_bytes())?;
  db_file.write_all(",".as_bytes())?;

  // Get audio sample size and append to database
  let wav_size = fs::metadata(wav_name).unwrap().len().to_string();
  db_file.write_all(wav_size.as_bytes())?;
  db_file.write_all(",".as_bytes())?;

  // Append transcript
  db_file.write_all(text.as_bytes())?;
  db_file.write_all("\n".as_bytes())?;

  Ok(())
}

fn record_and_store(database_name : &std::path::PathBuf, text : &str) -> Result<(), String>
{
    // Record audio to mimic arecord e.g.
    // arecord -r 16000 -f S16_LE $filename.wav
    let myrecorder = Recorder::new();
    let (recorded_vec, channels, freq) = myrecorder.record()?;

    println!(
        "Average Volume of your Recording = {:?}%",
        calculate_average_volume(&recorded_vec)
    );
    println!(
        "Max Volume of your Recording = {:?}%",
        calculate_max_volume(&recorded_vec)
    );

    let file_name = generate_file_name();
    save_wav(&file_name, channels, freq, &recorded_vec ).expect("Saving Wav did not quite succeeded");

    add_wav_to_database(database_name, &file_name, text ).expect("Writing recorded audio info to database failed"); 

  Ok(())
}


/// potential usage record --database=<desired database file name to write to> --text=<"Text to be spoken">
fn main() -> Result<(), String> {

    let record_args = Cli::from_args();
    println!("text: {}", record_args.text);
    println!("database: {:?}", record_args.database);

    // "all" means create three databases : train.txt, test.txt, dev.txt
    if record_args.database.to_str().expect("Improper name of database given") == "all" {
        println!("DataBase: {} . Please say: {}", "train.csv", record_args.text);
        record_and_store(&std::path::PathBuf::from("train.csv"), &record_args.text).unwrap();
        println!("DataBase: {} . Please say: {}", "dev.csv", record_args.text);
        record_and_store(&std::path::PathBuf::from("dev.csv"), &record_args.text).unwrap();
        println!("DataBase: {} . Please say: {}", "test.csv", record_args.text);
        record_and_store(&std::path::PathBuf::from("test.csv"), &record_args.text).unwrap();
    } else {
        println!("DataBase: {:?} . Please say: {}", record_args.database, record_args.text);
        record_and_store(&record_args.database, &record_args.text).unwrap();
    }
    
    Ok(())
}
