use std::ops::Mul;
use crate::chord::{Chord, ToChord};
use crate::midi::{Midi, ToMidi};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Tone {
    Rest,
    C,
    Db,
    D,
    Eb,
    E,
    F,
    Gb,
    G,
    Ab,
    A,
    Bb,
    B,
}

impl Tone {
    pub fn from(val: u8) -> Tone {
        let pos = val % 12;
        match pos {
            0 => Tone::C,
            1 => Tone::Db,
            2 => Tone::D,
            3 => Tone::Eb,
            4 => Tone::E,
            5 => Tone::F,
            6 => Tone::Gb,
            7 => Tone::G,
            8 => Tone::Ab,
            9 => Tone::A,
            10 => Tone::Bb,
            11 => Tone::B,
            _ => Tone::Rest
        }
    }

    pub fn u8(&self, oct: u8) -> Option<u8> {
        let base = (oct + 1) * 12;
        match self {
            Tone::C => { Some(base) }
            Tone::Db => { Some(base + 1) }
            Tone::D => { Some(base + 2) }
            Tone::Eb => { Some(base + 3) }
            Tone::E => { Some(base + 4) }
            Tone::F => { Some(base + 5) }
            Tone::Gb => { Some(base + 6) }
            Tone::G => { Some(base + 7) }
            Tone::Ab => { Some(base + 8) }
            Tone::A => { Some(base + 9) }
            Tone::Bb => { Some(base + 10) }
            Tone::B => { Some(base + 11) }
            Tone::Rest => { None }
        }
    }

    pub fn get(&self) -> Midi {
        self.oct(4)
    }

    pub fn oct(&self, oct: u8) -> Midi {
        Midi::from_tone(*self, oct)
    }
}

impl ToChord for Tone {
    fn chord(&self) -> Chord {
        Chord::note(self.midi())
    }
}

impl ToMidi for Tone {
    fn midi(&self) -> Midi {
        // C should be set to middle C, i.e., C4
        self.oct(4)
    }
}

impl Mul<u32> for Tone {
    type Output = Midi;

    fn mul(self, rhs: u32) -> Self::Output {
        self.midi() * rhs
    }
}
