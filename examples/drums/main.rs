use std::collections::HashMap;
use std::sync::{Arc, mpsc};
use std::thread;
use std::thread::JoinHandle;
use crossbeam::atomic::AtomicCell;
use eframe::egui;
use egui::Key::N;
use rand::Rng;
use midibox::drumlogue::Drumlogue::{*};
use midibox::tone::Tone;
use midibox::meter::{Bpm, SyncBpm};
use midibox::sequences::Seq;
use midibox::player::{PlayerConfig, try_run};
use midibox::scale::{Degree, Direction, Interval, Scale};
use midibox::{map_beat, map_chords, map_notes, Midibox, seq};
use midibox::arp::Arpeggio;
use midibox::chord::{Chord, ToChord};
use midibox::composite::PickChannel;
use midibox::dropout::random_dropout;
use midibox::drumlogue::Drumlogue;
use midibox::midi::{Midi, ToMidi};
use midibox::rand::{random_velocity, random_velocity_range};
use midibox::router::MapRouter;
use midibox::scale::Degree::{*};
use midibox::scale::Pitch::{*};
use midibox::scale::Direction::{*};
use midibox::tone::Tone::{Ab, C, D, E, Eb, F, G, Gb, Rest};



fn main() {
    env_logger::init();

    let mut chan_to_port = HashMap::new();
    for i in 0..2 {
        chan_to_port.insert(i, 0);
    }
    chan_to_port.insert(2, 4);
    chan_to_port.insert(3, 1);
    chan_to_port.insert(4, 2);
    chan_to_port.insert(5, 3);
    chan_to_port.insert(6, 5);

    // Our application state:
    let mut tempo = 550;
    let mut drum_pattern = 0;

    let drum_pattern_midibox: Arc<AtomicCell<usize>> = Arc::new(AtomicCell::new(drum_pattern));
    let drum_pattern_ui = drum_pattern_midibox.clone();

    let tempo_midibox = Arc::new(AtomicCell::new(tempo));
    let tempo_ui = tempo_midibox.clone();

    let handle = thread::spawn(|| {
        let pattern_left = &vec![
            false, false, false, true, false, false,
            true, false, false, false, true, false,
            false, false, false, true, false, false,
            true, false, false, false, false, true,
            false, false, false, true, false, false,
            true, false, false, false, true, false,
            false, false, false, true, false, true,
            true, false, false, false, true, false,
        ];

        let pattern_right = &vec![
            true, false, false, true, false, false,
            false, false, false, false, true, false,
            true, false, false, true, false, false,
            false, true, false, false, true, false,
            true, false, false, true, false, false,
            false, false, false, false, true, false,
            true, false, false, true, false, true,
            false, true, false, false, false, true,
        ];

        try_run(
            PlayerConfig::from_router(Box::new(MapRouter::new(chan_to_port))),
            &mut SyncBpm::new(tempo_midibox),
            &mut vec![
                PickChannel::new(
                    32,
                    drum_pattern_midibox,
                    || vec![kick_only(), drum_fast()]
                ),
                hat_accent(),
                hats(),
                broken_chords(pattern_left),
                broken_chords(pattern_right),
                bass(),
                harmony_chords(scale()).midibox()
            ]
        ).unwrap()
    });

    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(320.0, 240.0)),
        ..Default::default()
    };

    let _ui_result = eframe::run_simple_native("Drums", options, move |ctx, _frame| {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Drums");
            ui.add(egui::Slider::new(&mut tempo, 0..=1000).text("tempo"));
            tempo_ui.store(tempo);

            ui.add(egui::Slider::new(&mut drum_pattern, 0..=1).text("pattern"));
            drum_pattern_ui.store(drum_pattern);

            ui.label(format!("Tempo: {tempo}, Drum Pattern: {drum_pattern}"));
        });
    });

    // TODO: example thread not existing correctly on CTRL-C
    let _midibox_result = handle.join();
}

