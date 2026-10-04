use midibox::arp::Arpeggio;
use midibox::chord::Chord;
use midibox::midi::{Midi, MutMidi};
use midibox::sequences::Seq;
use midibox::tone::Tone;
use midibox::Midibox;

fn progression() -> Seq {
    Seq::chords(vec![
        Chord::new(vec![Tone::C.get(), Tone::E.get(), Tone::G.get()]).duration(5),
        Chord::new(vec![Tone::D.get(), Tone::F.get(), Tone::A.get()]).duration(4),
    ])
}

fn assert_two_cycles(mut arpeggio: Box<dyn Midibox>, expected: &[Midi]) {
    for _ in 0..2 {
        let mut duration = 0;
        for note in expected {
            let actual = arpeggio.next().unwrap();
            assert_eq!(actual, vec![*note]);
            duration += actual[0].duration;
        }
        assert_eq!(duration, progression().total_duration());
    }
}

#[test]
fn ascending_arpeggio_keeps_chord_boundaries() {
    assert_two_cycles(
        Arpeggio::ascend(progression(), 2),
        &[
            Tone::C.get() * 2,
            Tone::E.get() * 2,
            Tone::G.get(),
            Tone::D.get() * 2,
            Tone::F.get() * 2,
        ],
    );
}

#[test]
fn descending_arpeggio_keeps_chord_boundaries() {
    assert_two_cycles(
        Arpeggio::descend(progression(), 2),
        &[
            Tone::G.get() * 2,
            Tone::E.get() * 2,
            Tone::C.get(),
            Tone::A.get() * 2,
            Tone::F.get() * 2,
        ],
    );
}

#[test]
fn custom_arpeggio_keeps_chord_boundaries() {
    assert_two_cycles(
        Arpeggio::custom_order(progression(), 2, vec![1, 2, 0]),
        &[
            Tone::E.get() * 2,
            Tone::G.get() * 2,
            Tone::C.get(),
            Tone::F.get() * 2,
            Tone::A.get() * 2,
        ],
    );
}

#[test]
fn note_longer_than_chord_is_shortened() {
    let notes = vec![Tone::C.get() * 3, Tone::D.get() * 2];
    let mut arpeggio = Arpeggio::ascend(Seq::new(notes.clone()), 10);
    for note in notes.into_iter().cycle().take(4) {
        assert_eq!(arpeggio.next(), Some(vec![note]));
    }
}

#[test]
fn evenly_divisible_chords_keep_full_note_durations() {
    let mut arpeggio = Arpeggio::ascend(progression().duration(6), 2);
    for tone in [Tone::C, Tone::E, Tone::G, Tone::D, Tone::F, Tone::A] {
        assert_eq!(arpeggio.next(), Some(vec![tone.get() * 2]));
    }
}

#[test]
fn long_chord_does_not_overflow_elapsed_duration() {
    let mut arpeggio = Arpeggio::ascend(Seq::new(vec![Tone::C.get() * u32::MAX]), u32::MAX - 1);
    assert_eq!(arpeggio.next(), Some(vec![Tone::C.get() * (u32::MAX - 1)]));
    assert_eq!(arpeggio.next(), Some(vec![Tone::C.get()]));
    assert_eq!(arpeggio.next(), Some(vec![Tone::C.get() * (u32::MAX - 1)]));
}
