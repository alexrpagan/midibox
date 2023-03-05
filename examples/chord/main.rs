use std::collections::HashMap;
use midibox::{chord, seq};
use midibox::chord::{Chord, ToChord};
use midibox::drumlogue::Drumlogue::{BD, CH, HT, LT, OH, SP1};
use midibox::meter::Bpm;
use midibox::midi::{MutMidi, ToMidi};
use midibox::sequences::Seq;
use midibox::player::{PlayerConfig, try_run};
use midibox::router::MapRouter;
use midibox::scale::Scale;
use midibox::tone::Tone;
use midibox::tone::Tone::Rest;

fn main() {
    let _scale = Scale::major(Tone::C);

    let mut channel_id_to_port_id : HashMap<usize, usize> = HashMap::new();
    channel_id_to_port_id.insert(0, 1);
    channel_id_to_port_id.insert(1, 1);
    channel_id_to_port_id.insert(2, 0);
    channel_id_to_port_id.insert(3, 0);

    let drums = seq![
       BD,
       CH,
       Tone::Rest,
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

    let hats = seq![
        Rest * 2,
        SP1 * 2
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
        &mut vec![drums, hats, chords, roots]
    ).unwrap()
}
