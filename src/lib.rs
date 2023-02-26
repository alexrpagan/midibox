use std::ops::{Add, Mul, Sub};
use std::time::Duration;

pub mod sequences;
pub mod player;
pub mod router;
pub mod drumlogue;
pub mod rand;

pub const NOTE_ON_MSG: u8 = 0x90;
pub const NOTE_OFF_MSG: u8 = 0x80;

pub trait Meter {
    fn tick_duration(&self) -> Duration;
}

#[derive(Debug, Clone)]
pub struct Bpm {
    bpm: u32,
}

impl Meter for Bpm {
    fn tick_duration(&self) -> Duration {
        Duration::from_secs(60) / self.bpm
    }
}

impl Bpm {
    pub fn new(bpm: u32) -> Self {
        Bpm { bpm }
    }
}

pub trait Midibox {
    fn next(&mut self) -> Option<Vec<Midi>>;
}

pub trait ToMidi {
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
        self.transpose_down(interval)
    }
}

#[derive(Debug, Clone)]
pub struct Scale {
    root: Tone,
    intervals: Vec<u8>,
}

impl Scale {
    pub fn major(root: Tone) -> Self {
        Scale {
            root,
            intervals: vec![
                2, // Whole step
                2, // W
                1, // Half step
                2, // W
                2, // W
                2, // W
                1, // H
            ],
        }
    }

    pub fn tones(&self) -> Vec<Tone> {
        self.midi(4).into_iter().map(|m| m.tone).collect()
    }

    pub fn midi(&self, oct: u8) -> Vec<Midi> {
        let mut midi = Vec::new();
        midi.push(self.root.oct(oct));
        for interval in self.intervals.iter().take(self.intervals.len() - 1) {
            midi.push(Midi::from_option(
                midi.last().unwrap().u8_maybe().map(|v| v + interval)
            ))
        }
        midi
    }

    pub fn harmonize_up(&self, midi: Midi, harmonize: Degree) -> Option<Midi> {
        let tones = self.tones();
        let degree_maybe = tones.into_iter().position(|t| t.eq(&midi.tone));
        return match degree_maybe {
            None => None,
            Some(pos) => {
                let steps_to_raise: u8 = self.intervals
                    .iter()
                    .cycle()
                    .skip(pos)
                    .take(harmonize.steps())
                    .sum();
                let new = Midi::from_option(midi.u8_maybe().map(|v| v + steps_to_raise));
                return Some(midi.set_pitch(
                    new.tone,
                    new.oct,
                ));
            }
        };
    }

    pub fn harmonize_down(&self, midi: Midi, harmonize: Degree) -> Option<Midi> {
        let tones = self.tones();
        let degree_maybe = tones.into_iter().position(|t| t.eq(&midi.tone));
        return match degree_maybe {
            None => None,
            Some(pos) => {
                let scale_at_pos: Vec<&u8> = self.intervals
                    .iter()
                    .cycle()
                    .skip(pos)
                    .take(self.intervals.len())
                    .collect();

                let steps_to_lower: u8 = scale_at_pos
                    .into_iter()
                    .rev()
                    .cycle()
                    .take(harmonize.steps())
                    .sum();
                let new = Midi::from_option(midi.u8_maybe().map(|v| v - steps_to_lower));
                return Some(midi.set_pitch(
                    new.tone,
                    new.oct,
                ));
            }
        };
    }
}


#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Degree {
    Unison,
    Second,
    Third,
    Fourth,
    Fifth,
    Sixth,
    Seventh,
    Octave,
    Ninth,
    Tenth,
    Eleventh,
    Twelveth,
    Thirteenth,
}

impl Degree {
    fn steps(&self) -> usize {
        match self {
            Degree::Unison => 0,
            Degree::Second => 1,
            Degree::Third => 2,
            Degree::Fourth => 3,
            Degree::Fifth => 4,
            Degree::Sixth => 5,
            Degree::Seventh => 6,
            Degree::Octave => 7,
            Degree::Ninth => 8,
            Degree::Tenth => 9,
            Degree::Eleventh => 10,
            Degree::Twelveth => 11,
            Degree::Thirteenth => 12
        }
    }
}


#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Interval {
    Unison,
    Min2,
    Maj2,
    Min3,
    Maj3,
    Perf4,
    Flat5,
    Perf5,
    Min6,
    Maj6,
    Min7,
    Maj7,
    Oct,
    Min9,
    Maj9,
    Min10,
    Maj10,
}

impl Interval {
    pub fn steps(&self) -> u8 {
        match self {
            Interval::Unison => { 0 }
            Interval::Min2 => { 1 }
            Interval::Maj2 => { 2 }
            Interval::Min3 => { 3 }
            Interval::Maj3 => { 4 }
            Interval::Perf4 => { 5 }
            Interval::Flat5 => { 6 }
            Interval::Perf5 => { 7 }
            Interval::Min6 => { 8 }
            Interval::Maj6 => { 9 }
            Interval::Min7 => { 10 }
            Interval::Maj7 => { 11 }
            Interval::Oct => { 12 }
            Interval::Min9 => { 13 }
            Interval::Maj9 => { 14 }
            Interval::Min10 => { 15 }
            Interval::Maj10 => { 16 }
        }
    }
}

const DEFAULT_OCT: u8 = 4;
const DEFAULT_VELOCITY: u8 = 100;
const DEFAULT_DURATION: u32 = 1;

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


impl Mul<u32> for Tone {
    type Output = Midi;

    fn mul(self, rhs: u32) -> Self::Output {
        self.midi() * rhs
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Tone {
    Rest,
    C,
    Db,
    D,
    Eb,
    E,
    F,
    Gb,
    G,
    Ab,
    A,
    Bb,
    B,
}

impl Tone {
    pub fn from(val: u8) -> Tone {
        let pos = val % 12;
        match pos {
            0 => Tone::C,
            1 => Tone::Db,
            2 => Tone::D,
            3 => Tone::Eb,
            4 => Tone::E,
            5 => Tone::F,
            6 => Tone::Gb,
            7 => Tone::G,
            8 => Tone::Ab,
            9 => Tone::A,
            10 => Tone::Bb,
            11 => Tone::B,
            _ => Tone::Rest
        }
    }

    pub fn u8(&self, oct: u8) -> Option<u8> {
        let base = (oct + 1) * 12;
        match self {
            Tone::C => { Some(base) }
            Tone::Db => { Some(base + 1) }
            Tone::D => { Some(base + 2) }
            Tone::Eb => { Some(base + 3) }
            Tone::E => { Some(base + 4) }
            Tone::F => { Some(base + 5) }
            Tone::Gb => { Some(base + 6) }
            Tone::G => { Some(base + 7) }
            Tone::Ab => { Some(base + 8) }
            Tone::A => { Some(base + 9) }
            Tone::Bb => { Some(base + 10) }
            Tone::B => { Some(base + 11) }
            Tone::Rest => { None }
        }
    }

    pub fn get(&self) -> Midi {
        self.oct(4)
    }

    pub fn oct(&self, oct: u8) -> Midi {
        Midi::from_tone(*self, oct)
    }
}

impl ToMidi for Tone {
    fn midi(&self) -> Midi {
        // C should be set to middle C, i.e., C4
        self.oct(4)
    }
}

#[cfg(test)]
mod tests {
    use crate::{Degree, Midi, Scale, Tone};

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