fn bass() -> Box<dyn Midibox> {
    let scale = scale();
    seq![
        scale.make_chord(
            2,
            Second,
            &vec![
                Harmonize(Unison, UpOct(0)),
            ]
        ).unwrap(),
        scale.make_chord(
            2,
            Sixth,
            &vec![
                Harmonize(Fifth, UpOct(0)),
            ]
        ).unwrap(),
            scale.make_chord(
            2,
            Second,
            &vec![
                Harmonize(Third, UpOct(0)),
            ]
        ).unwrap(),
        scale.make_chord(
            2,
            Sixth,
            &vec![
                Harmonize(Unison, UpOct(0)),
            ]
        ).unwrap(),
        scale.make_chord(
            2,
            Second,
            &vec![
                Harmonize(Unison, UpOct(0)),
            ]
        ).unwrap(),
        scale.make_chord(
            2,
            Sixth,
            &vec![
                Harmonize(Fifth, UpOct(0)),
            ]
        ).unwrap(),
            scale.make_chord(
            2,
            Second,
            &vec![
                Harmonize(Third, UpOct(0)),
            ]
        ).unwrap(),
        scale.make_chord(
            2,
            Sixth,
            &vec![
                Harmonize(Unison, UpOct(0)),
            ]
        ).unwrap(),
        scale.make_chord(
            2,
            Fourth,
            &vec![
                Harmonize(Unison, UpOct(0)),
            ]
        ).unwrap(),
        scale.make_chord(
            2,
            Third,
            &vec![
                Harmonize(Third, UpOct(0)),
            ]
        ).unwrap(),
        scale.make_chord(
            3,
            Second,
            &vec![
                Harmonize(Unison, UpOct(0)),
            ]
        ).unwrap(),
        scale.make_chord(
            2,
            Sixth,
            &vec![
                Harmonize(Unison, UpOct(0)),
            ]
        ).unwrap()
    ].duration(32).midibox()
}

fn broken_chords(pattern: &Vec<bool>) -> Box<dyn Midibox> {
    let scale = scale();
    let chords = base_chords(scale).split_notes(pattern).midibox();
    let velocity = random_velocity_range(
        chords, 50, 110
    );

    let random_dropout = random_dropout(velocity, 0.01);
    map_chords(random_dropout, |c| {
        let i_1 = rand::thread_rng().gen_range(0..c.notes.len());
        let i_2 = rand::thread_rng().gen_range(0..c.notes.len());
        let i_3 = rand::thread_rng().gen_range(0..c.notes.len());
        Chord::new(vec![
            *c.notes.get(i_1).unwrap(),
            *c.notes.get(i_2).unwrap(),
            *c.notes.get(i_3).unwrap(),
        ])
    })
}

fn scale() -> Scale {
    Scale::major(F)
}

fn down_arpeggio() -> Box<dyn Midibox> {
    base_chords(scale()).midibox()
}

