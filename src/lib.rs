use midi::{Midi};

pub mod sequences;
pub mod player;
pub mod router;
pub mod drumlogue;
pub mod rand;
pub mod midi;
pub mod chord;
pub mod meter;
pub mod scale;
pub mod tone;

pub trait Midibox {
    fn next(&mut self) -> Option<Vec<Midi>>;
}