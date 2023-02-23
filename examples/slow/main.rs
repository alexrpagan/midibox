use midibox::{Bpm, Degree, Interval, Scale, Tone};
use midibox::sequences::Seq;
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
        PlayerConfig::for_port(2),
        Bpm::new(2500),
        vec![
            s1.clone(),
            s1.clone().harmonize_down(&scale, Degree::Fourth),
            s1.clone().harmonize_up(&scale, Degree::Tenth),
            s1.clone().harmonize_up(&scale, Degree::Seventh)
        ].into_iter().map(|seq| seq.midibox()).collect()
    ).unwrap()
}
