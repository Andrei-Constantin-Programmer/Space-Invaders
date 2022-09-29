use std::error::Error;
use crate::audio::my_audio::Sound;
use crate::audio::my_audio::AudioHandler;

mod audio;

fn main() -> Result <(), Box<dyn Error>> {

    let mut audio_handler = AudioHandler::new();

    audio_handler.play(&Sound::Startup);

    Ok(())
}
