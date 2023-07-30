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
}

pub struct Map<T>
    where T: Fn(Midi) -> Midi
{
    mapper: T,
    midibox: Box<dyn Midibox>,
}

impl<F: Fn(Midi) -> Midi + 'static> Map<F> {
    pub fn wrap(midibox: Box<dyn Midibox>, mapper: F) -> Box<dyn Midibox> {
        Box::new(Map {
            mapper,
            midibox
        })
    }
}

impl <F: Fn(Midi) -> Midi> Midibox for Map<F> {
    fn next(&mut self) -> Option<Vec<Midi>> {
        self.midibox.next()
            .map(|it|
                it.into_iter().map(|note| (self.mapper)(note)).collect::<Vec<Midi>>()
            )
    }
}