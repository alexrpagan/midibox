use std::ops::{Add, Mul, Sub};
use crate::chord::{Chord, ToChord};
use crate::scale::{Degree, Interval, Scale};
use crate::tone::Tone;

pub trait ToMidi: ToChord {
    fn midi(&self) -> Midi;

    fn is_rest(&self) -> bool {
        self.midi().is_rest()
    }

    fn u8_maybe(&self) -> Option<u8> {
        self.midi().u8_maybe()
    }

    fn set_velocity(&self, velocity: u8) -> Midi {
        self.midi().set_velocity(velocity)
    }

    fn set_duration(&self, duration: u32) -> Midi {
        self.midi().set_duration(duration)
    }

    fn set_pitch_u8(&self, val: Option<u8>) -> Midi {
        self.midi().set_pitch_u8(val)
    }

    fn set_pitch(&self, tone: Tone, oct: u8) -> Midi {
        self.midi().set_pitch(tone, oct)
    }

    fn transpose_up(&self, interval: Interval) -> Midi {
        self.midi().transpose_up(interval)
    }

    fn transpose_down(&self, interval: Interval) -> Midi {
        self.midi().transpose_down(interval)
    }
}


pub trait MutMidi: Sized {
    fn total_duration(&self) -> u32;
    fn duration(self, duration: u32) -> Self;
    fn velocity(self, velocity: u8) -> Self;
    fn pitch(self, tone: Tone, oct: u8) -> Self;
    fn scale_duration(self, factor: u32) -> Self;
    fn transpose_up(self, interval: &Interval) -> Self;
    fn transpose_down(self, interval: &Interval) -> Self;
    fn harmonize_up(self, scale: &Scale, degree: &Degree) -> Self;
    fn harmonize_down(self, scale: &Scale, degree: &Degree) -> Self;
}
const DEFAULT_OCT: u8 = 4;
const DEFAULT_VELOCITY: u8 = 100;
const DEFAULT_DURATION: u32 = 1;

pub const NOTE_ON_MSG: u8 = 0x90;
pub const NOTE_OFF_MSG: u8 = 0x80;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Midi {
    pub tone: Tone,
    pub oct: u8,
    pub velocity: u8,
    pub duration: u32,
}

impl Midi {
    pub fn rest() -> Self {
        Midi {
            tone: Tone::Rest,
            oct: DEFAULT_OCT,
            velocity: DEFAULT_VELOCITY,
            duration: DEFAULT_DURATION,
        }
    }

    pub fn oct(val: u8) -> u8 {
        (val / 12) - 1
    }

    pub fn from_option(val: Option<u8>) -> Midi {
        match val {
            None => Midi::rest(),
            Some(v) => Midi::from(v)
        }
    }

    pub fn from_tone(tone: Tone, oct: u8) -> Midi {
        Midi { tone, oct, velocity: DEFAULT_VELOCITY, duration: DEFAULT_DURATION }
    }

    pub fn from(val: u8) -> Midi {
        Midi::from_tone(Tone::from(val), Midi::oct(val))
    }

    pub fn is_rest(&self) -> bool {
        matches!(self.tone, Tone::Rest)
    }

    pub fn u8_maybe(&self) -> Option<u8> {
        self.tone.u8(self.oct)
    }

    pub fn set_velocity(&self, velocity: u8) -> Self {
        Midi { tone: self.tone, oct: self.oct, velocity, duration: self.duration }
    }

    pub fn set_duration(&self, duration: u32) -> Self {
        Midi { tone: self.tone, oct: self.oct, velocity: self.velocity, duration }
    }

    pub fn set_pitch_u8(&self, val: Option<u8>) -> Self {
        match val {
            None => self.set_pitch(Tone::Rest, 0),
            Some(v) => self.set_pitch(Tone::from(v), Midi::oct(v))
        }
    }

    pub fn set_pitch(&self, tone: Tone, oct: u8) -> Self {
        Midi { tone, oct, velocity: self.velocity, duration: self.duration }
    }

    pub fn transpose_up(&self, interval: Interval) -> Self {
        self.set_pitch_u8(self.u8_maybe().map(|v| v + interval.steps()))
    }

    pub fn transpose_down(&self, interval: Interval) -> Self {
        self.set_pitch_u8(self.u8_maybe().map(|v| v - interval.steps()))
    }
}

/// Transposes MIDI note up specified interval
impl Add<Interval> for Midi {
    type Output = Midi;

    fn add(self, rhs: Interval) -> Self::Output {
        self.transpose_up(rhs)
    }
}

/// Transposes MIDI node down specified interval
impl Sub<Interval> for Midi {
    type Output = Midi;

    fn sub(self, rhs: Interval) -> Self::Output {
        self.transpose_down(rhs)
    }
}

/// Sets duration of MIDI note to specified duration in `u32` ticks
impl Mul<u32> for Midi {
    type Output = Midi;

    fn mul(self, rhs: u32) -> Self::Output {
        self.clone().set_duration(self.duration * rhs)
    }
}

impl ToMidi for Midi {
    fn midi(&self) -> Midi {
        self.clone()
    }

    fn is_rest(&self) -> bool {
        self.is_rest()
    }

    fn u8_maybe(&self) -> Option<u8> {
        self.u8_maybe()
    }

