use std::ffi::CStr;
use std::os::raw::c_char;
use std::mem;
use std::sync::{Mutex, Arc};
use std::{thread};

use apres::*;
use apres::MIDIEvent::*;
use apres::controller::Controller;

pub struct ControllerWrapper {
    controller: Arc<Mutex<Controller>>,
    byte_cache: Arc<Mutex<Vec<u8>>>
}


#[no_mangle]
pub extern fn save(midi_ptr: *mut MIDI, path: *const c_char) {
    let midi = unsafe { mem::ManuallyDrop::new(Box::from_raw(midi_ptr)) };

    let cstr_path = unsafe {
        CStr::from_ptr(path)
    };

    let clean_path = cstr_path.to_str().expect("Not a valid UTF-8 string");
    midi.save(clean_path);
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
    let midi = unsafe { mem::ManuallyDrop::new(Box::from_raw(midi_ptr)) };

    let output = midi.get_ppqn();


    output
}

#[no_mangle]
pub extern fn set_format(midi_ptr: *mut MIDI, format: u16) {
    let mut midi = unsafe { mem::ManuallyDrop::new(Box::from_raw(midi_ptr)) };

    midi.set_format(format);

}

#[no_mangle]
pub extern fn set_ppqn(midi_ptr: *mut MIDI, ppqn: u16) {
    let mut midi = unsafe { mem::ManuallyDrop::new(Box::from_raw(midi_ptr)) };

    midi.set_ppqn(ppqn);

}

// NOTE: all tracks & ticks (not event ids) passed FROM here are + 1, 0 is used to indicate a failure
#[no_mangle]
pub extern fn get_event_track(midi_ptr: *mut MIDI, event_id: u64) -> u8 {
    let midi = unsafe { mem::ManuallyDrop::new(Box::from_raw(midi_ptr)) };
    let output = match midi.get_event_position(event_id) {
        Some((track, _tick)) => {
            *track + 1
        }
        None => {
            0
        }
    };

    output as u8
}

#[no_mangle]
pub extern fn get_event_tick(midi_ptr: *mut MIDI, event_id: u64) -> u64 {
    let midi = unsafe { mem::ManuallyDrop::new(Box::from_raw(midi_ptr)) };
    let output = match midi.get_event_position(event_id) {
        Some((_track, tick)) => {
            *tick + 1
        }
        None => {
            0
        }
    };

    output as u64
}

#[no_mangle]
pub extern fn get_track_length(midi_ptr: *mut MIDI, track: u8) -> u32 {
    let midi = unsafe { mem::ManuallyDrop::new(Box::from_raw(midi_ptr)) };
    let length = midi.get_track_length(track as usize) as u32;

    length
}

#[no_mangle]
pub extern fn count_tracks(midi_ptr: *mut MIDI) -> u32 {
    let midi = unsafe { mem::ManuallyDrop::new(Box::from_raw(midi_ptr)) };
    let count = midi.count_tracks() as u32;

    count
}

#[no_mangle]
pub extern fn count_events(midi_ptr: *mut MIDI) -> u32 {
    let midi = unsafe { mem::ManuallyDrop::new(Box::from_raw(midi_ptr)) };
    let count = midi.count_events() as u32;

    count
}


#[no_mangle]
pub extern fn replace_event(midi_ptr: *mut MIDI, event_id: u64, bytes_ptr: *mut u8, byte_length: u8) {

    let mut midi = unsafe { mem::ManuallyDrop::new(Box::from_raw(midi_ptr)) };

    let mut sub_bytes: Vec<u8> = unsafe {
        Vec::from_raw_parts(bytes_ptr, byte_length as usize, byte_length as usize)
    };

    match MIDIEvent::from_bytes(&mut sub_bytes, 0) {
        Ok(new_midi_event) => {
            midi.replace_event(event_id, new_midi_event);
        }
        Err(_) => ()
    }

    mem::forget(sub_bytes);
}


