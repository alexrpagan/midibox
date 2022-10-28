use std::collections::HashMap;
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
    fn iter(&self) -> Box<dyn Iterator<Item = Note> + '_>;
}

#[derive(Debug, Clone)]
pub struct FixedSequence {
    /// The notes that can be produced by a sequence
    notes: Vec<Option<u8>>,
    /// The velocity to use for notes produced by this sequence
    velocity: Option<u8>,
    /// How long to hold each note in discrete metronome ticks
    duration: u32,
    /// The index of the play head into note_values. Note that `next()` will increment this, so
    /// when initialized, this value _not_ the first note that will play of the sequence.
    /// TODO: Consider changing this behavior.
    head_position: usize,
}

impl FixedSequence {
    pub fn new(notes: Vec<Option<u8>>) -> Self {
        return FixedSequence {
            notes,
            velocity: None,
            duration: 1,
            head_position: 0,
        }
    }
    pub fn fast_forward(mut self, ticks: usize) -> Self {
        self.head_position = (self.head_position + ticks) % self.notes.len();
        self
    }
    pub fn duration(mut self, ticks: u32) -> Self {
        self.duration = ticks;
        self
    }
    pub fn velocity(mut self, velocity: Option<u8>) -> Self {
        self.velocity = velocity;
        self
    }
}

impl Midibox for FixedSequence {
    fn iter(&self) -> Box<dyn Iterator<Item = Note> + '_> {
        return Box::new(
            self.notes
                .iter()
                .map(|pitch| Note {
                    pitch: *pitch,
                    velocity: self.velocity,
                    duration: self.duration
                })
                .cycle()
                .skip(self.head_position));
    }
}

#[derive(Debug, Clone)]
pub struct Note {
    pub pitch: Option<u8>,
    pub velocity: Option<u8>,
    pub duration: u32,
}

#[derive(Debug, Clone)]
pub struct PlayingNote {
    pub channel_id: usize,
    pub start_tick_id: u64,
    pub note: Note,
}

#[derive(Debug, Clone)]
pub struct Player {
    tick_duration: Duration,
    tick_id: u64,
    note_id: u64,
    channels: Vec<Receiver<Vec<Note>>>,
    playing_notes: HashMap<u64, PlayingNote>
}

impl Player {
    pub fn new(
        meter: Box<dyn Meter>,
        channels: Vec<Receiver<Vec<Note>>>
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
