use std::collections::HashMap;
use std::error::Error;
use std::io::{stdin, stdout, Write};
use std::sync::{Arc, Barrier};
use std::thread;
use std::thread::{current, sleep};
use std::time::Duration;

use crossbeam::atomic::AtomicCell;
use crossbeam::channel::{bounded, Receiver, Sender};
use ctrlc;
use midir::{MidiOutput, MidiOutputConnection, MidiOutputPort};

trait Meter {
    fn tick_duration(&self) -> Duration;
}

#[derive(Debug, Clone)]
struct Bpm {
    bpm: u32
}

impl Meter for Bpm {
    fn tick_duration(&self) -> Duration {
        Duration::from_secs(60) / self.bpm
    }
}

impl Bpm {
    fn new(bpm: u32) -> Box<dyn Meter> {
        Box::new(Bpm { bpm })
    }
}

#[derive(Debug, Clone)]
struct FixedSequence {
    /// The notes that can be produced by a sequence
    note_values: Vec<u8>,
    /// How long to hold each note in discrete metronome ticks
    ticks_to_hold: u32,
    /// The index of the play head into note_values. Note that `next()` will increment this, so
    /// when initialized, this value _not_ the first note that will play of the sequence.
    /// TODO: Consider changing this behavior.
    head_position: usize,
}

impl Iterator for FixedSequence {
    type Item = SequenceNote;

    fn next(&mut self) -> Option<Self::Item> {
        let number_of_notes = self.note_values.len();
        if !(self.head_position < number_of_notes) {
            return None
        }
        self.head_position = (self.head_position + 1) % (number_of_notes);
        let note_value = self.note_values.get(self.head_position)?;
        Some(SequenceNote {
            midi: Some(*note_value),
            ticks_to_hold: self.ticks_to_hold
        })
    }
}

#[derive(Debug, Clone)]
struct SequenceNote {
    midi: Option<u8>,
    ticks_to_hold: u32,
}

#[derive(Debug, Clone)]
struct PlayingNote {
    channel_id: usize,
    start_tick_id: u64,
    note: SequenceNote,
}

#[derive(Debug, Clone)]
struct Player {
    tick_duration: Duration,
    tick_id: u64,
    note_id: u64,
    channels: Vec<Receiver<Vec<SequenceNote>>>,
    playing_notes: HashMap<u64, PlayingNote>
}

impl Player {
    /// Increment and return the note_id
    fn incr_note_id(&mut self) -> u64 {
        self.note_id += 1;
        return self.note_id;
    }

