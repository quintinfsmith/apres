use std::ffi::CStr;
use std::os::raw::c_char;
use std::fs::File;
use std::io::prelude::*;
use std::cmp::{max, min};
use std::fmt;

use std::mem;

use std::collections::{HashMap, HashSet};

use apres::*;

#[no_mangle]
pub extern fn save(midi_ptr: *mut MIDI, path: *const c_char) {
    let mut midi = unsafe { Box::from_raw(midi_ptr) };

    let cstr_path = unsafe {
        CStr::from_ptr(path)
    };

    let clean_path = cstr_path.to_str().expect("Not a valid UTF-8 string");
    midi.save(clean_path.to_string());

    Box::into_raw(midi);
}

#[no_mangle]
pub extern fn interpret(path: *const c_char) -> *mut MIDI {
    let cstr_path = unsafe {
        CStr::from_ptr(path)
    };

    let clean_path = cstr_path.to_str().expect("Not a valid UTF-8 string");
    let midi = MIDI::from_path(clean_path);
    Box::into_raw(Box::new( midi ))
}

#[no_mangle]
pub extern fn new() -> *mut MIDI {
    let midi = MIDI::new();
    Box::into_raw(Box::new( midi ))
}

#[no_mangle]
pub extern fn get_ppqn(midi_ptr: *mut MIDI) -> u16 {
    let mut midi = unsafe { Box::from_raw(midi_ptr) };

    let output = midi.get_ppqn();

    Box::into_raw(midi);

    output

}
#[no_mangle]
pub extern fn set_ppqn(midi_ptr: *mut MIDI, ppqn: u16) {
    let mut midi = unsafe { Box::from_raw(midi_ptr) };

    midi.set_ppqn(ppqn);

    Box::into_raw(midi);
}

// NOTE: all tracks & ticks (not event ids) passed FROM here are + 1, 0 is used to indicate a failure
#[no_mangle]
pub extern fn get_event_track(midi_ptr: *mut MIDI, event_id: u64) -> u8 {
    let mut midi = unsafe { Box::from_raw(midi_ptr) };
    let output = match midi.get_event_position(event_id) {
        Some((track, _tick)) => {
            *track + 1
        }
        None => {
            0
        }
    };

    Box::into_raw(midi);

    output as u8
}

#[no_mangle]
pub extern fn get_event_tick(midi_ptr: *mut MIDI, event_id: u64) -> u64 {
    let mut midi = unsafe { Box::from_raw(midi_ptr) };
    let output = match midi.get_event_position(event_id) {
        Some((_track, tick)) => {
            *tick + 1
        }
        None => {
            0
        }
    };

    Box::into_raw(midi);

    output as u64
}

#[no_mangle]
pub extern fn get_track_length(midi_ptr: *mut MIDI, track: u8) -> usize {
    let mut midi = unsafe { Box::from_raw(midi_ptr) };
    let length = midi.get_track_length(track as usize);

    Box::into_raw(midi);

    length
}

#[no_mangle]
pub extern fn count_tracks(midi_ptr: *mut MIDI) -> usize {
    let mut midi = unsafe { Box::from_raw(midi_ptr) };
    let count = midi.count_tracks();
    Box::into_raw(midi);
    count
}

#[no_mangle]
pub extern fn count_events(midi_ptr: *mut MIDI) -> usize {
    let mut midi = unsafe { Box::from_raw(midi_ptr) };
    let count = midi.count_events();
    Box::into_raw(midi);
    count
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
pub extern fn create_event(midi_ptr: *mut MIDI, track: u8, tick: u64, bytes_ptr: *mut u8, byte_length: u8) -> u64 {
    let mut midi = unsafe { Box::from_raw(midi_ptr) };

    let mut sub_bytes: Vec<u8> = unsafe { Vec::from_raw_parts(bytes_ptr, byte_length as usize, byte_length as usize) };
    let mut lead_byte = match sub_bytes.first() {
        Some(b) => {
            *b
        }
        None => {
            0
        }
    };

    sub_bytes.remove(0);
    sub_bytes.reverse();
    let new_event_id = match midi.process_mtrk_event(lead_byte, &mut sub_bytes, &mut (tick as usize), track as usize, &mut 0x90) {
        Some(created_event) => {
            created_event
        }
        None => {
            0 // 0 is reserved to denote 'no event'
        }
    };


    Box::into_raw(midi);
    mem::forget(sub_bytes);

    new_event_id
}

#[no_mangle]
pub extern fn set_event_position(midi_ptr: *mut MIDI, event_id: u64, track: u8, tick: u64) {
    let mut midi = unsafe { Box::from_raw(midi_ptr) };
    midi.move_event(track as usize, tick as usize, event_id);
    Box::into_raw(midi);
}
