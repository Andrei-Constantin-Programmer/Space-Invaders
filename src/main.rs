use std::error::Error;
use crate::audio::audio_handler;
use crate::audio::audio_handler::play;
use crate::audio::audio_handler::Sound;

mod audio;

fn main() -> Result <(), Box<dyn Error>> {

    let mut audio = audio_handler::create_audio();
    let sound_map = audio_handler::get_sound_map();

    play(&mut audio, &Sound::Startup, &sound_map);

    Ok(())
}
