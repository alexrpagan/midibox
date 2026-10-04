use midibox::dropout::Dropout;
use midibox::sequences::Seq;
use midibox::tone::Tone;

#[test]
fn non_dividing_notes_follow_the_elapsed_tick_phase() {
    for initially_playing in [true, false] {
        let note = (Tone::C.get() * 3).set_velocity(72);
        let mut dropout = Dropout::wrap(Seq::new(vec![note]).midibox(), 4, initially_playing);
        for tick in (0..48).step_by(3) {
            let notes = dropout.next().unwrap();
            let should_play = (tick / 4) % 2 == 0;
            let should_play = should_play == initially_playing;
            assert_eq!(notes.len(), 1);
            assert_eq!(notes[0].is_rest(), !should_play, "tick {tick}");
            assert_eq!(notes[0].duration, note.duration);
            assert_eq!(notes[0].velocity, note.velocity);
        }
    }
}

#[test]
fn notes_spanning_whole_periods_keep_the_correct_phase() {
    for duration in [4, 8, 12, 16] {
        let mut dropout =
            Dropout::wrap(Seq::new(vec![Tone::C.get() * duration]).midibox(), 4, true);
        for event in 0..4 {
            let notes = dropout.next().unwrap();
            assert_eq!(notes[0].is_rest(), (event * duration / 4) % 2 == 1);
        }
    }
}

#[test]
fn long_notes_do_not_overflow_elapsed_ticks() {
    let duration = u32::MAX - 1;
    let mut dropout = Dropout::wrap(
        Seq::new(vec![Tone::C.get() * duration]).midibox(),
        u32::MAX,
        true,
    );
    for event in 0..4_u64 {
        let notes = dropout.next().unwrap();
        let periods = event * u64::from(duration) / u64::from(u32::MAX);
        assert_eq!(notes[0].is_rest(), periods % 2 == 1);
    }
}

#[test]
fn zero_duration_keeps_switching_on_every_poll() {
    let mut dropout = Dropout::wrap(Seq::new(vec![Tone::C.get()]).midibox(), 0, true);
    for expected_rest in [true, false, true, false] {
        assert_eq!(dropout.next().unwrap()[0].is_rest(), expected_rest);
    }
}
