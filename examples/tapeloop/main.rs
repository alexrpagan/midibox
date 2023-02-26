use std::collections::HashMap;
use midibox::{Bpm, Interval, ToMidi, Tone};
use midibox::drumlogue::Drumlogue::{BD, CH, HT, LT, OH, SD};
use midibox::sequences::Seq;
use midibox::player::{PlayerConfig, try_run};
use midibox::router::MapRouter;
use Tone::Rest;

fn main() {
    let mut channel_id_to_port_id : HashMap<usize, usize> = HashMap::new();
    // H 03
    let drums = vec![
        Seq::new(vec![
            BD.midi() * 8,
            BD.midi() * 7,
            BD.midi().set_velocity(50) * 1,
            BD.midi() * 2,
            BD.midi() * 5,
            BD.midi().set_velocity(50) * 1,
            BD.midi() * 3,
            BD.midi() * 4,
            BD.midi().set_velocity(80) * 1,
        ]),
        Seq::new(vec![
            Rest.midi() * 2,
            SD.midi().set_velocity(10) * 1,
            SD.midi().set_velocity(20) * 1,
            Rest.midi() * 1,
            SD.midi().set_velocity(10) * 1,
            SD.midi().set_velocity(20) * 1,
            Rest.midi() * 1
        ]),
        Seq::new(vec![
            Rest.midi() * 1,
            LT.midi().set_velocity(70) * 1,
            Rest.midi() * 2,
            LT.midi().set_velocity(120) * 2,
            HT.midi().set_velocity(50) * 1,
        ]),
        Seq::new(vec![
            CH.midi().set_velocity(20) * 1,
            OH.midi().set_velocity(30) * 1,
            CH.midi().set_velocity(10) * 1,
            CH.midi().set_velocity(5) * 1,
        ])
    ];

    for i in 0..drums.len() {
        channel_id_to_port_id.insert(i, 1);
    }

    let roots = Seq::new(vec![
        Rest.midi() * 1,
        Tone::D.oct(2).set_velocity(60) * 2,
        Tone::D.oct(2).set_velocity(70) * 3,
        Tone::D.oct(2).set_velocity(80) * 1,
        Tone::Gb.oct(3) * (11 + 16),
        Tone::D.oct(2).set_velocity(60) * 3,
        Tone::D.oct(2).set_velocity(70) * 3,
        Tone::D.oct(2).set_velocity(80) * 1,
        Tone::Db.oct(3) * 11,
        Tone::G.oct(1).set_velocity(60) * 3,
        Tone::G.oct(1).set_velocity(70) * 3,
        Tone::G.oct(1).set_velocity(80) * 1,
        Tone::A.oct(2) * 1,
        Tone::B.oct(2) * 9,
        Tone::Db.oct(3) * 1,
    ]);

    let harmony_1 =  Seq::new(vec![
        Rest.midi() * 7,
        Rest.midi() * 7,
        Tone::A.oct(3).set_velocity(50) * 20,
        Rest.midi() * 7,
        Rest.midi() * 4,
        Tone::Gb.oct(3) * 7,
        Rest.midi() * 7,
        Rest.midi() * 4,
        Tone::Ab.midi().set_velocity(30) * 7
    ]);

    let harmony_2 =  Seq::new(vec![
        Rest.midi() * 7,
        Rest.midi() * 7,
        Rest.midi() * 4,
        Tone::B.oct(3).set_velocity(40) * 16,
        Rest.midi() * 7,
        Rest.midi() * 4,
        Tone::Gb.oct(3) * 7,
        Rest.midi() * 7,
        Rest.midi() * 7,
        Tone::Gb.midi().set_velocity(40) * 4
    ]);

    assert_eq!(roots.total_duration(), harmony_1.total_duration());
    assert_eq!(roots.total_duration(), harmony_2.total_duration());

    // Preset 490
    let synth = vec![
        roots.clone(),
        roots.clone().transpose_up(Interval::Perf5),
        harmony_1.clone(),
        harmony_2.clone(),
    ];

    for i in drums.len()..(drums.len() + synth.len()) {
        channel_id_to_port_id.insert(i, 0);
    }

    let mut all_seq = drums.clone();
    all_seq.extend(synth.clone());

    try_run(
        PlayerConfig::from_router(Box::new(MapRouter::new(channel_id_to_port_id))),
        &Bpm::new(300),
        &mut all_seq.into_iter().map(|it| it.midibox()).collect()
    ).unwrap()
}
