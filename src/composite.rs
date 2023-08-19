use std::sync::Arc;
use crossbeam::atomic::AtomicCell;
use crate::midi::Midi;
use crate::Midibox;

// Utility that allows dynamically choosing between one of several midibox
// instances while playing
pub struct PickChannel<F> where F: Fn() -> Vec<Box<dyn Midibox>> {
    boxen: Vec<Box<dyn Midibox>>,
    // TODO: should we expose a reset behavior vs. the always-advance behavior
    //       implemented here?
    reset: F,
    curr_pos: usize,
    curr_box: Arc<AtomicCell<usize>>,
    prev_box: usize,
    measure_size: usize
}

impl <F> PickChannel<F>
    where F: Fn() -> Vec<Box<dyn Midibox>> + 'static {
    pub fn new(
        measure_size: usize,
        curr_box: Arc<AtomicCell<usize>>,
        init: F
    ) -> Box<dyn Midibox> {
        Box::new(
            PickChannel {
                boxen: (init)(),
                reset: init,
                curr_pos: 0,
                prev_box: curr_box.load(),
                curr_box,
                measure_size
            }
        )
    }
}

impl <F> Midibox for PickChannel<F>
    where F: Fn() -> Vec<Box<dyn Midibox>> {
    fn next(&mut self) -> Option<Vec<Midi>> {
        // advance all boxen
        let results: Vec<Option<Vec<Midi>>> = self.boxen.iter_mut()
            .map(|it| it.next())
            .collect();
        let result = results.get(self.prev_box).unwrap_or(&None);
        self.curr_pos = (self.curr_pos + 1) % self.measure_size;
        let curr = self.curr_box.load();
        if curr != self.prev_box && self.curr_pos == 0 {
            self.prev_box = curr;
        }
        result.clone()
    }
}