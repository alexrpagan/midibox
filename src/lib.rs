use std::collections::HashMap;
use std::ops::{Add, Mul, Sub};
use std::thread::sleep;
use std::time::Duration;

use crossbeam::channel::Receiver;

pub trait Meter {
    fn tick_duration(&self) -> Duration;
}

#[derive(Debug, Clone)]
pub struct Bpm {
    bpm: u32
}

impl Meter for Bpm {
    fn tick_duration(&self) -> Duration {
        Duration::from_secs(60) / self.bpm
    }
}

impl Bpm {
    pub fn new(bpm: u32) -> Box<dyn Meter> {
        Box::new(Bpm { bpm })
    }
}

pub trait Midibox: Send + Sync {
    fn iter(&self) -> Box<dyn Iterator<Item = Midi> + '_>;
}

#[derive(Debug, Clone, Copy)]
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
            Interval::Min2   => { 1 }
            Interval::Maj2   => { 2 }
            Interval::Min3   => { 3 }
            Interval::Maj3   => { 4 }
            Interval::Perf4  => { 5 }
            Interval::Flat5  => { 6 }
            Interval::Perf5  => { 7 }
            Interval::Min6   => { 8 }
            Interval::Maj6   => { 9 }
            Interval::Min7   => { 10 }
            Interval::Maj7   => { 11 }
            Interval::Oct    => { 12 }
            Interval::Min9   => { 13 }
            Interval::Maj9   => { 14 }
            Interval::Min10  => { 15 }
            Interval::Maj10  => { 16 }
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
        return Midi {
            tone: Tone::Rest,
            oct: DEFAULT_OCT,
            velocity: DEFAULT_VELOCITY,
            duration: DEFAULT_DURATION
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
        return Midi { tone, oct, velocity: DEFAULT_VELOCITY, duration: DEFAULT_DURATION }
    }

    pub fn from(val: u8) -> Midi {
        return Midi::from_tone(Tone::from(val), Midi::oct(val));
    }

    pub fn u8_maybe(&self) -> Option<u8> {
        self.tone.u8(self.oct)
    }

    pub fn set_velocity(&self, velocity: u8) -> Self {
        return Midi { tone: self.tone, oct: self.oct, velocity, duration: self.duration };
    }

    pub fn set_duration(&self, duration: u32) -> Self {
        return Midi { tone: self.tone, oct: self.oct, velocity: self.velocity, duration };
    }

    pub fn set_pitch_u8(&self, val: Option<u8>) -> Self {
        match val {
            None => self.set_pitch(Tone::Rest, 0),
            Some(v) => self.set_pitch(Tone::from(v), Midi::oct(v))
        }
    }

    pub fn set_pitch(&self, tone: Tone, oct: u8) -> Self {
        return Midi { tone, oct, velocity: self.velocity, duration: self.duration }
    }

    pub fn transpose_up(&self, interval: Interval) -> Self {
        self.set_pitch_u8(self.u8_maybe().map(|v| v + interval.steps()))
    }

    pub fn transpose_down(&self, interval: Interval) -> Self {
        self.set_pitch_u8(self.u8_maybe().map(|v| v - interval.steps()))
    }
}

impl Add<Interval> for Midi {
    type Output = Midi;

    fn add(self, rhs: Interval) -> Self::Output {
        return self.transpose_up(rhs);
    }
}

impl Sub<Interval> for Midi {
    type Output = Midi;

    fn sub(self, rhs: Interval) -> Self::Output {
        return self.transpose_down(rhs);
    }
}

impl Mul<u32> for Midi {
    type Output = Midi;

    fn mul(self, rhs: u32) -> Self::Output {
        return self.clone().set_duration(self.duration * rhs)
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
    B
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
            Tone::C  => { Some(base) }
            Tone::Db => { Some(base + 1) }
            Tone::D  => { Some(base + 2) }
            Tone::Eb => { Some(base + 3) }
            Tone::E  => { Some(base + 4) }
            Tone::F  => { Some(base + 5) }
            Tone::Gb => { Some(base + 6) }
            Tone::G  => { Some(base + 7) }
            Tone::Ab => { Some(base + 8) }
            Tone::A  => { Some(base + 9) }
            Tone::Bb => { Some(base + 10) }
            Tone::B  => { Some(base + 11) }
            Tone::Rest => { None }
        }
    }

    pub fn get(&self) -> Midi {
        return self.oct(4);
    }

    pub fn oct(&self, oct: u8) -> Midi {
        return Midi::from_tone(self.clone(), oct);
    }
}

#[derive(Debug, Clone)]
pub struct FixedSequence {
    /// The notes that can be produced by a sequence
    notes: Vec<Midi>,
    /// The index of the play head into notes
    head_position: usize,
}

impl FixedSequence {
    pub fn new(notes: Vec<Midi>) -> Self {
        return FixedSequence {
            notes,
            head_position: 0,
        }
    }

