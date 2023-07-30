use std::collections::HashMap;
use midibox::tone::Tone;
use midibox::drumlogue::Drumlogue::{BD, CH, CP, HT, LT, OH, RS, SD};
use midibox::scale::Interval::Oct;
use midibox::meter::Bpm;
use midibox::sequences::Seq;
use midibox::player::{PlayerConfig, try_run};
use midibox::router::MapRouter;
use midibox::tone::Tone::Rest;
use midibox::midi::ToMidi;
use midibox::scale::Scale;

fn main() {
    let mut channel_id_to_port_id : HashMap<usize, usize> = HashMap::new();
    // Drumlogue H Bank 1
    for i in 0..8 {
        channel_id_to_port_id.insert(i, 1);
    }
    // Minilogue preset 440
    for i in 8..11 {
        channel_id_to_port_id.insert(i, 0);
    }

    let _scale = Scale::major(Tone::D);

    let bass_seq = Seq::new(vec![
        Tone::D.oct(2) * 16,
        Tone::E.oct(2) * 16,
        Tone::Gb.oct(2) * 16,
        Tone::G.oct(2) * 16,
    ]);

    let bass_mask = vec![
        true,
        false,
        false,
        true,
        false,

        true,
        false,
        true,
        true,
        false,
    ];

    try_run(
        PlayerConfig::from_router(Box::new(MapRouter::new(channel_id_to_port_id))),
        &mut Bpm::new(450),
        &mut vec![
            Seq::new(vec![
                RS.set_velocity(30) * 1,
                Rest * 2,
                Rest * 4
            ]),
            Seq::new(vec![
                Rest * 1,
                CH.set_velocity(15) * 2
            ]),
            Seq::new(vec![
                BD.set_velocity(100) * 8,
                BD.set_velocity(50) * 1,
                BD.set_velocity(70) * 1,
                Rest * 7,
                BD * 8,
                BD * 4,
                BD * 2,
                BD.set_velocity(15) * 1,
                BD.set_velocity(20) * 1,
            ]),
            Seq::new(vec![
                Rest * 4,
                SD * 4,
            ]),
            Seq::new(vec![
                Rest * 12,
                CP * 4,
            ]),
            Seq::new(vec![
                Rest * 8,
                HT.set_velocity(30) * 1,
                LT.set_velocity(70) * 1,
                Rest * 4
            ]),
            Seq::new(vec![
                Rest * 8,
                Rest * 2,
                BD.set_velocity(30) * 1,
                BD.set_velocity(100) * 1,
                Rest * 2
            ]),
            Seq::new(vec![
                Rest * 2,
                CH.set_velocity(40) * 2,
                Rest * 2,
                CH.set_velocity(44) * 2,
                Rest * 2,
                OH.set_velocity(42) * 1,
                CH.set_velocity(43) * 1,
            ]),
            Seq::new(vec![
                Tone::Gb.oct(3) * 8,
                Tone::D.oct(3) * 7,
                Tone::Gb.oct(2) * 6,
                Tone::B.oct(2) * 8,

                Tone::Gb.oct(3) * 4,
                Tone::G.oct(3) * (8 - 2 + 4),
                Tone::B.oct(2) * (8 + 2),
                Tone::A.oct(2) * 4,
            ]).transpose_up(Oct),
            bass_seq.clone().split_notes(&bass_mask),
        ].into_iter().map(|seq| seq.midibox()).collect()
    ).unwrap()
}
