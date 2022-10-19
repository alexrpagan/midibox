use std::thread::sleep;
use std::time::Duration;
use std::io::{stdin, stdout, Write};
use std::error::Error;

use midir::{MidiOutput, MidiOutputPort};

fn main() {
    match run() {
        Ok(_) => (),
        Err(err) => println!("Error: {}", err)
    }
}

trait Sequence: Iterator {
}

#[derive(Debug)]
struct FixedSequence {
    // the notes that can be produced by a sequence
    note_values: Vec<u8>,
    // the index of the play head into note_values
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
        let note_value = self.note_values.get(self.head_position).unwrap();
        Some(SequenceNote {
            midi: Some(*note_value),
            ticks_to_hold: 1
        })
    }
}

trait Note {
    fn note(&self) -> Option<u8>;
}

#[derive(Debug)]
struct SequenceNote {
    midi: Option<u8>,
    ticks_to_hold: u32,
}

impl Note for SequenceNote {
    fn note(&self) -> Option<u8> {
        self.midi
    }
}


#[derive(Debug)]
struct RawNote {
    midi: Option<u8>,
    duration: Duration
}

impl Note for RawNote {
    fn note(&self) -> Option<u8> {
        self.midi
    }
}

#[derive(Debug)]
struct Midibox<'a> {
    // the tempo of the Music Box in beats per minute
    bpm: u32,
    // The sequence of notes to draw from
    seq: &'a mut FixedSequence,
}

impl Midibox<'_> {
    fn tick_sleep(&self) -> Duration {
        Duration::from_secs(60) / self.bpm
    }
}

impl Iterator for Midibox<'_> {
    type Item = RawNote;

    fn next(&mut self) -> Option<Self::Item> {
        return match self.seq.next() {
            None => None,
            Some(note) => Some(RawNote {
                midi: note.midi,
                duration: note.ticks_to_hold * self.tick_sleep()
            })
        }
    }
}

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

    println!("\nOpening connection");
    let midibox = Midibox {
        bpm: 250,
        seq: &mut FixedSequence {
            note_values: vec![
                60, // C4 (middle C)
                67, // G
                64, // E
                71, // B
                69, // A
            ],
            head_position: 0
        },
    };

    let midi_port_name = "midibox-out";
    let mut conn_out = midi_out.connect(out_port, midi_port_name)
        .expect("Failed to connect to device");

    midibox.for_each(|note| {
        const NOTE_ON_MSG: u8 = 0x90;
        const NOTE_OFF_MSG: u8 = 0x80;
        const VELOCITY: u8 = 0x64;
        match note.note() {
            None => {}
            Some(v) => {
                let _ = conn_out.send(&[NOTE_ON_MSG, v, VELOCITY]);
            }
        }

        sleep(note.duration);

        // TODO: gracefully exit on interrupt so that we always send an appropriate note-off
        match note.note() {
            None => {}
            Some(v) => {
                let _ = conn_out.send(&[NOTE_OFF_MSG, v, VELOCITY]);
            }
        }
    });
    println!("\nClosing connection");
    // This is optional, the connection would automatically be closed as soon as it goes out of scope
    conn_out.close();
    println!("Connection closed");
    Ok(())
}