#[no_mangle]
pub extern "C" fn get_event_property(midi_ptr: *mut MIDI, event_id: u64, argument: u8) -> *mut u8 {
    let midi = unsafe { mem::ManuallyDrop::new(Box::from_raw(midi_ptr)) };
    let mut value = Vec::new();
    match midi.get_event(event_id) {
        Some(midievent) => {
            value = get_midi_property(midievent, argument);
        }
        None => ()
    };

    let mut boxed_slice: Box<[u8]> = value.into_boxed_slice();

    let array: *mut u8 = boxed_slice.as_mut_ptr();
    // Prevent the slice from being destroyed (Leak the memory).
    mem::forget(boxed_slice);

    array
}

#[no_mangle]
pub extern fn get_event_property_count(midi_ptr: *mut MIDI, event_id: u64) -> u8 {
    let midi = unsafe { mem::ManuallyDrop::new(Box::from_raw(midi_ptr)) };
    match midi.get_event(event_id) {
        Some(midievent) => {
            get_midi_property_count(midievent)
        }
        None => {
            0
        }
    }
}

#[no_mangle]
pub extern fn get_event_property_length(midi_ptr: *mut MIDI, event_id: u64, argument: u8) -> u8 {
    let midi = unsafe { mem::ManuallyDrop::new(Box::from_raw(midi_ptr)) };
    let mut value = Vec::new();
    match midi.get_event(event_id) {
        Some(midievent) => {
            value = get_midi_property(midievent, argument);
        }
        None => ()
    };


    value.len() as u8
}

#[no_mangle]
pub extern fn get_event_type(midi_ptr: *mut MIDI, event_id: u64) -> u8 {
    let midi = unsafe { mem::ManuallyDrop::new(Box::from_raw(midi_ptr)) };

    let mut output = 0;
    match midi.get_event(event_id) {
        Some(midievent) => {
            output = get_midi_type_code(midievent);
        }
        None => ()
    };


    output
}

#[no_mangle]
pub extern fn create_event(midi_ptr: *mut MIDI, track: u8, tick: u64, bytes_ptr: *mut u8, byte_length: u8) -> u64 {
    let mut midi = unsafe { mem::ManuallyDrop::new(Box::from_raw(midi_ptr)) };

    let mut sub_bytes: Vec<u8> = unsafe { Vec::from_raw_parts(bytes_ptr, byte_length as usize, byte_length as usize) };


    let new_event_id = match MIDIEvent::from_bytes(&mut sub_bytes, 0) {
        Ok(new_event) => {
            midi.insert_event(track as usize, tick as usize, new_event).ok().unwrap()
        }
        Err(_e) => {
            0 // 0 is reserved to denote 'no event'
        }
    };

    mem::forget(sub_bytes);

    new_event_id
}

#[no_mangle]
pub extern fn set_event_position(midi_ptr: *mut MIDI, event_id: u64, track: u8, tick: u64) {
    let mut midi = unsafe { mem::ManuallyDrop::new(Box::from_raw(midi_ptr)) };
    midi.move_event(track as usize, tick as usize, event_id);
}

#[no_mangle]
pub extern fn device_exists(channel: u8, device_id:u8) -> u8 {
    match Controller::new(channel, device_id) {
        Ok(mut controller) => {
            controller.kill();
            1
        }
        Err(_e) => {
            0
        }
    }
}

#[no_mangle]
pub extern fn new_controller(channel: u8, device_id: u8) -> *mut ControllerWrapper {
    // TODO: device verification
    let controller = Controller::new(channel, device_id).ok().unwrap();

    Box::into_raw(
        Box::new(
            ControllerWrapper {
                controller: Arc::new(Mutex::new(controller)),
                byte_cache: Arc::new(Mutex::new(Vec::new()))
            }
        )
    )
}

