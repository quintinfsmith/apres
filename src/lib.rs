use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::fs::File;
use std::io::prelude::*;
use std::cmp::max;

use std::mem;

use std::collections::HashMap;
use std::slice;
pub mod midi;
use midi::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}


#[no_mangle]
pub extern fn interpret(path: *const c_char) -> *mut MIDI {
    let cstr_path = unsafe {
        CStr::from_ptr(path)
    };

    let clean_path = cstr_path.to_str().expect("Not a valid UTF-8 string");
    let midi = MIDI::from_path(clean_path.to_string());
    Box::into_raw(Box::new( midi ))
}

#[no_mangle]
pub extern fn new() -> *mut MIDI {
    let midi = MIDI::new();
    Box::into_raw(Box::new( midi ))
}

#[no_mangle]
pub extern fn get_track_length(midi_ptr: *mut MIDI, track: usize) -> usize {
    let mut midi = unsafe { Box::from_raw(midi_ptr) };
    let length = midi.get_track_length(track);

    Box::into_raw(midi);

    length
}

#[no_mangle]
pub extern fn get_track_count(midi_ptr: *mut MIDI) -> usize {

    let mut midi = unsafe { Box::from_raw(midi_ptr) };

    let count = midi.get_track_count();

    Box::into_raw(midi);

    count
}

#[no_mangle]
pub extern fn get_tick_length(midi_ptr: *mut MIDI, track: usize, tick: usize) -> usize {
    let mut midi = unsafe { Box::from_raw(midi_ptr) };

    let length = midi.get_tick_length(track, tick);

    Box::into_raw(midi);

    length
}

#[no_mangle]
pub extern fn get_nth_event_in_tick(midi_ptr: *mut MIDI, track: usize, tick: usize, n: usize) -> u64 {
    let mut midi = unsafe { Box::from_raw(midi_ptr) };

    let event_id = midi.get_nth_event_id_in_tick(track, tick, n).expect("Event Doesn't Exist");


    Box::into_raw(midi);

    event_id
}

#[no_mangle]
pub extern fn set_event_property(midi_ptr: *mut MIDI, event_id: u64, argument: u8, value: *const c_char) {
    let mut midi = unsafe { Box::from_raw(midi_ptr) };

    let cstr_value = unsafe {
        CStr::from_ptr(value)
    };
    let value_vector = cstr_value.to_bytes().to_vec();

    match midi.get_event_mut(event_id) {
        Some(midievent) => {
            midievent.set_property(argument, value_vector.clone());
        }
        None => ()
    };

    Box::into_raw(midi);
}

#[no_mangle]
pub extern "C" fn get_event_property(midi_ptr: *mut MIDI, event_id: u64, argument: u8) -> *mut u8 {
    let midi = unsafe { Box::from_raw(midi_ptr) };
    let mut value = Vec::new();
    match midi.get_event(event_id) {
        Some(midievent) => {
            value = midievent.get_property(argument).clone();
        }
        None => ()
    };

    Box::into_raw(midi);


    let mut boxed_slice: Box<[u8]> = value.into_boxed_slice();

    let array: *mut u8 = boxed_slice.as_mut_ptr();
    // Prevent the slice from being destroyed (Leak the memory).
    mem::forget(boxed_slice);

    array
}

#[no_mangle]
pub extern fn get_event_property_length(midi_ptr: *mut MIDI, event_id: u64, argument: u8) -> u8 {
    let midi = unsafe { Box::from_raw(midi_ptr) };
    let mut value = Vec::new();
    match midi.get_event(event_id) {
        Some(midievent) => {
            value = midievent.get_property(argument).clone();
        }
        None => ()
    };

    Box::into_raw(midi);
    value.len() as u8
}

#[no_mangle]
pub extern fn get_event_type(midi_ptr: *mut MIDI, event_id: u64) -> u8 {
    let midi = unsafe { Box::from_raw(midi_ptr) };

    let mut output = 0;
    match midi.get_event(event_id) {
        Some(midievent) => {
            output = midievent.get_type() as u8;
        }
        None => ()
    };

    Box::into_raw(midi);

    output
}

#[no_mangle]
pub extern fn get_active_tick_count(midi_ptr: *mut MIDI, track: usize) -> usize {
    let midi = unsafe { Box::from_raw(midi_ptr) };
    let output = match midi.get_track(track) {
        Some(miditrack) => {
            miditrack.get_active_tick_count()
        }
        None => {
            0
        }
    };

    Box::into_raw(midi);

    output
}

#[no_mangle]
pub extern fn get_active_tick(midi_ptr: *mut MIDI, track: usize, n: usize) -> usize {
    let midi = unsafe { Box::from_raw(midi_ptr) };

    let output = match midi.get_track(track) {
        Some(miditrack) => {
            miditrack.get_active_tick(n)
        }
        None => {
            0
        }
    };

    Box::into_raw(midi);

    output
}
