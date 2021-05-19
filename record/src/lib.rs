extern crate sdl2;
extern crate wav;

pub mod recorder {
    use sdl2::audio::{AudioCallback, AudioSpecDesired};
    use std::i16;
    use std::sync::mpsc;
    use rand::{distributions::Alphanumeric, Rng};
    use chrono::{DateTime, Utc};
    use std::fs::OpenOptions;
    use std::fs::File;
    use std::fs;
    use std::io::prelude::*;
    use std::path::Path;

    const RECORDING_LENGTH_SECONDS: usize = 3;

    struct Recording {
        record_buffer: Vec<i16>,
        pos: usize,
        done_sender: mpsc::Sender<Vec<i16>>,
        done: bool,
    }

    pub struct Recorder {
        capture_device :  sdl2::audio::AudioDevice<Recording>,
        desired_spec : sdl2::audio::AudioSpecDesired,
        done_receiver: mpsc::Receiver<Vec<i16>>,
    }

    // Append the input of the callback to the record_buffer.
    // When the record_buffer is full, send it to the main thread via done_sender.
    impl AudioCallback for Recording {
        type Channel = i16;

        fn callback(&mut self, input: &mut [i16]) {
            if self.done {
                return;
            }

            for x in input {
                self.record_buffer[self.pos] = *x;
                self.pos += 1;
                if self.pos >= self.record_buffer.len() {
                    self.done = true;
                    self.done_sender
                        .send(self.record_buffer.clone())
                        .expect("could not send record buffer");
                    break;
                }
            }
        }
    }

    impl Recorder {

        pub fn new() -> Self {
                let sdl_context = sdl2::init().expect("SDL2 initialization failed");
                let audio_subsystem = sdl_context.audio().unwrap();

                let desired_spec = AudioSpecDesired {
                    freq: Some(16000), //        freq: None,
                    channels: Some(1),
                    samples: None,
                };

                eprintln!(
                    "Capturing {:} seconds... Please rock!",
                    RECORDING_LENGTH_SECONDS
                );

                let (done_sender, done_receiver) = mpsc::channel();

                let capture_device = audio_subsystem.open_capture(None, &desired_spec, |spec| {
                    eprintln!("Capture Spec = {:?}", spec);
                    Recording {
                        record_buffer: vec![
                            0;
                            spec.freq as usize
                                * RECORDING_LENGTH_SECONDS
                                * spec.channels as usize
                        ],
                        pos: 0,
                        done_sender,
                        done: false,
                    }
                }).expect("Error: Cannot Open capture device");

                eprintln!(
                    "AudioDriver: {:?}",
                    capture_device.subsystem().current_audio_driver()
                );
                Self {
                    capture_device: capture_device,
                    desired_spec: desired_spec,
                    done_receiver: done_receiver,
                }
            }


        pub fn record(&self) -> Result<(Vec<i16>, u16, u32), String> {

            self.capture_device.resume();

            // Wait until the recording is done.
            let recorded_vec = self.done_receiver.recv().map_err(|e| e.to_string())?;

            self.capture_device.pause();

            // Device is automatically closed when dropped.
            // Depending on your system it might be even important that the capture_device is dropped
            // before the playback starts.

            Ok((recorded_vec,
                    self.desired_spec.channels.unwrap() as u16,
                    self.desired_spec.freq.unwrap() as u32))
        }

        /// Returns a percent value
        fn calculate_average_volume(&self, recorded_vec: &[i16]) -> f32 {
            let sum: i64 = recorded_vec.iter().map(|&x| (x as i64).abs()).sum();
            (sum as f32) / (recorded_vec.len() as f32) / (i16::MAX as f32) * 100.0
        }

        /// Returns a percent value
        fn calculate_max_volume(&self, recorded_vec: &[i16]) -> f32 {
            let max: i64 = recorded_vec
                .iter()
                .map(|&x| (x as i64).abs())
                .max()
                .expect("expected at least one value in recorded_vec");
            (max as f32) / (i16::MAX as f32) * 100.0
        }

        // Implements: filename=`cat /dev/urandom | tr -cd 'a-f0-9' | head -c 12``date -I`
        fn generate_file_name(&self) -> String {
            
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

        // TODO: fixing

        fn save_wav(&self, cwd : &Option<str>, channels : u16, freq : u32, recorded_vec: &Vec<i16>) -> Result<(), String> 
        {

            let mut file_name = Path::new();
            file_name.push(self.generate_file_name());

            println!("Saving data to {}", file_name);
            let mut out_file = File::create(Path::new(file_name)).expect("Unable to open file");
            let header = wav::Header::new(1, channels, freq, 16); 
            let bit_depth = wav::BitDepth::Sixteen(recorded_vec.clone());
            wav::write(header, &bit_depth, &mut out_file).expect("Problem writting data");
            
            Ok(())
        }

        /// Create database file if it does not exist or open existing one and append wav metadata
        /// Put given metadat of wav into database
        fn add_wav_to_database(&self, database_name : &std::path::PathBuf, wav_name : &str, text : &str) -> std::io::Result<()>
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

        pub fn record_and_store(&self, database_name : &std::path::PathBuf, text : &str) -> Result<(), String>
        {
            // Record audio to mimic arecord e.g.
            // arecord -r 16000 -f S16_LE $filename.wav
            let (recorded_vec, channels, freq) = self.record()?;

            println!(
                "Average Volume of your Recording = {:?}%",
                self.calculate_average_volume(&recorded_vec)
            );
            println!(
                "Max Volume of your Recording = {:?}%",
                self.calculate_max_volume(&recorded_vec)
            );

            self.save_wav(None, channels, freq, &recorded_vec ).expect("Saving Wav did not quite succeeded");

            self.add_wav_to_database(database_name, None, text ).expect("Writing recorded audio info to database failed"); 

          Ok(())
        }

        pub fn store(&self, recorded_vec : &Vec<i16>, channels : u16, freq : u32, unrecognized_dir: &str, text : &str) -> Result<(), String>
        {

            let mut database_name = std::path::PathBuf::new();
            database_name.push(unrecognized_dir);
            database_name.push("failures.csv");

            self.save_wav(Some(unrecognized_dir), channels, freq, &recorded_vec ).expect("Saving Wav did not quite succeeded");
            self.add_wav_to_database(&database_name, Some(unrecognized_dir), text ).expect("Writing recorded audio info to database failed"); 

          Ok(())
        }

    }

}
