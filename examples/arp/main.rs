use std::collections::HashMap;
use log::warn;
use midibox::arp::Arpeggio;

use midibox::{chord, seq};
use midibox::chord::{Chord, ToChord};
use midibox::meter::Bpm;
use midibox::midi::{MutMidi, ToMidi};
use midibox::sequences::Seq;
use midibox::player::{PlayerConfig, try_run};
use midibox::rand::RandomVelocity;
use midibox::router::MapRouter;
use midibox::scale::Interval::Oct;
use midibox::scale::{Degree, Interval, Scale};
use midibox::tone::Tone;


fn main() {
    env_logger::init();
    let emaj_scale = Scale::major(Tone::E);

    let texture =
        chord![
            Tone::E.oct(3),
            Tone::Eb.oct(3),
            Tone::Gb.oct(3),
            Tone::Ab.oct(3),
            Tone::E.oct(3),
            Tone::Eb.oct(3),
            Tone::Ab.oct(3)
        ];

    let bass =
        chord![
            Tone::E.oct(3),
            Tone::B.oct(3),
            Tone::Ab.oct(2)
        ];

    let texture_seq = seq![
        texture.clone().harmonize_down(&emaj_scale, &Degree::Third),
        texture.clone(),
        texture.clone().harmonize_up(&emaj_scale, &Degree::Second),
        texture.clone().harmonize_up(&emaj_scale, &Degree::Fourth),
        texture.clone(),
        texture.clone().harmonize_up(&emaj_scale, &Degree::Third)
    ].duration(200);

    let chord_seq = seq![
        bass.clone().harmonize_down(&emaj_scale, &Degree::Third),
        bass.clone(),
        bass.clone().harmonize_up(&emaj_scale, &Degree::Second),
        bass.clone().harmonize_up(&emaj_scale, &Degree::Fourth),
        bass.clone(),
        bass.clone().harmonize_up(&emaj_scale, &Degree::Third)
    ];

    try_run(
        PlayerConfig::for_port(0),
        &Bpm::new(800),
        &mut vec![
            RandomVelocity::wrap(Arpeggio::ascend(
                texture_seq.clone().velocity(30),
                4,
            )),
            RandomVelocity::wrap(Arpeggio::descend(
                texture_seq.clone().velocity(40),
                12
            )),
            RandomVelocity::wrap(Arpeggio::ascend(
                texture_seq.clone().velocity(30) + Oct,
                20
            )),
            RandomVelocity::wrap(Arpeggio::ascend(
                texture_seq.clone().velocity(30) + Oct + Oct,
                20
            )),
            Arpeggio::ascend(
                chord_seq.clone().velocity(50) - Oct,
                40
            ),
            Arpeggio::ascend(
                chord_seq.clone().velocity(50) + Oct,
                40
            ),
        ]
    ).unwrap()
}
