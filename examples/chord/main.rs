use midibox::{Bpm, Degree, Interval, Scale, Tone};
use midibox::sequences::{Chord, Seq};
use midibox::player::{PlayerConfig, try_run};

fn main() {
    let scale = Scale::major(Tone::C);

    let chords = Seq::chords(vec![
        Chord::new(vec![
            Tone::G.oct(2) * 32,
            Tone::E.oct(3),
            Tone::B.oct(3),
        ]),
        Chord::new(vec![
            Tone::C.oct(2) * 32,
            Tone::E.oct(3),
            Tone::A.oct(3),
        ])
    ])
        .split_notes(&vec![true, false, true, true, false, false, true, false, true, false])
        .midibox();

    let roots = Seq::new(
        vec![
            Tone::C.oct(1) * 32,
            Tone::F.oct(1) * 20,
            Tone::A.oct(2) * 12
        ]
    ).midibox();

    try_run(
        PlayerConfig::for_port(0),
        &Bpm::new(500),
        &mut vec![
            chords, roots
        ]
    ).unwrap()
}
