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

pub trait External {
    //For FFI bindings
    fn set_property(&mut self, argument: u8, bytes: Vec<u8>);
    fn get_property(&self, argument: u8) -> Vec<u8>;
}

fn get_midi_type_code(midievent) -> u8 {
    match midievent {
        SequenceNumber(_) => 22,
        Text(_) => 1,
        CopyRightNotice(_) => 2,
        TrackName(_) => 3,
        InstrumentName(_) => 4,
        Lyric(_) => 5,
        Marker(_) => 6,
        CuePoint(_) => 7,
        EndOfTrack => 8,
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
        PolyphonicOperation(_) => 77
        _ => 0 // Should be Unreachable
    }
}


impl MIDIEvent for SequenceNumberEvent {
    fn set_property(&mut self, _argument: u8, bytes: Vec<u8>) {
        self.sequence = (bytes[1] as u16 * 256) + (bytes[0] as u16);
    }
    fn get_property(&self, _: u8) -> Vec<u8> {
        vec![
            (self.sequence / 256) as u8,
            (self.sequence % 256) as u8
        ]
    }
}

impl MIDIEvent for TextEvent {
    fn set_property(&mut self, _argument: u8, bytes: Vec<u8>) {
        self.text = std::str::from_utf8(bytes.as_slice()).unwrap().to_string();
    }
    fn get_property(&self, _: u8) -> Vec<u8> {
        self.text.as_bytes().to_vec()
    }
}

impl MIDIEvent for CopyRightNoticeEvent {
    fn set_property(&mut self, _argument: u8, bytes: Vec<u8>) {
        self.text = std::str::from_utf8(bytes.as_slice()).unwrap().to_string();
    }

    fn get_property(&self, _: u8) -> Vec<u8> {
        self.text.as_bytes().to_vec()
    }
}

impl MIDIEvent for TrackNameEvent {
    fn set_property(&mut self, _argument: u8, bytes: Vec<u8>) {
        self.track_name = std::str::from_utf8(bytes.as_slice()).unwrap().to_string();
    }
    fn get_property(&self, _: u8) -> Vec<u8> {
        self.track_name.as_bytes().to_vec()
    }
}

impl MIDIEvent for InstrumentNameEvent {
    fn set_property(&mut self, _argument: u8, bytes: Vec<u8>) {
        self.instrument_name = std::str::from_utf8(bytes.as_slice()).unwrap().to_string();
    }
    fn get_property(&self, _: u8) -> Vec<u8> {
        self.instrument_name.as_bytes().to_vec()
    }
}
impl MIDIEvent for LyricEvent {
    fn set_property(&mut self, _argument: u8, bytes: Vec<u8>) {
        self.lyric = std::str::from_utf8(bytes.as_slice()).unwrap().to_string();
    }
    fn get_property(&self, _: u8) -> Vec<u8> {
        self.lyric.as_bytes().to_vec()
    }
}
impl MIDIEvent for MarkerEvent {
    fn set_property(&mut self, _argument: u8, bytes: Vec<u8>) {
        self.text = std::str::from_utf8(bytes.as_slice()).unwrap().to_string();
    }
    fn get_property(&self, _: u8) -> Vec<u8> {
        self.text.as_bytes().to_vec()
    }
}
impl MIDIEvent for CuePointEvent {
    fn set_property(&mut self, _argument: u8, bytes: Vec<u8>) {
        self.text = std::str::from_utf8(bytes.as_slice()).unwrap().to_string();
    }
    fn get_property(&self, _: u8) -> Vec<u8> {
        self.text.as_bytes().to_vec()
    }
}

impl MIDIEvent for EndOfTrackEvent {
    fn set_property(&mut self, _argument: u8, _bytes: Vec<u8>) {
        // non-applicable
    }
    fn get_property(&self, _: u8) -> Vec<u8> {
        Vec::new()
    }
}

impl MIDIEvent for ChannelPrefixEvent {
    fn set_property(&mut self, _argument: u8, bytes: Vec<u8>) {
        self.channel = bytes[0];
    }
    fn get_property(&self, _: u8) -> Vec<u8> {
        vec![self.channel]
    }
}

impl SetTempoEvent {
    pub fn new(us_per_quarter_note: u32) -> SetTempoEvent {
        SetTempoEvent {
            us_per_quarter_note: min(us_per_quarter_note, 0x00FFFFFF)
        }
    }

    pub fn set_bpm(&mut self, bpm: f64) {
        // The minimum BPM is ~3.57(60000000 / u16::MAX)
        let adj_bpm = if 3.5762788 >= bpm {
            3.5762788
        } else {
            bpm
        };

        // It's ok to lose the precision here (f64 -> u32) because it's microseconds as percieved by a human
        self.us_per_quarter_note = if 60000000_f64 / adj_bpm < 0x00FFFFFF as f64 {
            (60000000_f64 / adj_bpm) as u32
        } else {
            0x00FFFFFF
        };
    }

