use std::f64::consts::PI;
use std::thread;
use std::time::Duration;
use apres::{self, controller, MIDIEvent};
fn callback(c: &mut controller::Controller, t: &mut u8, event: &MIDIEvent) {
    println!("!");
}

fn main() {
    let mc = apres::listen(0, 0, &mut 1, callback);
}
