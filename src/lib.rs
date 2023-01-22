use std::collections::HashMap;
use std::error::Error;
use std::io::{stdin, stdout, Write};
use std::ops::{Add, Mul, Sub};
use std::sync::{Arc, Barrier};
use std::thread;
use std::thread::sleep;
use std::time::Duration;

use crossbeam::atomic::AtomicCell;
use crossbeam::channel::{bounded, Receiver, Sender};
use ctrlc;
use midir::{MidiOutput, MidiOutputConnection, MidiOutputPort};

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
    pub fn new(bpm: u32) -> Box<dyn Meter> {
        Box::new(Bpm { bpm })
    }
}

pub trait Midibox: Send + Sync {
    fn iter(&self) -> Box<dyn Iterator<Item=Vec<Midi>> + '_>;
}


#[derive(Debug, Clone)]
pub struct Scale {
    root: Tone,
    intervals: Vec<u8>,
}

impl Scale {
    pub fn major(root: Tone) -> Self {
        return Scale {
            root,
            intervals: vec![
                2, // Whole step
                2, // W
                1, // Half step
                2, // W
                2, // W
                2, // W
                1,  // H
            ],
        };
    }

    pub fn tones(&self) -> Vec<Tone> {
        self.midi(4).into_iter().map(|m| m.tone).collect()
    }

    pub fn midi(&self, oct: u8) -> Vec<Midi> {
        let mut midi = Vec::new();
        midi.push(self.root.oct(oct));
        for interval in self.intervals.clone().into_iter().take(self.intervals.len() - 1) {
            midi.push(Midi::from_option(
                midi.last().unwrap().u8_maybe().map(|v| v + interval)
            ))
        }
        return midi;
    }

    pub fn harmonize_up(&self, midi: Midi, harmonize: Degree) -> Option<Midi> {
        let tones = self.tones();
        let degree_maybe = tones.into_iter().position(|t| t.eq(&midi.tone));
        return match degree_maybe {
            None => None,
            Some(pos) => {
                let steps_to_raise: u8 = self.intervals
                    .clone()
                    .into_iter()
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
                let scale_at_pos: Vec<u8> = self.intervals
                    .clone()
                    .into_iter()
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
        return match self {
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
        };
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
        return Midi {
            tone: Tone::Rest,
            oct: DEFAULT_OCT,
            velocity: DEFAULT_VELOCITY,
            duration: DEFAULT_DURATION,
        };
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
        return Midi { tone, oct, velocity: DEFAULT_VELOCITY, duration: DEFAULT_DURATION };
    }

    pub fn from(val: u8) -> Midi {
        return Midi::from_tone(Tone::from(val), Midi::oct(val));
    }

    pub fn is_rest(&self) -> bool {
        return match self.tone {
            Tone::Rest => true,
            _ => false
        };
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
        return Midi { tone, oct, velocity: self.velocity, duration: self.duration };
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
        return self.clone().set_duration(self.duration * rhs);
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
        };
    }

    pub fn empty() -> Self {
        return FixedSequence {
            notes: Vec::new(),
            head_position: 0,
        };
    }

    pub fn len(&self) -> usize {
        self.notes.len()
    }

    pub fn len_ticks(&self) -> u32 {
        return self.notes.clone().into_iter().map(|m| m.duration).sum();
    }

    pub fn fast_forward(mut self, ticks: usize) -> Self {
        self.head_position = (self.head_position + ticks) % self.notes.len();
        self
    }

    pub fn duration(mut self, duration: u32) -> Self {
        self.notes = self.notes.into_iter().map(|m| m.set_duration(duration)).collect();
        self
    }

    pub fn velocity(mut self, velocity: u8) -> Self {
        self.notes = self.notes.into_iter().map(|m| m.set_velocity(velocity)).collect();
        self
    }

    pub fn scale_duration(mut self, factor: u32) -> Self {
        self.notes = self.notes.into_iter().map(|m| m * factor).collect();
        self
    }

    pub fn extend(mut self, rhs: &Self) -> Self {
        let mut extend = self.notes;
        extend.append(&mut rhs.notes.clone());
        self.notes = extend;
        self
    }

    pub fn repeat(mut self, times: usize) -> Self {
        self.notes = self.notes.repeat(times);
        self
    }

    pub fn reverse(mut self) -> Self {
        self.notes = self.notes.into_iter().rev().collect();
        self
    }

    pub fn transpose_up(mut self, interval: Interval) -> Self {
        self.notes = self.notes.into_iter().map(|m| m + interval).collect();
        self
    }

    pub fn transpose_down(mut self, interval: Interval) -> Self {
        self.notes = self.notes.into_iter().map(|m| m - interval).collect();
        self
    }

    pub fn harmonize_up(mut self, scale: &Scale, degree: Degree) -> Self {
        self.notes = self.notes.into_iter()
            .map(|m| if m.is_rest() {
                m
            } else {
                scale
                    .harmonize_up(m, degree)
                    .unwrap_or_else(|| m.set_pitch(Tone::Rest, 4))
            })
            .collect();
        self
    }

    pub fn harmonize_down(mut self, scale: &Scale, degree: Degree) -> Self {
        self.notes = self.notes.into_iter()
            .map(|m| if m.is_rest() {
                m
            } else {
                scale
                    .harmonize_down(m, degree)
                    .unwrap_or_else(|| m.set_pitch(Tone::Rest, 4))
            })
            .collect();
        self
    }

    /// Splits each note into a series of metronome ticks adding to the note's duration
    pub fn split_to_ticks(mut self) -> Self {
        self.notes = self.notes.into_iter().flat_map(|m| {
            let old_duration = m.duration as usize;
            return vec![m.set_duration(1)].repeat(old_duration).into_iter();
        }).collect::<Vec<Midi>>();
        return self;
    }

    /// mask is a sequence of bits representing notes to play or mute
    ///
    /// If the bit corresponding to a note in this sequence is false, the note will be muted.
    ///
    /// The mask will be applied starting from the first note of the sequence and will repeat to
    /// match the total number of notes in this sequence.
    pub fn mask(mut self, mask: Vec<bool>) -> Self {
        self.notes = self.notes.into_iter()
            .zip(mask.into_iter().cycle()).map(|(midi, should_play)| {
            return if should_play {
                midi
            } else {
                midi.set_pitch(Tone::Rest, 4)
            };
        }).collect();
        self
    }

    pub fn split_notes(self, mask: Vec<bool>) -> Self {
        self.split_to_ticks().mask(mask)
    }
}

impl Add<FixedSequence> for FixedSequence {
    type Output = FixedSequence;