fn base_chords(scale: Scale) -> Seq {
    seq![
        scale.make_chord(
            3,
            Second,
            &vec![
                Harmonize(Unison, UpOct(-2)),
                Harmonize(Third, Up),
                Harmonize(Fifth, UpOct(-1)),
                Harmonize(Ninth, Up)
            ]
        ).unwrap(),
        scale.make_chord(
            3,
            Sixth,
            &vec![
                Harmonize(Unison, UpOct(-2)),
                Harmonize(Third, UpOct(-1)),
                Harmonize(Fifth, UpOct(-1)),
                Harmonize(Ninth, Up)
            ]
        ).unwrap(),
            scale.make_chord(
            3,
            Second,
            &vec![
                Harmonize(Unison, UpOct(-2)),
                Harmonize(Third, Up),
                Harmonize(Fifth, UpOct(-1)),
                Harmonize(Ninth, Up)
            ]
        ).unwrap(),
        scale.make_chord(
            2,
            Sixth,
            &vec![
                Harmonize(Unison, UpOct(-2)),
                Harmonize(Third, Up),
                Harmonize(Fifth, UpOct(-1)),
                Harmonize(Ninth, Up)
            ]
        ).unwrap(),
            scale.make_chord(
            3,
            Second,
            &vec![
                Harmonize(Unison, UpOct(-2)),
                Harmonize(Third, UpOct(-1)),
                Harmonize(Fourth, Up),
                Harmonize(Second, Up),
                Harmonize(Ninth, Up),
                Harmonize(Seventh, UpOct(-1)),
            ]
        ).unwrap(),
        scale.make_chord(
            3,
            Sixth,
            &vec![
                Harmonize(Unison, UpOct(-2)),
                Harmonize(Third, UpOct(-1)),
                Harmonize(Fifth, UpOct(-1)),
                Harmonize(Second, Up),
                Harmonize(Ninth, Up)
            ]
        ).unwrap(),
            scale.make_chord(
            3,
            Second,
            &vec![
                Harmonize(Unison, UpOct(-2)),
                Harmonize(Third, Up),
                Harmonize(Fifth, UpOct(-1)),
                Harmonize(Ninth, Up)
            ]
        ).unwrap(),
        scale.make_chord(
            2,
            Sixth,
            &vec![
                Harmonize(Unison, UpOct(-2)),
                Harmonize(Third, Up),
                Harmonize(Fifth, UpOct(-1)),
                Harmonize(Ninth, Up)
            ]
        ).unwrap(),
        scale.make_chord(
            3,
            Fourth,
            &vec![
                Harmonize(Unison, UpOct(-1)),
                Harmonize(Third, UpOct(-1)),
                Harmonize(Third, UpOct(1)),
            ]
        ).unwrap(),
        scale.make_chord(
            3,
            Third,
            &vec![
                Harmonize(Unison, Up),
                Harmonize(Third, UpOct(1)),
            ]
        ).unwrap(),
        scale.make_chord(
            3,
            Second,
            &vec![
                Harmonize(Unison, UpOct(-2)),
                Harmonize(Fifth, UpOct(-1)),
                Harmonize(Ninth, Up)
            ]
        ).unwrap(),
        scale.make_chord(
            2,
            Sixth,
            &vec![
                Harmonize(Unison, UpOct(-2)),
                Harmonize(Third, Up),
                Harmonize(Fifth, UpOct(-1)),
                Harmonize(Ninth, Up)
            ]
        ).unwrap()

    ].duration(32)
}

fn harmony_chords(scale: Scale) -> Seq {
    seq![
        scale.make_chord(
            3,
            Second,
            &vec![
                Harmonize(Third, Up),
                Harmonize(Ninth, Up)
            ]
        ).unwrap(),
        scale.make_chord(
            3,
            Sixth,
            &vec![
                Harmonize(Third, Up),
                Harmonize(Ninth, Up)
            ]
        ).unwrap(),
            scale.make_chord(
            3,
            Second,
            &vec![
                Harmonize(Third, Up),
                Harmonize(Ninth, Up)
            ]
        ).unwrap(),
        scale.make_chord(
            2,
            Sixth,
            &vec![
                Harmonize(Third, Up),
                Harmonize(Ninth, Up)
            ]
        ).unwrap(),
            scale.make_chord(
            3,
            Second,
            &vec![
                Harmonize(Third, Up),
                Harmonize(Second, Up),
                Harmonize(Ninth, Up),
                Harmonize(Seventh, Up),
            ]
        ).unwrap(),
        scale.make_chord(
            3,
            Sixth,
            &vec![
                Harmonize(Third, Up),
                Harmonize(Second, Up),
                Harmonize(Ninth, Up)
            ]
        ).unwrap(),
            scale.make_chord(
            3,
            Second,
            &vec![
                Harmonize(Third, Up),
                Harmonize(Ninth, Up)
            ]
        ).unwrap(),
        scale.make_chord(
            2,
            Sixth,
            &vec![
                Harmonize(Third, Up),
                Harmonize(Ninth, Up)
            ]
        ).unwrap(),
        scale.make_chord(
            3,
            Fourth,
            &vec![
                Harmonize(Unison, Up),
                Harmonize(Third, UpOct(1)),
            ]
        ).unwrap(),
        scale.make_chord(
            3,
            Third,
            &vec![
                Harmonize(Unison, Up),
                Harmonize(Third, UpOct(1)),
            ]
        ).unwrap(),
        scale.make_chord(
            3,
            Second,
            &vec![
                Harmonize(Third, Up),
                Harmonize(Ninth, Up)
            ]
        ).unwrap(),
        scale.make_chord(
            2,
            Sixth,
            &vec![
                Harmonize(Third, Up),
                Harmonize(Second, Up),
                Harmonize(Ninth, Up)
            ]
        ).unwrap()

    ].duration(32)
}

