use std::collections::HashMap;
use midibox::tone::Tone;
use midibox::drumlogue::Drumlogue::{BD, CH, HT, LT, OH, SD};
use midibox::meter::Bpm;
use midibox::midi::ToMidi;
use midibox::sequences::Seq;
use midibox::player::{PlayerConfig, try_run};
use midibox::router::MapRouter;
use midibox::scale::Interval;
use midibox::midi::Tone::Rest;

fn main() {
    let mut channel_id_to_port_id : HashMap<usize, usize> = HashMap::new();
    // H 03
    let drums = vec![
        Seq::new(vec![
            BD * 8,
            BD * 7,
            BD.set_velocity(50) * 1,
            BD * 2,
            BD * 5,
            BD.set_velocity(50) * 1,
            BD * 3,
            BD * 4,
            BD.set_velocity(80) * 1,
        ]),
        Seq::new(vec![
            Rest * 2,
            SD.set_velocity(10) * 1,
            SD.set_velocity(20) * 1,
            Rest * 1,
            SD.set_velocity(10) * 1,
            SD.set_velocity(20) * 1,
            Rest * 1
        ]),
        Seq::new(vec![
            Rest * 1,
            LT.set_velocity(70) * 1,
            Rest * 2,
            LT.set_velocity(120) * 2,
            HT.set_velocity(50) * 1,
        ]),
        Seq::new(vec![
            CH.set_velocity(20) * 1,
            OH.set_velocity(30) * 1,
            CH.set_velocity(10) * 1,
            CH.set_velocity(5) * 1,
        ])
    ];

    for i in 0..drums.len() {
        channel_id_to_port_id.insert(i, 1);
    }

    let roots = Seq::new(vec![
        Rest * 1,
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
        Rest * 7,
        Rest * 7,
        Tone::A.oct(3).set_velocity(50) * 20,
        Rest * 7,
        Rest * 4,
        Tone::Gb.oct(3) * 7,
        Rest * 7,
        Rest * 4,
        Tone::Ab.midi().set_velocity(30) * 7
    ]);

    let harmony_2 =  Seq::new(vec![
        Rest * 7,
        Rest * 7,
        Rest * 4,
        Tone::B.oct(3).set_velocity(40) * 16,
        Rest * 7,
        Rest * 4,
        Tone::Gb.oct(3) * 7,
        Rest * 7,
        Rest * 7,
        Tone::Gb.set_velocity(40) * 4
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