    fn set_velocity(&self, velocity: u8) -> Midi {
        self.set_velocity(velocity)
    }

    fn set_duration(&self, duration: u32) -> Midi {
        self.set_duration(duration)
    }

    fn set_pitch_u8(&self, val: Option<u8>) -> Midi {
        self.set_pitch_u8(val)
    }

    fn set_pitch(&self, tone: Tone, oct: u8) -> Midi {
        self.set_pitch(tone, oct)
    }

    fn transpose_up(&self, interval: Interval) -> Midi {
        self.transpose_up(interval)
    }

    fn transpose_down(&self, interval: Interval) -> Midi {
        self.transpose_down(interval)
    }
}

impl ToChord for Midi {
    fn chord(&self) -> Chord {
        Chord::note(self.midi())
    }
}

#[cfg(test)]
mod tests {
    use crate::midi::{Midi, Tone};
    use crate::scale::{Degree, Scale};
    use crate::tone::Tone;

    #[test]
    fn tone() {
        assert_eq!(Tone::C.u8(4), Some(60));
        assert_eq!(Tone::Db.u8(4), Some(61));
        assert_eq!(Tone::D.u8(4), Some(62));
        assert_eq!(Tone::Eb.u8(4), Some(63));
        assert_eq!(Tone::E.u8(4), Some(64));
        assert_eq!(Tone::F.u8(4), Some(65));
        assert_eq!(Tone::Gb.u8(4), Some(66));
        assert_eq!(Tone::G.u8(4), Some(67));
        assert_eq!(Tone::Ab.u8(4), Some(68));
        assert_eq!(Tone::A.u8(4), Some(69));
        assert_eq!(Tone::Bb.u8(4), Some(70));
        assert_eq!(Tone::B.u8(4), Some(71));
    }

    #[test]
    fn from() {
        assert_eq!(Tone::from(53), Tone::F);
        assert_eq!(Tone::from(60), Tone::C);
        assert_eq!(Tone::from(61), Tone::Db);
        assert_eq!(Tone::from(100), Tone::E);
    }

    #[test]
    fn scale() {
        assert_eq!(
            Scale::major(Tone::C).midi(4),
            vec![
                Tone::C.oct(4),
                Tone::D.oct(4),
                Tone::E.oct(4),
                Tone::F.oct(4),
                Tone::G.oct(4),
                Tone::A.oct(4),
                Tone::B.oct(4),
            ]
        );
        assert_eq!(
            Scale::major(Tone::D).midi(4),
            vec![
                Tone::D.oct(4),
                Tone::E.oct(4),
                Tone::Gb.oct(4),
                Tone::G.oct(4),
                Tone::A.oct(4),
                Tone::B.oct(4),
                Tone::Db.oct(5),
            ]
        );
    }

    #[test]
    fn harmonize_up() {
        assert_eq!(
            Scale::major(Tone::C).harmonize_up(
                Tone::C.oct(4),
                Degree::Sixth,
            ),
            Some(Tone::A.oct(4))
        );
        assert_eq!(
            Scale::major(Tone::C).harmonize_up(
                Tone::B.oct(4),
                Degree::Fifth,
            ),
            Some(Tone::F.oct(5))
        );
        assert_eq!(
            Scale::major(Tone::C).harmonize_up(
                Tone::A.oct(5),
                Degree::Tenth,
            ),
            Some(Tone::C.oct(7))
        );
        assert_eq!(
            Scale::major(Tone::C).harmonize_up(
                Tone::A.oct(5),
                Degree::Tenth,
            ),
            Some(Tone::C.oct(7))
        );

        assert_eq!(
            Scale::major(Tone::C).harmonize_up(
                Tone::A.oct(5),
                Degree::Second,
            ),
            Some(Tone::B.oct(5))
        )
    }

    #[test]
    fn harmonize_down() {
        assert_eq!(
            Scale::major(Tone::C).harmonize_down(
                Tone::C.oct(4),
                Degree::Fourth,
            ),
            Some(Tone::G.oct(3))
        );
        assert_eq!(
            Scale::major(Tone::C).harmonize_down(
                Tone::C.oct(4),
                Degree::Second,
            ),
            Some(Tone::B.oct(3))
        );
        assert_eq!(
            Scale::major(Tone::C).harmonize_down(
                Tone::C.oct(4),
                Degree::Third,
            ),
            Some(Tone::A.oct(3))
        );
        assert_eq!(
            Scale::major(Tone::C).harmonize_down(
                Tone::C.oct(4),
                Degree::Tenth,
            ),
            Some(Tone::A.oct(2))
        );
        assert_eq!(
            Scale::major(Tone::C).harmonize_down(
                Tone::C.oct(4),
                Degree::Sixth,
            ),
            Some(Tone::E.oct(3))
        );

        assert_eq!(
            Scale::major(Tone::C).harmonize_down(
                Tone::F.oct(5),
                Degree::Fifth,
            ),
            Some(Tone::B.oct(4))
        );
        assert_eq!(
            Scale::major(Tone::C).harmonize_down(
                Tone::B.oct(5),
                Degree::Fifth,
            ),
            Some(Tone::E.oct(5))
        );
        assert_eq!(
            Scale::major(Tone::C).harmonize_down(
                Tone::B.oct(5),
                Degree::Second,
            ),
            Some(Tone::A.oct(5))
        )
    }
}


