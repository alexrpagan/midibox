use midibox::Bpm;
use midibox::drumlogue::Drumlogue::{BD, CH, CP, HT, LT, OH, RS, SD};
use midibox::sequences::FixedSequence;
use midibox::player::run;
use midibox::Tone::Rest;
use midibox::ToMidi;

fn main() {
    run(
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
        ].into_iter().map(|seq| seq.midibox()).collect()
    )
}
