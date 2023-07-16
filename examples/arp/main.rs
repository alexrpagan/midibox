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
    let s1 = seq![
        chord![
            Tone::G.oct(2) * 128,
            Tone::E.oct(3),
            Tone::B.oct(3)
        ].velocity(70),

        chord![
            Tone::C.oct(2) * 128,
            Tone::E.oct(3),
            Tone::A.oct(3)
        ].velocity(40),

        chord![
            Tone::D.oct(3) * 128,
            Tone::Gb.oct(3),
            Tone::A.oct(3)
        ].velocity(20),

        chord![
            Tone::D.oct(3) * 128,
            Tone::Gb.oct(3),
            Tone::B.oct(3)
        ].velocity(50)
    ];

    try_run(
        PlayerConfig::for_port(2),
        &Bpm::new(2000),
        &mut vec![
            Arpeggio::ascend(s1.clone() - Oct, 10)
        ]
    ).unwrap()
}
