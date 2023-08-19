use std::collections::HashMap;
use rand::Rng;
use midibox::drumlogue::Drumlogue::{*};
use midibox::tone::Tone;
use midibox::meter::Bpm;
use midibox::sequences::Seq;
use midibox::player::{PlayerConfig, try_run};
use midibox::scale::{Degree, Direction, Interval, Scale};
use midibox::{map_chords, map_notes, Midibox, seq};
use midibox::arp::Arpeggio;
use midibox::chord::{Chord, ToChord};
use midibox::dropout::random_dropout;
use midibox::drumlogue::Drumlogue;
use midibox::midi::ToMidi;
use midibox::rand::{random_velocity, random_velocity_range};
use midibox::router::MapRouter;
use midibox::scale::Degree::{*};
use midibox::scale::Pitch::{*};
use midibox::scale::Direction::{*};
use midibox::tone::Tone::{C, D, E, Eb, F, Rest};

fn main() {
    env_logger::init();

    let mut chan_to_port = HashMap::new();
    chan_to_port.insert(0, 0);
    chan_to_port.insert(1, 0);
    chan_to_port.insert(2, 0);
    chan_to_port.insert(3, 0);
    try_run(
        PlayerConfig::from_router(Box::new(MapRouter::new(chan_to_port))),
        &mut Bpm::new(300),
        &mut vec![
            bass(scale()),
            Arpeggio::ascend(base_chords(scale()), 4),
            Arpeggio::ascend(harm(scale(), 3), 3),
            Arpeggio::descend(harm(scale(), 4), 2),
        ]
    ).unwrap()
}

fn scale() -> Scale {
    Scale::major(Eb)
}

fn bass(scale: Scale) -> Box<dyn Midibox> {
    seq![
        scale.make_chord(
            2,
            Third,
            &vec![
                Harmonize(Third, UpOct(-1)),
                Harmonize(Fifth, Up),
            ]
        ).unwrap(),

        scale.make_chord(
            2,
            Fourth,
            &vec![
                Harmonize(Third, UpOct(-1)),
                Harmonize(Fifth, Up),
            ]
        ).unwrap(),

        scale.make_chord(
            2,
            Unison,
            &vec![
                Harmonize(Third, UpOct(-1)),
                Harmonize(Fifth, Up),
            ]
        ).unwrap()

    ].duration(64).split_notes(&vec![true, false, false, false, false]).midibox()
}

fn base_chords(scale: Scale) -> Seq {
    seq![
        scale.make_chord(
            4,
            Third,
            &vec![
                Harmonize(Third, UpOct(-1)),
                Harmonize(Unison, UpOct(1)),
                Harmonize(Fifth, Up),
            ]
        ).unwrap(),

        scale.make_chord(
            4,
            Fourth,
            &vec![
                Harmonize(Third, UpOct(-1)),
                Harmonize(Unison, UpOct(1)),
                Harmonize(Fifth, Up),
            ]
        ).unwrap(),

        scale.make_chord(
            4,
            Unison,
            &vec![
                Harmonize(Third, UpOct(-1)),
                Harmonize(Unison, UpOct(1)),
                Harmonize(Fifth, Up),
            ]
        ).unwrap()

    ].duration(64)
}

fn harm(scale: Scale, oct: u8) -> Seq {
    seq![
        scale.make_chord(
            oct,
            Third,
            &vec![
                Harmonize(Fifth, UpOct(-1)),
                Harmonize(Third, Up)
            ]
        ).unwrap(),

        scale.make_chord(
            oct,
            Fourth,
            &vec![
                Harmonize(Seventh, Up),
                Harmonize(Third, Up)
            ]
        ).unwrap(),

        scale.make_chord(
            oct,
            Unison,
            &vec![
                Harmonize(Fifth, UpOct(-1)),
                Harmonize(Seventh, Up),
                Harmonize(Ninth, Up)
            ]
        ).unwrap()

    ].duration(64)
}
