use std::ops::Deref;
use std::sync::Arc;
use rand::distributions::uniform::SampleRange;
use crate::{Map, map_notes, Midibox};
use rand::Rng;
use crate::midi::Midi;

pub fn random_velocity(midibox: Box<dyn Midibox>) -> Box<dyn Midibox> {
    map_notes(midibox, |m| {
        let v = rand::thread_rng().gen_range(0..99);
        let factor = (v as f64) / (100_f64);
        m.set_velocity((m.velocity as f64 * factor) as u8)
    })
}

pub fn random_velocity_range(
    midibox: Box<dyn Midibox>,
    min_velocity: u8,
    max_velocity: u8
) -> Box<dyn Midibox> {
    map_notes(midibox, move |m|
        m.set_velocity(rand::thread_rng().gen_range(min_velocity..max_velocity))
    )
}