use crate::Midibox;
use crate::chord::Chord;
use crate::midi::{Midi, MutMidi};
use crate::sequences::Seq;

pub struct Arpeggio {
    // which chord (i.e., index into seq) are we playing?
    chord_position: usize,
    // how many iterations have we played on this chord?
    iterations_at_position: usize,
    // how long have we spent playing this chord?
    duration_at_position: u32,
    // which chords should we play
    to_play: Seq,
    // what chord are we playing?
    current_chord: Option<Chord>,
    // how should it be played?
    pattern: Pattern
}

pub struct Pattern {
    mask: Vec<Box<dyn SelectMidi>>
}

trait SelectMidi {
    fn select(&self, chord: &Chord, iterations_at_position: usize) -> Vec<Midi>;
}

pub struct Ascend {
    note_duration: u32,
}

impl SelectMidi for Ascend {
    fn select(&self, chord: &Chord, ticks: usize) -> Vec<Midi> {
        let selected = chord.notes.get(ticks % chord.notes.len());
        return match selected {
            None => vec![],
            Some(note) => vec![note.set_duration(self.note_duration)]
        };
    }
}

pub struct Descend {
    note_duration: u32,
}

impl SelectMidi for Descend {
    fn select(&self, chord: &Chord, ticks: usize) -> Vec<Midi> {
        let selected = chord.notes.get((chord.notes.len() - 1) - ticks % chord.notes.len());
        return match selected {
            None => vec![],
            Some(note) => vec![note.set_duration(self.note_duration)]
        };
    }
}

pub struct CustomOrder {
    note_order: Vec<usize>,
    note_duration: u32
}

impl SelectMidi for CustomOrder {
    fn select(&self, chord: &Chord, iterations_at_position: usize) -> Vec<Midi> {
        let to_play: Option<&Midi> = self.note_order.get(
            iterations_at_position % self.note_order.len()
        ).and_then(|position| chord.notes.get(*position));

        return to_play
            .map(|n| vec![n.set_duration(self.note_duration)])
            .unwrap_or(vec![]);
    }
}


impl Arpeggio {
    pub fn wrap(seq: Seq, pattern: Pattern) -> Box<dyn Midibox> {
        Box::new(Arpeggio {
            chord_position: 0,
            iterations_at_position: 0,
            duration_at_position: 0,
            to_play: seq,
            current_chord: None,
            pattern
        })
    }

    pub fn ascend(seq: Seq, note_duration: u32) -> Box<dyn Midibox> {
        Arpeggio::wrap(seq, Pattern {
            mask: vec![Box::new(Ascend { note_duration })]
        })
    }

    pub fn descend(seq: Seq, note_duration: u32) -> Box<dyn Midibox> {
        Arpeggio::wrap(seq, Pattern {
            mask: vec![Box::new(Descend { note_duration })]
        })
    }

    pub fn custom_order(seq: Seq, note_duration: u32, note_order: Vec<usize>) -> Box<dyn Midibox> {
        Arpeggio::wrap(seq, Pattern {
            mask: vec![Box::new(CustomOrder { note_order, note_duration })]
        })
    }
}

impl Midibox for Arpeggio {
    fn next(&mut self) -> Option<Vec<Midi>> {
        if self.current_chord == None {
            self.current_chord = self.to_play.get_chords()
                .get(self.chord_position)
                .map(|c| c.clone())
        }

        // if we can't index into chords, just don't play anything
        if self.current_chord == None {
            self.chord_position = 0;
            self.iterations_at_position = 0;
            self.duration_at_position = 0;
            return None;
        }
        let chord = self.current_chord.clone().unwrap();

        let mut result: Vec<Midi> = vec![];
        for select in self.pattern.mask.iter() {
            result.append(&mut select.select(&chord, self.iterations_at_position));
        }
        let max_duration = result.iter()
            .map(|to_play| to_play.duration)
            .max()
            .unwrap_or(0);

        self.iterations_at_position += 1;
        self.duration_at_position += max_duration;

        if self.duration_at_position >= chord.total_duration() {
            self.chord_position = (self.chord_position + 1) % self.to_play.get_chords().len();
            self.duration_at_position = 0;
            self.iterations_at_position = 0;
            self.current_chord = None;
        }

        return Some(result.clone());
    }
}
