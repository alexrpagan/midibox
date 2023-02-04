use std::collections::HashMap;
use std::error::Error;
use std::io::{stdin, stdout, Write};
use std::sync::Arc;
use std::thread;
use std::thread::sleep;
use std::time::Duration;

use crossbeam::atomic::AtomicCell;
use crossbeam::channel::{bounded, Receiver, Sender};
use ctrlc;
use midir::{MidiOutput, MidiOutputConnection, MidiOutputPort};
use crate::{Meter, Midi, Midibox, NOTE_OFF_MSG, NOTE_ON_MSG};

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
    // could these be slices?
    channels: Vec<Vec<Vec<Midi>>>,
    // could just be a vec / slice
    positions: HashMap<usize, usize>,
    playing_notes: HashMap<u64, PlayingNote>,
}

impl Player {
    pub fn new(
        meter: Box<dyn Meter>,
        channels: Vec<Arc<dyn Midibox>>,
    ) -> Self {
        // better way to initialize?
        let mut pos : HashMap<usize, usize> = HashMap::new();
        for i in 0..channels.len() {
            pos.insert(i, 0);
        }

        Player {
            meter,
            tick_id: 0,
            note_id: 0,
            channels: channels.into_iter()
                .map(|m| m.render())
                .collect(),
            positions: pos,
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
        for (channel_id, note_channel) in self.channels.clone().into_iter().enumerate() {
            if !self.should_poll_channel(channel_id) {
                continue;
            }

            let note_index = *(
                self.positions
                    .get(&channel_id)
                    .expect("missing play position for channel")
            );

            match note_channel.get(note_index) {
                Some(notes) => {
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
                            note: *note,
                        });
                    }
                    self.positions.insert(channel_id, (note_index + 1) % note_channel.len());
                }
                None => {
                    println!("No input from channel {}", channel_id);
                }
            }
        }

        let mut notes: Vec<PlayingNote> = Vec::new();
        notes.extend(
            self.playing_notes
                .values()
                .filter(|note| note.start_tick_id == self.tick_id)
                .map(|note| note)
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
        for (note_id, playing) in self.playing_notes.clone() {
            if should_clear(&playing) {
                self.playing_notes.remove(&note_id);
                notes.push(playing);
            }
        }

        notes
    }
}

pub fn run(bpm: Box<dyn Meter>, sequences: Vec<Arc<dyn Midibox>>) {
    match try_run(bpm, sequences) {
        Ok(_) => (),
        Err(err) => println!("Error: {}", err)
    }
}

pub fn try_run(bpm: Box<dyn Meter>, sequences: Vec<Arc<dyn Midibox>>) -> Result<(), Box<dyn Error>> {
    let midi_out = MidiOutput::new("Midi Outputs")?;

    // TODO: factor out MIDI connection logic into separate module with YAML config
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

    let player_running = Arc::clone(&running);
    let player_cleanup_finished = Arc::clone(&clean_up_finished);

    let mut player = Player::new(
        bpm,
        sequences.clone()
    );

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
        Err(e) => println!("Error while receiving MIDI data {}", e)
    }
}
