use midibox::{Bpm, Degree, Interval, Scale, Tone};
use midibox::sequences::FixedSequence;
use midibox::player::run;

// preset 494, 21
fn main() {
    let _scale = Scale::major(Tone::B);

    let roots = FixedSequence::new(vec![
        Tone::C.oct(3),
        Tone::E.oct(3),
        Tone::G.oct(3),
        Tone::B.oct(3),
        Tone::D.oct(3),
        Tone::B.oct(2),
        Tone::G.oct(3),
        Tone::D.oct(4)
    ]).transpose_down(Interval::Min2);

    let fast = roots.clone().duration(2).repeat(5);
    let slow = roots.clone().duration(5).repeat(2);
    let slow_ff1 = roots.clone().duration(5).fast_forward(1).repeat(2);

    run(
        Bpm::new(500),
        vec![
            (
                fast.clone()
                    + fast.clone().transpose_down(Interval::Perf4)
                    + fast.clone()
                    + fast.clone().transpose_down(Interval::Min3)
            ).midibox(),
            (
               slow_ff1.clone()
                    + slow_ff1.clone().transpose_down(Interval::Perf4)
                    + slow.clone().split_notes(vec![true, false, false])
                    + slow.clone().split_notes(vec![false, true, false]).transpose_down(Interval::Min3)
            ).midibox()
        ]
    )
}
