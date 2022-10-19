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

#[derive(Debug)]
struct MusicBox {
    bpm: u32
}

impl MusicBox {
    fn tick_sleep(&self) -> Duration {
        Duration::from_secs(60) / self.bpm
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
            out_ports.get(input.trim().parse::<usize>()?)
                     .ok_or("invalid output port selected")?
        }
    };

    println!("\nOpening connection");
    let music_box = MusicBox {
       bpm: 250,
    };

    let mut conn_out = midi_out.connect(out_port, "midir-test")?;
    println!("Connection open. Listen!");
    {
        let notes: [u8; 8] = [66, 65, 63, 61, 59, 58, 56, 54];

        notes.iter().for_each(|note| {
            const NOTE_ON_MSG: u8 = 0x90;
            const NOTE_OFF_MSG: u8 = 0x80;
            const VELOCITY: u8 = 0x64;
            // We're ignoring errors in here
            let _ = conn_out.send(&[NOTE_ON_MSG, *note, VELOCITY]);
            sleep(music_box.tick_sleep());
            let _ = conn_out.send(&[NOTE_OFF_MSG, *note, VELOCITY]);
        });
    }
    println!("\nClosing connection");
    // This is optional, the connection would automatically be closed as soon as it goes out of scope
    conn_out.close();
    println!("Connection closed");
    Ok(())
}
