use midibox::{Bpm, Degree, Interval, Midi, Midibox, Scale, Tone};
use midibox::sequences::Seq;
use rand::Rng;
use midibox::player::{PlayerConfig, try_run};

struct RandomVelocity {
    factor: f64,
    midibox: Box<dyn Midibox>,
}

impl RandomVelocity {
    fn wrap(midibox: Box<dyn Midibox>) -> Box<dyn Midibox> {
        return Box::new(RandomVelocity {
            factor: 1 as f64,
            midibox
        })
    }
}

impl Midibox for RandomVelocity {
    fn update(&mut self) {
        let v = rand::thread_rng().gen_range(0..99);
        self.factor = (v as f64) / (100 as f64);
    }

    fn get(&self, i: usize) -> Option<Vec<Midi>> {
        return self.midibox.get(i)
            .map(|it|
                it.into_iter()
                    .map(|note| {
                        note.set_velocity((note.velocity as f64 * self.factor) as u8)
                    }).collect::<Vec<Midi>>()
            );
    }

    fn len(&self) -> usize {
        return self.midibox.len();
    }
}

fn main() {
    let scale = Scale::major(Tone::Gb);

    let s1 = Seq::new(vec![
        Tone::G.oct(2)  * 128,
        Tone::B.oct(2)  * 128,
        Tone::E.oct(2)  * 128,
        Tone::D.oct(2)  * 128,
        Tone::C.oct(2)  * 128,
        Tone::E.oct(2)  * 128,
        Tone::B.oct(2)  * 128,
        Tone::C.oct(2)  * 128,
    ]).transpose_down(Interval::Min2);

    try_run(
        PlayerConfig::for_port(2),
        Bpm::new(600),
        &mut vec![
            s1.clone().split_notes(vec![true, false, false]),
            s1.clone().harmonize_down(&scale, Degree::Fourth).split_notes(vec![false, true, false]),
            s1.clone().harmonize_up(&scale, Degree::Tenth).split_notes(vec![true, false, true]),
            s1.clone().harmonize_up(&scale, Degree::Seventh).split_notes(vec![false, false, false, true]),
        ].into_iter().map(|seq| RandomVelocity::wrap(seq.midibox())).collect()
    ).unwrap()
}
