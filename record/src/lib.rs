extern crate sdl2;

pub mod recorder {
    use sdl2::audio::{AudioCallback, AudioSpecDesired};
    use std::i16;
    use std::sync::mpsc;

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
    }

}