#[no_mangle]
pub extern fn controller_listen(controller_ptr: *mut ControllerWrapper) {
    let cw = unsafe {
        mem::ManuallyDrop::new(Box::from_raw(controller_ptr))
    };

    let controller_arc = Arc::clone(&cw.controller);
    let cache_arc = Arc::clone(&cw.byte_cache);
    loop {
        match cw.controller.try_lock() {
            Ok(ref mut mutex) => {
                mutex.force_listening();
                break;
            }
            Err(_e) => { }
        }
    }

    thread::spawn(move || {
        let mut alive = true;
        while alive {
            let mut next_byte: Option<u8> = None;
            // Get the neext byte
            loop {
                match controller_arc.try_lock() {
                    Ok(ref mut mutex) => {
                        if mutex.is_listening() {
                            next_byte = mutex.poll_next_byte();
                            break;
                        } else {
                            alive = false;
                            break;
                        }
                    }
                    Err(_e) => { }
                }
            }

            // If there is a next byte, cache it
            match next_byte {
                Some(byte) => {
                    loop {
                        match cache_arc.try_lock() {
                            Ok(ref mut mutex) => {
                                mutex.push(byte);
                                break;
                            }
                            Err(_e) => {}
                        }
                    }
                }
                None => { }
            }
        }
    });
}

#[no_mangle]
pub extern fn controller_poll_next_byte(controller_ptr: *mut ControllerWrapper) -> *mut u8 {
    let cw = unsafe {
        mem::ManuallyDrop::new(Box::from_raw(controller_ptr))
    };

    let cache_arc = Arc::clone(&cw.byte_cache);

    let mut byte_list = vec![];
    loop {
        match cache_arc.try_lock() {
            Ok(ref mut mutex) => {
                if mutex.len() > 0 {
                    byte_list.push(1);
                    byte_list.push(mutex[0]);
                    mutex.drain(0..1);
                } else {
                    byte_list.push(0);
                    byte_list.push(0);
                }
                break;
            }
            Err(_e) => {}
        }
    }

    let mut boxed_slice: Box<[u8]> = byte_list.clone().into_boxed_slice();
    let output: *mut u8 = boxed_slice.as_mut_ptr();

    // Prevent the slice from being destroyed (Leak the memory).
    mem::forget(boxed_slice);

    output
}

