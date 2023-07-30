use std::collections::HashMap;
use tonic::transport::server::Router;
use midibox::tone::Tone::{*};
use midibox::meter::Bpm;
use midibox::sequences::Seq;
use midibox::player::{PlayerConfig, try_run};
use midibox::scale::{Degree, Interval, Scale};
use midibox::{chord, Midibox, seq};
use midibox::arp::Arpeggio;
use midibox::chord::ToChord;
use midibox::drumlogue::Drumlogue;
use midibox::drumlogue::Drumlogue::{*};
use midibox::midi::MutMidi;
use midibox::midi::ToMidi;
use midibox::rand::RandomVelocity;
use midibox::router::MapRouter;
use midibox::scale::Degree::{Fifth, Unison};
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
        &Bpm::new(175),
        &mut vec![
            drum_midibox(),
            bass_midibox(),
            phase_with_arp(true, 5, false),
            phase_with_arp(false, 4, true),
        ]
    ).unwrap()
}

fn bass(scale: Scale, base_oct8: u8) -> Seq {
    seq![
        scale.make_chord(
            base_oct8,
            Degree::Fourth,
            &vec![
                Harmonize(Unison, Up),
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
    seq![
        BD,
        SP1.set_velocity(50),
        SP1.set_velocity(75),
        BD,
        OH.set_velocity(15),
        BD,
        SP1.set_velocity(43),
        SP1,
        RS.set_velocity(25),
        BD,
        SP1.set_velocity(65),
        SP1.set_velocity(56),
        BD,
        OH.set_velocity(30),
        BD
    ].midibox()
}

fn phase_with_arp(arp_up: bool, note_duration: u32, inv: bool) -> Box<dyn Midibox> {
    let seq =
        (primary_phase(Scale::major(C), 3, inv) +
        primary_phase(Scale::major(D), 3, inv)).duration(64).velocity(50);

    RandomVelocity::wrap(if arp_up {
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