    pub fn set_uspqn(&mut self, uspqn: u32) {
        self.us_per_quarter_note = min(uspqn, 0x00FFFFFF);
    }

    pub fn get_uspqn(&self) -> u32 {
        self.us_per_quarter_note
    }

    pub fn get_bpm(&self) -> f64 {
        60_000_000 as f64 / self.us_per_quarter_note as f64
    }
}

impl MIDIEvent for SetTempoEvent {
    fn set_property(&mut self, _argument: u8, bytes: Vec<u8>) {
        self.us_per_quarter_note = (bytes[2] as u32 * 256u32.pow(2)) + (bytes[1] as u32 * 256) + (bytes[0] as u32);
    }
    fn get_property(&self, _: u8) -> Vec<u8> {
        vec![
            ((self.us_per_quarter_note / 256u32.pow(2)) % 256) as u8,
            ((self.us_per_quarter_note / 256u32.pow(1)) % 256) as u8,
            (self.us_per_quarter_note % 256) as u8
        ]
    }
}
impl MIDIEvent for SMPTEOffsetEvent {
    fn set_property(&mut self, argument: u8, bytes: Vec<u8>) {
        match argument {
            0 => {
                self.hour = bytes[0];
            }
            1 => {
                self.minute = bytes[0];
            }
            2 => {
                self.second = bytes[0];
            }
            3 => {
                self.ff = bytes[0];
            }
            4 => {
                self.fr = bytes[0];
            }
            _ => ()
        };
    }
    fn get_property(&self, argument: u8) -> Vec<u8> {
        let output = match argument {
            0 => {
                self.hour
            }
            1 => {
                self.minute
            }
            2 => {
                self.second
            }
            3 => {
                self.ff
            }
            4 => {
                self.fr
            }
            _ => {
                0
            }
        };

        vec![output]
    }
}

impl MIDIEvent for TimeSignatureEvent {
    fn set_property(&mut self, argument: u8, bytes: Vec<u8>) {
        match argument {
            0 => {
                self.numerator = bytes[0];
            }
            1 => {
                self.denominator = bytes[0];
            }
            2 => {
                self.clocks_per_metronome = bytes[0];
            }
            3 => {
                self.thirtysecondths_per_quarter = bytes[0];
            }
            _ => ()
        };
    }
    fn get_property(&self, argument: u8) -> Vec<u8> {
        let output = match argument {
            0 => {
                self.numerator
            }
            1 => {
                self.denominator
            }
            2 => {
                self.clocks_per_metronome
            }
            3 => {
                self.thirtysecondths_per_quarter
            }
            _ => {
                0
            }
        };

        vec![output]
    }
}

impl MIDIEvent for KeySignatureEvent {
    fn set_property(&mut self, _: u8, bytes: Vec<u8>) {
        self.key = std::str::from_utf8(bytes.as_slice()).unwrap().to_string();
    }
    fn get_property(&self, _: u8) -> Vec<u8> {
       self.key.as_bytes().to_vec()
    }
}


// ChannelEvents /////////////////////////
impl MIDIEvent for NoteOnEvent {

    fn set_property(&mut self, argument: u8, bytes: Vec<u8>) {
        match argument {
            0 => {
                self.channel = bytes[0] & 0x0F;
            }
            1 => {
                self.note = bytes[0] & 0x7F;
            }
            2 => {
                self.velocity = bytes[0] & 0x7F;
            }
            _ => ()
        };
    }

    fn get_property(&self, argument: u8) -> Vec<u8> {
        let output = match argument {
            0 => {
                self.channel
            }
            1 => {
                self.note
            }
            2 => {
                self.velocity
            }
            _ => {
                0
            }
        };

        vec![output]
    }
}

impl MIDIEvent for NoteOffEvent {
    fn set_property(&mut self, argument: u8, bytes: Vec<u8>) {
        match argument {
            0 => {
                self.channel = bytes[0] & 0x0F;
            }
            1 => {
                self.note = bytes[0] & 0x7F;
            }
            2 => {
                self.velocity = bytes[0] & 0x7F;
            }
            _ => ()
        };
    }

    fn get_property(&self, argument: u8) -> Vec<u8> {
        let output = match argument {
            0 => {
                self.channel
            }
            1 => {
                self.note
            }
            2 => {
                self.velocity
            }
            _ => {
                0
            }
        };

        vec![output]
    }
}

impl MIDIEvent for AfterTouchEvent {
    fn set_property(&mut self, argument: u8, bytes: Vec<u8>) {
        match argument {
            0 => {
                self.channel = bytes[0] & 0x0F;
            }
            1 => {
                self.note = bytes[0] & 0x7F;
            }
            2 => {
                self.pressure = bytes[0] & 0x7F;
            }
            _ => ()
        };
    }
    fn get_property(&self, argument: u8) -> Vec<u8> {
        let output = match argument {
            0 => {
                self.channel
            }
            1 => {
                self.note
            }
            2 => {
                self.pressure
            }
            _ => {
                0
            }
        };

        vec![output]
    }
}