    /// Increment and return the tick_id
    fn incr_tick_id(&mut self) -> u64 {
        self.tick_id += 1;
        return self.tick_id;
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
    /// return a vector
    fn poll_channels(&mut self) -> Vec<PlayingNote> {
        for (channel_id, note_channel) in self.channels.clone().iter().enumerate() {
            if !self.should_poll_channel(channel_id) {
                continue
            }
            match note_channel.try_recv() {
                Ok(notes) => {
                    println!("Channel {} sent notes {:?}", channel_id, notes);
                    for note in notes {
                        let note_id = self.incr_note_id();
                        if note.ticks_to_hold == 0 {
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

    fn clear_elapsed_notes(&mut self) -> Vec<PlayingNote> {
        let current_tick = self.tick_id;
        return self.clear_notes(|note| {
            return note.start_tick_id + (note.note.ticks_to_hold as u64) == current_tick
        });
    }

    fn clear_all_notes(&mut self) -> Vec<PlayingNote> {
        return self.clear_notes(|_| true);
    }

    fn clear_notes<F>(&mut self, should_clear: F) -> Vec<PlayingNote> where
        F: Fn(&PlayingNote) -> bool
    {
        let mut notes : Vec<PlayingNote> = Vec::new();
        self.playing_notes.clone().iter()
            .filter(|(_, playing)| should_clear(playing.clone()))
            .for_each(|(note_id, playing)| {
                notes.push(playing.clone());
                self.playing_notes.remove(&note_id);
            });
        notes
    }
}

/// Launches a thread that feeds notes from the provided sequence into a bounded channel, returning a
/// receiver that can be used to poll for notes.
fn spawn_sequence(
    running: &Arc<AtomicCell<bool>>,
    starting_line: &Arc<Barrier>,
    sequence: &FixedSequence
) -> Receiver<Vec<SequenceNote>> {
    let (note_tx, note_rx): (Sender<Vec<SequenceNote>>, Receiver<Vec<SequenceNote>>) = bounded(16);
    let barrier = Arc::clone(&starting_line);
    let to_run = Arc::clone(&running);
    let mut sequence_copy = sequence.clone();
    thread::spawn(move || {
        barrier.wait();

        println!("Midibox Starting.");
        while to_run.load() {
            match note_tx.send(vec![sequence_copy.next().unwrap()]) {
                Ok(_) => {}
                Err(e) => {
                    println!("Encountered error while sending batch of notes {}", e);
                }
            }
        }
        println!("Midibox Exiting.");
    });

    note_rx
}

fn main() {
    match run() {
        Ok(_) => (),
        Err(err) => println!("Error: {}", err)
    }
}

const NOTE_ON_MSG: u8 = 0x90;
const NOTE_OFF_MSG: u8 = 0x80;
const VELOCITY: u8 = 0x64;

fn run() -> Result<(), Box<dyn Error>> {
    let midi_out = MidiOutput::new("Midi Outputs")?;

    // Get an output port (read from console if multiple are available)
    let out_ports = midi_out.ports();
    let out_port: &MidiOutputPort = match out_ports.len() {
        0 => return Err("no output port found".into()),
        1 => {
            println!("Choosing the only available output port: {}", midi_out.port_name(&out_ports[0]).unwrap());
            &out_ports[0]
        },
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

    let mut conn_out = midi_out.connect(out_port, "midibox-out")?;

    let (stop_tx, stop_rx) = bounded(1);
    let (raw_midi_tx, raw_midi_rx): (Sender<[u8; 3]>, Receiver<[u8; 3]>) = bounded(1024);
    let device_cleanup_finished = Arc::clone(&clean_up_finished);
    thread::spawn(move || {
        println!("MIDI Device Starting.");

        while !device_cleanup_finished.load() {
            forward_to_midi_device(&raw_midi_rx, &mut conn_out);
        }

        // drain the channel in case the player sent any cleanup messages
        while !raw_midi_rx.is_empty() {
            forward_to_midi_device(&raw_midi_rx, &mut conn_out);
        }

        stop_tx.send(()).expect("Could not send stop signal.");
        println!("MIDI Device Exiting.");
    });

    let sigint_running = Arc::clone(&running);
    ctrlc::set_handler(move || sigint_running.store(false))?;

    // Set up sequences to play
    let seq_1 = &FixedSequence {
        note_values: vec![
            60, // C4 (middle C)
            67, // G
            64, // E
            71, // B
            69, // A
        ],
        ticks_to_hold: 2,
        head_position: 0
    };
    let seq_2 = &FixedSequence {
        note_values: vec![
            60, // C4 (middle C)
            67, // G
            64, // E
            71, // B
            69, // A
        ],
        ticks_to_hold: 3,
        head_position: 2
    };
    let seq_3 = &FixedSequence {
        note_values: vec![
            60, // C4 (middle C)
            67, // G
            64, // E
            71, // B
            69, // A
        ],
        ticks_to_hold: 5,
        head_position: 1
    };
    let seq_4 = &FixedSequence {
        note_values: vec![
            36, // C2
            45, // A2
        ],
        ticks_to_hold: 32,
        head_position: 1
    };
    let sequences : Vec<&FixedSequence> = vec![
        seq_1,
        seq_2,
        //seq_3,
        seq_4
    ];

    //
    let starting_line = Arc::new(Barrier::new(sequences.len() + 1));
    let mut player = Player {
        tick_duration: Bpm::new(500).tick_duration(),
        tick_id: 0,
        note_id: 0,
        channels: sequences.iter()
            .map(|seq| spawn_sequence(&running, &starting_line, seq))
            .collect(),
        playing_notes: HashMap::new(),
    };
    starting_line.wait();

    let player_running = Arc::clone(&running);
    let player_cleanup_finished = Arc::clone(&clean_up_finished);
    thread::spawn(move || {
        println!("Player Starting.");
        while player_running.load() {
            println!("Seq ID: {}", player.tick_id);
            for note in player.poll_channels() {
                send_note_to_device(&raw_midi_tx, note, NOTE_ON_MSG)
            }

            sleep(player.tick_duration);
            player.incr_tick_id();

            for note in player.clear_elapsed_notes() {
                send_note_to_device(&raw_midi_tx, note, NOTE_OFF_MSG)
            }
        }
        for note in player.clear_all_notes() {
            send_note_to_device(&raw_midi_tx, note, NOTE_OFF_MSG)
        }
        player_cleanup_finished.store(true);
        println!("Player Exiting.");
    });

    stop_rx.recv().expect("Failed while receiving stop signal.");
    Ok(())
}

fn send_note_to_device(raw_midi_tx: &Sender<[u8; 3]>, playing: PlayingNote, code: u8) {
    match playing.note.midi {
        None => { /* resting */ }
        Some(v) => {
            raw_midi_tx.send([code, v, VELOCITY]).expect("Failed to send note!")
        }
    }
}

fn forward_to_midi_device(raw_midi_rx: &Receiver<[u8; 3]>, conn_out: &mut MidiOutputConnection) {
    match raw_midi_rx.recv_timeout(Duration::from_secs(1)) {
        Ok(msg) => {
            let _ = conn_out.send(&msg);
        }
        Err(e) => {
            println!("Error while recieving MIDI data {}", e);
        }
    }
}

