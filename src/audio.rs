pub mod audio_handler {
    use std::{collections::HashMap, hash::Hash};

    use derivative::Derivative;
    use rusty_audio::Audio;

    #[derive(Debug, Eq, Derivative)]
    #[derivative(PartialEq, Hash)]
    pub enum Sound {
        Startup,
        Explode,
        Lose,
        Move,
        Pew,
        Win,
    }

    pub fn get_sound_map() -> HashMap<Sound, String>
    {
        let map = HashMap::from(
            [
                (Sound::Startup, String::from("startup")),
                (Sound::Explode, String::from("explode")),
                (Sound::Lose, String::from("lose")),
                (Sound::Move, String::from("move")),
                (Sound::Pew, String::from("pew")),
                (Sound::Win, String::from("win")),
            ]
        );
        map
    }

    pub fn create_audio() -> Audio
    {
        let mut audio = Audio::new();
        audio.add("explode", "explode.wav");
        audio.add("lose", "lose.wav");
        audio.add("move", "move.wav");
        audio.add("pew", "pew.wav");
        audio.add("startup", "startup.wav");
        audio.add("win", "win.wav");

        audio
    }

    pub fn play(audio: &mut Audio, sound: &Sound, sound_map: &HashMap<Sound, String>)
    {
        let binding = sound_map;
        let audio_name = binding.get(sound);
        match audio_name {
            Some(name) => {
                audio.play(name);
                audio.wait();
            }
            _ => {}
        }
        
    }
}