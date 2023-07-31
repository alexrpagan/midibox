use crate::chord::Chord;
use crate::midi::Midi;
use crate::Midibox;


/// Maps a function over individual note produced by a Midibox
pub struct Map<T>
    where T: Fn(Midi) -> Midi
{
    mapper: T,
    midibox: Box<dyn Midibox>,
}

impl<F> Map<F>
where F: Fn(Midi) -> Midi + 'static
{
    pub fn wrap(midibox: Box<dyn Midibox>, mapper: F) -> Box<dyn Midibox> {
        Box::new(Map { mapper, midibox })
    }
}

impl <F> Midibox for Map<F>
where F: Fn(Midi) -> Midi {
    fn next(&mut self) -> Option<Vec<Midi>> {
        self.midibox.next()
            .map(|it|
                it.into_iter().map(|note| (self.mapper)(note)).collect::<Vec<Midi>>()
            )
    }
}

/// Maps a function over groups of simultaneous notes produced by a Midibox
pub struct MapChord<T>
    where T: Fn(Chord) -> Chord
{
    mapper: T,
    midibox: Box<dyn Midibox>,
}

impl<F> MapChord<F>
where F: Fn(Chord) -> Chord + 'static {
    pub fn wrap(midibox: Box<dyn Midibox>, mapper: F) -> Box<dyn Midibox> {
        Box::new(MapChord { mapper, midibox })
    }
}

impl <F> Midibox for MapChord<F>
where F: Fn(Chord) -> Chord {
    fn next(&mut self) -> Option<Vec<Midi>> {
        self.midibox.next().map(|it| (self.mapper)(Chord::new(it)).notes)
    }
}