impl MIDIEvent for BankSelectEvent {


    fn get_property(&self, argument: u8) -> Vec<u8> {
        match argument {
            0 => {
                vec![self.channel]
            }
            _ => {
                vec![
                    (self.value >> 8) as u8,
                    (self.value & 0xFF) as u8
                ]
            }
        }
    }

    fn set_property(&mut self, argument: u8, bytes: Vec<u8>) {
        match argument {
            0 => {
                self.channel = bytes[0];
            }
            _ => {
                self.value = ((bytes[0] as u16) << 8) + (bytes[1] as u16);
            }
        }
    }
}


impl MIDIEvent for ModulationWheelEvent {

    fn get_property(&self, argument: u8) -> Vec<u8> {
        match argument {
            0 => {
                vec![self.channel]
            }
            _ => {
                vec![
                    (self.value >> 8) as u8,
                    (self.value & 0xFF) as u8
                ]
            }
        }
    }
    fn set_property(&mut self, argument: u8, bytes: Vec<u8>) {
        match argument {
            0 => {
                self.channel = bytes[0];
            }
            _ => {
                self.value = ((bytes[0] as u16) << 8) + (bytes[1] as u16);
            }
        }
    }
}

impl MIDIEvent for BreathControllerEvent {

    fn get_property(&self, argument: u8) -> Vec<u8> {
        match argument {
            0 => {
                vec![self.channel]
            }
            _ => {
                vec![
                    (self.value >> 8) as u8,
                    (self.value & 0xFF) as u8
                ]
            }
        }
    }
    fn set_property(&mut self, argument: u8, bytes: Vec<u8>) {
        match argument {
            0 => {
                self.channel = bytes[0];
            }
            _ => {
                self.value = ((bytes[0] as u16) << 8) + (bytes[1] as u16);
            }
        }
    }
}
impl MIDIEvent for FootPedalEvent {

    fn get_property(&self, argument: u8) -> Vec<u8> {
        match argument {
            0 => {
                vec![self.channel]
            }
            _ => {
                vec![
                    (self.value >> 8) as u8,
                    (self.value & 0xFF) as u8
                ]
            }
        }
    }
    fn set_property(&mut self, argument: u8, bytes: Vec<u8>) {
        match argument {
            0 => {
                self.channel = bytes[0];
            }
            _ => {
                self.value = ((bytes[0] as u16) << 8) + (bytes[1] as u16);
            }
        }
    }
}

impl MIDIEvent for PortamentoTimeEvent {
    fn get_property(&self, argument: u8) -> Vec<u8> {
        match argument {
            0 => {
                vec![self.channel]
            }
            _ => {
                vec![
                    (self.value >> 8) as u8,
                    (self.value & 0xFF) as u8
                ]
            }
        }
    }
    fn set_property(&mut self, argument: u8, bytes: Vec<u8>) {
        match argument {
            0 => {
                self.channel = bytes[0];
            }
            _ => {
                self.value = ((bytes[0] as u16) << 8) + (bytes[1] as u16);
            }
        }
    }
}

impl MIDIEvent for DataEntrySliderEvent {

    fn get_property(&self, argument: u8) -> Vec<u8> {
        match argument {
            0 => {
                vec![self.channel]
            }
            1 => {
                vec![
                    (self.value >> 8) as u8,
                    (self.value & 0xFF) as u8
                ]
            }
            _ => {
                // Sent as +1, 0 indicates None
                match self.target {
                    Some(target) => {
                        vec![
                            ((target + 1) >> 8) as u8,
                            ((target + 1) & 0xFF) as u8
                        ]
                    }
                    None => {
                        vec![0,0]
                    }
                }
            }
        }
    }

    fn set_property(&mut self, argument: u8, bytes: Vec<u8>) {
        match argument {
            0 => {
                self.channel = bytes[0]
            }
            1 => {
                self.value = ((bytes[0] as u16) << 8) + (bytes[1] as u16);
            }
            2 => {
                let tmp_target = ((bytes[0] as u16) << 8) + (bytes[1] as u16) - 1;
                if tmp_target > 0 {
                    self.target = Some(tmp_target);
                } else {
                    self.target = None;
                }
            },
            _ => {}
        }
    }
}


impl MIDIEvent for VolumeEvent {

    fn get_property(&self, argument: u8) -> Vec<u8> {
        match argument {
            0 => {
                vec![self.channel]
            }
            _ => {
                vec![
                    (self.value >> 8) as u8,
                    (self.value & 0xFF) as u8
                ]
            }
        }
    }
    fn set_property(&mut self, argument: u8, bytes: Vec<u8>) {
        match argument {
            0 => {
                self.channel = bytes[0];
            }
            _ => {
                self.value = ((bytes[0] as u16) << 8) + (bytes[1] as u16);
            }
        }
    }
}
impl MIDIEvent for BalanceEvent {

