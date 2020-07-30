use std::env;
pub mod lib;
use lib::MIDILike;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mlo = MIDILike::from_path(args.get(1).unwrap().to_string());
    for i in 0 .. mlo.get_track_count() {
        println!("{} {}", i, mlo.get_active_tick_count(i));
    }
}
