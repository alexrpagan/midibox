use std::collections::HashMap;
use midibox::{Bpm, Tone};
use midibox::drumlogue::Drumlogue::{BD, CH, CP, HT, LT, OH, RS, SD};
use midibox::Interval::Oct;
use midibox::sequences::FixedSequence;
use midibox::player::{PlayerConfig, try_run};
use midibox::router::MapRouter;
use midibox::Tone::Rest;
use midibox::ToMidi;

fn main() {
    let mut channel_id_to_port_id : HashMap<usize, usize> = HashMap::new();
    // H Bank 1
    for i in 0..8 {
        channel_id_to_port_id.insert(i, 1);
    }
    // Minilogue preset 440
    channel_id_to_port_id.insert(8, 2);
    channel_id_to_port_id.insert(9, 2);

    try_run(
        PlayerConfig {
            router: Box::new(MapRouter::new(channel_id_to_port_id)),
        },
        Bpm::new(1800),
        vec![
            FixedSequence::new(vec![
                RS.midi().set_velocity(30) * 4,
                Rest.midi() * 8,
                Rest.midi() * 16
            ]),
            FixedSequence::new(vec![
                Rest.midi() * 4,
                CH.midi().set_velocity(15) * 8
            ]),
            FixedSequence::new(vec![
                BD.midi().set_velocity(100) * 32,
                BD.midi().set_velocity(50) * 4,
                BD.midi().set_velocity(70) * 4,
                Rest.midi() * 28,
                BD.midi() * 32,
                BD.midi() * 16,
                BD.midi() * 8,
                BD.midi().set_velocity(15) * 4,
                BD.midi().set_velocity(20) * 4,
            ]),
            FixedSequence::new(vec![
                Rest.midi() * 16,
                SD.midi() * 16,
            ]),
            FixedSequence::new(vec![
                Rest.midi() * 48,
                CP.midi() * 16,
            ]),
            FixedSequence::new(vec![
                Rest.midi() * 32,
                HT.midi().set_velocity(30) * 4,
                LT.midi().set_velocity(70) * 4,
                Rest.midi() * 16
            ]),
            FixedSequence::new(vec![
                Rest.midi() * 32,
                Rest.midi() * 8,
                BD.midi().set_velocity(30) * 4,
                BD.midi().set_velocity(100) * 4,
                Rest.midi() * 8
            ]),
            FixedSequence::new(vec![
                Rest.midi() * 8,
                CH.midi().set_velocity(40) * 8,
                Rest.midi() * 8,
                CH.midi().set_velocity(44) * 8,
                Rest.midi() * 8,
                OH.midi().set_velocity(42) * 4,
                CH.midi().set_velocity(43) * 4,
            ]),
            FixedSequence::new(vec![
                Tone::Gb.oct(3) * 32,
                Tone::D.oct(3) * (32 - 8),
                Tone::Gb.oct(2) * (16 + 8),
                Tone::B.oct(2) * 32,

                Tone::Gb.oct(3) * 16,
                Tone::G.oct(3) * (32 - 8 + 16),
                Tone::B.oct(2) * (32 + 8),
                Tone::A.oct(2) * 16,
            ]).transpose_up(Oct),
            FixedSequence::new(vec![
                Tone::D.oct(2) * 64,
                Tone::E.oct(2) * 64,
                Tone::Gb.oct(2) * 64,
                Tone::G.oct(2) * 64,
            ]).split_notes(vec![
                true, false, false, false,
                false, false, false, false,
                false, false, false, false,
                true, false, false, false,
                false, false, false, false,

                true, false, false, false,
                false, false, false, false,
                true, false, false, false,
                true, false, false, false,
                false, false, false, false,
            ])
        ].into_iter().map(|seq| seq.midibox()).collect()
    ).unwrap()
}
