use std::collections::HashMap;
use midibox::{Bpm, Chord, MutMidi, Scale, seq, chord, ToMidi, ToChord, Tone};
use midibox::drumlogue::Drumlogue::{BD, CH, HT, LT, OH};
use midibox::sequences::Seq;
use midibox::player::{PlayerConfig, try_run};
use midibox::router::MapRouter;
use midibox::Tone::Rest;

fn main() {
    let _scale = Scale::major(Tone::C);

    let mut channel_id_to_port_id : HashMap<usize, usize> = HashMap::new();
    channel_id_to_port_id.insert(0, 1);
    channel_id_to_port_id.insert(1, 0);
    channel_id_to_port_id.insert(2, 0);

    let drums = seq![
       BD,
       CH,
       Rest,
       BD,
       OH,
       Rest,
       LT,
       HT,
       BD,
       CH,
       BD,
       BD,
       Rest,
       BD,
       Rest,
       HT
    ].midibox();

    // preset 204
    let chords = seq![
        chord![
            Tone::G.oct(2) * 32,
            Tone::E.oct(3),
            Tone::B.oct(3)
        ].velocity(70),

        chord![
            Tone::C.oct(2) * 32,
            Tone::E.oct(3),
            Tone::A.oct(3)
        ].velocity(40),

        chord![
            Tone::D.oct(3) * 32,
            Tone::Gb.oct(3),
            Tone::A.oct(3)
        ].velocity(20),

        chord![
            Tone::D.oct(3) * 32,
            Tone::Gb.oct(3),
            Tone::B.oct(3)
        ].velocity(50)
    ]
        .split_notes(&vec![true, false, false, true, false, false, true, true, false, false])
        .midibox();

    let roots =
        seq![
            Tone::C.oct(1) * 32,
            Tone::F.oct(1) * 20,
            Tone::A.oct(2) * 12,
            Tone::B.oct(2) * 32,
            Tone::G.oct(2) * 32
        ].midibox();

    try_run(
        PlayerConfig::from_router(Box::new(MapRouter::new(channel_id_to_port_id))),
        &Bpm::new(500),
        &mut vec![drums, chords, roots]
    ).unwrap()
}