#[no_mangle]
pub extern fn controller_kill(controller_ptr: *mut ControllerWrapper) {
    let cw = unsafe {
        mem::ManuallyDrop::new(Box::from_raw(controller_ptr))
    };

    loop {
        match cw.controller.try_lock() {
            Ok(ref mut mutex) => {
                mutex.kill();
                break;
            }
            Err(_e) => {
            }
        }
    }
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
        SequencerSpecific(_) => 14,

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
        BankSelectLSB(_, _) => 35,
        ModulationWheel(_, _) => 36,
        ModulationWheelLSB(_, _) => 37,
        BreathController(_, _) => 38,
        BreathControllerLSB(_, _) => 39,
        FootPedal(_, _) => 40,
        FootPedalLSB(_, _) => 41,
        PortamentoTime(_, _) => 42,
        PortamentoTimeLSB(_, _) => 43,
        DataEntry(_, _) => 44,
        DataEntryLSB(_, _) => 45,
        Volume(_, _) => 46,
        VolumeLSB(_, _) => 47,
        Balance(_, _) => 48,
        BalanceLSB(_, _) => 49,
        Pan(_, _) => 50,
        PanLSB(_, _) => 51,
        Expression(_, _) => 52,
        ExpressionLSB(_, _) => 53,
        EffectControl1(_, _) => 54,
        EffectControl1LSB(_, _) => 55,
        EffectControl2(_, _) => 56,
        EffectControl2LSB(_, _) => 57,
        HoldPedal(_, _) => 58,
        Portamento(_, _) => 59,
        Sustenuto(_, _) => 60,
        SoftPedal(_, _) => 61,
        Legato(_, _) => 62,
        Hold2Pedal(_, _) => 63,
        SoundVariation(_, _) => 64,
        SoundTimbre(_, _) => 65,
        SoundReleaseTime(_, _) => 66,
        SoundAttack(_, _) => 67,
        SoundBrightness(_, _) => 68,
        SoundControl1(_, _) => 69,
        SoundControl2(_, _) => 70,
        SoundControl3(_, _) => 71,
        SoundControl4(_, _) => 72,
        SoundControl5(_, _) => 73,
        GeneralPurpose1(_, _) => 74,
        GeneralPurpose1LSB(_, _) => 75,
        GeneralPurpose2(_, _) => 76,
        GeneralPurpose2LSB(_, _) => 77,
        GeneralPurpose3(_, _) => 78,
        GeneralPurpose3LSB(_, _) => 79,
        GeneralPurpose4(_, _) => 80,
        GeneralPurpose4LSB(_, _) => 81,
        GeneralPurpose5(_, _) => 82,
        GeneralPurpose6(_, _) => 83,
        GeneralPurpose7(_, _) => 84,
        GeneralPurpose8(_, _) => 85,
        EffectsLevel(_, _) => 86,
        TremuloLevel(_, _) => 87,
        ChorusLevel(_, _) => 88,
        CelesteLevel(_, _) => 89,
        PhaserLevel(_, _) => 90,
        DataIncrement(_) => 91,
        DataDecrement(_) => 92,
        RegisteredParameterNumber(_, _) => 93,
        RegisteredParameterNumberLSB(_, _) => 94,
        NonRegisteredParameterNumber(_, _) => 95,
        NonRegisteredParameterNumberLSB(_, _) => 96,
        AllControllersOff(_) => 97,
        LocalControl(_, _) => 98,
        AllNotesOff(_) => 99,
        AllSoundOff(_) => 100,
        OmniOff(_) => 101,
        OmniOn(_) => 102,
        MonophonicOperation(_, _) => 103,
        PolyphonicOperation(_) => 104,
        TimeCode(_, _, _, _, _) => 105
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

        Text(text) |
        CopyRightNotice(text) |
        TrackName(text) |
        InstrumentName(text) |
        Lyric(text) |
        Marker(text) |
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
                0 => { hour }
                1 => { minute }
                2 => { second }
                3 => { ff }
                4 => { fr }
                _ => { 0 }
            };

            vec![output]
        }

        TimeSignature(numerator, denominator, cpm, thirtysecondths_per_quarter) => {
            let output = match property_index {
                0 => { numerator }
                1 => { denominator }
                2 => { cpm }
                3 => { thirtysecondths_per_quarter }
                _ => { 0 }
            };

            vec![output]
        }


        NoteOn(channel, note, velocity) |
        NoteOff(channel, note, velocity) |
        AfterTouch(channel, note, velocity) => {
            match property_index {
                0 => { vec![channel] }
                1 => { vec![note] }
                2 => { vec![velocity] }
                _ => { vec![] }
            }
        }


        ControlChange(channel, controller, value) => {
            match property_index {
                0 => { vec![channel] }
                1 => { vec![controller] }
                2 => { vec![value] }
                _ => { vec![] }
            }
        }

        ProgramChange(channel, program) => {
            match property_index {
                0 => { vec![channel] }
                1 => { vec![program] }
                _ => { vec![] }
            }
        }

        ChannelPressure(channel, pressure) => {
            match property_index {
                0 => { vec![channel] }
                1 => { vec![pressure] }
                _ => { vec![] }
            }
        }

        PitchWheelChange(channel, value) => {
            match property_index {
                0 => { vec![ channel ] }
                1 => {
                    let unsigned_value = get_pitchwheel_value(value);
                    vec![
                        (unsigned_value / 256) as u8,
                        (unsigned_value % 256) as u8
                    ]
                }
                _ => { vec![] }
            }
        }

        SequencerSpecific(data) |
        SystemExclusive(data) => {
            data.clone()
        }

        MTCQuarterFrame(message_type, value) => {
            match property_index {
                0 => { vec![ message_type ] }
                1 => { vec![ value ] }
                _ => { vec![] }
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

        BankSelect(channel, value) |
        BankSelectLSB(channel, value) |
        ModulationWheel(channel, value) |
        ModulationWheelLSB(channel, value) |
        BreathController(channel, value) |
        BreathControllerLSB(channel, value) |
        FootPedal(channel, value) |
        FootPedalLSB(channel, value) |
        PortamentoTime(channel, value) |
        PortamentoTimeLSB(channel, value) |
        DataEntry(channel, value) |
        DataEntryLSB(channel, value) |
        Volume(channel, value) |
        VolumeLSB(channel, value) |
        Balance(channel, value) |
        BalanceLSB(channel, value) |
        Pan(channel, value) |
        PanLSB(channel, value) |
        Expression(channel, value) |
        ExpressionLSB(channel, value) |
        EffectControl1(channel, value) |
        EffectControl1LSB(channel, value) |
        EffectControl2(channel, value) |
        EffectControl2LSB(channel, value) |
        HoldPedal(channel, value) |
        Portamento(channel, value) |
        Sustenuto(channel, value) |
        SoftPedal(channel, value) |
        Legato(channel, value) |
        Hold2Pedal(channel, value) |
        SoundVariation(channel, value) |
        SoundTimbre(channel, value) |
        SoundReleaseTime(channel, value) |
        SoundAttack(channel, value) |
        SoundBrightness(channel, value) |
        SoundControl1(channel, value) |
        SoundControl2(channel, value) |
        SoundControl3(channel, value) |
        SoundControl4(channel, value) |
        SoundControl5(channel, value) |
        GeneralPurpose1(channel, value) |
        GeneralPurpose1LSB(channel, value) |
        GeneralPurpose2(channel, value) |
        GeneralPurpose2LSB(channel, value) |
        GeneralPurpose3(channel, value) |
        GeneralPurpose3LSB(channel, value) |
        GeneralPurpose4(channel, value) |
        GeneralPurpose4LSB(channel, value) |
        GeneralPurpose5(channel, value) |
        GeneralPurpose6(channel, value) |
        GeneralPurpose7(channel, value) |
        GeneralPurpose8(channel, value) |
        EffectsLevel(channel, value) |
        TremuloLevel(channel, value) |
        ChorusLevel(channel, value) |
        CelesteLevel(channel, value) |
        PhaserLevel(channel, value) |
        RegisteredParameterNumber(channel, value) |
        NonRegisteredParameterNumber(channel, value) |
        RegisteredParameterNumberLSB(channel, value) |
        NonRegisteredParameterNumberLSB(channel, value) |
        LocalControl(channel, value) |
        MonophonicOperation(channel, value) => {
            match property_index {
                0 => { vec![channel] }
                1 => { vec![value] }
                _ => { vec![] }
            }
        }

        DataIncrement(channel) |
        DataDecrement(channel) |
        AllControllersOff(channel) |
        AllNotesOff(channel) |
        AllSoundOff(channel) |
        OmniOff(channel) |
        OmniOn(channel) |
        PolyphonicOperation(channel) => {
            vec![channel]
        }

        TimeCode(rate, hour, minute, second, frame) => {
            match property_index {
                0 => {
                    let coded = if rate == 24.0 {
                        0
                    } else if rate == 25.0 {
                        1
                    } else if rate == 29.97 {
                        2
                    } else if rate == 30.0 {
                        3
                    } else {
                        3
                    };

                    vec![coded]
                }
                1 => { vec![hour] }
                2 => { vec![minute] }
                3 => { vec![second] }
                4 => { vec![frame] }
                _ => { vec![] }
            }
        }

        KeySignature(key) => {
            let (mi, sf) = get_mi_sf(&key);
            match property_index {
                0 => { vec![mi] }
                1 => { vec![sf] }
                _ => { vec![] }
            }
        }

        _ => {
            vec![]
        }
    }
}

