use std::collections::HashMap;
use rand::Rng;
use midibox::drumlogue::Drumlogue::{*};
use midibox::tone::Tone;
use midibox::meter::Bpm;
use midibox::sequences::Seq;
use midibox::player::{PlayerConfig, try_run};
use midibox::scale::{Degree, Direction, Interval, Scale};
use midibox::{Map, MapChord, Midibox, seq};
use midibox::chord::{Chord, ToChord};
use midibox::dropout::random_dropout;
use midibox::drumlogue::Drumlogue;
use midibox::midi::ToMidi;
use midibox::rand::{random_velocity, random_velocity_range};
use midibox::router::MapRouter;
use midibox::scale::Degree::{*};
use midibox::scale::Pitch::{*};
use midibox::scale::Direction::{*};
use midibox::tone::Tone::{C, D, E, F};

fn main() {
    env_logger::init();

    let mut chan_to_port = HashMap::new();
    for i in 0..2 {
        chan_to_port.insert(i, 0);
    }
    chan_to_port.insert(2, 1);
    chan_to_port.insert(3, 2);
    chan_to_port.insert(4, 3);

    let pattern_left = &vec![
        false, false, false, true, false, false,
        true, false, false, false, true, false,
        false, false, false, true, false, false,
        true, false, false, false, false, true,
        false, false, false, true, false, false,
        true, false, false, false, true, false,
        false, false, false, true, false, true,
        true, false, false, false, true, false,
    ];

    let pattern_right = &vec![
        true, false, false, true, false, false,
        false, false, false, false, true, false,
        true, false, false, true, false, false,
        false, true, false, false, true, false,
        true, false, false, true, false, false,
        false, false, false, false, true, false,
        true, false, false, true, false, true,
        false, true, false, false, false, true,
    ];

    try_run(
        PlayerConfig::from_router(Box::new(MapRouter::new(chan_to_port))),
        &mut Bpm::new(550),
        &mut vec![
            drum(),
            hat_accent(),
            chords(pattern_left),
            chords(pattern_right),
            bass()
        ]
    ).unwrap()
}

fn bass() -> Box<dyn Midibox> {
    let scale = Scale::major(F);
    seq![
        scale.make_chord(
            2,
            Second,
            &vec![
                Harmonize(Unison, UpShiftOct(0)),
            ]
        ).unwrap(),
        scale.make_chord(
            2,
            Sixth,
            &vec![
                Harmonize(Fifth, UpShiftOct(0)),
            ]
        ).unwrap(),
            scale.make_chord(
            2,
            Second,
            &vec![
                Harmonize(Third, UpShiftOct(0)),
            ]
        ).unwrap(),
        scale.make_chord(
            2,
            Sixth,
            &vec![
                Harmonize(Unison, UpShiftOct(0)),
            ]
        ).unwrap(),
        scale.make_chord(
            2,
            Fourth,
            &vec![
                Harmonize(Unison, UpShiftOct(0)),
            ]
        ).unwrap(),
        scale.make_chord(
            2,
            Third,
            &vec![
                Harmonize(Third, UpShiftOct(0)),
            ]
        ).unwrap(),
        scale.make_chord(
            3,
            Second,
            &vec![
                Harmonize(Unison, UpShiftOct(0)),
            ]
        ).unwrap(),
        scale.make_chord(
            2,
            Sixth,
            &vec![
                Harmonize(Unison, UpShiftOct(0)),
            ]
        ).unwrap()

    ].duration(32).midibox()
}

fn chords(pattern: &Vec<bool>) -> Box<dyn Midibox> {
    let scale = Scale::major(F);
    let chords = seq![
        scale.make_chord(
            3,
            Second,
            &vec![
                Harmonize(Unison, UpShiftOct(-2)),
                Harmonize(Third, Up),
                Harmonize(Fifth, UpShiftOct(-1)),
                Harmonize(Ninth, Up)
            ]
        ).unwrap(),
        scale.make_chord(
            3,
            Sixth,
            &vec![
                Harmonize(Unison, UpShiftOct(-2)),
                Harmonize(Third, UpShiftOct(-1)),
                Harmonize(Fifth, UpShiftOct(-1)),
                Harmonize(Ninth, Up)
            ]
        ).unwrap(),
            scale.make_chord(
            3,
            Second,
            &vec![
                Harmonize(Unison, UpShiftOct(-2)),
                Harmonize(Third, Up),
                Harmonize(Fifth, UpShiftOct(-1)),
                Harmonize(Ninth, Up)
            ]
        ).unwrap(),
        scale.make_chord(
            2,
            Sixth,
            &vec![
                Harmonize(Unison, UpShiftOct(-2)),
                Harmonize(Third, Up),
                Harmonize(Fifth, UpShiftOct(-1)),
                Harmonize(Ninth, Up)
            ]
        ).unwrap(),
        scale.make_chord(
            3,
            Fourth,
            &vec![
                Harmonize(Unison, UpShiftOct(-1)),
                Harmonize(Third, UpShiftOct(1)),
            ]
        ).unwrap(),
        scale.make_chord(
            3,
            Third,
            &vec![
                Harmonize(Unison, Up),
                Harmonize(Third, UpShiftOct(1)),
            ]
        ).unwrap(),
        scale.make_chord(
            3,
            Second,
            &vec![
                Harmonize(Unison, UpShiftOct(-2)),
                Harmonize(Third, Up),
                Harmonize(Fifth, UpShiftOct(-1)),
                Harmonize(Ninth, Up)
            ]
        ).unwrap(),
        scale.make_chord(
            2,
            Sixth,
            &vec![
                Harmonize(Unison, UpShiftOct(-2)),
                Harmonize(Third, Up),
                Harmonize(Fifth, UpShiftOct(-1)),
                Harmonize(Ninth, Up)
            ]
        ).unwrap()

    ].duration(32).split_notes(pattern).midibox();

    let velocity = random_velocity_range(
        chords, 50, 100
    );
    let random_dropout = random_dropout(velocity, 0.01);
    MapChord::wrap(random_dropout, |c| {
        let i_1 = rand::thread_rng().gen_range(0..c.notes.len());
        let i_2 = rand::thread_rng().gen_range(0..c.notes.len());
        Chord::new(vec![
            *c.notes.get(i_1).unwrap(),
            *c.notes.get(i_2).unwrap(),
        ])
    })
}

fn hat_accent() -> Box<dyn Midibox> {
    seq![
        Tone::Rest * 16,
        Drumlogue::OH,
        Tone::Rest * 15,
        Tone::Rest * 16,
        Drumlogue::OH,
        Tone::Rest * 9,
        Drumlogue::OH,
        Tone::Rest * 3,
        Drumlogue::OH,
        Tone::Rest * 1
    ].midibox()
}

fn drum() -> Box<dyn Midibox> {
    Map::wrap(
    seq![
        BD,
        CH,
        BD,
        CH,
        RS,
        CH,
        CH,
        BD,
        BD,
        RS,
        CH,
        BD,
        RS,
        CH,
        CH,
        CH,
        BD,
        CH,
        CH,
        BD,
        RS,
        CH,
        CH,
        BD,
        CH,
        RS,
        CH,
        BD,
        RS,
        CH,
        BD,
        CH
    ].midibox(),
        |m| {
            if m.tone == CH.midi().tone {
                return m.set_velocity(rand::thread_rng().gen_range(50..70))
            }
            m
        }
    )
}