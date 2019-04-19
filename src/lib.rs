use std::ffi::CStr;
use std::os::raw::c_char;
use std::fs::File;
use std::io::prelude::*;
use std::cmp::max;

use std::mem;

use std::collections::HashMap;
use std::slice;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

pub enum MIDIEvent {
    // Meta Events ///////////////////////////
//    SequenceNumberEvent(u32),
//    TextEvent(Vec<u8>), // 2
//    CopyRightNoticeEvent(Vec<u8>), // 3
//    TrackNameEvent(Vec<u8>), // 4
//    InstrumentNameEvent(Vec<u8>), // 5
//    LyricEvent(Vec<u8>),
//    MarkerEvent(Vec<u8>),
//    CuePointEvent(Vec<u8>),
//// TODO; figure out what prefix is, u8 for now
//    ChannelPrefixEvent(u8),
    EndOfTrackEvent,
    SetTempoEvent(u32),
////TODO: Figure out what ff/fr are, u16 for now
    //SMPTEOffsetEvent((u32, u32, u32, u16, u16)),
    TimeSignatureEvent((u8, u8, u8, u8)),
    KeySignatureEvent((u8, u8)), // 'mi', 'number of sharps'
//    SequencerSpecificEvent(Vec<u8>),
    //////////////////////////////////////////

    // ChannelEvents /////////////////////////
    NoteOnEvent((u8, u8, u8)),
    NoteOffEvent((u8, u8, u8)),
    PolyphonicKeyPressureEvent((u8, u8, u8)),
    ControlChangeEvent((u8, u8, u8)),
    ProgramChangeEvent((u8, u8)),
    ChannelPressureEvent((u8, u8)),
    PitchWheelChangeEvent((u8, u8, u8))
    //////////////////////////////////////////

    //CommonEvent(Vec<u8>),
}

// System Common Events //////////////////
// TODO: All System RealTime Events
//////////////////////////////////////////

pub struct MIDILike {
    ppqn: u32,
    midi_format: u16, // 16 because the format is store in 2 bytes, even though it's 0-2
    tracks: Vec<HashMap<usize, Vec<u64>>>, // Outer Vector is list of track, not every tick in a track has an event, some have many
    events: HashMap<u64, MIDIEvent>,
    event_id_gen: u64
}


impl MIDILike {
    fn new() -> MIDILike {
        MIDILike {
            event_id_gen: 0,
            ppqn: 120,
            midi_format: 1,
            tracks: Vec::new(),
            events: HashMap::new()
        }
    }

    fn get_track_length(&self, track: usize) -> usize {
        let length: usize;
        if (track >= self.tracks.len() ) {
            length = 0;
        } else {
            let mut largest_tick = 0;
            for key in self.tracks[track].keys() {
                largest_tick = max(*key, largest_tick);
            }
            length = largest_tick + 1;
        }

        length
    }

    fn get_active_tick_count(&self, track: usize) -> usize {
        if (track >= self.tracks.len() ) {
            0
        } else {
            let n = self.tracks[track].keys().len();
            n
        }
    }

    fn get_nth_active_tick(&self, track: usize, n: usize) -> usize {
        let mut sorted_keys = Vec::new();
        for key in self.tracks[track].keys() {
            sorted_keys.push(key);
        }
        sorted_keys.sort();
        *sorted_keys[n]
    }

    fn get_tick_length(&self, track: usize, tick: usize) -> usize {
        let length: usize;
        if tick >= self.get_track_length(track) {
            length = 0;
        } else {
            length = match self.tracks[track].get(&tick) {
                Some(eventlist) => {
                    eventlist.len()
                }
                None => 0
            };
        }

        length
    }


    fn get_nth_event_id_in_tick(&self, track: usize, tick: usize, n: usize) -> Result<u64, u32> {
        if n < self.get_tick_length(track, tick) {
            Ok(self.tracks[track][&tick][n])
        } else {
            Err(0)
        }
    }

    fn set_ppqn(&mut self, new_ppqn: u32) {
        self.ppqn = new_ppqn;
    }
    fn set_format(&mut self, new_format: u16) {
        self.midi_format = new_format;
    }

    fn add_event(&mut self, track: usize, tick: usize, event: MIDIEvent) {
        let new_event_id = self.event_id_gen;
        self.events.insert(new_event_id, event);
        while track >= self.tracks.len() {
            self.tracks.push(HashMap::new());
        }
        self.tracks[track].entry(tick)
            .and_modify(|eventlist| {
                (*eventlist).push(new_event_id)
            })
            .or_insert(vec![new_event_id]);

        self.event_id_gen += 1;
    }
}


fn pop_n(bytes: &mut Vec<u8>, n: usize) -> u32 {
    let mut tn: u32 = 0;
    for _ in 0..n {
        tn *= 256;
        match bytes.pop() {
            Some(x) => {
                tn += x as u32;
            },
            None => { }
        }
    }
    tn
}

// TODO: May only ever have to be u8
fn from_twos_complement(value: u32, bits: u8) -> u32 {
    let mut complement = 0u32;

    for _ in 0..bits {
        complement <<= 2;
        complement += 1;
    }
    (value - 1) ^ complement
}

