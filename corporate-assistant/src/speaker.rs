use tts::{Error, TTS};

pub struct Speaker {
    t: Option<TTS>,
}

impl Speaker {
    pub fn new() -> Result<Speaker, Error> {
        match TTS::default() {
            Ok(tts) => Ok(Speaker { t: Some(tts) }),
            Err(e) => Err(e),
        }
    }

    pub fn none() -> Result<Speaker, Error> {
        Ok(Speaker { t: None })
    }

    pub fn speak(&mut self, text: &str, interrupt: bool) -> Result<(), Error> {
        match &mut self.t {
            Some(tts) => match tts.speak(text, interrupt) {
                Ok(_) => Ok(()),
                Err(e) => Err(e),
            },
            None => Ok(()),
        }
    }

    pub fn is_speaking(&self) -> Result<bool, Error> {
        match &self.t {
            Some(tts) => match tts.is_speaking() {
                Ok(r) => Ok(r),
                Err(e) => Err(e),
            },
            None => Ok(false),
        }
    }
}
