# Midibox

`Midibox` provides an API for playing MIDI `Sequence`s.

`Sequences` are Rust `Iterable`s that generate `Note`s to play by an instance of `Midibox`, which 
supplies a `Meter` (incl. tempo as BPM, time signature) via a CLI that communicates with the user 
and manages connections to external and software MIDI devices.

## `Note`
Each `Note` can describe:
- Which pitch to play, represented as a `u8` (`60` == C4 == Middle C)
- Velocity, represented as `u8` 
- How long each note should sound, represented as:
  - A number of `ticks: u32` of a `Metronome`
  - A concrete `Duration` of wall-clock time

TODO:
- [ ] Link to good description of MIDI spec

# Roadmap

## Send MIDI to External Devices
10/19/2022: 
Incorporated `midir` to establish connection to a MIDI port. Implemented basic sequencer.

Next:
- [x] Support for physical devices over USB
- [x] Support of simple looping sequences (fixed note sequence, static duration)
- [ ] Run sequences in a new thread that can be gracefully stopped via user input (e.g., Ctrl-C / `SIGINT`)
- [ ] Support for dynamic note duration -- i.e., duration is a function of previous n events
- [ ] Support for dynamic velocity -- i.e., velocity is function of previous n events
- [ ] Break core library classes out of `Midibox` execution
- [ ] Create CLI that just manages connection & configuration for external MIDI devices, runs 
     `Midibox` executor
- [ ] Support for sending MIDI to software synthesizers running in a DAW like Ableton Live.

Misc:
- [ ] Create Github & check in

### Future: Distributed Execution

There is a pool of N (=1 to start) coordinators and M workers. Clients propose new plans to 
coordinators, and if accepted, plans are sent to workers, which download the plan executable, 
finish processing their current batch of input events, wait to start, and then begin running 
`Midibox` simultaneously.

We could use WASM (straightforwardly? -- TBD) as a intermediate representation, e.g., for wire 
transfer across multiple devices.

Parameters:
- Global wall clock (chrony?)
- Global logical clock (happens before?)
- Batch size
- Delay before accepting next batch (e.g., coordination period)

Design Goals:
- Minimize start up time before accepting new ensemble
- Minimize event-delivery jitter (i.e., delta between actual event time and expected event time)
