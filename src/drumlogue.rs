use crate::{Midi, ToMidi, Tone};
use Drumlogue::{BD, CH, CP, HT, LT, OH, RS, SD, SP1, SP2};

pub enum Drumlogue {
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

impl ToMidi for Drumlogue {
    fn midi(&self) -> Midi {
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
