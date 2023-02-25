use std::collections::HashMap;
use Drumlogue::CP;
use midibox::{Bpm, Degree, Interval, Scale, ToMidi, Tone};
use midibox::drumlogue::Drumlogue;
use midibox::drumlogue::Drumlogue::{BD, CH, LT, OH, SP1};
use midibox::sequences::Seq;
use midibox::player::{PlayerConfig, try_run};
use midibox::router::MapRouter;
use Tone::Rest;

fn main() {
    let mut channel_id_to_port_id : HashMap<usize, usize> = HashMap::new();
    // preset 494, 21
    for i in 0..2 {
        channel_id_to_port_id.insert(i, 0);
    }
    // Bank A 07 Rolling
    for i in 2..6 {
        channel_id_to_port_id.insert(i, 1);
    }
    let sequence = Seq::new(vec![
        Tone::C.oct(3),
        Tone::G.oct(3),
        Tone::E.oct(2),
        Tone::B.oct(3),
    ]);
    let roots =
        sequence.clone() + sequence.clone().harmonize_up(&Scale::major(Tone::C), Degree::Third);

    let fast = roots.clone().duration(2).split_notes(vec![true, false]).repeat(5);
    let slow_ff1 = roots.clone().duration(5).repeat(2);

    assert_eq!(fast.total_duration(), slow_ff1.total_duration());

    try_run(
        PlayerConfig::from_router(Box::new(MapRouter::new(channel_id_to_port_id))),
        &Bpm::new(500),
        &mut vec![
            (
                fast.clone()
                    + fast.clone().transpose_down(Interval::Perf4)
                    + fast.clone()
                    + fast.clone().transpose_down(Interval::Min3)
                    + fast.clone()
                    + fast.clone().transpose_down(Interval::Min2)
                    + fast.clone()
                    + fast.clone().transpose_up(Interval::Maj3)
            ).midibox(),
            (
                slow_ff1.clone()
                    .split_notes(vec![true, false, false])
                    + slow_ff1.clone()
                    .split_notes(vec![false, true, false, false, true])
                    .transpose_down(Interval::Perf4)
                    + slow_ff1.clone()
                    .split_notes(vec![true, false, false])
                    + slow_ff1.clone()
                    .split_notes(vec![false, true, false, false, true])
                    .transpose_down(Interval::Min3)
                    + slow_ff1.clone()
                    .split_notes(vec![true, false, false])
                    + slow_ff1.clone()
                    .split_notes(vec![false, true, false, false, true])
                    .transpose_down(Interval::Min2)
                    + slow_ff1.clone()
                    .split_notes(vec![true, false, false])
                    + slow_ff1.clone()
                    .split_notes(vec![false, true, false, false, true])
                    .transpose_up(Interval::Maj3)
            ).midibox(),

            Seq::new(vec![
                BD.midi() * 1,
                LT.midi() * 1,
                BD.midi() * 1,
                BD.midi() * 1,
                CP.midi() * 2,
                Rest.midi() * 2,
                BD.midi() * 2,
                BD.midi() * 1,
                BD.midi() * 1,
                CP.midi() * 1,
                BD.midi() * 3,
            ]).midibox(),
            Seq::new(vec![
                Rest.midi() * 2,
                LT.midi() * 4,
                LT.midi() * 2,
                LT.midi() * 2,
            ]).midibox(),
            Seq::new(vec![
                CH.midi() * 1,
                CH.midi() * 1,
                OH.midi() * 1,
                CH.midi() * 1,
                CH.midi() * 1,
                OH.midi() * 1,
                CH.midi() * 1,
                OH.midi() * 1,
            ]).midibox(),
            Seq::new(vec![
                SP1.midi() * 5
            ])
                .split_notes(vec![true, false, false, false, true])
                .midibox()
        ]
    ).unwrap()
}
