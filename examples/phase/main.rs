use std::collections::HashMap;
use std::ops::Add;
use log::warn;
use tonic::transport::server::Router;
use midibox::arp::Arpeggio;

use midibox::{chord, Midibox, seq};
use midibox::chord::{Chord, ToChord};
use midibox::dropout::Dropout;
use midibox::meter::Bpm;
use midibox::midi::{MutMidi, ToMidi};
use midibox::sequences::Seq;
use midibox::player::{PlayerConfig, try_run};
use midibox::rand::RandomVelocity;
use midibox::router::MapRouter;
use midibox::scale::Interval::Oct;
use midibox::scale::{Degree, Interval, Scale};
use midibox::tone::Tone;

struct DropoutSpec {
    duration: u32,
    started: bool
}

fn bass(chord: Chord) -> Box<dyn Midibox> {
    let scale = &Scale::major(Tone::E);
    let roots = seq![chord].duration(30).repeat(4).add(seq![chord].duration(20)).transpose_down(Oct);
    let seq  =
            roots.clone().harmonize_up(scale, Degree::Sixth) +
            roots.clone().harmonize_up(scale, Degree::Fourth) +
            roots.clone() +
            roots.clone().harmonize_up(scale, Degree::Sixth) +
            roots.clone().harmonize_up(scale, Degree::Fourth) +
            roots.clone() +
            roots.clone().harmonize_up(scale, Degree::Sixth) +
            roots.clone().harmonize_up(scale, Degree::Fourth) +
            roots.clone() +
            roots.clone().harmonize_up(scale, Degree::Sixth) +
            roots.clone().harmonize_up(scale, Degree::Third) +
            roots.clone();
    return seq.midibox();
}

fn get_voice(
    chord: Chord,
    note_duration: u32,
    velocity: u8,
    ascend: bool,
    dropout: Option<DropoutSpec>
) -> Box<dyn Midibox> {
    let scale = &Scale::major(Tone::E);

    let chord_inv1 = chord.clone().rotate_left(1);
    let chord_inv2 = chord.clone().rotate_left(1);

    let seq = seq![
            chord_inv1.clone().harmonize_up(scale, &Degree::Sixth),
            chord_inv1.clone().harmonize_up(scale, &Degree::Fourth),
            chord_inv1.clone(),
            chord_inv2.clone().harmonize_up(scale, &Degree::Sixth),
            chord_inv2.clone().harmonize_up(scale, &Degree::Fourth),
            chord_inv2.clone(),
            chord.clone().harmonize_up(scale, &Degree::Sixth),
            chord.clone().harmonize_up(scale, &Degree::Fourth),
            chord.clone(),
            chord.clone().harmonize_up(scale, &Degree::Sixth),
            chord.clone().harmonize_up(scale, &Degree::Third),
            chord.clone()
        ].velocity(velocity).duration(140); // least common multiple of 4,5,6,7 is 420 so divide by 3

    let with_arp =  RandomVelocity::wrap(
        match ascend {
            true => Arpeggio::ascend(seq, note_duration),
            false => Arpeggio::descend(seq, note_duration)
        }
    );

    return match dropout {
        None => with_arp,
        Some(spec) => Dropout::wrap(with_arp, spec.duration, spec.started)
    }
}

fn main() {
    env_logger::init();

    let e5 =
        chord![
            Tone::E.oct(3),
            Tone::B.oct(3)
        ];

    let e_maj_inv =
        chord![
            Tone::B.oct(2),
            Tone::Ab.oct(3)
        ];

    let esus =
        chord![
            Tone::E.oct(3),
            Tone::B.oct(3),
            Tone::Gb.oct(3),
            Tone::Db.oct(3)
        ];

    let g_sharp_5 =
        chord![
            Tone::Ab.oct(4),
            Tone::Eb.oct(4),
            Tone::B.oct(4)
        ];

    let mut channel_to_port: HashMap<usize, usize> = HashMap::new();
    channel_to_port.insert(0, 2);
    for i in 1..13 {
        channel_to_port.insert(i, 0);
    }

    try_run(
        PlayerConfig::from_router(Box::new(MapRouter::new(channel_to_port))),
        &Bpm::new(300),
        &mut vec![
            Dropout::wrap(bass(chord![
            Tone::E.oct(2),
            Tone::B.oct(2)
        ]), 140, false),
            Dropout::wrap(bass(chord![
            Tone::E.oct(2),
            Tone::B.oct(2),
            Tone::Ab.oct(3)
        ]), 280, true),
            get_voice(
                e_maj_inv.clone(),
                6,
                20,
                true,
                Some(DropoutSpec{ duration: 70, started: false})
            ),
            get_voice(
                e5.clone(),
                4,
                20,
                true,
               Some(DropoutSpec{ duration: 70, started: true})
            ),
            get_voice(
                esus.clone(),
                7,
                20,
                true,
                Some(DropoutSpec{ duration: 70, started: true})
            ),
            get_voice(
                g_sharp_5.clone(),
                5,
                20,
                true,
                None
            ),
            get_voice(
                g_sharp_5.clone(),
                2,
                20,
                false,
                Some(DropoutSpec{ duration: 140, started: false})
            ),
            get_voice(
                g_sharp_5.clone(),
                3,
                25,
                true,
                Some(DropoutSpec{ duration: 280, started: false})
            ),
            get_voice(e_maj_inv.clone(), 12, 20, false, None),
            get_voice(e5.clone().transpose_up(&Oct),8, 20, false, None),
            get_voice(
                esus.clone(),
                14,
                20,
                false,
                Some(DropoutSpec{ duration: 70, started: false})
            ),
            get_voice(
                g_sharp_5.clone().transpose_down(&Oct),
                10,
                20,
                false,
                Some(DropoutSpec { duration: 140, started: false})
            )
        ]
    ).unwrap()
}
