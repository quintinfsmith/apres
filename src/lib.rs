use std::ffi::CStr;
use std::os::raw::c_char;
use std::fs::File;
use std::io::prelude::*;
use std::cmp::{max, min};
use std::fmt;

use std::mem;
use std::collections::{HashMap, HashSet};

use apres::*;
use apres::MIDIEvent::*;

#[no_mangle]
pub extern fn save(midi_ptr: *mut MIDI, path: *const c_char) {
    let mut midi = unsafe { Box::from_raw(midi_ptr) };

    let cstr_path = unsafe {
        CStr::from_ptr(path)
    };

    let clean_path = cstr_path.to_str().expect("Not a valid UTF-8 string");
    midi.save(clean_path);

    Box::into_raw(midi);
}

#[no_mangle]
pub extern fn interpret(path: *const c_char) -> *mut MIDI {
    let cstr_path = unsafe {
        CStr::from_ptr(path)
    };

    let clean_path = cstr_path.to_str().expect("Not a valid UTF-8 string");
    let midi = match MIDI::from_path(clean_path) {
        Ok(_midi) => {
            _midi
        }
        Err(_) => {
            // This is a bit of a work around. setting the ppqn will be interpreted as a bad midi.
            // It's kinda shit, I know.
            let mut _midi = MIDI::new();
            _midi.set_ppqn(0);
            _midi
        }
    };
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
pub extern fn get_track_length(midi_ptr: *mut MIDI, track: u8) -> u32 {
    let mut midi = unsafe { Box::from_raw(midi_ptr) };
    let length = midi.get_track_length(track as usize) as u32;

    Box::into_raw(midi);

    length
}

#[no_mangle]
pub extern fn count_tracks(midi_ptr: *mut MIDI) -> u32 {
    let mut midi = unsafe { Box::from_raw(midi_ptr) };
    let count = midi.count_tracks() as u32;
    Box::into_raw(midi);
    count
}

#[no_mangle]
pub extern fn count_events(midi_ptr: *mut MIDI) -> u32 {
    let mut midi = unsafe { Box::from_raw(midi_ptr) };
    let count = midi.count_events() as u32;
    Box::into_raw(midi);
    count
}


#[no_mangle]
pub extern fn replace_event(midi_ptr: *mut MIDI, event_id: u64, bytes_ptr: *mut u8, byte_length: u8) {

    let mut midi = unsafe { Box::from_raw(midi_ptr) };

    let mut sub_bytes: Vec<u8> = unsafe {
        Vec::from_raw_parts(bytes_ptr, byte_length as usize, byte_length as usize)
    };

    match MIDIEvent::from_bytes(&mut sub_bytes, 0) {

        Ok(new_midi_event) => {
            midi.replace_event(event_id, new_midi_event);
        }
        Err(_) => ()
    }

    Box::into_raw(midi);
    mem::forget(sub_bytes);
}


#[no_mangle]
pub extern "C" fn get_event_property(midi_ptr: *mut MIDI, event_id: u64, argument: u8) -> *mut u8 {
    let midi = unsafe { Box::from_raw(midi_ptr) };
    let mut value = Vec::new();
    match midi.get_event(event_id) {
        Some(midievent) => {
            value = get_midi_property(midievent, argument);
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
            value = get_midi_property(midievent, argument);
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
            output = get_midi_type_code(midievent);
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


    let new_event_id = match MIDIEvent::from_bytes(&mut sub_bytes, 0) {
        Ok(new_event) => {
            midi.insert_event(track as usize, tick as usize, new_event)
        }
        Err(_e) => {
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


fn get_midi_type_code(midievent: MIDIEvent) -> u8 {
    match midievent {
        SequenceNumber(_) => 22,
        Text(_) => 1,
        CopyRightNotice(_) => 2,
        TrackName(_) => 3,
        InstrumentName(_) => 4,
        Lyric(_) => 5,
        Marker(_) => 6,
        CuePoint(_) => 7,
        ChannelPrefix(_) => 9,
        SetTempo(_) => 10,
        SMPTEOffset(_, _, _, _, _) => 11,
        TimeSignature(_, _, _, _) => 12,
        KeySignature(_) => 13,
        //SequencerSpecific => 14,

        NoteOn(_, _, _) => 15,
        NoteOff(_, _, _) => 16,
        AfterTouch(_, _, _) => 17,
        ControlChange(_, _, _) => 18,
        ProgramChange(_, _) => 19,
        ChannelPressure(_, _) => 20,
        PitchWheelChange(_, _) => 21,

        SystemExclusive(_) => 23,
        MTCQuarterFrame(_, _) => 24,
        SongPositionPointer(_) => 25,
        SongSelect(_) => 26,

        TuneRequest => 27,
        MIDIClock => 28,
        MIDIStart => 29,
        MIDIContinue => 30,
        MIDIStop => 31,
        ActiveSense => 32,
        Reset => 33,
        EndOfTrack => 8,

        BankSelect(_, _) => 34,
        ModulationWheel(_, _) => 35,
        BreathController(_, _) => 36,
        FootPedal(_, _) => 37,
        PortamentoTime(_, _) => 38,
        DataEntrySlider(_, _) => 39,
        Volume(_, _) => 40,
        Balance(_, _) => 41,
        Pan(_, _) => 42,
        Expression(_, _) => 43,
        EffectControl(_, _, _) => 44,
        Slider(_, _, _) => 45,
        HoldPedal(_, _) => 46,
        Portamento(_, _) => 47,
        Sustenuto(_, _) => 48,
        SoftPedal(_, _) => 49,
        Legato(_, _) => 50,
        Hold2Pedal(_, _) => 51,
        SoundVariation(_, _) => 52,
        SoundTimbre(_, _) => 53,
        SoundReleaseTime(_, _) => 54,
        SoundAttack(_, _) => 55,
        SoundBrightness(_, _) => 56,
        SoundControl(_, _, _) => 57,
        GeneralButtonOn(_, _) => 58,
        GeneralButtonOff(_, _) => 59,
        EffectsLevel(_, _) => 60,
        TremuloLevel(_, _) => 61,
        ChorusLevel(_, _) => 62,
        CelesteLevel(_, _) => 63,
        PhaserLevel(_, _) => 64,
        DataButtonIncrement(_) => 65,
        DataButtonDecrement(_) => 66,
        RegisteredParameterNumber(_, _) => 67,
        NonRegisteredParameterNumber(_, _) => 68,
        AllControllersOff(_) => 69,
        LocalKeyboardEnable(_) => 70,
        LocalKeyboardDisable(_) => 71,
        AllNotesOff(_) => 72,
        AllSoundOff(_) => 73,
        OmniOff(_) => 74,
        OmniOn(_) => 75,
        MonophonicOperation(_, _) => 76,
        PolyphonicOperation(_) => 77,
        _ => 0 // Should be Unreachable
    }
}

fn get_midi_property(midievent: MIDIEvent, property_index: u8) -> Vec<u8> {
    match midievent {
        SequenceNumber(sequence) => {
            vec![
                (sequence / 256) as u8,
                (sequence % 256) as u8
            ]
        }
        Text(text) => {
            text.as_bytes().to_vec()
        }

        CopyRightNotice(notice) => {
            notice.as_bytes().to_vec()
        }

        TrackName(name) => {
            name.as_bytes().to_vec()
        }

        InstrumentName(name) => {
            name.as_bytes().to_vec()
        }

        Lyric(lyric) => {
            lyric.as_bytes().to_vec()
        }

        Marker(text) => {
            text.as_bytes().to_vec()
        }

        CuePoint(text) => {
            text.as_bytes().to_vec()
        }

        ChannelPrefix(channel) => {
            vec![channel]
        }

        SetTempo(uspqn) => {
            vec![
                ((uspqn / 256u32.pow(2)) % 256) as u8,
                ((uspqn / 256u32.pow(1)) % 256) as u8,
                (uspqn % 256) as u8
            ]
        }

        SMPTEOffset(hour, minute, second, ff, fr) => {
            let output = match property_index {
                0 => {
                    hour
                }
                1 => {
                    minute
                }
                2 => {
                    second
                }
                3 => {
                    ff
                }
                4 => {
                    fr
                }
                _ => {
                    0
                }
            };

            vec![output]
        }

        TimeSignature(numerator, denominator, cpm, thirtysecondths_per_quarter) => {
            let output = match property_index {
                0 => {
                    numerator
                }
                1 => {
                    denominator
                }
                2 => {
                    cpm
                }
                3 => {
                    thirtysecondths_per_quarter
                }
                _ => {
                    0
                }
            };

            vec![output]
        }

        KeySignature(key) => {
           key.as_bytes().to_vec()
        }
        //SequencerSpecific => 14,

        NoteOn(channel, note, velocity) => {
            match property_index {
                0 => {
                    vec![channel]
                }
                1 => {
                    vec![note]
                }
                2 => {
                    vec![velocity]
                }
                _ => {
                    vec![]
                }
            }
        }

        NoteOff(channel, note, velocity) => {
            match property_index {
                0 => {
                    vec![channel]
                }
                1 => {
                    vec![note]
                }
                2 => {
                    vec![velocity]
                }
                _ => {
                    vec![]
                }
            }
        }

        AfterTouch(channel, note, pressure) => {
            match property_index {
                0 => {
                   vec![channel]
                }
                1 => {
                    vec![note]
                }
                2 => {
                    vec![pressure]
                }
                _ => {
                    vec![]
                }
            }
        }

        ControlChange(channel, controller, value) => {
            match property_index {
                0 => {
                   vec![channel]
                }
                1 => {
                    vec![controller]
                }
                2 => {
                    vec![value]
                }
                _ => {
                    vec![]
                }
            }
        }

        ProgramChange(channel, program) => {
            match property_index {
                0 => {
                    vec![channel]
                }
                1 => {
                    vec![program]
                }
                _ => {
                    vec![]
                }
            }
        }

        ChannelPressure(channel, pressure) => {
            match property_index {
                0 => {
                    vec![channel]
                }
                1 => {
                    vec![pressure]
                }
                _ => {
                    vec![]
                }
            }
        }

        PitchWheelChange(channel, value) => {
            match property_index {
                0 => {
                    vec![ channel ]
                }
                1 => {
                    let unsigned_value = get_pitchwheel_value(value);
                    vec![
                        (unsigned_value / 256) as u8,
                        (unsigned_value % 256) as u8
                    ]
                }
                _ => {
                    vec![]
                }
            }
        }

        SystemExclusive(data) => {
            data.clone()
        }

        MTCQuarterFrame(message_type, value) => {
            match property_index {
                0 => {
                   vec![ message_type ]
                }
                1 => {
                   vec![ value ]
                }
                _ => {
                    vec![]
                }
            }
        }

        SongPositionPointer(beat) => {
            vec![
                (beat / 256) as u8,
                (beat % 256) as u8
            ]
        }

        SongSelect(song) => {
            vec![
                song & 0x7F
            ]
        }

        BankSelect(channel, value) => {
            match property_index {
                0 => {
                    vec![channel]
                }
                1 => {
                    vec![
                        (value >> 8) as u8,
                        (value & 0xFF) as u8
                    ]
                }
                _ => {
                    vec![]
                }
            }
        }

        ModulationWheel(channel, value) => {
            match property_index {
                0 => {
                    vec![channel]
                }
                1 => {
                    vec![
                        (value >> 8) as u8,
                        (value & 0xFF) as u8
                    ]
                }
                _ => {
                    vec![]
                }
            }
        }
        BreathController(channel, value) => {
            match property_index {
                0 => {
                    vec![channel]
                }
                1 => {
                    vec![
                        (value >> 8) as u8,
                        (value & 0xFF) as u8
                    ]
                }
                _ => {
                    vec![]
                }
            }
        }
        FootPedal(channel, value) => {
            match property_index {
                0 => {
                    vec![channel]
                }
                1 => {
                    vec![
                        (value >> 8) as u8,
                        (value & 0xFF) as u8
                    ]
                }
                _ => {
                    vec![]
                }
            }
        }
        PortamentoTime(channel, value) => {
            match property_index {
                0 => {
                    vec![channel]
                }
                1 => {
                    vec![
                        (value >> 8) as u8,
                        (value & 0xFF) as u8
                    ]
                }
                _ => {
                    vec![]
                }
            }
        }
        DataEntrySlider(channel, value) => {
            match property_index {
                0 => {
                    vec![channel]
                }
                1 => {
                    vec![
                        (value >> 8) as u8,
                        (value & 0xFF) as u8
                    ]
                }
                _ => {
                    vec![]
                }
            }
        }
        Volume(channel, value) => {
            match property_index {
                0 => {
                    vec![channel]
                }
                1 => {
                    vec![
                        (value >> 8) as u8,
                        (value & 0xFF) as u8
                    ]
                }
                _ => {
                    vec![]
                }
            }
        }
        Balance(channel, value) => {
            match property_index {
                0 => {
                    vec![channel]
                }
                1 => {
                    vec![
                        (value >> 8) as u8,
                        (value & 0xFF) as u8
                    ]
                }
                _ => {
                    vec![]
                }
            }
        }
        Pan(channel, value) => {
            match property_index {
                0 => {
                    vec![channel]
                }
                1 => {
                    vec![
                        (value >> 8) as u8,
                        (value & 0xFF) as u8
                    ]
                }
                _ => {
                    vec![]
                }
            }
        }
        Expression(channel, value) => {
            match property_index {
                0 => {
                    vec![channel]
                }
                1 => {
                    vec![
                        (value >> 8) as u8,
                        (value & 0xFF) as u8
                    ]
                }
                _ => {
                    vec![]
                }
            }
        }
        EffectControl(channel, which, value) => {
            match property_index {
                0 => {
                    vec![channel]
                }
                1 => {
                    vec![which]
                }
                2 => {
                    vec![
                        (value >> 8) as u8,
                        (value & 0xFF) as u8
                    ]
                }
                _ => {
                    vec![]
                }
            }
        }
        Slider(channel, which, value) => {
            match property_index {
                0 => {
                    vec![channel]
                }
                1 => {
                    vec![which]
                }
                2 => {
                    vec![value]
                }
                _ => {
                    vec![]
                }
            }
        }
        HoldPedal(channel, value) => {
            match property_index {
                0 => {
                    vec![channel]
                }
                1 => {
                    vec![value]
                }
                _ => {
                    vec![]
                }
            }
        }
        Portamento(channel, value) => {
            match property_index {
                0 => {
                    vec![channel]
                }
                1 => {
                    vec![value]
                }
                _ => {
                    vec![]
                }
            }
        }
        Sustenuto(channel, value) => {
            match property_index {
                0 => {
                    vec![channel]
                }
                1 => {
                    vec![value]
                }
                _ => {
                    vec![]
                }
            }
        }
        SoftPedal(channel, value) => {
            match property_index {
                0 => {
                    vec![channel]
                }
                1 => {
                    vec![value]
                }
                _ => {
                    vec![]
                }
            }
        }
        Legato(channel, value) => {
            match property_index {
                0 => {
                    vec![channel]
                }
                1 => {
                    vec![value]
                }
                _ => {
                    vec![]
                }
            }
        }
        Hold2Pedal(channel, value) => {
            match property_index {
                0 => {
                    vec![channel]
                }
                1 => {
                    vec![value]
                }
                _ => {
                    vec![]
                }
            }
        }
        SoundVariation(channel, value) => {
            match property_index {
                0 => {
                    vec![channel]
                }
                1 => {
                    vec![value]
                }
                _ => {
                    vec![]
                }
            }
        }
        SoundTimbre(channel, value) => {
            match property_index {
                0 => {
                    vec![channel]
                }
                1 => {
                    vec![value]
                }
                _ => {
                    vec![]
                }
            }
        }
        SoundReleaseTime(channel, value) => {
            match property_index {
                0 => {
                    vec![channel]
                }
                1 => {
                    vec![value]
                }
                _ => {
                    vec![]
                }
            }
        }
        SoundAttack(channel, value) => {
            match property_index {
                0 => {
                    vec![channel]
                }
                1 => {
                    vec![value]
                }
                _ => {
                    vec![]
                }
            }
        }
        SoundBrightness(channel, value) => {
            match property_index {
                0 => {
                    vec![channel]
                }
                1 => {
                    vec![value]
                }
                _ => {
                    vec![]
                }
            }
        }
        SoundControl(channel, which, value) => {
            match property_index {
                0 => {
                    vec![channel]
                }
                1 => {
                    vec![which]
                }
                2 => {
                    vec![value]
                }
                _ => {
                    vec![]
                }
            }
        }

        GeneralButtonOn(channel, which) => {
            match property_index {
                0 => {
                    vec![channel]
                }
                1 => {
                    vec![which]
                }
                _ => {
                    vec![]
                }
            }
        }

        GeneralButtonOff(channel, which) => {
            match property_index {
                0 => {
                    vec![channel]
                }
                1 => {
                    vec![which]
                }
                _ => {
                    vec![]
                }
            }
        }

        EffectsLevel(channel, value) => {
            match property_index {
                0 => {
                    vec![channel]
                }
                1 => {
                    vec![value]
                }
                _ => {
                    vec![]
                }
            }
        }

        TremuloLevel(channel, value) => {
            match property_index {
                0 => {
                    vec![channel]
                }
                1 => {
                    vec![value]
                }
                _ => {
                    vec![]
                }
            }
        }

        ChorusLevel(channel, value) => {
            match property_index {
                0 => {
                    vec![channel]
                }
                1 => {
                    vec![value]
                }
                _ => {
                    vec![]
                }
            }
        }

        CelesteLevel(channel, value) => {
            match property_index {
                0 => {
                    vec![channel]
                }
                1 => {
                    vec![value]
                }
                _ => {
                    vec![]
                }
            }
        }

        PhaserLevel(channel, value) => {
            match property_index {
                0 => {
                    vec![channel]
                }
                1 => {
                    vec![value]
                }
                _ => {
                    vec![]
                }
            }
        }

        DataButtonIncrement(channel) => {
            match property_index {
                0 => {
                    vec![channel]
                }
                _ => {
                    vec![]
                }
            }
        }

        DataButtonDecrement(channel) => {
            match property_index {
                0 => {
                    vec![channel]
                }
                _ => {
                    vec![]
                }
            }
        }

        RegisteredParameterNumber(channel, value) => {
            match property_index {
                0 => {
                    vec![channel]
                }
                1 => {
                    vec![
                        (value >> 8) as u8,
                        (value & 0xFF) as u8
                    ]
                }
                _ => {
                    vec![]
                }
            }
        }

        NonRegisteredParameterNumber(channel, value) => {
            match property_index {
                0 => {
                    vec![channel]
                }
                1 => {
                    vec![
                        (value >> 8) as u8,
                        (value & 0xFF) as u8
                    ]
                }
                _ => {
                    vec![]
                }
            }
        }

        AllControllersOff(channel) => {
            vec![channel]
        }

        LocalKeyboardEnable(channel) => {
            vec![channel]
        }

        LocalKeyboardDisable(channel) => {
            vec![channel]
        }

        AllNotesOff(channel) => {
            vec![channel]
        }

        AllSoundOff(channel) => {
            vec![channel]
        }

        OmniOff(channel) => {
            vec![channel]
        }

        OmniOn(channel) => {
            vec![channel]
        }

        MonophonicOperation(channel, value) => {
            match property_index {
                0 => {
                    vec![channel]
                }
                1 => {
                    vec![value]
                }
                _ => {
                    vec![]
                }
            }
        }

        PolyphonicOperation(channel) => {
            vec![channel]
        }

        _ => {
            vec![]
        }
    }
}
