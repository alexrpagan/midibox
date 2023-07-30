use std::collections::HashMap;
use midibox::drumlogue::Drumlogue::{BD, CH, CP, OH, SP1};
use midibox::meter::Bpm;
use midibox::sequences::Seq;
use midibox::rand::{random_velocity};
use midibox::player::{PlayerConfig, try_run};
use midibox::router::MapRouter;
use midibox::scale::{Degree, Interval, Scale};
use midibox::tone::Tone::Rest;
use midibox::tone::Tone;


fn main() {
    let mut channel_id_to_port_id : HashMap<usize, usize> = HashMap::new();

    let drums = vec![
        random_velocity(Seq::new(vec![
            CH * 1,
            CH * 1,
            OH * 1,
            CH * 1,
            CH * 1,
            CH * 1,
            CH * 1,
            OH * 1,
        ]).midibox()),
        Seq::new(vec![
            BD * 4,
            BD * 4,
            BD * 4,
            BD * 4,
            BD * 4,
            BD * 4,
            BD * 4,
            BD * 1,
            BD * 3,
            BD * 4,
            BD * 4,
            BD * 4,
            BD * 4,
            BD * 4,
            BD * 4,
            BD * 4,
            BD * 1,
            BD * 2,
            BD * 1,
        ]).midibox(),
        Seq::new(vec![
            Rest * 4,
            CP * 4
        ]).midibox(),
        Seq::new(vec![
            SP1 * 1,
            Rest * 3,
            SP1 * 1
        ]).midibox()
    ];

    let scale = Scale::major(Tone::Gb);

    let roots = Seq::new(vec![
        Tone::G.oct(2)  * 96,
        Tone::B.oct(2)  * 64,
        Tone::E.oct(2)  * 96,
        Tone::D.oct(2)  * 64,
        Tone::C.oct(2)  * 96,
        Tone::E.oct(2)  * 64,
        Tone::B.oct(2)  * 96,
        Tone::C.oct(2)  * 64,
    ]).transpose_down(Interval::Min2);

    let synth: Vec<_> = vec![
        roots.clone()
            .split_notes(&vec![true, false, false]),
        roots.clone().harmonize_down(&scale, Degree::Fourth)
            .split_notes(&vec![false, true, false]),
        roots.clone().harmonize_up(&scale, Degree::Tenth)
            .split_notes(&vec![true, false, true]),
        roots.clone().harmonize_up(&scale, Degree::Seventh)
            .split_notes(&vec![false, false, false, true]),
    ].into_iter().map(|seq| random_velocity(seq.midibox())).collect();

    for i in 0..drums.len() {
        channel_id_to_port_id.insert(i, 1);
    }
    for i in drums.len()..(drums.len() + synth.len()) {
        channel_id_to_port_id.insert(i, 0);
    }

    let mut channels= Vec::new();
    channels.extend(drums);
    channels.extend(synth);

    try_run(
        PlayerConfig::from_router(Box::new(MapRouter::new(channel_id_to_port_id))),
        &mut Bpm::new(400),
        &mut channels
    ).unwrap()
}