// TODO: may need to be longer or shorter than 32
fn get_variable_length_number(bytes: &mut Vec<u8>) -> u64 {
    let mut n = 0u64;
    loop {
        n <<= 7;
        match bytes.pop() {
            Some(x) => {
                n += (x & 0x7F) as u64;
                if x & 0x80 == 0 {
                    break;
                }
            },
            None => {
                break;
            }
        }
    }
    n
}


fn process_mtrk_event(leadbyte: u8, bytes: &mut Vec<u8>, current_deltatime: &mut usize, mlo: &mut MIDILike, track: usize, fallback_cmd: &mut u8) {
    let mut channel: u8;

    let mut a: u8;
    let mut b: u8;
    let mut c: u8;

    let mut n: u32;
    let mut varlength: u64;

    let mut dump: Vec<u8>;

    let mut leadnibble: u8 = leadbyte >> 4;


    // CHANNEL EVENT
    if leadnibble == 8
      || leadnibble == 9
      || leadnibble == 10
      || leadnibble == 11
      || leadnibble == 14 {

        channel = leadbyte & 0x0F;
        b = bytes.pop().unwrap();
        c = bytes.pop().unwrap();
        if leadnibble == 9 && c == 0x00 {
            a = leadbyte & 0xEF;
            leadnibble = 8;
        } else {
            a = leadbyte;
        }
        if leadnibble == 8 {
            mlo.add_event(track, *current_deltatime, MIDIEvent::NoteOffEvent((a, b, c)));
        } else if leadnibble == 9 {
            mlo.add_event(track, *current_deltatime, MIDIEvent::NoteOnEvent((a, b, c)));
        } else if leadnibble == 10 {
            mlo.add_event(track, *current_deltatime, MIDIEvent::PolyphonicKeyPressureEvent((a, b, c)));

        } else if leadnibble == 11 {
            mlo.add_event(track, *current_deltatime, MIDIEvent::ControlChangeEvent((a, b, c)));
        } else if leadnibble == 14 {
            mlo.add_event(track, *current_deltatime, MIDIEvent::PitchWheelChangeEvent((a, b, c)));
        }

    } else if leadnibble == 12 || leadnibble == 13 {
    // ProgramChange/ChannelPressure
        channel = leadbyte & 0x0F;
        b = bytes.pop().unwrap();
        if leadnibble == 12 {
            mlo.add_event(track, *current_deltatime, MIDIEvent::ProgramChangeEvent((leadbyte, b)));
        } else if leadnibble == 13 {
            mlo.add_event(track, *current_deltatime, MIDIEvent::ChannelPressureEvent((leadbyte, b)));

        }
    } else if leadbyte == 0xF0 {
        // System Common
        dump = Vec::new();
        loop {
            match bytes.pop() {
                Some(byte) => {
                    if byte == 0xF7 {
                        dump.push(byte)
                    } else {
                        break;
                    }
                },
                None => {
                    break;
                }
            }
        }
        // TODO ADD EVENT
    } else if leadbyte == 0xF2 { // Song Position Pointer
        b = bytes.pop().unwrap();
        c = bytes.pop().unwrap();
        dump = vec![b, c];
        // TODO ADD EVENT
    } else if leadbyte == 0xF3 {
        b = bytes.pop().unwrap();
        // TODO ADD EVENT
    } else if leadbyte == 0xF6 {
        // TODO ADD EVENT
    } else if leadbyte == 0xF7 {
        varlength = get_variable_length_number(bytes);
        n = pop_n(bytes, varlength as usize);
        // TODO ADD EVENT
    //} else if [0xF1, 0xF4, 0xF5].contains(leadbyte) {
        // Undefined Behaviour
    } else if leadbyte == 0xFF {
        a = bytes.pop().unwrap(); // Meta Type
        varlength = get_variable_length_number(bytes);
        if (a == 0x51) {
            mlo.add_event(track, *current_deltatime, MIDIEvent::SetTempoEvent(pop_n(bytes, varlength as usize)));
        } else {
            dump = Vec::new();
            for _ in 0..varlength {
                match bytes.pop() {
                    Some(byte) => {
                        dump.push(byte);
                    },
                    None => {
                        break; // TODO: Should throw error
                    }
                }
            }
            // TODO: All of this
            if a == 2 {
            } else if a == 3 {
            } else if a == 4 {
            } else if a == 5 {
            } else if a == 0x2F {
                mlo.add_event(track, *current_deltatime, MIDIEvent::EndOfTrackEvent);
            } else if a == 0x51 {
            } else if a == 0x58 {
                mlo.add_event(track, *current_deltatime, MIDIEvent::TimeSignatureEvent((dump[0], dump[1],dump[2], dump[3])));
            } else if a == 0x59 {
            }
        }
    } else if leadbyte >= 0xF8 {
        // ADD EVENT
    } else if leadbyte < 0x80 { // Implicitly a Channel Event
        bytes.push(leadbyte);
        process_mtrk_event(*fallback_cmd, bytes, current_deltatime, mlo, track, fallback_cmd);
    } else {
        // Undefined Behaviour
    }

    if leadnibble >= 8 && leadnibble < 15 {
        *fallback_cmd = leadbyte.clone();
    }
}


