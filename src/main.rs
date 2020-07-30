use std::env;
pub mod lib;
use lib::MIDILike;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut mlo = MIDILike::from_path(args.get(1).unwrap().to_string());
    mlo.save(args.get(2).unwrap().to_string());
}