    fn add(self, rhs: FixedSequence) -> Self::Output {
        return self.clone().extend(&rhs.clone());
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
        return self.transpose_up(rhs);
    }
}

impl Midibox for FixedSequence {
    fn iter(&self) -> Box<dyn Iterator<Item=Vec<Midi>> + '_> {
        return Box::new(
            self.notes
                .iter()
                .map(|m| vec![*m])
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

pub struct Player {
    meter: Box<dyn Meter>,
    tick_id: u64,
    note_id: u64,
    channels: Vec<Receiver<Vec<Midi>>>,
    playing_notes: HashMap<u64, PlayingNote>,
}

impl Player {
    pub fn new(
        meter: Box<dyn Meter>,
        channels: Vec<Receiver<Vec<Midi>>>,
    ) -> Self {
        Player {
            meter,
            tick_id: 0,
            note_id: 0,
            channels: channels.clone(),
            playing_notes: HashMap::new(),
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
        sleep(self.meter.tick_duration());
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
                continue;
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

        let mut notes: Vec<PlayingNote> = Vec::new();
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
            return note.start_tick_id + (note.note.duration as u64) == current_tick;
        });
    }

    pub fn clear_all_notes(&mut self) -> Vec<PlayingNote> {
        return self.clear_notes(|_| true);
    }

    fn clear_notes<F>(&mut self, should_clear: F) -> Vec<PlayingNote> where
        F: Fn(&PlayingNote) -> bool
    {
        let mut notes: Vec<PlayingNote> = Vec::new();
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

pub fn run(bpm: u32, sequence: Vec<Arc<dyn Midibox>>) {
    match try_run(bpm, sequence) {
        Ok(_) => (),
        Err(err) => println!("Error: {}", err)
    }
}

pub fn try_run(bpm: u32, sequences: Vec<Arc<dyn Midibox>>) -> Result<(), Box<dyn Error>> {
    let midi_out = MidiOutput::new("Midi Outputs")?;

    // Get an output port (read from console if multiple are available)
    let out_ports = midi_out.ports();
    let out_port: &MidiOutputPort = match out_ports.len() {
        0 => return Err("no output port found".into()),
        1 => {
            println!("Choosing the only available output port: {}", midi_out.port_name(&out_ports[0]).unwrap());
            &out_ports[0]
        }
        _ => {
            println!("\nAvailable output ports:");
            for (i, p) in out_ports.iter().enumerate() {
                println!("{}: {}", i, midi_out.port_name(p).unwrap());
            }
            print!("Please select output port: ");
            stdout().flush()?;
            let mut input = String::new();
            stdin().read_line(&mut input)?;
            out_ports
                .get(input.trim().parse::<usize>()?)
                .ok_or("invalid output port selected")?
        }
    };

    // flag to determine whether to keep running (e.g., while looping in sequence / player threads)
    let running = Arc::new(AtomicCell::new(true));
    let clean_up_finished = Arc::new(AtomicCell::new(false));

    let mut device_conn = midi_out.connect(out_port, "midibox-out")?;

    // channel to indicate that `main` should exit
    let (exit_tx, exit_rx) = bounded(1);

    // channel to transmit messages from the Player to the MIDI device
    let (raw_midi_tx, raw_midi_rx): (Sender<[u8; 3]>, Receiver<[u8; 3]>) = bounded(1024);

    // true when the player is done accepting input and has sent NOTE OFF for all playing notes
    let device_cleanup_finished = Arc::clone(&clean_up_finished);
    thread::spawn(move || {
        println!("MIDI Device Starting.");

        while !device_cleanup_finished.load() {
            forward_to_midi_device(&raw_midi_rx, &mut device_conn);
        }

        // drain the channel in case the player sent any note-off messages
        while !raw_midi_rx.is_empty() {
            forward_to_midi_device(&raw_midi_rx, &mut device_conn);
        }

        exit_tx.send(()).expect("Could not send stop signal.");
        println!("MIDI Device Exiting.");
    });

    let ctrlc_running = Arc::clone(&running);
    ctrlc::set_handler(move || ctrlc_running.store(false))?;

    // make sure that all sequence threads have started before starting ticker
    let starting_line = Arc::new(Barrier::new(sequences.len() + 1));
    let mut player = Player::new(
        Bpm::new(bpm),
        sequences.iter()
            .map(|seq| spawn_sequence(&running, &starting_line, &seq))
            .collect(),
    );
    starting_line.wait();

    let player_running = Arc::clone(&running);
    let player_cleanup_finished = Arc::clone(&clean_up_finished);
    println!("Player Starting.");
    while player_running.load() {
        println!("Time: {}", player.time());
        for note in player.poll_channels() {
            send_note_to_device(&raw_midi_tx, note, NOTE_ON_MSG)
        }
        player.tick();
        for note in player.clear_elapsed_notes() {
            send_note_to_device(&raw_midi_tx, note, NOTE_OFF_MSG)
        }
    }
    for note in player.clear_all_notes() {
        send_note_to_device(&raw_midi_tx, note, NOTE_OFF_MSG)
    }
    player_cleanup_finished.store(true);
    println!("Player Exiting.");

    exit_rx.recv()?;
    Ok(())
}

/// Launches a thread that feeds notes from the provided sequence into a bounded channel, returning a
/// receiver that can be used to poll for notes.
fn spawn_sequence(
    running: &Arc<AtomicCell<bool>>,
    starting_line: &Arc<Barrier>,
    sequence: &Arc<dyn Midibox>,
) -> Receiver<Vec<Midi>> {
    let (note_tx, note_rx)
        : (Sender<Vec<Midi>>, Receiver<Vec<Midi>>) = bounded(1024);
    let barrier = Arc::clone(&starting_line);
    let to_run = Arc::clone(&running);
    let midibox = sequence.clone();
    thread::spawn(move || {
        // block until all other sequence threads are ready to start
        barrier.wait();
        let mut seq_iter = midibox.iter();
        println!("Midibox Starting.");
        while to_run.load() {
            // TODO: gracefully handle this error instead of `unwrap`
            let batch = seq_iter.next().unwrap();
            match note_tx.send(batch) {
                Ok(_) => {}
                Err(e) => {
                    println!("Encountered while sending notes: {}", e);
                }
            }
        }
        println!("Midibox Exiting.");
    });

    note_rx
}

fn send_note_to_device(raw_midi_tx: &Sender<[u8; 3]>, playing: PlayingNote, midi_status: u8) {
    match playing.note.u8_maybe() {
        None => { /* resting */ }
        Some(v) => {
            raw_midi_tx
                .send([midi_status, v, playing.note.velocity])
                .expect("Failed to send note!")
        }
    }
}

fn forward_to_midi_device(
    raw_midi_rx: &Receiver<[u8; 3]>,
    device_conn: &mut MidiOutputConnection,
) {
    match raw_midi_rx.recv_timeout(Duration::from_secs(30)) {
        Ok(msg) => {
            let _ = device_conn.send(&msg);
        }
        Err(e) => println!("Error while recieving MIDI data {}", e)
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