fn _interpret(bytes: &mut Vec<u8>) -> MIDILike {
    bytes.reverse();
    let mut mlo: MIDILike = MIDILike::new();
    let mut sub_bytes: Vec<u8>;
    let mut chunkcount: HashMap<(u8, u8,u8, u8), u16> = HashMap::new();
    let mut current_track: usize = 0;
    let mut current_deltatime: usize = 0;

    let mut chunk_type: (u8, u8, u8, u8);


    // TODO: These Probably don't need to be 32
    let mut divword: u32;
    let mut smpte: u32;
    let mut tpf: u32;
    let mut midi_format: u16;

    let mut track_length: u32;

    let mut ppqn = 120;
    let mut fallback_byte = 0x90u8;
    while bytes.len() > 0 {
        chunk_type = (
            bytes.pop().unwrap(),
            bytes.pop().unwrap(),
            bytes.pop().unwrap(),
            bytes.pop().unwrap()
        );

        let val = chunkcount.entry(chunk_type).or_insert(0);
        *val += 1;

        current_deltatime = 0;
        if chunk_type == ('M' as u8, 'T' as u8, 'h' as u8, 'd' as u8) {
            pop_n(bytes, 4); // Get Size
            midi_format = pop_n(bytes, 2) as u16; // Midi Format
            pop_n(bytes, 2); // Get Number of tracks
            divword = pop_n(bytes, 2);
            if divword & 0x8000 > 0 {
                smpte = from_twos_complement(((divword & 0x7F00) >> 8) as u32, 7);
                tpf = divword & 0x00FF;

            } else {
                ppqn = divword & 0x7FFF;
            }
            mlo.set_ppqn(ppqn);
            mlo.set_format(midi_format);
        } else if chunk_type == ('M' as u8, 'T' as u8, 'r' as u8, 'k' as u8) {
            track_length = pop_n(bytes, 4);
            sub_bytes = Vec::new();
            for _ in 0..track_length {
                sub_bytes.push(bytes.pop().unwrap())
            }
            sub_bytes.reverse();
            while sub_bytes.len() > 0 {
                current_deltatime += get_variable_length_number(&mut sub_bytes) as usize;
                match sub_bytes.pop() {
                    Some(byte) => {
                        process_mtrk_event(byte, &mut sub_bytes, &mut current_deltatime, &mut mlo, current_track, &mut fallback_byte);
                    },
                    None => {}
                }
            }
            current_track += 1;
        } else {
            break;
        }
    }
    mlo
}


#[no_mangle]
pub extern fn interpret(path: *const c_char) -> *mut MIDILike {
    let cstr_path = unsafe {
        CStr::from_ptr(path)
    };

    let mut bytes: Vec<u8> = Vec::new();
    let clean_path = cstr_path.to_str().expect("Not a valid UTF-8 string");
    let mut file = File::open(clean_path).expect("Unable to open the file");
    file.read_to_end(&mut bytes).expect("Unable to read file");

   Box::into_raw(Box::new( _interpret(&mut bytes) ))
}

#[no_mangle]
pub extern fn get_track_length(midilike_ptr: *mut MIDILike, track: usize) -> usize {
    let mut midilike = unsafe { Box::from_raw(midilike_ptr) };

    let length = midilike.get_track_length(track);

    Box::into_raw(midilike);

    length
}

#[no_mangle]
pub extern fn get_active_tick_count(midilike_ptr: *mut MIDILike, track: usize) -> usize {
    let mut midilike = unsafe { Box::from_raw(midilike_ptr) };

    let length = midilike.get_active_tick_count(track);

    Box::into_raw(midilike);

    length
}


#[no_mangle]
pub extern fn get_track_count(midilike_ptr: *mut MIDILike) -> usize {

    let mut midilike = unsafe { Box::from_raw(midilike_ptr) };

    let count = midilike.tracks.len();

    Box::into_raw(midilike);

    count
}

#[no_mangle]
pub extern fn get_tick_length(midilike_ptr: *mut MIDILike, track: usize, tick: usize) -> usize {
    let mut midilike = unsafe { Box::from_raw(midilike_ptr) };

    let length = midilike.get_tick_length(track, tick);

    Box::into_raw(midilike);

    length
}

#[no_mangle]
pub extern fn get_nth_active_tick(midilike_ptr: *mut MIDILike, track: usize, n: usize) -> usize {
    let mut midilike = unsafe { Box::from_raw(midilike_ptr) };

    let tick = midilike.get_nth_active_tick(track, n);

    Box::into_raw(midilike);

    tick
}

#[no_mangle]
pub extern fn get_nth_event_in_tick(midilike_ptr: *mut MIDILike, track: usize, tick: usize, n: usize) -> u64 {
    let mut midilike = unsafe { Box::from_raw(midilike_ptr) };

    let event_id = midilike.get_nth_event_id_in_tick(track, tick, n).expect("Event Doesn't Exist");


    Box::into_raw(midilike);

    event_id
}


