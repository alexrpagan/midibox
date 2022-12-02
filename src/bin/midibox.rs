use std::error::Error;
use std::io::{stdin, stdout, Write};
use std::sync::{Arc, Barrier};
use std::thread;
use std::time::Duration;

use crossbeam::atomic::AtomicCell;
use crossbeam::channel::{bounded, Receiver, Sender};
use ctrlc;
use midir::{MidiOutput, MidiOutputConnection, MidiOutputPort};

use musicbox::{Bpm, FixedSequence, Harmonize, Midi, Midibox, Player, PlayingNote, Scale, Tone};
use musicbox::Interval::{Min3, Oct, Perf5};

const NOTE_ON_MSG: u8 = 0x90;
const NOTE_OFF_MSG: u8 = 0x80;

fn main() {
    let scale1 = Scale::major(Tone::G);
    let scale2 = Scale::major(Tone::C);

    let s1 = FixedSequence::new(vec![
        Tone::E.oct(4)  * 16,
        Tone::A.oct(3)  * 16,
        Tone::B.oct(3)  * 16,
    ]);

    let s2 = FixedSequence::new(vec![
        Tone::F.oct(3)  * 16
    ]);

    let s3 = FixedSequence::new(vec![
        Tone::C.oct(3)  * 16
    ]);

    let low  : Vec<bool> = vec![true,  false, false, false, true, true, false, true, false, false, false, true, false, true, false, false];
    let high : Vec<bool> = low.clone().into_iter().map(|x| !x).collect();

    match run(400, vec![
        ((s1.clone() + s2.clone() + s1.clone() + s3.clone()).velocity(70) - Oct - Oct)
            .flatten().mask(low.clone())
        ,
        (s1.clone().velocity(70) - Oct - Oct)
            .harmonize(&scale1, Harmonize::Third)
            .flatten().mask(low.clone())
            +
            (s2.clone().velocity(70) - Oct - Oct)
                .harmonize(&scale2, Harmonize::Third)
                .flatten().mask(low.clone())
            +
            (s1.clone().velocity(70) - Oct - Oct)
                .harmonize(&scale1, Harmonize::Third)
                .flatten().mask(low.clone())
            +
            (s3.clone().velocity(70) - Oct - Oct)
                .harmonize(&scale2, Harmonize::Third)
                .flatten().mask(low.clone())
        ,
        ((s1.clone().velocity(70) - Oct - Oct)
            .flatten().mask(high.clone())
            .harmonize(&scale1, Harmonize::Fifth)
            +
            (s2.clone().velocity(70)
                .harmonize(&scale2, Harmonize::Fourth)
                .flatten().mask(high.clone()))
            +
            (s1.clone().velocity(70) - Oct - Oct)
                .flatten().mask(high.clone())
                .harmonize(&scale1, Harmonize::Fifth)
            +
            ((s3.clone().velocity(70) - Oct - Oct)
                .harmonize(&scale2, Harmonize::Fifth)
                .flatten().mask(high.clone())))
        ,
        (s1.clone().velocity(70) - Oct)
            .flatten().mask(high.clone())
            .harmonize(&scale1, Harmonize::Seventh)
            +
            (s2.clone().velocity(70) - Oct)
                .harmonize(&scale2, Harmonize::Seventh)
                .flatten().mask(high.clone())
            +
                    (s1.clone().velocity(70) - Oct)
            .flatten().mask(high.clone())
            .harmonize(&scale1, Harmonize::Seventh)
            +
            (s3.clone().velocity(70) - Oct)
                .harmonize(&scale2, Harmonize::Seventh)
                .flatten().mask(high.clone())

    ].into_iter().map(|v| -> Arc<dyn Midibox> { Arc::new(v) }).collect()) {
        Ok(_) => (),
        Err(err) => println!("Error: {}", err)
    }
}


/// Launches a thread that feeds notes from the provided sequence into a bounded channel, returning a
/// receiver that can be used to poll for notes.
fn spawn_sequence(
    running: &Arc<AtomicCell<bool>>,
    starting_line: &Arc<Barrier>,
    sequence: &Arc<dyn Midibox>
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
            let batch = vec![seq_iter.next().unwrap()];
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

fn run(bpm: u32, sequences: Vec<Arc<dyn Midibox>>) -> Result<(), Box<dyn Error>> {
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
            .collect()
    );
    starting_line.wait();

    let player_running = Arc::clone(&running);
    let player_cleanup_finished = Arc::clone(&clean_up_finished);
    thread::spawn(move || {
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
    });

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
    device_conn: &mut MidiOutputConnection
) {
    match raw_midi_rx.recv_timeout(Duration::from_secs(30)) {
        Ok(msg) => {
            let _ = device_conn.send(&msg);
        }
        Err(e) => println!("Error while recieving MIDI data {}", e)
    }
}