fn hat_accent() -> Box<dyn Midibox> {
    seq![
        Tone::Rest * 16,
        Drumlogue::OH,
        CH,
        Tone::Rest * 14,
        Tone::Rest * 16,
        Drumlogue::OH,
        CH,
        Tone::Rest * 8,
        Drumlogue::OH,
        Tone::Rest * 3,
        Drumlogue::OH,
        Tone::Rest * 1
    ].midibox()
}

fn kick_only() -> Box<dyn Midibox> {
    seq![
        BD,
        Rest,
        Rest,
        Rest
    ].midibox()
}

fn drum_fast() -> Box<dyn Midibox> {
    let seq = seq![
        BD,
        Rest,
        BD,
        Rest,
        RS,
        Rest,
        Rest,
        BD,
        BD,
        RS,
        Rest,
        BD,
        RS,
        Rest,
        Rest,
        Rest,
        BD,
        Rest,
        Rest,
        BD,
        RS,
        Rest,
        Rest,
        BD,
        Rest,
        RS,
        Rest,
        BD,
        RS,
        Rest,
        BD,
        Rest
    ].midibox();

    map_notes(
        seq,
        |m| {
            if m.tone == BD.midi().tone {
                return m.set_velocity(rand::thread_rng().gen_range(90..120))
            }
            if m.tone == RS.midi().tone {
                return m.set_velocity(rand::thread_rng().gen_range(90..120))
            }
            m
        }
    )
}

fn hats() -> Box<dyn Midibox> {
    let seq = seq![
        SP1,
        SP1,
        SP1,
        CH,
        SP1,
        CH,
        CH,
        SP1,
        SP1,
        SP1,
        CH,
        SP1,
        SP1,
        CH,
        CH,
        CH,
        SP1,
        CH,
        CH,
        SP1,
        SP1,
        CH,
        CH,
        SP1,
        CH,
        SP1,
        CH,
        SP1,
        SP1,
        CH,
        SP1,
        CH
    ].midibox();

    map_beat(
        seq,
        32 * 4,
        |m, b| {
            let boost1 = if b % 3 == 0 { 20 } else { 0 };
            let boost2 = if b % 4 == 0 { 10 } else { 0 };
            let r1 = (50 + boost1 + boost2)..(70 + boost1 + boost2);
            let r2 = (10 + boost1 + boost2)..(20 + boost1 + boost2);

            let ch;
            let sp1;
            // on the fourth repeat, switch the
            if b > 32 * 3 {
                ch = r1;
                sp1 = r2;
            } else {
                ch = r2;
                sp1 = r1;
            }

            if m.tone == CH.midi().tone {
                return m.set_velocity(
                    rand::thread_rng().gen_range(ch).try_into().unwrap()
                )
            }
            if m.tone == SP1.midi().tone {
                return m.set_velocity(
                    rand::thread_rng().gen_range(sp1).try_into().unwrap()
                )
            }
            m
        }
    )
}