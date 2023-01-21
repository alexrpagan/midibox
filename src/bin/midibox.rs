use std::sync::Arc;
use musicbox::{Degree, FixedSequence, Midibox, run, Scale, Tone};
use musicbox::Interval::Oct;

fn main() {
    let c_maj = Scale::major(Tone::C);

    let s1 = FixedSequence::new(vec![
        Tone::E.oct(4)  * 16,
        Tone::A.oct(4)  * 16,
        Tone::C.oct(4)  * 32,

        Tone::A.oct(4)  * 16,
        Tone::C.oct(4)  * 16,
        Tone::F.oct(3)  * 32,

        Tone::E.oct(4)  * 16,
        Tone::A.oct(4)  * 16,
        Tone::C.oct(4)  * 32,

        Tone::E.oct(4)  * 16,
        Tone::A.oct(4)  * 16,
        Tone::F.oct(4)  * 16,
        Tone::C.oct(4)  * 16,
    ]);

    match run(300, vec![
        s1.clone()
            .velocity(70)
            .transpose_down(Oct).flatten().mask(vec![true, false, true, true, false, true, false, true]),
        s1.clone()
            .velocity(60)
            .transpose_down(Oct)
            .harmonize_down(&c_maj, Degree::Fourth)
            .flatten().mask(vec![false, true, false, false, true, false, true, false]),
        s1.clone()
            .flatten().mask(vec![true, false, false, true]),
        s1.clone()
            .velocity(110)
            .harmonize_up(&c_maj, Degree::Third)
            .flatten().mask(vec![false, true, false, false, true]),
        s1.clone()
            .velocity(60)
            .harmonize_down(&c_maj, Degree::Second)
            .flatten().mask(vec![false, false, true, false, false, true]),
        s1.clone()
            .velocity(90)
            .harmonize_up(&c_maj, Degree::Fifth)
            .flatten().mask(vec![true, true, false, false, true]),
        s1.clone()
            .velocity(80)
            .harmonize_down(&c_maj, Degree::Fourth)
            .flatten().mask(vec![false, false, true, true, false, false, true]),
    ].into_iter().map(|v| -> Arc<dyn Midibox> { Arc::new(v) }).collect()) {
        Ok(_) => (),
        Err(err) => println!("Error: {}", err)
    }
}
