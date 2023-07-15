use crate::midi::{Midi, MutMidi};
use crate::scale::{Degree, Interval, Scale};
use crate::tone::Tone;


#[macro_export]
macro_rules! chord {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x.midi());
            )*
            Chord::new(temp_vec)
        }
    };
}


#[derive(Debug, Clone, PartialEq)]
pub struct Chord {
    pub notes: Vec<Midi>
}

impl Chord {
    pub fn new(notes: Vec<Midi>) -> Self {
        Chord { notes }
    }

    pub fn note(note: Midi) -> Self {
        Chord { notes: vec![note] }
    }
}

pub trait ToChord {
    fn chord(&self) -> Chord;
}

impl ToChord for Chord {
    fn chord(&self) -> Chord {
        self.clone()
    }
}




impl MutMidi for Chord {
    fn total_duration(&self) -> u32 {
        self.notes.iter().map(|n| n.duration).max().unwrap_or_else(|| 0)
    }

    fn duration(mut self, duration: u32) -> Self {
        self.notes = self.notes.into_iter().map(|m| m.set_duration(duration)).collect();
        self
    }

    fn velocity(mut self, velocity: u8) -> Self {
        self.notes = self.notes.into_iter().map(|m| m.set_velocity(velocity)).collect();
        self
    }

    fn pitch(mut self, tone: Tone, oct: u8) -> Self {
        self.notes = self.notes.into_iter().map(|m| m.set_pitch(tone, oct)).collect();
        self
    }

    fn scale_duration(mut self, factor: u32) -> Self {
        self.notes = self.notes.into_iter().map(|m| m * factor).collect();
        self
    }

    fn transpose_up(mut self, interval: &Interval) -> Self {
        self.notes = self.notes.into_iter().map(|m| m + *interval).collect();
        self
    }

    fn transpose_down(mut self, interval: &Interval) -> Self {
        self.notes = self.notes.into_iter().map(|m| m - *interval).collect();
        self
    }

    fn harmonize_up(mut self, scale: &Scale, degree: &Degree) -> Self {
        self.notes = self.notes.into_iter()
            .map(|m| if m.is_rest() {
                m
            } else {
                scale
                    .harmonize_up(m, *degree)
                    .unwrap_or_else(|| m.set_pitch(Tone::Rest, 4))
            })
            .collect();
        self
    }

    fn harmonize_down(mut self, scale: &Scale, degree: &Degree) -> Self {
        self.notes = self.notes.into_iter()
            .map(|m| if m.is_rest() {
                m
            } else {
                scale
                    .harmonize_down(m, *degree)
                    .unwrap_or_else(|| m.set_pitch(Tone::Rest, 4))
            })
            .collect();
        self
    }
}