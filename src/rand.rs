use crate::{Map, Midibox};
use rand::Rng;
use crate::midi::Midi;

pub fn random_velocity(midibox: Box<dyn Midibox>) -> Box<dyn Midibox> {
    Map::wrap(midibox, |m| {
        let v = rand::thread_rng().gen_range(0..99);
        let factor = (v as f64) / (100_f64);
        m.set_velocity((m.velocity as f64 * factor) as u8)
    })
}