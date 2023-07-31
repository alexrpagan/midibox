use std::collections::HashMap;
use std::time::Duration;
use rand::Rng;
use tonic::transport::server::Router;
use midibox::tone::Tone::{*};
use midibox::meter::{Bpm, Oscillate};
use midibox::sequences::Seq;
use midibox::player::{PlayerConfig, try_run};
use midibox::scale::{Degree, Interval, Scale};
use midibox::{chord, Map, Midibox, seq};
use midibox::arp::Arpeggio;
use midibox::chord::ToChord;
use midibox::dropout::random_dropout;
use midibox::drumlogue::Drumlogue;
use midibox::drumlogue::Drumlogue::{*};
use midibox::midi::MutMidi;
use midibox::midi::ToMidi;
use midibox::rand::{random_velocity};
use midibox::router::MapRouter;
use midibox::scale::Degree::{Fifth, Third, Unison};
use midibox::scale::Direction::{Up, UpShiftOct};
use midibox::scale::Pitch::Harmonize;



fn main() {
    env_logger::init();

    let mut channel_to_port = HashMap::new();
    channel_to_port.insert(0, 0);
    for i in 1..4 {
        channel_to_port.insert(i, 1);
    }

    try_run(
        PlayerConfig::from_router(Box::new(MapRouter::new(channel_to_port))),
        &mut Oscillate::new(175, 275, 0.25),
        &mut vec![
            random_dropout(drum_midibox(), 0.05),
            random_dropout(bass_midibox(), 0.2),
            random_dropout(phase_with_arp(true, 5, false), 0.05),
            random_dropout(phase_with_arp(false, 4, true), 0.1),
        ]
    ).unwrap()
}

fn bass(scale: Scale, base_oct8: u8) -> Seq {
    seq![
        scale.make_chord(
            base_oct8,
            Degree::Fourth,
            &vec![
                Harmonize(Third, Up),
            ]
        ).unwrap(),

        scale.make_chord(
            base_oct8,
            Degree::Fourth,
            &vec![
                Harmonize(Unison, Up),
            ]
        ).unwrap(),

        scale.make_chord(
            base_oct8,
            Degree::Third,
            &vec![
                Harmonize(Unison, Up),
            ]
        ).unwrap()
    ]
        .duration(64)
        .velocity(50)
}

fn bass_midibox() -> Box<dyn Midibox> {
    let seq =
        (
            bass(Scale::major(C), 2) +
            bass(Scale::major(D), 2)
        );
    return seq.split_notes(&vec![false, true, false, false, true]).midibox()
}

fn drum_midibox() -> Box<dyn Midibox> {
    Map::wrap(
        seq![
        BD,
        SP1,
        SP1,
        BD,
        OH,
        BD,
        SP1,
        SP1,
        RS,
        BD,
        SP1,
        SP1,
        BD,
        OH,
        BD,
        BD,
        SP1,
        SP1,
        SP1,
        SP1,
        BD,
        SP1,
        SP1,
        BD,
        RS,
        SP1,
        SP1,
        SP1,
        OH,
        BD
    ].midibox(),
        |m| {
            let mut rng = rand::thread_rng();
            if m.tone == BD.midi().tone {
                m.set_velocity(rng.gen_range(75..80))
            } else if m.tone == SP1.midi().tone {
                m.set_velocity(rng.gen_range(50..80))
            } else if m.tone == OH.midi().tone {
                m.set_velocity(rng.gen_range(20..40))
            } else if m.tone == RS.midi().tone {
                m.set_velocity(rng.gen_range(20..40))
            } else {
                m
            }
        }
    )
}

fn phase_with_arp(arp_up: bool, note_duration: u32, inv: bool) -> Box<dyn Midibox> {
    let seq =
        (primary_phase(Scale::major(C), 3, inv) +
        primary_phase(Scale::major(D), 3, inv)).duration(64).velocity(50);

    random_velocity(if arp_up {
        Arpeggio::ascend(seq, note_duration)
    } else {
        Arpeggio::descend(seq, note_duration)
    })
}

fn primary_phase(scale: Scale, base_oct: u8, inv: bool) -> Seq {
    let rotate = if inv { 3 } else { 0 };

    seq![
        scale.make_chord(
            base_oct,
            Degree::Fourth,
            &vec![
                Harmonize(Degree::Fifth, UpShiftOct(1)),
                Harmonize(Degree::Seventh, Up),
                Harmonize(Degree::Third, UpShiftOct(-1)),
                Harmonize(Degree::Ninth, Up),
                Harmonize(Degree::Eleventh, Up),
                Harmonize(Degree::Unison, UpShiftOct(1))
            ]
        ).unwrap().rotate_left(rotate),

        scale.make_chord(
            base_oct - 1,
            Degree::Sixth,
            &vec![
                Harmonize(Degree::Fifth, UpShiftOct(1)),
                Harmonize(Degree::Third, UpShiftOct(1)),
                Harmonize(Degree::Second, UpShiftOct(1)),
                Harmonize(Degree::Seventh, Up),
            ]
        ).unwrap().rotate_left(rotate),

        scale.make_chord(
            base_oct,
            Degree::Unison,
            &vec![
                Harmonize(Degree::Fifth, UpShiftOct(1)),
                Harmonize(Degree::Third, UpShiftOct(1)),
                Harmonize(Degree::Seventh, Up),
                Harmonize(Degree::Ninth, Up),
                Harmonize(Degree::Unison, UpShiftOct(1))
            ]
        ).unwrap().rotate_left(rotate)
    ]
}