fn get_midi_property_count(midievent: MIDIEvent) -> u8 {
    match midievent {
        SequenceNumber(_sequence) => {
            1
        }

        Text(_text) |
        CopyRightNotice(_text) |
        TrackName(_text) |
        InstrumentName(_text) |
        Lyric(_text) |
        Marker(_text) |
        CuePoint(_text) => {
            1
        }

        ChannelPrefix(_channel) => {
            1
        }

        SetTempo(_uspqn) => {
            1
        }

        SMPTEOffset(_hour, _minute, _second, _ff, _fr) => {
            5
        }

        TimeSignature(_numerator, _denominator, _cpm, _thirtysecondths_per_quarter) => {
            4
        }


        NoteOn(_channel, _note, _velocity) |
        NoteOff(_channel, _note, _velocity) |
        AfterTouch(_channel, _note, _velocity) => {
            3
        }


        ControlChange(_channel, _controller, _value) => {
            3
        }

        ProgramChange(_channel, _program) => {
            2
        }

        ChannelPressure(_channel, _pressure) => {
            2
        }

        PitchWheelChange(_channel, _value) => {
            2
        }

        SequencerSpecific(_data) |
        SystemExclusive(_data) => {
            1
        }

        MTCQuarterFrame(_message_type, _value) => {
            2
        }

        SongPositionPointer(_beat) => {
            1
        }

        SongSelect(_song) => {
            1
        }

        BankSelect(_channel, _value) |
        BankSelectLSB(_channel, _value) |
        ModulationWheel(_channel, _value) |
        ModulationWheelLSB(_channel, _value) |
        BreathController(_channel, _value) |
        BreathControllerLSB(_channel, _value) |
        FootPedal(_channel, _value) |
        FootPedalLSB(_channel, _value) |
        PortamentoTime(_channel, _value) |
        PortamentoTimeLSB(_channel, _value) |
        DataEntry(_channel, _value) |
        DataEntryLSB(_channel, _value) |
        Volume(_channel, _value) |
        VolumeLSB(_channel, _value) |
        Balance(_channel, _value) |
        BalanceLSB(_channel, _value) |
        Pan(_channel, _value) |
        PanLSB(_channel, _value) |
        Expression(_channel, _value) |
        ExpressionLSB(_channel, _value) |
        EffectControl1(_channel, _value) |
        EffectControl1LSB(_channel, _value) |
        EffectControl2(_channel, _value) |
        EffectControl2LSB(_channel, _value) |
        HoldPedal(_channel, _value) |
        Portamento(_channel, _value) |
        Sustenuto(_channel, _value) |
        SoftPedal(_channel, _value) |
        Legato(_channel, _value) |
        Hold2Pedal(_channel, _value) |
        SoundVariation(_channel, _value) |
        SoundTimbre(_channel, _value) |
        SoundReleaseTime(_channel, _value) |
        SoundAttack(_channel, _value) |
        SoundBrightness(_channel, _value) |
        SoundControl1(_channel, _value) |
        SoundControl2(_channel, _value) |
        SoundControl3(_channel, _value) |
        SoundControl4(_channel, _value) |
        SoundControl5(_channel, _value) |
        GeneralPurpose1(_channel, _value) |
        GeneralPurpose1LSB(_channel, _value) |
        GeneralPurpose2(_channel, _value) |
        GeneralPurpose2LSB(_channel, _value) |
        GeneralPurpose3(_channel, _value) |
        GeneralPurpose3LSB(_channel, _value) |
        GeneralPurpose4(_channel, _value) |
        GeneralPurpose4LSB(_channel, _value) |
        GeneralPurpose5(_channel, _value) |
        GeneralPurpose6(_channel, _value) |
        GeneralPurpose7(_channel, _value) |
        GeneralPurpose8(_channel, _value) |
        EffectsLevel(_channel, _value) |
        TremuloLevel(_channel, _value) |
        ChorusLevel(_channel, _value) |
        CelesteLevel(_channel, _value) |
        PhaserLevel(_channel, _value) |
        RegisteredParameterNumber(_channel, _value) |
        NonRegisteredParameterNumber(_channel, _value) |
        RegisteredParameterNumberLSB(_channel, _value) |
        NonRegisteredParameterNumberLSB(_channel, _value) |
        LocalControl(_channel, _value) |
        MonophonicOperation(_channel, _value) => {
            2
        }

        DataIncrement(_channel) |
        DataDecrement(_channel) |
        AllControllersOff(_channel) |
        AllNotesOff(_channel) |
        AllSoundOff(_channel) |
        OmniOff(_channel) |
        OmniOn(_channel) |
        PolyphonicOperation(_channel) => {
            1
        }

        TimeCode(_rate, _hour, _minute, _second, _frame) => {
            5
        }

        KeySignature(_key) => {
            2
        }

        _ => {
            0
        }
    }
}