    fn get_property(&self, argument: u8) -> Vec<u8> {
        match argument {
            0 => {
                vec![self.channel]
            }
            _ => {
                vec![
                    (self.value >> 8) as u8,
                    (self.value & 0xFF) as u8
                ]
            }
        }
    }
    fn set_property(&mut self, argument: u8, bytes: Vec<u8>) {
        match argument {
            0 => {
                self.channel = bytes[0];
            }
            _ => {
                self.value = ((bytes[0] as u16) << 8) + (bytes[1] as u16);
            }
        }
    }
}


impl MIDIEvent for PanEvent {

    fn get_property(&self, argument: u8) -> Vec<u8> {
        match argument {
            0 => {
                vec![self.channel]
            }
            _ => {
                vec![
                    (self.value >> 8) as u8,
                    (self.value & 0xFF) as u8
                ]
            }
        }
    }
    fn set_property(&mut self, argument: u8, bytes: Vec<u8>) {
        match argument {
            0 => {
                self.channel = bytes[0];
            }
            _ => {
                self.value = ((bytes[0] as u16) << 8) + (bytes[1] as u16);
            }
        }
    }
}
impl MIDIEvent for ExpressionEvent {

    fn get_property(&self, argument: u8) -> Vec<u8> {
        match argument {
            0 => {
                vec![self.channel]
            }
            _ => {
                vec![
                    (self.value >> 8) as u8,
                    (self.value & 0xFF) as u8
                ]
            }
        }
    }
    fn set_property(&mut self, argument: u8, bytes: Vec<u8>) {
        match argument {
            0 => {
                self.channel = bytes[0];
            }
            _ => {
                self.value = ((bytes[0] as u16) << 8) + (bytes[1] as u16);
            }
        }
    }
}

impl MIDIEvent for EffectControlEvent {

    fn get_property(&self, argument: u8) -> Vec<u8> {
        match argument {
            0 => {
                vec![self.channel]
            }
            1 => {
                vec![
                    (self.value >> 8) as u8,
                    (self.value & 0xFF) as u8
                ]
            }
            _ => {
                vec![self.which]
            }
        }
    }
    fn set_property(&mut self, argument: u8, bytes: Vec<u8>) {
        match argument {
            0 => {
                self.channel = bytes[0];
            }
            1 => {
                self.value = ((bytes[0] as u16) << 8) + (bytes[1] as u16);
            }
            _ => {
                self.which = bytes[0]
            }
        }
    }
}

impl MIDIEvent for SliderEvent {

    fn get_property(&self, argument: u8) -> Vec<u8> {
        match argument {
            0 => {
                vec![self.channel]
            }
            1 => {
                vec![self.value]
            }
            _ => {
                vec![self.which]
            }
        }
    }
    fn set_property(&mut self, argument: u8, bytes: Vec<u8>) {
        match argument {
            0 => {
                self.channel = bytes[0];
            }
            1 => {
                self.value = bytes[0];
            }
            _ => {
                self.which = bytes[0];
            }
        }
    }
}

impl MIDIEvent for HoldPedalEvent {

    fn get_property(&self, argument: u8) -> Vec<u8> {
        match argument {
            0 => {
                vec![self.channel]
            }
            _ => {
                vec![self.value]
            }
        }
    }
    fn set_property(&mut self, argument: u8, bytes: Vec<u8>) {
        match argument {
            0 => {
                self.channel = bytes[0];
            }
            _ => {
                self.value = bytes[0];
            }
        }
    }
}


impl MIDIEvent for PortamentoEvent {

    fn get_property(&self, argument: u8) -> Vec<u8> {
        match argument {
            0 => {
                vec![self.channel]
            }
            _ => {
                vec![self.value]
            }
        }
    }
    fn set_property(&mut self, argument: u8, bytes: Vec<u8>) {
        match argument {
            0 => {
                self.channel = bytes[0];
            }
            _ => {
                self.value = bytes[0];
            }
        }
    }
}

impl MIDIEvent for SustenutoEvent {

    fn get_property(&self, argument: u8) -> Vec<u8> {
        match argument {
            0 => {
                vec![self.channel]
            }
            _ => {
                vec![self.value]
            }
        }
    }
    fn set_property(&mut self, argument: u8, bytes: Vec<u8>) {
        match argument {
            0 => {
                self.channel = bytes[0];
            }
            _ => {
                self.value = bytes[0];
            }
        }
    }
}

impl MIDIEvent for SoftPedalEvent {

    fn get_property(&self, argument: u8) -> Vec<u8> {
        match argument {
            0 => {
                vec![self.channel]
            }
            _ => {
                vec![self.value]
            }
        }
    }
    fn set_property(&mut self, argument: u8, bytes: Vec<u8>) {
        match argument {
            0 => {
                self.channel = bytes[0];
            }
            _ => {
                self.value = bytes[0];
            }
        }
    }
}

impl MIDIEvent for LegatoEvent {

    fn get_property(&self, argument: u8) -> Vec<u8> {
        match argument {
            0 => {
                vec![self.channel]
            }
            _ => {
                vec![self.value]
            }
        }
    }

