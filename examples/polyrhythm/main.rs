use std::collections::HashMap;
use Drumlogue::CP;
use midibox::{Bpm, Degree, Interval, Scale, Tone};
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
        &Bpm::new( 500),
        & mut vec![
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
                BD * 1,
                LT * 1,
                BD * 1,
                BD * 1,
                CP * 2,
                Rest * 2,
                BD * 2,
                BD * 1,
                BD * 1,
                CP * 1,
                BD * 3,
            ]).midibox(),
            Seq::new(vec![
                Rest * 2,
                LT * 4,
                LT * 2,
                LT * 2,
            ]).midibox(),
            Seq::new(vec![
                CH * 1,
                CH * 1,
                OH * 1,
                CH * 1,
                CH * 1,
                OH * 1,
                CH * 1,
                OH * 1,
            ]).midibox(),
            Seq::new(vec![
                SP1 * 5
            ])
                .split_notes(vec![true, false, false, false, true])
                .midibox()
        ]
    ).unwrap()
}