    pub fn empty() -> Self {
        return FixedSequence {
            notes: Vec::new(),
            head_position: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.notes.len()
    }

    pub fn len_ticks(&self) -> u32 {
        self.notes.clone().into_iter().map(|m| m.duration).sum()
    }

    pub fn fast_forward(mut self, ticks: usize) -> Self {
        self.head_position = (self.head_position + ticks) % self.notes.len();
        self
    }

    pub fn duration(mut self, duration: u32) -> Self {
        self.notes = self.notes.clone().into_iter().map(|m| m.set_duration(duration)).collect();
        self
    }

    pub fn velocity(mut self, velocity: u8) -> Self {
        self.notes = self.notes.clone().into_iter().map(|m| m.set_velocity(velocity)).collect();
        self
    }

    pub fn scale_duration(mut self, factor: u32) -> Self {
        self.notes = self.notes.clone().into_iter().map(|m| m * factor).collect();
        self
    }

    pub fn extend(mut self, rhs: Self) -> Self {
        let mut extend = self.notes.clone();
        extend.append(&mut rhs.notes.clone());
        self.notes = extend;
        self
    }

    pub fn repeat(mut self, times: usize) -> Self {
        self.notes = self.notes.repeat(times);
        self
    }

    pub fn reverse(mut self) -> Self {
        self.notes = self.notes.clone().into_iter().rev().collect();
        self
    }

    pub fn transpose_up(mut self, interval: Interval) -> Self {
        self.notes = self.notes.clone().into_iter().map(|m| m + interval).collect();
        self
    }

    pub fn transpose_down(mut self, interval: Interval) -> Self {
        self.notes = self.notes.clone().into_iter().map(|m| m - interval).collect();
        self
    }
}

impl Add<FixedSequence> for FixedSequence {
    type Output = FixedSequence;

    fn add(self, rhs: FixedSequence) -> Self::Output {
        return self.clone().extend(rhs)
    }
}

impl Mul<usize> for FixedSequence {
    type Output = FixedSequence;

    fn mul(self, rhs: usize) -> Self::Output {
        self.clone().repeat(rhs)
    }
}

impl Sub<Interval> for FixedSequence {
    type Output = FixedSequence;

    fn sub(self, rhs: Interval) -> Self::Output {
        return self.transpose_down(rhs);
    }
}

impl Add<Interval> for FixedSequence {
    type Output = FixedSequence;

    fn add(self, rhs: Interval) -> Self::Output {
        return self.transpose_up(rhs)
    }
}

impl Midibox for FixedSequence {
    fn iter(&self) -> Box<dyn Iterator<Item = Midi> + '_> {
        return Box::new(
            self.notes
                .iter()
                .map(|m| *m)
                .cycle()
                .skip(self.head_position));
    }
}

#[derive(Debug, Clone, Copy)]
pub struct PlayingNote {
    pub channel_id: usize,
    pub start_tick_id: u64,
    pub note: Midi,
}

#[derive(Debug, Clone)]
pub struct Player {
    tick_duration: Duration,
    tick_id: u64,
    note_id: u64,
    channels: Vec<Receiver<Vec<Midi>>>,
    playing_notes: HashMap<u64, PlayingNote>
}

impl Player {
    pub fn new(
        meter: Box<dyn Meter>,
        channels: Vec<Receiver<Vec<Midi>>>
    ) -> Self {
        Player {
            tick_duration: meter.tick_duration(),
            tick_id: 0,
            note_id: 0,
            channels: channels.clone(),
            playing_notes: HashMap::new()
        }
    }

    /// Increment and return the note_id
    fn incr_note_id(&mut self) -> u64 {
        self.note_id += 1;
        return self.note_id;
    }

    /// Increment and return the tick_id
    pub fn tick(&mut self) -> u64 {
        self.tick_id += 1;
        sleep(self.tick_duration);
        return self.tick_id;
    }

    /// Gets the current time in ticks since start
    pub fn time(&self) -> u64 {
        self.tick_id
    }

    /// Determines whether we need to poll the channel for new notes in the sequence
    /// Each channel may send a set of notes to the player -- but cannot send any more notes until
    /// those are done playing. So check that there are no active notes for the channel.
    fn should_poll_channel(&self, channel_id: usize) -> bool {
        self.playing_notes.values()
            .filter(|v| v.channel_id == channel_id)
            .count() == 0
    }

    /// Perform a non-blocking poll of all channels the player is connected to. Each channel may
    /// return a vectors of notes to play simultaneously.
    pub fn poll_channels(&mut self) -> Vec<PlayingNote> {
        // TODO: how to get rid of this clone?
        for (channel_id, note_channel) in self.channels.clone().iter().enumerate() {
            if !self.should_poll_channel(channel_id) {
                continue
            }
            match note_channel.try_recv() {
                Ok(notes) => {
                    println!("Channel {} sent notes {:?}", channel_id, notes);
                    for note in notes {
                        let note_id = self.incr_note_id();
                        if note.duration == 0 {
                            continue; // ignore zero-duration notes
                        }
                        // track the note we're about to play so that we can stop it after the
                        // specified number of ticks has elapsed.
                        self.playing_notes.insert(note_id, PlayingNote {
                            channel_id,
                            start_tick_id: self.tick_id,
                            note,
                        });
                    }
                }
                Err(e) => {
                    println!("Error while reading {} from channel {}", e, channel_id);
                }
            }
        }

        let mut notes : Vec<PlayingNote> = Vec::new();
        notes.extend(
            self.playing_notes
                .values()
                .filter(|note| note.start_tick_id == self.tick_id)
                .map(|note| note.clone())
        );
        notes
    }

    pub fn clear_elapsed_notes(&mut self) -> Vec<PlayingNote> {
        let current_tick = self.tick_id;
        return self.clear_notes(|note| {
            return note.start_tick_id + (note.note.duration as u64) == current_tick
        });
    }

    pub fn clear_all_notes(&mut self) -> Vec<PlayingNote> {
        return self.clear_notes(|_| true);
    }

    fn clear_notes<F>(&mut self, should_clear: F) -> Vec<PlayingNote> where
        F: Fn(&PlayingNote) -> bool
    {
        let mut notes : Vec<PlayingNote> = Vec::new();
        // TODO: how to get rid of this clone?
        self.playing_notes.clone().iter()
            .filter(|(_, playing)| should_clear(playing.clone()))
            .for_each(|(note_id, playing)| {
                notes.push(playing.clone());
                self.playing_notes.remove(&note_id);
            });
        notes
    }
}

#[cfg(test)]
mod tests {
    use crate::Tone;

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
}