    fn set_property(&mut self, argument: u8, bytes: Vec<u8>) {
        match argument {
            0 => {
                self.channel = bytes[0];
            }
            _ => {
                self.value = bytes[0];
            }
        }
    }
}

impl MIDIEvent for Hold2PedalEvent {

    fn get_property(&self, argument: u8) -> Vec<u8> {
        match argument {
            0 => {
                vec![self.channel]
            }
            _ => {
                vec![self.value]
            }
        }
    }
    fn set_property(&mut self, argument: u8, bytes: Vec<u8>) {
        match argument {
            0 => {
                self.channel = bytes[0];
            }
            _ => {
                self.value = bytes[0];
            }
        }
    }
}

impl MIDIEvent for SoundVariationEvent {

    fn get_property(&self, argument: u8) -> Vec<u8> {
        match argument {
            0 => {
                vec![self.channel]
            }
            _ => {
                vec![self.value]
            }
        }
    }
    fn set_property(&mut self, argument: u8, bytes: Vec<u8>) {
        match argument {
            0 => {
                self.channel = bytes[0];
            }
            _ => {
                self.value = bytes[0];
            }
        }
    }
}

impl MIDIEvent for SoundTimbreEvent {

    fn get_property(&self, argument: u8) -> Vec<u8> {
        match argument {
            0 => {
                vec![self.channel]
            }
            _ => {
                vec![self.value]
            }
        }
    }
    fn set_property(&mut self, argument: u8, bytes: Vec<u8>) {
        match argument {
            0 => {
                self.channel = bytes[0];
            }
            _ => {
                self.value = bytes[0];
            }
        }
    }
}

impl MIDIEvent for SoundReleaseTimeEvent {

    fn get_property(&self, argument: u8) -> Vec<u8> {
        match argument {
            0 => {
                vec![self.channel]
            }
            _ => {
                vec![self.value]
            }
        }
    }
    fn set_property(&mut self, argument: u8, bytes: Vec<u8>) {
        match argument {
            0 => {
                self.channel = bytes[0];
            }
            _ => {
                self.value = bytes[0];
            }
        }
    }
}

impl MIDIEvent for SoundAttackEvent {

    fn get_property(&self, argument: u8) -> Vec<u8> {
        match argument {
            0 => {
                vec![self.channel]
            }
            _ => {
                vec![self.value]
            }
        }
    }
    fn set_property(&mut self, argument: u8, bytes: Vec<u8>) {
        match argument {
            0 => {
                self.channel = bytes[0];
            }
            _ => {
                self.value = bytes[0];
            }
        }
    }
}



impl MIDIEvent for SoundBrightnessEvent {

    fn get_property(&self, argument: u8) -> Vec<u8> {
        match argument {
            0 => {
                vec![self.channel]
            }
            _ => {
                vec![self.value]
            }
        }
    }
    fn set_property(&mut self, argument: u8, bytes: Vec<u8>) {
        match argument {
            0 => {
                self.channel = bytes[0];
            }
            _ => {
                self.value = bytes[0];
            }
        }
    }
}


impl MIDIEvent for SoundControlEvent {

    fn get_property(&self, argument: u8) -> Vec<u8> {
        match argument {
            0 => {
                vec![self.value]
            }
            _ => {
                vec![self.which]
            }
        }
    }
    fn set_property(&mut self, argument: u8, bytes: Vec<u8>) {
        match argument {
            0 => {
                self.value = bytes[0]
            }
            _ => {
                self.which = bytes[0]
            }
        }
    }
}

impl MIDIEvent for GeneralButtonOnEvent {

    fn get_property(&self, argument: u8) -> Vec<u8> {
        match argument {
            0 => {
                vec![self.channel]
            }
            _ => {
                vec![self.which]
            }
        }
    }
    fn set_property(&mut self, argument: u8, bytes: Vec<u8>) {
        match argument {
            0 => {
                self.channel = bytes[0];
            }
            _ => {
                self.which = bytes[0];
            }
        }
    }
}


impl MIDIEvent for GeneralButtonOffEvent {

    fn get_property(&self, argument: u8) -> Vec<u8> {
        match argument {
            0 => {
                vec![self.channel]
            }
            _ => {
                vec![self.which]
            }
        }
    }
    fn set_property(&mut self, argument: u8, bytes: Vec<u8>) {
        match argument {
            0 => {
                self.channel = bytes[0];
            }
            _ => {
                self.which = bytes[0];
            }
        }
    }
}

impl MIDIEvent for EffectsLevelEvent {

    fn get_property(&self, argument: u8) -> Vec<u8> {
        match argument {
            0 => {
                vec![self.channel]
            }
            _ => {
                vec![self.value]
            }
        }
    }
    fn set_property(&mut self, argument: u8, bytes: Vec<u8>) {
        match argument {
            0 => {
                self.channel = bytes[0];
            }
            _ => {
                self.value = bytes[0];
            }
        }
    }
}


