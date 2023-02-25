use std::collections::HashMap;
use midibox::{Bpm, Degree, Interval, Scale, ToMidi, Tone};
use midibox::drumlogue::Drumlogue::{BD, CH, OH};
use midibox::sequences::Seq;
use midibox::rand::RandomVelocity;
use midibox::player::{PlayerConfig, try_run};
use midibox::router::MapRouter;


fn main() {
    let mut channel_id_to_port_id : HashMap<usize, usize> = HashMap::new();

    let drums = vec![
        RandomVelocity::wrap(Seq::new(vec![
            CH.midi().set_velocity(100) * 1,
            CH.midi().set_velocity(100) * 1,
            OH.midi().set_velocity(100) * 1,
            CH.midi().set_velocity(100) * 1,
        ]).midibox()),
        Seq::new(vec![
            BD.midi() * 4
        ]).midibox()
    ];
    for i in 0..drums.len() {
        channel_id_to_port_id.insert(i, 1);
    }

    let scale = Scale::major(Tone::Gb);

    let roots = Seq::new(vec![
        Tone::G.oct(2)  * 128,
        Tone::B.oct(2)  * 128,
        Tone::E.oct(2)  * 128,
        Tone::D.oct(2)  * 128,
        Tone::C.oct(2)  * 128,
        Tone::E.oct(2)  * 128,
        Tone::B.oct(2)  * 128,
        Tone::C.oct(2)  * 128,
    ]).transpose_down(Interval::Min2);

    let synth: Vec<_> = vec![
        roots.clone().split_notes(vec![true, false, false]),
        roots.clone().harmonize_down(&scale, Degree::Fourth).split_notes(vec![false, true, false]),
        roots.clone().harmonize_up(&scale, Degree::Tenth).split_notes(vec![true, false, true]),
        roots.clone().harmonize_up(&scale, Degree::Seventh).split_notes(vec![false, false, false, true]),
    ].into_iter().map(|seq| RandomVelocity::wrap(seq.midibox())).collect();

    for i in drums.len()..(drums.len() + synth.len()) {
        channel_id_to_port_id.insert(i, 0);
    }

    let mut channels= Vec::new();
    channels.extend(drums);
    channels.extend(synth);

    try_run(
        PlayerConfig::from_router(Box::new(MapRouter::new(channel_id_to_port_id))),
        &Bpm::new(600),
        &mut channels
    ).unwrap()
}
