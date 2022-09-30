use std::{collections::HashMap};

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

pub struct AudioHandler {
    sound_map: HashMap<Sound, String>,
    audio: Audio,
}

impl AudioHandler {
    pub fn new() -> Self {
        Self {
            sound_map: get_sound_map(),
            audio: create_audio(),
        }
    }

    pub fn play(&mut self, sound: &Sound)
    {
        let binding = &self.sound_map;
        let audio_name = binding.get(sound);
        match audio_name {
            Some(name) => {
                self.audio.play(name);
                self.audio.wait();
            }
            _ => {}
        }
    }
}

fn get_sound_map() -> HashMap<Sound, String>
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

fn create_audio() -> Audio
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

