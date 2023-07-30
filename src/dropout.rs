use crate::{Map, Midibox};
use rand::Rng;
use crate::chord::Chord;
use crate::midi::{Midi, MutMidi};
use crate::tone::Tone;


pub fn random_dropout(midibox: Box<dyn Midibox>, p: f64) -> Box<dyn Midibox> {
    Map::wrap(midibox, move |m| {
        if rand::thread_rng().gen_bool(p) {
            m.set_pitch(Tone::Rest, 3)
        } else {
            m
        }
    })
}

pub struct Dropout {
    duration: u32,
    duration_seen: u32,
    playing: bool,
    midibox: Box<dyn Midibox>,
}

impl Dropout {
    pub fn wrap(midibox: Box<dyn Midibox>, duration: u32, playing: bool) -> Box<dyn Midibox> {
        Box::new(Dropout {
            duration,
            playing,
            duration_seen: 0,
            midibox
        })
    }
}

impl Midibox for Dropout {
    fn next(&mut self) -> Option<Vec<Midi>> {
        if self.duration_seen >= self.duration {
            self.duration_seen = 0;
            self.playing = !self.playing;
        }
        let to_play: Option<Vec<Midi>> = self.midibox.next();
        return match to_play {
            Some(notes) => {
                let to_play_chord = Chord { notes: notes.clone() };
                self.duration_seen += to_play_chord.total_duration();
                if self.playing {
                    // forward the notes
                    return Some(notes)
                } else {
                    // otherwise take a rest
                    return Some(notes.iter().map(|n| n.set_pitch(Tone::Rest, 0)).collect())
                }
            }
            None => None
        };
    }
}