impl MIDIEvent for TremuloLevelEvent {

    fn get_property(&self, argument: u8) -> Vec<u8> {
        match argument {
            0 => {
                vec![self.channel]
            }
            _ => {
                vec![self.value]
            }
        }
    }
    fn set_property(&mut self, argument: u8, bytes: Vec<u8>) {
        match argument {
            0 => {
                self.channel = bytes[0];
            }
            _ => {
                self.value = bytes[0];
            }
        }
    }
}


impl MIDIEvent for ChorusLevelEvent {

    fn get_property(&self, argument: u8) -> Vec<u8> {
        match argument {
            0 => {
                vec![self.channel]
            }
            _ => {
                vec![self.value]
            }
        }
    }
    fn set_property(&mut self, argument: u8, bytes: Vec<u8>) {
        match argument {
            0 => {
                self.channel = bytes[0];
            }
            _ => {
                self.value = bytes[0];
            }
        }
    }
}

impl MIDIEvent for CelesteLevelEvent {

    fn get_property(&self, argument: u8) -> Vec<u8> {
        match argument {
            0 => {
                vec![self.channel]
            }
            _ => {
                vec![self.value]
            }
        }
    }
    fn set_property(&mut self, argument: u8, bytes: Vec<u8>) {
        match argument {
            0 => {
                self.channel = bytes[0];
            }
            _ => {
                self.value = bytes[0];
            }
        }
    }
}

impl MIDIEvent for PhaserLevelEvent {

    fn get_property(&self, argument: u8) -> Vec<u8> {
        match argument {
            0 => {
                vec![self.channel]
            }
            _ => {
                vec![self.value]
            }
        }
    }
    fn set_property(&mut self, argument: u8, bytes: Vec<u8>) {
        match argument {
            0 => {
                self.channel = bytes[0];
            }
            _ => {
                self.value = bytes[0];
            }
        }
    }
}


impl MIDIEvent for DataButtonIncrementEvent {

    // target is sent +1, 0 indicating no target
    fn get_property(&self, argument: u8) -> Vec<u8> {
        match argument {
            0 => {
                vec![self.channel]
            }
            _ => {
                match self.target {
                    Some(n) => {
                        vec![
                            ((n + 1) >> 8) as u8,
                            ((n + 1) & 0xFF) as u8
                        ]
                    }
                    None => {
                        vec![0]
                    }
                }
            }
        }
    }
    // target is received as +1, 0 indicating no target
    fn set_property(&mut self, argument: u8, bytes: Vec<u8>) {
        match argument {
            0 => {
                self.channel = bytes[0];
            }
            _ => {
                if bytes[0] > 0 {
                    self.target = Some(
                        (((bytes[0] as u16) << 8) + (bytes[1] as u16)) - 1
                    );
                } else {
                    self.target = None;
                }
            }
        }
    }
}

impl MIDIEvent for DataButtonDecrementEvent {

    // target is sent +1, 0 indicating no target
    fn get_property(&self, argument: u8) -> Vec<u8> {
        match argument {
            0 => {
                vec![self.channel]
            }
            _ => {
                match self.target {
                    Some(n) => {
                        vec![
                            ((n + 1) >> 8) as u8,
                            ((n + 1) & 0xFF) as u8
                        ]
                    }
                    None => {
                        vec![0]
                    }
                }
            }
        }
    }
    // target is received as +1, 0 indicating no target
    fn set_property(&mut self, argument: u8, bytes: Vec<u8>) {
        match argument {
            0 => {
                self.channel = bytes[0];
            }
            _ => {
                if bytes[0] > 0 {
                    self.target = Some(
                        (((bytes[0] as u16) << 8) + (bytes[1] as u16)) - 1
                    );
                } else {
                    self.target = None;
                }
            }
        }
    }
}

impl MIDIEvent for RegisteredParameterNumberEvent {


    fn get_property(&self, argument: u8) -> Vec<u8> {
        match argument {
            0 => {
                vec![self.channel]
            }
            _ => {
                vec![
                    (self.value >> 8) as u8,
                    (self.value & 0xFF) as u8
                ]
            }
        }
    }

    fn set_property(&mut self, argument: u8, bytes: Vec<u8>) {
        match argument {
            0 => {
                self.channel = bytes[0];
            }
            _ => {
                self.value = ((bytes[0] as u16) << 8) + (bytes[1] as u16);
            }
        }
    }
}


impl MIDIEvent for NonRegisteredParameterNumberEvent {

    fn get_property(&self, argument: u8) -> Vec<u8> {
        match argument {
            0 => {
                vec![self.channel]
            }
            _ => {
                vec![
                    (self.value >> 8) as u8,
                    (self.value & 0xFF) as u8
                ]
            }
        }
    }
    fn set_property(&mut self, argument: u8, bytes: Vec<u8>) {
        match argument {
            0 => {
                self.channel = bytes[0];
            }
            _ => {
                self.value = ((bytes[0] as u16) << 8) + (bytes[1] as u16);
            }
        }
    }
}

