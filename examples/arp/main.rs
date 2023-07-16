use log::warn;
use midibox::arp::Arpeggio;

use midibox::{chord, seq};
use midibox::chord::{Chord, ToChord};
use midibox::meter::Bpm;
use midibox::midi::{MutMidi, ToMidi};
use midibox::sequences::Seq;
use midibox::player::{PlayerConfig, try_run};
use midibox::scale::Interval::Oct;
use midibox::tone::Tone;


fn main() {
    env_logger::init();

    let s1 = seq![
        chord![
            Tone::C.oct(3) * 128,
            Tone::E.oct(3),
            Tone::B.oct(3),
            Tone::D.oct(4),
            Tone::Gb.oct(5)
        ].velocity(70),

        chord![
            Tone::A.oct(2) * 128,
            Tone::C.oct(3),
            Tone::E.oct(3),
            Tone::G.oct(3),
            Tone::B.oct(3)
        ].velocity(40)
    ];

    try_run(
        PlayerConfig::for_port(0),
        &Bpm::new(2000),
        &mut vec![
            Arpeggio::custom_order(
                s1.clone() + Oct,
                10,
                vec![0, 1, 2, 3, 4, 1, 4, 0, 1, 2, 3, 4, 0, 3]
            ),
            Arpeggio::descend(s1.clone(), 20)
        ]
    ).unwrap()
}
