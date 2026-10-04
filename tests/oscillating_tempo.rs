use std::time::Duration;

use midibox::meter::{Meter, Oscillate};

#[test]
fn non_dividing_step_reaches_bounds_before_reversing() {
    let mut meter = Oscillate::new(100, 110, 6.0);
    for bpm in [100, 106, 110, 104, 100, 106, 110, 104, 100] {
        assert_eq!(meter.tick_duration(), Duration::from_secs(60) / bpm);
    }
}

#[test]
fn step_larger_than_range_never_produces_zero_bpm() {
    let mut meter = Oscillate::new(1, 3, 5.0);
    let actual: Vec<_> = (0..6).map(|_| meter.tick_duration()).collect();
    let expected: Vec<_> = [1, 3, 1, 3, 1, 3]
        .into_iter()
        .map(|bpm| Duration::from_secs(60) / bpm)
        .collect();
    assert_eq!(actual, expected);
}

#[test]
fn equal_bounds_produce_a_constant_tempo() {
    let mut meter = Oscillate::new(100, 100, 6.0);
    for _ in 0..6 {
        assert_eq!(meter.tick_duration(), Duration::from_secs(60) / 100);
    }
}

#[test]
fn evenly_dividing_steps_keep_their_existing_tempo_sequence() {
    let mut meter = Oscillate::new(100, 110, 5.0);
    for bpm in [100, 105, 110, 105, 100, 105, 110] {
        assert_eq!(meter.tick_duration(), Duration::from_secs(60) / bpm);
    }
}
