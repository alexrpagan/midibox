use midi::{Midi};
use crate::chord::Chord;
use crate::dropout::Dropout;
use crate::map::{Map, MapChord};
use crate::scale::Interval::Perf5;

pub mod sequences;
pub mod router;
pub mod dropout;
pub mod drumlogue;
pub mod rand;
pub mod arp;
pub mod midi;
pub mod player;
pub mod chord;
pub mod meter;
pub mod map;
pub mod scale;
pub mod tone;

pub trait Midibox {
    fn next(&mut self) -> Option<Vec<Midi>>;
}


// Common utility functions:

pub fn map_notes<F>(around: Box<dyn Midibox>, f: F) -> Box<dyn Midibox>
    where F: Fn(Midi) -> Midi + 'static
{
    Map::wrap(around, f)
}

pub fn map_chords<F>(around: Box<dyn Midibox>, f: F) -> Box<dyn Midibox>
    where F: Fn(Chord) -> Chord + 'static
{
    MapChord::wrap(around, f)
}