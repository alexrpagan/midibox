use Drumlogue::{BD, CH, CP, HT, LT, OH, RS, SD, SP1, SP2};
use midibox::{Bpm, Degree, Interval, Scale, Tone, Midi};
use midibox::sequences::FixedSequence;
use midibox::player::run;
use midibox::Tone::Rest;

enum Drumlogue {
    BD,
    SD,
    LT,
    HT,
    CH,
    OH,
    RS,
    CP,
    SP1,
    SP2,
}

impl Drumlogue {
    pub fn tone(&self) -> Midi {
        match self {
            BD => { Tone::C.oct(2) }
            SD => { Tone::E.oct(2) }
            LT => { Tone::A.oct(2) }
            HT => { Tone::D.oct(3) }
            CH => { Tone::Gb.oct(2) }
            OH => { Tone::Bb.oct(2) }
            RS => { Tone::Db.oct(2) }
            CP => { Tone::Eb.oct(2) }
            SP1 => { Tone::E.oct(3) }
            SP2 => { Tone::F.oct(3) }
        }
    }
}

fn main() {
    run(
        Bpm::new(1800),
        vec![
            FixedSequence::new(vec![
                RS.tone().set_velocity(30) * 4,
                Rest.get() * 8,
                Rest.get() * 16
            ]),
            FixedSequence::new(vec![
                Rest.get() * 4,
                CH.tone().set_velocity(15) * 8
            ]),
            FixedSequence::new(vec![
                BD.tone().set_velocity(100) * 16,
                BD.tone().set_velocity(70) * 16,
                BD.tone().set_velocity(50) * 16,
                BD.tone().set_velocity(70) * 16,
                BD.tone() * 16,
                BD.tone() * 16,
                BD.tone() * 16,
                BD.tone() * 8,
                BD.tone().set_velocity(15) * 4,
                BD.tone().set_velocity(20) * 4,
            ]),
            FixedSequence::new(vec![
                Rest.get() * 16,
                SD.tone() * 16,
            ]),
            FixedSequence::new(vec![
                Rest.get() * 48,
                CP.tone() * 16,
            ]),
            FixedSequence::new(vec![
                Rest.get() * 32,
                HT.tone().set_velocity(30) * 4,
                LT.tone().set_velocity(70) * 4,
                Rest.get() * 16
            ]),
            FixedSequence::new(vec![
                Rest.get() * 32,
                Rest.get() * 8,
                BD.tone().set_velocity(30) * 4,
                BD.tone().set_velocity(100) * 4,
                Rest.get() * 8
            ]),
            FixedSequence::new(vec![
                Rest.get() * 8,
                CH.tone().set_velocity(40) * 8,
                Rest.get() * 8,
                CH.tone().set_velocity(44) * 8,
                Rest.get() * 8,
                OH.tone().set_velocity(42) * 4,
                CH.tone().set_velocity(43) * 4,
            ]),
        ].into_iter().map(|seq| seq.midibox()).collect()
    )
}
