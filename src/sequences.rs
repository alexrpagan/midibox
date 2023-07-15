use std::ops::{Add, Sub};
use crate::Midibox;
use crate::chord::Chord;
use crate::midi::{Midi, MutMidi};
use crate::scale::{Degree, Interval, Scale};
use crate::tone::Tone;

#[macro_export]
macro_rules! seq {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x.chord());
            )*
            Seq::chords(temp_vec)
        }
    };
}

// A looping sequence of statically defined notes.
#[derive(Debug, Clone)]
pub struct Seq {
    /// The notes that can be produced by a sequence
    notes: Vec<Chord>,
    /// The index of the play head into notes
    head_position: usize,
}

impl Seq {
    pub fn new(notes: Vec<Midi>) -> Self {
        Seq {
            notes: notes.into_iter().map(|n| Chord::note(n)).collect(),
            head_position: 0,
        }
    }

    pub fn chords(notes: Vec<Chord>) -> Self {
        Seq {
            notes,
            head_position: 0,
        }
    }

    pub fn empty() -> Self {
        Seq {
            notes: Vec::new(),
            head_position: 0,
        }
    }

    pub fn get_chords(&self) -> &Vec<Chord> {
        return &self.notes;
    }

    pub fn render(&self) -> IterSeq {
        IterSeq {
            iter: Box::new(
                self.notes
                    .clone()
                    .into_iter()
                    .map(|m| m.notes.clone())
                    .cycle()
                    .skip(self.head_position)
            )
        }
    }

    pub fn midibox(&self) -> Box<dyn Midibox> {
        Box::new(self.render())
    }

    pub fn len(&self) -> usize {
        self.notes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.notes.is_empty()
    }

    pub fn total_duration(&self) -> u32 {
        return self.notes.iter().map(|it| it.total_duration()).sum()
    }

    pub fn fast_forward(mut self, ticks: usize) -> Self {
        self.head_position = (self.head_position + ticks) % self.notes.len();
        self
    }

    pub fn duration(mut self, duration: u32) -> Self {
        self.notes = self.notes.into_iter().map(|c| c.duration(duration)).collect();
        self
    }

    pub fn velocity(mut self, velocity: u8) -> Self {
        self.notes = self.notes.into_iter().map(|c| c.velocity(velocity)).collect();
        self
    }

    pub fn scale_duration(mut self, factor: u32) -> Self {
        self.notes = self.notes.into_iter().map(|c| c.scale_duration(factor)).collect();
        self
    }

    pub fn extend(mut self, rhs: &Self) -> Self {
        let mut extend = self.notes;
        extend.append(&mut rhs.notes.clone());
        self.notes = extend;
        self
    }

    pub fn repeat(mut self, times: usize) -> Self {
        let mut new_notes: Vec<Chord> = Vec::with_capacity(
            self.notes.len() * times
        );
        for _ in 0..times {
            new_notes.extend(self.notes.clone())
        }
        self.notes = new_notes;
        self
    }

    pub fn reverse(mut self) -> Self {
        self.notes = self.notes.into_iter().rev().collect();
        self
    }

    pub fn transpose_up(mut self, interval: Interval) -> Self {
        self.notes = self.notes.into_iter().map(|c| c.transpose_up(&interval)).collect();
        self
    }

    pub fn transpose_down(mut self, interval: Interval) -> Self {
        self.notes = self.notes.into_iter().map(|c| c.transpose_down(&interval)).collect();
        self
    }

    pub fn harmonize_up(mut self, scale: &Scale, degree: Degree) -> Self {
        self.notes = self.notes.into_iter()
            .map(|m| m.harmonize_up(scale, &degree))
            .collect();
        self
    }

    pub fn harmonize_down(mut self, scale: &Scale, degree: Degree) -> Self {
        self.notes = self.notes.into_iter()
            .map(|m| m.harmonize_down(scale, &degree))
            .collect();
        self
    }

    /// Splits each note into a series of metronome ticks adding to the note's duration
    pub fn split_to_ticks(mut self) -> Self {
        self.notes = self.notes.into_iter().flat_map(|c| {
            let old_duration = c.total_duration() as usize;
            let mut notes: Vec<Chord> = Vec::new();
            for _ in 0..old_duration {
                notes.push(c.clone().duration(1))
            }
            notes
        }).collect::<Vec<Chord>>();
        self
    }

    /// mask is a sequence of bits representing notes to play or mute
    ///
    /// If the bit corresponding to a note in this sequence is false, the note will be muted.
    ///
    /// The mask will be applied starting from the first note of the sequence and will repeat to
    /// match the total number of notes in this sequence.
    pub fn mask(mut self, mask: &Vec<bool>) -> Self {
        self.notes = self.notes.into_iter()
            .zip(mask.into_iter().cycle()).map(|(c, should_play)| {
            if *should_play {
                c
            } else {
                c.pitch(Tone::Rest, 4)
            }
        }).collect();
        self
    }

    pub fn split_notes(self, mask: &Vec<bool>) -> Self {
        self.split_to_ticks().mask(mask)
    }
}

impl Add<Seq> for Seq {
    type Output = Seq;

    fn add(self, rhs: Seq) -> Self::Output {
        self.extend(&rhs)
    }
}

impl Sub<Interval> for Seq {
    type Output = Seq;

    fn sub(self, rhs: Interval) -> Self::Output {
        self.transpose_down(rhs)
    }
}

impl Add<Interval> for Seq {
    type Output = Seq;

    fn add(self, rhs: Interval) -> Self::Output {
        self.transpose_up(rhs)
    }
}

pub struct IterSeq {
    iter: Box<dyn Iterator<Item=Vec<Midi>>>
}

impl Midibox for IterSeq {
    fn next(&mut self) -> Option<Vec<Midi>> {
        self.iter.next()
    }
}