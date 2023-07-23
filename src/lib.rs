use midi::{Midi};
use crate::dropout::Dropout;

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
pub mod scale;
pub mod tone;

pub trait Midibox {
    fn next(&mut self) -> Option<Vec<Midi>>;
    fn name(&self) -> Option<&str> {
        None
    }
}

pub struct NamedMidibox {
    name: String,
    delegate: Box<dyn Midibox>
}

impl NamedMidibox {
    fn wrap(name: &str, delegate: Box<dyn Midibox>) -> Box<dyn Midibox>{
        return Box::new(NamedMidibox {
          name: name.to_string(), delegate
        })
    }
}

impl Midibox for NamedMidibox {
    fn next(&mut self) -> Option<Vec<Midi>> {
        return self.delegate.next();
    }

    fn name(&self) -> Option<&str> {
        Some(&self.name)
    }
}