impl MIDIEvent for AllControllersOffEvent {

    fn get_property(&self, _: u8) -> Vec<u8> {
        vec![self.channel]
    }
    fn set_property(&mut self, _: u8, bytes: Vec<u8>) {
        self.channel = bytes[0];
    }
}

impl MIDIEvent for LocalKeyboardEnableEvent {

    fn get_property(&self, _: u8) -> Vec<u8> {
        vec![self.channel]
    }
    fn set_property(&mut self, _: u8, bytes: Vec<u8>) {
        self.channel = bytes[0];
    }
}

impl MIDIEvent for LocalKeyboardDisableEvent {

    fn get_property(&self, _: u8) -> Vec<u8> {
        vec![self.channel]
    }
    fn set_property(&mut self, _: u8, bytes: Vec<u8>) {
        self.channel = bytes[0];
    }
}


impl MIDIEvent for AllNotesOffEvent {

    fn get_property(&self, _: u8) -> Vec<u8> {
        vec![self.channel]
    }
    fn set_property(&mut self, _: u8, bytes: Vec<u8>) {
        self.channel = bytes[0];
    }
}

impl MIDIEvent for AllSoundOffEvent {

    fn get_property(&self, _: u8) -> Vec<u8> {
        vec![self.channel]
    }
    fn set_property(&mut self, _: u8, bytes: Vec<u8>) {
        self.channel = bytes[0];
    }
}


impl MIDIEvent for OmniOffEvent {

    fn get_property(&self, _: u8) -> Vec<u8> {
        vec![self.channel]
    }
    fn set_property(&mut self, _: u8, bytes: Vec<u8>) {
        self.channel = bytes[0];
    }
}


impl MIDIEvent for OmniOnEvent {

    fn get_property(&self, _: u8) -> Vec<u8> {
        vec![self.channel]
    }
    fn set_property(&mut self, _: u8, bytes: Vec<u8>) {
        self.channel = bytes[0];
    }
}


impl MIDIEvent for MonophonicOperationEvent {


    fn get_property(&self, argument: u8) -> Vec<u8> {
        match argument {
            0 => {
                vec![self.channel]
            }
            _ => {
                vec![self.value]
            }
        }
    }
    fn set_property(&mut self, argument: u8, bytes: Vec<u8>) {
        match argument {
            0 => {
                self.channel = bytes[0];
            }
            _ => {
                self.value = bytes[0];
            }
        }
    }
}


impl PolyphonicOperationEvent {
    pub fn new(channel: u8) -> PolyphonicOperationEvent {
        PolyphonicOperationEvent {
            channel
        }
    }
}
impl MIDIEvent for PolyphonicOperationEvent {

    fn get_property(&self, _: u8) -> Vec<u8> {
        vec![self.channel]
    }
    fn set_property(&mut self, _: u8, bytes: Vec<u8>) {
        self.channel = bytes[0];
    }
}

// End ControlChangeEvents

impl MIDIEvent for ControlChangeEvent {
    fn set_property(&mut self, argument: u8, bytes: Vec<u8>) {
        match argument {
            0 => {
                self.channel = bytes[0] & 0x0F;
            }
            1 => {
                self.controller = bytes[0] & 0x7F;
            }
            2 => {
                self.value = bytes[0] & 0x7F;
            }
            _ => ()
        };
    }
    fn get_property(&self, argument: u8) -> Vec<u8> {
        let output = match argument {
            0 => {
                self.channel
            }
            1 => {
                self.controller
            }
            2 => {
                self.value
            }
            _ => {
                0
            }
        };

        vec![output]
    }
}

impl MIDIEvent for ProgramChangeEvent {
    fn set_property(&mut self, argument: u8, bytes: Vec<u8>) {
        match argument {
            0 => {
                self.channel = bytes[0] & 0x0F;
            }
            1 => {
                self.program = bytes[0] & 0x7F;
            }
            _ => ()
        };
    }
    fn get_property(&self, argument: u8) -> Vec<u8> {
        let output = match argument {
            0 => {
                self.channel
            }
            1 => {
                self.program
            }
            _ => {
                0
            }
        };

        vec![output]
    }
}

impl MIDIEvent for ChannelPressureEvent {
    fn set_property(&mut self, argument: u8, bytes: Vec<u8>) {
        match argument {
            0 => {
                self.channel = bytes[0] & 0x0F;
            }
            1 => {
                self.pressure = bytes[0] & 0x7F;
            }
            _ => ()
        };
    }
    fn get_property(&self, argument: u8) -> Vec<u8> {
        let output = match argument {
            0 => {
                self.channel
            }
            1 => {
                self.pressure
            }
            _ => {
                0
            }
        };
        vec![output]
    }
}


