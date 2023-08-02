use std::collections::HashMap;
use rand::Rng;
use midibox::drumlogue::Drumlogue::{*};
use midibox::tone::Tone;
use midibox::meter::Bpm;
use midibox::sequences::Seq;
use midibox::player::{PlayerConfig, try_run};
use midibox::scale::{Degree, Direction, Interval, Scale};
use midibox::{map_chords, map_notes, Midibox, seq};
use midibox::chord::{Chord, ToChord};
use midibox::dropout::random_dropout;
use midibox::drumlogue::Drumlogue;
use midibox::midi::ToMidi;
use midibox::rand::{random_velocity, random_velocity_range};
use midibox::router::MapRouter;
use midibox::scale::Degree::{*};
use midibox::scale::Pitch::{*};
use midibox::scale::Direction::{*};
use midibox::tone::Tone::{C, D, E, F, Rest};

fn main() {
    env_logger::init();

    let mut chan_to_port = HashMap::new();
    for i in 0..2 {
        chan_to_port.insert(i, 0);
    }
    chan_to_port.insert(2, 4);
    chan_to_port.insert(3, 1);
    chan_to_port.insert(4, 2);
    chan_to_port.insert(5, 3);

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
            hats(),
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
            Second,
            &vec![
                Harmonize(Unison, UpShiftOct(-2)),
                Harmonize(Third, UpShiftOct(-1)),
                Harmonize(Fourth, Up),
                Harmonize(Second, Up),
                Harmonize(Ninth, Up),
                Harmonize(Seventh, UpShiftOct(-1)),
            ]
        ).unwrap(),
        scale.make_chord(
            3,
            Sixth,
            &vec![
                Harmonize(Unison, UpShiftOct(-2)),
                Harmonize(Third, UpShiftOct(-1)),
                Harmonize(Fifth, UpShiftOct(-1)),
                Harmonize(Second, Up),
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
                Harmonize(Third, UpShiftOct(-1)),
                Harmonize(Third, UpShiftOct(1)),
            ]
        ).unwrap(),
        scale.make_chord(
            3,
            Third,
            &vec![
                Harmonize(Unison, Up),
                Harmonize(Third, UpShiftOct(-1)),
                Harmonize(Third, UpShiftOct(1)),
            ]
        ).unwrap(),
        scale.make_chord(
            3,
            Second,
            &vec![
                Harmonize(Unison, UpShiftOct(-2)),
                Harmonize(Third, UpShiftOct(-1)),
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
        chords, 50, 110
    );

    let random_dropout = random_dropout(velocity, 0.01);
    map_chords(random_dropout, |c| {
        let i_1 = rand::thread_rng().gen_range(0..c.notes.len());
        let i_2 = rand::thread_rng().gen_range(0..c.notes.len());
        let i_3 = rand::thread_rng().gen_range(0..c.notes.len());
        Chord::new(vec![
            *c.notes.get(i_1).unwrap(),
            *c.notes.get(i_2).unwrap(),
            *c.notes.get(i_3).unwrap(),
        ])
    })
}

fn hat_accent() -> Box<dyn Midibox> {
    seq![
        Tone::Rest * 16,
        Drumlogue::OH,
        CH,
        Tone::Rest * 14,
        Tone::Rest * 16,
        Drumlogue::OH,
        CH,
        Tone::Rest * 8,
        Drumlogue::OH,
        Tone::Rest * 3,
        Drumlogue::OH,
        Tone::Rest * 1
    ].midibox()
}

fn drum() -> Box<dyn Midibox> {
    let seq = seq![
        BD,
        Rest,
        BD,
        Rest,
        RS,
        Rest,
        Rest,
        BD,
        BD,
        RS,
        Rest,
        BD,
        RS,
        Rest,
        Rest,
        Rest,
        BD,
        Rest,
        Rest,
        BD,
        RS,
        Rest,
        Rest,
        BD,
        Rest,
        RS,
        Rest,
        BD,
        RS,
        Rest,
        BD,
        Rest
    ].midibox();

    map_notes(
        seq,
        |m| {
            if m.tone == BD.midi().tone {
                return m.set_velocity(rand::thread_rng().gen_range(100..120))
            }
            if m.tone == RS.midi().tone {
                return m.set_velocity(rand::thread_rng().gen_range(100..120))
            }
            m
        }
    )
}

fn hats() -> Box<dyn Midibox> {
    let seq = seq![
        SP1,
        CH,
        SP1,
        CH,
        SP1,
        CH,
        CH,
        SP1,
        SP1,
        SP1,
        CH,
        SP1,
        SP1,
        CH,
        CH,
        CH,
        SP1,
        CH,
        CH,
        SP1,
        SP1,
        CH,
        CH,
        SP1,
        CH,
        SP1,
        CH,
        SP1,
        SP1,
        CH,
        SP1,
        CH
    ].midibox();

    map_notes(
        seq,
        |m| {
            if m.tone == CH.midi().tone {
                return m.set_velocity(rand::thread_rng().gen_range(50..70))
            }
            if m.tone == SP1.midi().tone {
                return m.set_velocity(rand::thread_rng().gen_range(30..40))
            }
            m
        }
    )
}