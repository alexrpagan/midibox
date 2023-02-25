use midibox::{Bpm, Degree, Interval, Scale, Tone};
use midibox::sequences::Seq;
use midibox::rand::RandomVelocity;
use midibox::player::{PlayerConfig, try_run};


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
        PlayerConfig::for_port(0),
        &Bpm::new(600),
        &mut vec![
            s1.clone().split_notes(vec![true, false, false]),
            s1.clone().harmonize_down(&scale, Degree::Fourth).split_notes(vec![false, true, false]),
            s1.clone().harmonize_up(&scale, Degree::Tenth).split_notes(vec![true, false, true]),
            s1.clone().harmonize_up(&scale, Degree::Seventh).split_notes(vec![false, false, false, true]),
        ].into_iter().map(|seq| RandomVelocity::wrap(seq.midibox())).collect()
    ).unwrap()
}