impl PitchWheelChangeEvent {
    pub fn new(channel: u8, value: f64) -> PitchWheelChangeEvent {
        PitchWheelChangeEvent {
            channel: channel & 0x0F,
            value
        }
    }
    pub fn new_from_lsb_msb(channel: u8, lsb: u8, msb: u8) -> PitchWheelChangeEvent {
        let unsigned_value: f64 = (((msb as u16) << 7) + (lsb as u16)) as f64;
        let new_value: f64 = ((unsigned_value * 2_f64) as f64 / 0x3FFF as f64) - 1_f64;
        PitchWheelChangeEvent::new(channel, new_value)
    }

    fn get_unsigned_value(&self) -> u16 {
        if self.value < 0_f64 {
            ((1_f64 + self.value) * (0x2000 as f64)) as u16
        } else if self.value > 0_f64 {
            (self.value * (0x1FFF as f64)) as u16 + 0x2000
        } else {
            0x2000
        }
    }

    pub fn set_value(&mut self, value: f64) {
        self.value = value;
    }
}

impl MIDIEvent for PitchWheelChangeEvent {

    fn set_property(&mut self, argument: u8, bytes: Vec<u8>) {
        match argument {
            0 => {
                self.channel = bytes[0] & 0x0F;
            }
            1 => {
                let unsigned_value = (((bytes[0] as u16) * 256) + (bytes[1] as u16)) as f64;
                let new_value: f64 = ((unsigned_value * 2_f64) / 0x3FFF as f64) - 1_f64;

                self.value = new_value;
            }
            _ => ()
        };
    }

    fn get_property(&self, argument: u8) -> Vec<u8> {
        match argument {
            0 => {
                vec![
                    self.channel
                ]
            }
            1 => {
                let unsigned_value = self.get_unsigned_value();
                vec![
                    (unsigned_value / 256) as u8,
                    (unsigned_value % 256) as u8
                ]
            }
            _ => {
                vec![0]
            }
        }
    }
}


impl MIDIEvent for SystemExclusiveEvent {
    fn set_property(&mut self, _:u8, bytes: Vec<u8>) {
        self.data = bytes.clone()
    }

    fn get_property(&self, _argument: u8) -> Vec<u8> {
        self.data.clone()
    }

}


impl MIDIEvent for MTCQuarterFrameEvent {
    fn set_property(&mut self, argument: u8, bytes: Vec<u8>) {
        match argument {
            0 => {
                self.message_type = bytes[0];
            }
            1 => {
                self.value = bytes[0];
            }
            _ => ()
        };
    }

    fn get_property(&self, argument: u8) -> Vec<u8> {
        match argument {
            0 => {
               vec![ self.message_type ]
            }
            1 => {
               vec![ self.value ]
            }
            _ => {
                Vec::new()
            }
        }
    }
}

impl MIDIEvent for SongPositionPointerEvent {
    fn set_property(&mut self, _: u8, bytes: Vec<u8>) {
        self.beat = ((bytes[0] as u16) * 256) + (bytes[1] as u16);
    }

    fn get_property(&self, _: u8) -> Vec<u8> {
        vec![
            (self.beat / 256) as u8,
            (self.beat % 256) as u8
        ]
    }
}

impl MIDIEvent for SongSelectEvent {
    fn set_property(&mut self, _: u8, bytes: Vec<u8>) {
        self.song = bytes[0] & 0x7F;
    }
    fn get_property(&self, _: u8) -> Vec<u8> {
        vec![
            self.song & 0x7F
        ]
    }
}

impl MIDIEvent for TuneRequestEvent {
    fn set_property(&mut self, _: u8, _bytes: Vec<u8>) { }
    fn get_property(&self, _: u8) -> Vec<u8> {
        Vec::new()
    }
}

impl MIDIEvent for MIDIClockEvent {
    fn set_property(&mut self, _: u8, _bytes: Vec<u8>) { }
    fn get_property(&self, _: u8) -> Vec<u8> {
        Vec::new()
    }
}

impl MIDIEvent for MIDIStartEvent {
    fn set_property(&mut self, _: u8, _bytes: Vec<u8>) { }
    fn get_property(&self, _: u8) -> Vec<u8> {
        Vec::new()
    }
}

impl MIDIEvent for MIDIContinueEvent {
    fn set_property(&mut self, _: u8, _bytes: Vec<u8>) { }
    fn get_property(&self, _: u8) -> Vec<u8> {
        Vec::new()
    }
}

impl MIDIEvent for MIDIStopEvent {
    fn set_property(&mut self, _: u8, _bytes: Vec<u8>) { }
    fn get_property(&self, _: u8) -> Vec<u8> {
        Vec::new()
    }
}

impl MIDIEvent for ActiveSenseEvent {
    fn set_property(&mut self, _: u8, _bytes: Vec<u8>) { }
    fn get_property(&self, _: u8) -> Vec<u8> {
        Vec::new()
    }
}

impl MIDIEvent for ResetEvent {
    fn set_property(&mut self, _: u8, _bytes: Vec<u8>) { }
    fn get_property(&self, _: u8) -> Vec<u8> {
        Vec::new()
    }
}

