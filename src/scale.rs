use std::time::Duration;
use crate::chord::Chord;
use crate::midi::{Midi};
use crate::tone::Tone;

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

    pub fn make_chord(&self, oct: u8, degree: Degree, pitches: &Vec<Pitch>) -> Option<Chord> {
        let scale_root = Midi::from_tone(self.root, oct);
        match self.harmonize_up(scale_root, degree) {
            None => None,
            Some(chord_root) => {
                let notes = pitches.iter().map(|chord_pitch| match chord_pitch {
                    Pitch::Harmonize(degree, direction) => {
                        let harmd = if direction.is_up() {
                            self.harmonize_up(chord_root, degree.clone())
                        } else {
                            self.harmonize_down(chord_root, degree.clone())
                        };
                        harmd.map(|n|
                            n.set_pitch(
                                n.tone,
                                (n.oct as i8 + direction.octaves_to_move()) as u8
                            )
                        )
                    }
                    Pitch::Transpose(interval, direction) => {
                        let posed = if direction.is_up() {
                            chord_root.transpose_up(interval.clone())
                        } else {
                            chord_root.transpose_down( interval.clone())
                        };
                        Some(posed.set_pitch(
                            posed.tone,
                            (posed.oct as i8 + direction.octaves_to_move()) as u8)
                        )
                    }
                });
                let mut chord_notes: Vec<Midi> = vec![];
                for note in notes {
                    match note {
                        None => return None,
                        Some(n) => chord_notes.push(n)
                    }
                }
                return Some(Chord::new(chord_notes))
            }
        }
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

pub enum Pitch {
    Harmonize(Degree, Direction),
    Transpose(Interval, Direction)
}

pub enum Direction {
    Up,
    Down,
    UpShiftOct(i8),
    DownShiftOct(i8)
}

impl Direction {
    fn is_up(&self) -> bool {
        match self {
            Direction::Up => true,
            Direction::Down => false,
            Direction::UpShiftOct(_) => true,
            Direction::DownShiftOct(_) => false
        }
    }

    fn octaves_to_move(&self) -> i8 {
        match self {
            Direction::Up => 0,
            Direction::Down => 0,
            Direction::UpShiftOct(o) => *o as i8,
            Direction::DownShiftOct(o) => -(*o as i8)
        }
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
    Thirteenth
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
