use crate::{Midi, Midibox};
use rand::Rng;

pub struct RandomVelocity {
    factor: f64,
    midibox: Box<dyn Midibox>,
}

impl RandomVelocity {
    pub fn wrap(midibox: Box<dyn Midibox>) -> Box<dyn Midibox> {
        return Box::new(RandomVelocity {
            factor: 1 as f64,
            midibox
        })
    }
}

impl Midibox for RandomVelocity {
    fn next(&mut self) -> Option<Vec<Midi>> {
        let v = rand::thread_rng().gen_range(0..99);
        self.factor = (v as f64) / (100 as f64);
        return self.midibox.next()
            .map(|it|
                it.into_iter()
                    .map(|note| {
                        note.set_velocity((note.velocity as f64 * self.factor) as u8)
                    }).collect::<Vec<Midi>>()
            );
    }
}
