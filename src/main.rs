use std::env;
pub mod lib;
use lib::*;
use lib::midi::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut mlo = MIDI::new();
    mlo.push_event(0, 0, NoteOnEvent::new(0, 64, 100));
    mlo.push_event(0, 120, NoteOffEvent::new(0, 64, 100));

    mlo.push_event(0, 0, NoteOnEvent::new(0, 64, 100));
    mlo.push_event(0, 120, NoteOffEvent::new(0, 64, 100));

    mlo.push_event(0, 0, NoteOnEvent::new(0, 64, 100));
    mlo.push_event(0, 120, NoteOffEvent::new(0, 64, 100));

    mlo.push_event(0, 0, NoteOnEvent::new(0, 52, 100));
    mlo.push_event(0, 120, NoteOffEvent::new(0, 52, 100));
    mlo.save("/home/pent/test.mid".to_string());

    //let mut mlo = MIDI::from_path(args.get(1).unwrap().to_string());
    //mlo.save(args.get(2).unwrap().to_string());
}
