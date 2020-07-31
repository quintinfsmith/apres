use std::ffi::{CStr, CString};
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

// For python Bindings
pub enum MIDIEventType {
    SequenceNumber = 22,
    Text = 1,
    CopyRightNotice = 2,
    TrackName = 3,
    InstrumentName = 4,
    Lyric = 5,
    Marker = 6,
    CuePoint = 7,
    EndOfTrack = 8,
    ChannelPrefix = 9,
    SetTempo = 10,
    SMPTEOffset = 11,
    TimeSignature = 12,
    KeySignature = 13,
    SequencerSpecific = 14,

    NoteOn = 15,
    NoteOff = 16,
    PolyphonicKeyPressure = 17,
    ControlChange = 18,
    ProgramChange = 19,
    ChannelPressure = 20,
    PitchWheelChange = 21,

    SystemExclusive = 23,
    MTCQuarterFrame = 24,
    SongPositionPointer = 25,
    SongSelect = 26,
    TuneRequest = 27,
    MIDIClock = 28,
    MIDIStart = 29,
    MIDIContinue = 30,
    MIDIStop = 31,
    ActiveSense = 32,
    Reset = 33
}

pub trait MIDIEvent {
    fn to_bytes(&self) -> Vec<u8>;
    fn get_eid(&self) -> u8;
    fn is_meta(&self) -> bool;
    fn set_property(&mut self, argument: u8, bytes: Vec<u8>);
    fn get_property(&self, argument: u8) -> Vec<u8>;
    fn get_type(&self) -> MIDIEventType;
}

impl MIDIEvent { }

pub struct SequenceNumberEvent {
    sequence: u16
}

impl MIDIEvent for SequenceNumberEvent {
    fn to_bytes(&self) -> Vec<u8> {
        vec![
            0xFF, 0x00, 0x02,
            (self.sequence / 256) as u8,
            (self.sequence % 256) as u8
        ]
    }
    fn is_meta(&self) -> bool {
        true
    }
    fn get_eid(&self) -> u8 {
        0x00
    }
    fn set_property(&mut self, argument: u8, bytes: Vec<u8>) {
        self.sequence = (bytes[1] as u16 * 256) + (bytes[0] as u16);
    }
    fn get_property(&self, _: u8) -> Vec<u8> {
        vec![
            (self.sequence / 256) as u8,
            (self.sequence % 256) as u8
        ]
    }
    fn get_type(&self) -> MIDIEventType {
        MIDIEventType::SequenceNumber
    }
}

pub struct TextEvent {
    text: String
}
impl MIDIEvent for TextEvent {
    fn to_bytes(&self) -> Vec<u8> {
        let text_bytes = self.text.as_bytes();
        let mut length_bytes = to_variable_length_bytes(text_bytes.len());

        let mut output = vec![0xFF, 0x01];
        output.extend(length_bytes.iter().copied());
        output.extend(text_bytes.iter().copied());

        output
    }
    fn is_meta(&self) -> bool {
        true
    }
    fn get_eid(&self) -> u8 {
        0x01
    }
    fn set_property(&mut self, argument: u8, bytes: Vec<u8>) {
        self.text = std::str::from_utf8(bytes.as_slice()).unwrap().to_string();
    }
    fn get_property(&self, _: u8) -> Vec<u8> {
        self.text.as_bytes().to_vec()
    }
    fn get_type(&self) -> MIDIEventType {
        MIDIEventType::Text
    }
}
pub struct CopyRightNoticeEvent {
    text: String
}
impl MIDIEvent for CopyRightNoticeEvent {
    fn to_bytes(&self) -> Vec<u8> {
        let text_bytes = self.text.as_bytes();
        let mut length_bytes = to_variable_length_bytes(text_bytes.len());

        let mut output = vec![0xFF, 0x02];
        output.extend(length_bytes.iter().copied());
        output.extend(text_bytes.iter().copied());

        output
    }
    fn is_meta(&self) -> bool {
        true
    }
    fn get_eid(&self) -> u8 {
        0x02
    }
    fn set_property(&mut self, argument: u8, bytes: Vec<u8>) {
        self.text = std::str::from_utf8(bytes.as_slice()).unwrap().to_string();
    }
    fn get_property(&self, _: u8) -> Vec<u8> {
        self.text.as_bytes().to_vec()
    }
    fn get_type(&self) -> MIDIEventType {
        MIDIEventType::CopyRightNotice
    }
}
pub struct TrackNameEvent {
    track_name: String
}
impl MIDIEvent for TrackNameEvent {
    fn to_bytes(&self) -> Vec<u8> {
        let text_bytes = self.track_name.as_bytes();
        let mut length_bytes = to_variable_length_bytes(text_bytes.len());

        let mut output = vec![0xFF, 0x03];
        output.extend(length_bytes.iter().copied());
        output.extend(text_bytes.iter().copied());

        output
    }
    fn is_meta(&self) -> bool {
        true
    }
    fn get_eid(&self) -> u8 {
        0x03
    }
    fn set_property(&mut self, argument: u8, bytes: Vec<u8>) {
        self.track_name = std::str::from_utf8(bytes.as_slice()).unwrap().to_string();
    }
    fn get_property(&self, _: u8) -> Vec<u8> {
        self.track_name.as_bytes().to_vec()
    }
    fn get_type(&self) -> MIDIEventType {
        MIDIEventType::TrackName
    }
}
pub struct InstrumentNameEvent {
    instrument_name: String
}
impl MIDIEvent for InstrumentNameEvent {
    fn to_bytes(&self) -> Vec<u8> {
        let text_bytes = self.instrument_name.as_bytes();
        let mut length_bytes = to_variable_length_bytes(text_bytes.len());

        let mut output = vec![0xFF, 0x04];
        output.extend(length_bytes.iter().copied());
        output.extend(text_bytes.iter().copied());

        output
    }
    fn is_meta(&self) -> bool {
        true
    }
    fn get_eid(&self) -> u8 {
        0x04
    }
    fn set_property(&mut self, argument: u8, bytes: Vec<u8>) {
        self.instrument_name = std::str::from_utf8(bytes.as_slice()).unwrap().to_string();
    }
    fn get_property(&self, _: u8) -> Vec<u8> {
        self.instrument_name.as_bytes().to_vec()
    }
    fn get_type(&self) -> MIDIEventType {
        MIDIEventType::InstrumentName
    }
}
pub struct LyricEvent {
    lyric: String
}
impl MIDIEvent for LyricEvent {
    fn to_bytes(&self) -> Vec<u8> {
        let text_bytes = self.lyric.as_bytes();
        let mut length_bytes = to_variable_length_bytes(text_bytes.len());

        let mut output = vec![0xFF, 0x05];
        output.extend(length_bytes.iter().copied());
        output.extend(text_bytes.iter().copied());

        output
    }
    fn is_meta(&self) -> bool {
        true
    }
    fn get_eid(&self) -> u8 {
        0x05
    }
    fn set_property(&mut self, argument: u8, bytes: Vec<u8>) {
        self.lyric = std::str::from_utf8(bytes.as_slice()).unwrap().to_string();
    }
    fn get_property(&self, _: u8) -> Vec<u8> {
        self.lyric.as_bytes().to_vec()
    }
    fn get_type(&self) -> MIDIEventType {
        MIDIEventType::Lyric
    }
}
pub struct MarkerEvent {
    text: String
}
impl MIDIEvent for MarkerEvent {
    fn to_bytes(&self) -> Vec<u8> {
        let text_bytes = self.text.as_bytes();
        let mut length_bytes = to_variable_length_bytes(text_bytes.len());

        let mut output = vec![0xFF, 0x06];
        output.extend(length_bytes.iter().copied());
        output.extend(text_bytes.iter().copied());

        output
    }
    fn is_meta(&self) -> bool {
        true
    }
    fn get_eid(&self) -> u8 {
        0x06
    }
    fn set_property(&mut self, argument: u8, bytes: Vec<u8>) {
        self.text = std::str::from_utf8(bytes.as_slice()).unwrap().to_string();
    }
    fn get_property(&self, _: u8) -> Vec<u8> {
        self.text.as_bytes().to_vec()
    }
    fn get_type(&self) -> MIDIEventType {
        MIDIEventType::Marker
    }
}
pub struct CuePointEvent {
    text: String
}
impl MIDIEvent for CuePointEvent {
    fn to_bytes(&self) -> Vec<u8> {
        let text_bytes = self.text.as_bytes();
        let mut length_bytes = to_variable_length_bytes(text_bytes.len());

        let mut output = vec![0xFF, 0x07];
        output.extend(length_bytes.iter().copied());
        output.extend(text_bytes.iter().copied());

        output
    }
    fn is_meta(&self) -> bool {
        true
    }
    fn get_eid(&self) -> u8 {
        0x07
    }
    fn set_property(&mut self, argument: u8, bytes: Vec<u8>) {
        self.text = std::str::from_utf8(bytes.as_slice()).unwrap().to_string();
    }
    fn get_property(&self, _: u8) -> Vec<u8> {
        self.text.as_bytes().to_vec()
    }
    fn get_type(&self) -> MIDIEventType {
        MIDIEventType::CuePoint
    }
}

pub struct EndOfTrackEvent { }
impl EndOfTrackEvent {
    pub fn new() -> Box<EndOfTrackEvent> {
        Box::new(EndOfTrackEvent {})
    }
}
impl MIDIEvent for EndOfTrackEvent {
    fn to_bytes(&self) -> Vec<u8> {
        vec![0xFF, 0x2F, 0x00]
    }
    fn is_meta(&self) -> bool {
        true
    }
    fn get_eid(&self) -> u8 {
        0x2F
    }
    fn set_property(&mut self, argument: u8, bytes: Vec<u8>) {
        // non-applicable
    }
    fn get_property(&self, _: u8) -> Vec<u8> {
        Vec::new()
    }
    fn get_type(&self) -> MIDIEventType {
        MIDIEventType::EndOfTrack
    }
}

pub struct ChannelPrefixEvent {
    channel: u8
}
impl MIDIEvent for ChannelPrefixEvent {
    fn to_bytes(&self) -> Vec<u8> {
        vec![0xFF, 0x20, 0x01, self.channel]
    }
    fn is_meta(&self) -> bool {
        true
    }
    fn get_eid(&self) -> u8 {
        0x20
    }
    fn set_property(&mut self, argument: u8, bytes: Vec<u8>) {
        self.channel = bytes[0];
    }
    fn get_property(&self, _: u8) -> Vec<u8> {
        vec![self.channel]
    }
    fn get_type(&self) -> MIDIEventType {
        MIDIEventType::ChannelPrefix
    }
}

pub struct SetTempoEvent {
    // Note: Stored in u32 but is a 3 byte value
    us_per_quarter_note: u32
}
impl SetTempoEvent {
    pub fn new(us_per_quarter_note: u32) -> Box<SetTempoEvent> {
        Box::new(
            SetTempoEvent {
                us_per_quarter_note: us_per_quarter_note
            }
        )
    }
}
impl MIDIEvent for SetTempoEvent {
    fn to_bytes(&self) -> Vec<u8> {
        vec![
            0xFF, 0x51, 0x03,
            ((self.us_per_quarter_note / 256u32.pow(2)) % 256) as u8,
            ((self.us_per_quarter_note / 256u32.pow(1)) % 256) as u8,
            (self.us_per_quarter_note % 256) as u8,
        ]
    }
    fn is_meta(&self) -> bool {
        true
    }
    fn get_eid(&self) -> u8 {
        0x51
    }
    fn set_property(&mut self, argument: u8, bytes: Vec<u8>) {
        self.us_per_quarter_note = (bytes[2] as u32 * 256u32.pow(2)) + (bytes[1] as u32 * 256) + (bytes[0] as u32);
    }
    fn get_property(&self, _: u8) -> Vec<u8> {
        vec![
            ((self.us_per_quarter_note / 256u32.pow(2)) % 256) as u8,
            ((self.us_per_quarter_note / 256u32.pow(1)) % 256) as u8,
            (self.us_per_quarter_note % 256) as u8
        ]
    }
    fn get_type(&self) -> MIDIEventType {
        MIDIEventType::SetTempo
    }
}
//TODO: Figure out what ff/fr are, u16 for now
pub struct SMPTEOffsetEvent {
    hour: u8,
    minute: u8,
    second: u8,
    ff: u8,
    fr: u8
}

impl MIDIEvent for SMPTEOffsetEvent {
    fn to_bytes(&self) -> Vec<u8> {
        vec![0xFF, 0x54, 05, self.hour, self.minute, self.second, self.ff, self.fr]
    }
    fn is_meta(&self) -> bool {
        true
    }
    fn get_eid(&self) -> u8 {
        0x54
    }
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
    fn get_type(&self) -> MIDIEventType {
        MIDIEventType::SMPTEOffset
    }
}

pub struct TimeSignatureEvent {
    numerator: u8,
    denominator: u8,
    clocks_per_metronome: u8,
    thirtysecondths_per_quarter: u8
}

impl TimeSignatureEvent {
    pub fn new(numerator: u8, denominator: u8, cpm: u8, tspq: u8) -> Box<TimeSignatureEvent> {
        Box::new(
            TimeSignatureEvent {
                numerator: numerator,
                denominator: denominator,
                clocks_per_metronome: cpm,
                thirtysecondths_per_quarter: tspq
            }
        )
    }
}
impl MIDIEvent for TimeSignatureEvent {
    fn to_bytes(&self) -> Vec<u8> {
        vec![0xFF, 0x58, 04, self.numerator, self.denominator, self.clocks_per_metronome, self.thirtysecondths_per_quarter]
    }
    fn is_meta(&self) -> bool {
        true
    }
    fn get_eid(&self) -> u8 {
        0x58
    }
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
    fn get_type(&self) -> MIDIEventType {
        MIDIEventType::TimeSignature
    }
}

// TODO: Constructor based on actual name and calculate sf and mn based on that, maybe
pub struct KeySignatureEvent {
    key: String
}
impl KeySignatureEvent {
    fn get_mi_sf(&self) -> (u8, u8) {
        match self.key.as_str() {
            "A" => (0, 3),
            "A#" | "Bb" => (0, 8 | 2),
            "B" => (0, 5),
            "C" => (0, 0),
            "C#" | "Db" => (0, 7),
            "D" => (0, 2),
            "D#" | "Eb" => (0, 8 | 3),
            "E" => (0, 4),
            "F" => (0, 8 | 1),
            "F#" | "Gb" => (0, 6),
            "G" => (0, 1),
            "Am" => (1, 0),
            "A#m" | "Bbm" => (1, 7),
            "Bm" => (1, 2),
            "Cm" => (1, 8 | 3),
            "C#m" | "Dbm" => (1, 4),
            "Dm" => (1, 8 | 1),
            "D#m" | "Ebm" => (1, 6),
            "Em" => (1, 1),
            "Fm" => (1, 8 | 4),
            "F#m" | "Gbm" => (1, 3),
            "Gm" => (1, 8 | 2),
            _ => {
                (0, 0) // Default to C
            }
        }
    }

    fn get_key_from_mi_sf(mi: u8, sf: u8) -> String {
        let map = vec![
            vec![
                "C", "G", "D", "A",
                "E", "B", "F#", "C#",
                "C", "F", "Bb", "Eb",
                "Ab", "Db", "Gb", "Cb"
            ],
            vec![
                "Am", "Em", "Bm", "F#m",
                "C#m", "G#m", "D#m", "A#m",
                "Am", "Dm", "Gm", "Cm",
                "Fm", "Bbm", "Ebm", "Abm"
            ]
        ];

        map[mi as usize][sf as usize].to_string()
    }
}

impl MIDIEvent for KeySignatureEvent {
    fn to_bytes(&self) -> Vec<u8> {
        let (mi, sf) = self.get_mi_sf();
        vec![0xFF, 0x59, 0x02, sf, mi]
    }
    fn is_meta(&self) -> bool {
        true
    }
    fn get_eid(&self) -> u8 {
        0x59
    }
    fn set_property(&mut self, _: u8, bytes: Vec<u8>) {
        self.key = std::str::from_utf8(bytes.as_slice()).unwrap().to_string();
    }
    fn get_property(&self, _: u8) -> Vec<u8> {
       self.key.as_bytes().to_vec()
    }
    fn get_type(&self) -> MIDIEventType {
        MIDIEventType::KeySignature
    }
}

pub struct SequencerSpecificEvent {
    data: Vec<u8>
}

impl MIDIEvent for SequencerSpecificEvent {
    fn to_bytes(&self) -> Vec<u8> {
        let mut length_bytes = to_variable_length_bytes(self.data.len());

        let mut output = vec![0xFF, 0x7F];
        output.extend(length_bytes.iter().copied());
        output.extend(self.data.iter().copied());

        output
    }
    fn is_meta(&self) -> bool {
        true
    }
    fn get_eid(&self) -> u8 {
        0x7F
    }
    fn set_property(&mut self, argument: u8, bytes: Vec<u8>) {
        self.data = bytes.clone();
    }
    fn get_property(&self, _: u8) -> Vec<u8> {
        self.data.clone()
    }
    fn get_type(&self) -> MIDIEventType {
        MIDIEventType::SequencerSpecific
    }
}

// ChannelEvents /////////////////////////
pub struct NoteOnEvent {
    channel: u8,
    note: u8,
    velocity: u8
}

impl NoteOnEvent {
    pub fn new(channel: u8, note: u8, velocity: u8) -> Box<NoteOnEvent> {
        Box::new(
            NoteOnEvent {
                channel: channel,
                note: note,
                velocity: velocity
            }
        )
    }

}
impl MIDIEvent for NoteOnEvent {
    fn to_bytes(&self) -> Vec<u8> {
        vec![
            0x90 | self.channel,
            self.note,
            self.velocity
        ]
    }
    fn is_meta(&self) -> bool {
        false
    }
    fn get_eid(&self) -> u8 {
        0x90
    }

    fn set_property(&mut self, argument: u8, bytes: Vec<u8>) {
        match argument {
            0 => {
                self.channel = bytes[0];
            }
            1 => {
                self.note = bytes[0];
            }
            2 => {
                self.velocity = bytes[0];
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
    fn get_type(&self) -> MIDIEventType {
        MIDIEventType::NoteOn
    }
}

pub struct NoteOffEvent{
	channel: u8,
	note: u8,
	velocity: u8
}
impl NoteOffEvent {
    pub fn new(channel: u8, note: u8, velocity: u8) -> Box<NoteOffEvent> {
        Box::new(
            NoteOffEvent {
                channel: channel,
                note: note,
                velocity: velocity
            }
        )
    }
}
impl MIDIEvent for NoteOffEvent {
    fn to_bytes(&self) -> Vec<u8> {
        vec![
            0x80 | self.channel,
            self.note,
            self.velocity
        ]
    }
    fn is_meta(&self) -> bool {
        false
    }
    fn get_eid(&self) -> u8 {
        0x80
    }
    fn set_property(&mut self, argument: u8, bytes: Vec<u8>) {
        match argument {
            0 => {
                self.channel = bytes[0];
            }
            1 => {
                self.note = bytes[0];
            }
            2 => {
                self.velocity = bytes[0];
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
    fn get_type(&self) -> MIDIEventType {
        MIDIEventType::NoteOff
    }
}

pub struct PolyphonicKeyPressureEvent {
	channel: u8,
	note: u8,
	pressure: u8
}
impl PolyphonicKeyPressureEvent {
    pub fn new(channel: u8, note: u8, pressure: u8) -> Box<PolyphonicKeyPressureEvent> {
        Box::new(
            PolyphonicKeyPressureEvent {
                channel: channel,
                note: note,
                pressure: pressure
            }
        )
    }
}
impl MIDIEvent for PolyphonicKeyPressureEvent {
    fn to_bytes(&self) -> Vec<u8> {
        vec![
            0xA0 | self.channel,
            self.note,
            self.pressure
        ]
    }
    fn is_meta(&self) -> bool {
        false
    }
    fn get_eid(&self) -> u8 {
        0xA0
    }
    fn set_property(&mut self, argument: u8, bytes: Vec<u8>) {
        match argument {
            0 => {
                self.channel = bytes[0];
            }
            1 => {
                self.note = bytes[0];
            }
            2 => {
                self.pressure = bytes[0];
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
    fn get_type(&self) -> MIDIEventType {
        MIDIEventType::PolyphonicKeyPressure
    }
}

pub struct ControlChangeEvent {
	channel: u8,
	controller: u8,
	value: u8
}
impl ControlChangeEvent {
    pub fn new(channel: u8, controller: u8, value:u8) -> Box<ControlChangeEvent> {
        Box::new(
            ControlChangeEvent {
                channel: channel,
                controller: controller,
                value: value
            }
        )
    }
}
impl MIDIEvent for ControlChangeEvent {
    fn to_bytes(&self) -> Vec<u8> {
        vec![
            0xB0 | self.channel,
            self.controller,
            self.value
        ]
    }
    fn is_meta(&self) -> bool {
        false
    }
    fn get_eid(&self) -> u8 {
        0xB0
    }
    fn set_property(&mut self, argument: u8, bytes: Vec<u8>) {
        match argument {
            0 => {
                self.channel = bytes[0];
            }
            1 => {
                self.controller = bytes[0];
            }
            2 => {
                self.value = bytes[0];
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
    fn get_type(&self) -> MIDIEventType {
        MIDIEventType::ControlChange
    }
}

pub struct ProgramChangeEvent {
    channel: u8,
    program: u8,
}
impl ProgramChangeEvent {
    pub fn new(channel: u8, program: u8) -> Box<ProgramChangeEvent> {
        Box::new(
            ProgramChangeEvent {
                channel: channel,
                program: program
            }
        )
    }
}
impl MIDIEvent for ProgramChangeEvent {
    fn to_bytes(&self) -> Vec<u8> {
        vec![
            0xC0 | self.channel,
            self.program
        ]
    }
    fn is_meta(&self) -> bool {
        false
    }
    fn get_eid(&self) -> u8 {
        0xC0
    }
    fn set_property(&mut self, argument: u8, bytes: Vec<u8>) {
        match argument {
            0 => {
                self.channel = bytes[0];
            }
            1 => {
                self.program = bytes[0];
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
    fn get_type(&self) -> MIDIEventType {
        MIDIEventType::ProgramChange
    }
}

pub struct ChannelPressureEvent {
    channel: u8,
    pressure: u8
}
impl ChannelPressureEvent {
    pub fn new(channel: u8, pressure: u8) -> Box<ChannelPressureEvent> {
        Box::new(
            ChannelPressureEvent {
                channel: channel,
                pressure: pressure
            }
        )
    }
}

impl MIDIEvent for ChannelPressureEvent {
    fn to_bytes(&self) -> Vec<u8> {
        vec![
            0xD0 | self.channel,
            self.pressure
        ]
    }
    fn is_meta(&self) -> bool {
        false
    }
    fn get_eid(&self) -> u8 {
        0xD0
    }
    fn set_property(&mut self, argument: u8, bytes: Vec<u8>) {
        match argument {
            0 => {
                self.channel = bytes[0];
            }
            1 => {
                self.pressure = bytes[0];
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
    fn get_type(&self) -> MIDIEventType {
        MIDIEventType::ChannelPressure
    }
}

pub struct PitchWheelChangeEvent {
	channel: u8,
    value: u16
}

impl PitchWheelChangeEvent {
    pub fn new(channel: u8, value: u16) -> Box<PitchWheelChangeEvent> {
        Box::new(
            PitchWheelChangeEvent {
                channel: channel,
                value: value
            }
        )
    }
}

impl MIDIEvent for PitchWheelChangeEvent {
    fn to_bytes(&self) -> Vec<u8> {
        vec![
            0xE0 | self.channel,
            (self.value & 0x7F) as u8,
            ((self.value >> 7) & 0x7F) as u8
        ]
    }
    fn is_meta(&self) -> bool {
        false
    }
    fn get_eid(&self) -> u8 {
        0xE0
    }
    fn set_property(&mut self, argument: u8, bytes: Vec<u8>) {
        match argument {
            0 => {
                self.channel = bytes[0];
            }
            1 => {
                self.value = ((bytes[0] as u16) * 256) + (bytes[1] as u16);
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
                vec![
                    (self.value / 256) as u8,
                    (self.value % 256) as u8
                ]
            }
            _ => {
                vec![0]
            }
        }
    }
    fn get_type(&self) -> MIDIEventType {
        MIDIEventType::PitchWheelChange
    }
}

pub struct SystemExclusiveEvent {
    data: Vec<u8>
}
impl SystemExclusiveEvent {
    pub fn new(data: Vec<u8>) -> Box<SystemExclusiveEvent> {
        Box::new(
            SystemExclusiveEvent {
                data: data.clone()
            }
        )
    }
}

impl MIDIEvent for SystemExclusiveEvent {
    fn to_bytes(&self) -> Vec<u8> {
        let mut output = vec![0xF0];
        output.extend(self.data.iter().copied());
        output.push(0xF7);
        output
    }
    fn is_meta(&self) -> bool {
        false
    }
    fn get_eid(&self) -> u8 {
        0xF0
    }

    fn set_property(&mut self, argument: u8, bytes: Vec<u8>) {
        self.data = bytes.clone()
    }

    fn get_property(&self, argument: u8) -> Vec<u8> {
        self.data.clone()
    }

    fn get_type(&self) -> MIDIEventType {
        MIDIEventType::SystemExclusive
    }
}

pub struct MTCQuarterFrameEvent {
    message_type: u8,
    value: u8
}

impl MIDIEvent for MTCQuarterFrameEvent {
    fn to_bytes(&self) -> Vec<u8> {
        let mut b = 0;
        b |= self.message_type;
        b <<= 3;
        b |= self.value;

        vec![ 0xF1, b ]
    }
    fn is_meta(&self) -> bool {
        false
    }
    fn get_eid(&self) -> u8 {
        0xF1
    }

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
    fn get_type(&self) -> MIDIEventType {
        MIDIEventType::MTCQuarterFrame
    }
}

pub struct SongPositionPointerEvent {
    beat: u16
}
impl MIDIEvent for SongPositionPointerEvent {
    fn to_bytes(&self) -> Vec<u8> {
        vec![
            0xF2,
            (self.beat & 0x7F) as u8,
            ((self.beat >> 7) & 0x7F) as u8
        ]
    }
    fn is_meta(&self) -> bool {
        false
    }
    fn get_eid(&self) -> u8 {
        0xF2
    }

    fn set_property(&mut self, _: u8, bytes: Vec<u8>) {
        self.beat = ((bytes[0] as u16) * 256) + (bytes[1] as u16);
    }

    fn get_property(&self, _: u8) -> Vec<u8> {
        vec![
            (self.beat / 256) as u8,
            (self.beat % 256) as u8
        ]
    }

    fn get_type(&self) -> MIDIEventType {
        MIDIEventType::SongPositionPointer
    }
}

pub struct SongSelectEvent {
    song: u8
}
impl MIDIEvent for SongSelectEvent {
    fn to_bytes(&self) -> Vec<u8> {
        vec![
            0xF3,
            self.song & 0x7F
        ]
    }
    fn is_meta(&self) -> bool {
        false
    }
    fn get_eid(&self) -> u8 {
        0xF3
    }

    fn set_property(&mut self, _: u8, bytes: Vec<u8>) {
        self.song = bytes[0] & 0x7F;
    }

    fn get_property(&self, _: u8) -> Vec<u8> {
        vec![
            self.song & 0x7F
        ]
    }

    fn get_type(&self) -> MIDIEventType {
        MIDIEventType::SongSelect
    }
}

pub struct TuneRequestEvent { }
impl MIDIEvent for TuneRequestEvent {
    fn to_bytes(&self) -> Vec<u8> {
        vec![ 0xF6 ]
    }
    fn is_meta(&self) -> bool {
        false
    }
    fn get_eid(&self) -> u8 {
        0xF6
    }

    fn set_property(&mut self, _: u8, bytes: Vec<u8>) { }
    fn get_property(&self, _: u8) -> Vec<u8> {
        Vec::new()
    }

    fn get_type(&self) -> MIDIEventType {
        MIDIEventType::TuneRequest
    }
}

pub struct MIDIClockEvent { }
impl MIDIEvent for MIDIClockEvent {
    fn to_bytes(&self) -> Vec<u8> {
        vec![ 0xF8 ]
    }
    fn is_meta(&self) -> bool {
        false
    }
    fn get_eid(&self) -> u8 {
        0xF8
    }

    fn set_property(&mut self, _: u8, bytes: Vec<u8>) { }
    fn get_property(&self, _: u8) -> Vec<u8> {
        Vec::new()
    }

    fn get_type(&self) -> MIDIEventType {
        MIDIEventType::MIDIClock
    }
}

pub struct MIDIStartEvent { }
impl MIDIEvent for MIDIStartEvent {
    fn to_bytes(&self) -> Vec<u8> {
        vec![ 0xFA ]
    }
    fn is_meta(&self) -> bool {
        false
    }
    fn get_eid(&self) -> u8 {
        0xFA
    }

    fn set_property(&mut self, _: u8, bytes: Vec<u8>) { }
    fn get_property(&self, _: u8) -> Vec<u8> {
        Vec::new()
    }

    fn get_type(&self) -> MIDIEventType {
        MIDIEventType::MIDIStart
    }
}

pub struct MIDIContinueEvent { }
impl MIDIEvent for MIDIContinueEvent {
    fn to_bytes(&self) -> Vec<u8> {
        vec![ 0xFB ]
    }
    fn is_meta(&self) -> bool {
        false
    }
    fn get_eid(&self) -> u8 {
        0xFB
    }

    fn set_property(&mut self, _: u8, bytes: Vec<u8>) { }
    fn get_property(&self, _: u8) -> Vec<u8> {
        Vec::new()
    }

    fn get_type(&self) -> MIDIEventType {
        MIDIEventType::MIDIContinue
    }
}

pub struct MIDIStopEvent { }
impl MIDIEvent for MIDIStopEvent {
    fn to_bytes(&self) -> Vec<u8> {
        vec![ 0xFC ]
    }
    fn is_meta(&self) -> bool {
        false
    }
    fn get_eid(&self) -> u8 {
        0xFC
    }

    fn set_property(&mut self, _: u8, bytes: Vec<u8>) { }

    fn get_property(&self, _: u8) -> Vec<u8> {
        Vec::new()
    }

    fn get_type(&self) -> MIDIEventType {
        MIDIEventType::MIDIStop
    }
}

pub struct ActiveSenseEvent { }
impl MIDIEvent for ActiveSenseEvent {
    fn to_bytes(&self) -> Vec<u8> {
        vec![ 0xFE ]
    }
    fn is_meta(&self) -> bool {
        false
    }
    fn get_eid(&self) -> u8 {
        0xFE
    }

    fn set_property(&mut self, _: u8, bytes: Vec<u8>) { }

    fn get_property(&self, _: u8) -> Vec<u8> {
        Vec::new()
    }

    fn get_type(&self) -> MIDIEventType {
        MIDIEventType::ActiveSense
    }
}

pub struct ResetEvent { }
impl MIDIEvent for ResetEvent {
    fn to_bytes(&self) -> Vec<u8> {
        vec![ 0xFF ]
    }
    fn is_meta(&self) -> bool {
        false
    }
    fn get_eid(&self) -> u8 {
        0xFF
    }

    fn set_property(&mut self, _: u8, bytes: Vec<u8>) { }

    fn get_property(&self, _: u8) -> Vec<u8> {
        Vec::new()
    }

    fn get_type(&self) -> MIDIEventType {
        MIDIEventType::Reset
    }
}


pub struct MIDITrack {
    ticks: HashMap<usize, Vec<u64>>
}

impl MIDITrack {
    pub fn new() -> MIDITrack {
        MIDITrack {
            ticks: HashMap::new()
        }
    }
    fn get_active_tick_count(&self) -> usize {
        self.ticks.len()
    }
    fn get_active_tick(&self, n: usize) -> usize {
        let mut tick_sorted = Vec::new();
        for k in self.ticks.keys() {
            tick_sorted.push(*k);
        }
        tick_sorted.sort();

        tick_sorted[n]
    }

    fn get_event_count(&self, tick: usize) -> usize {
        match self.ticks.get(&tick) {
            Some(event_list) => {
                event_list.len()
            }
            None => {
                0
            }
        }
    }
    pub fn insert_event(&mut self, tick: usize, event_id: u64) {
        self.ticks.entry(tick)
            .and_modify(|eventlist| {
                (*eventlist).push(event_id)
            })
            .or_insert(vec![event_id]);
    }

    pub fn push_event(&mut self, wait: usize, event_id: u64) {
        let last_tick = self.len() - 1;

        self.ticks.entry(last_tick + wait)
            .and_modify(|eventlist| {
                (*eventlist).push(event_id)
            })
            .or_insert(vec![event_id]);
    }

    // Generate a list of delays, paired with the events to call after each delay (Like a MIDI file)
    fn get_pair_map(&self) -> Vec<(usize, u64)> {
        let mut output = Vec::new();

        let mut ticks_ordered: Vec<usize> = Vec::new();
        for tick in self.ticks.keys() {
            ticks_ordered.push(*tick);
        }
        ticks_ordered.sort();

        let mut event_ids: Vec<u64>;
        let mut previous_tick = 0;
        for tick in ticks_ordered.iter() {
            match self.ticks.get(&tick) {
                Some(eid_list) => {
                    for (i, eid) in eid_list.iter().enumerate() {
                        if i == 0 {
                            output.push((tick - previous_tick, *eid));
                        } else {
                            output.push((0, *eid));
                        }
                    }
                    previous_tick = *tick;
                }
                None => ()
            }
        }

        output
    }

    fn len(&self) -> usize {
        let mut largest_tick = 0;
        for key in self.ticks.keys() {
            largest_tick = max(*key, largest_tick);
        }
        largest_tick + 1
    }

}

pub struct MIDI {
    ppqn: u16,
    midi_format: u16, // 16 because the format stores in 2 bytes, even though it only requires 2 bits (0,1,2)
    tracks: Vec<MIDITrack>,
    events: HashMap<u64, Box<dyn MIDIEvent>>,
    event_id_gen: u64
}


impl MIDI {
    pub fn new() -> MIDI {
        MIDI {
            event_id_gen: 0,
            ppqn: 120,
            midi_format: 1,
            tracks: Vec::new(),
            events: HashMap::new()
        }
    }

    pub fn from_path(file_path: String) -> MIDI {
        let mut midibytes = Vec::new();
        match File::open(file_path) {
            Ok(mut file) => {
                let file_length = match file.metadata() {
                    Ok(meta) => {
                        meta.len()
                    }
                    Err(e) => {
                        0
                    }
                };
                let mut buffer: Vec<u8> = vec![0; file_length as usize];
                file.read(&mut buffer);

                for byte in buffer.iter() {
                    midibytes.push(*byte);
                }
            }
            Err(e) => {}
        }

        MIDI::from_bytes(midibytes)
    }

    fn from_bytes(file_bytes: Vec<u8>) -> MIDI {
        let mut bytes = &mut file_bytes.clone();
        bytes.reverse();
        let mut mlo: MIDI = MIDI::new();
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

        let mut ppqn: u16 = 120;
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
                    ppqn = (divword & 0x7FFF) as u16;
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

    pub fn to_bytes(&self) -> Vec<u8> {
        // First 8  bytes will always be the same
        let mut output: Vec<u8> = vec!['M' as u8, 'T' as u8, 'h' as u8, 'd' as u8, 0, 0, 0, 6];

        let format: u16 = self.get_format();
        output.push((format / 256) as u8);
        output.push((format % 256) as u8);

        let track_count: u16 = self.tracks.len() as u16;
        output.push((track_count / 256) as u8);
        output.push((track_count % 256) as u8);

        let ppqn: u16 = self.get_ppqn();
        output.push((ppqn / 256) as u8);
        output.push((ppqn % 256) as u8);

        // Tracks (MTrk)
        let mut track_event_bytes: Vec<u8>;
        let mut working_event: Box<dyn MIDIEvent>;
        let mut track_byte_length: u32;
        for (i, track) in self.tracks.iter().enumerate() {
            output.push('M' as u8);
            output.push('T' as u8);
            output.push('r' as u8);
            output.push('k' as u8);


            track_event_bytes = Vec::new();
            for (tick_delay, eid) in track.get_pair_map().iter() {
                match self.get_event(*eid) {
                    Some(working_event) => {
                        track_event_bytes.extend(to_variable_length_bytes(*tick_delay).iter().copied());
                        track_event_bytes.extend(working_event.to_bytes());
                    }
                    None => {
                    }
                }
            }

            // Automatically handle EndOfTrackEvent Here instead of requiring it be in the MIDITrack Object
            track_event_bytes.push(0);
            track_event_bytes.extend(EndOfTrackEvent::new().to_bytes().iter().copied());

            // track length in bytes
            track_byte_length = track_event_bytes.len() as u32;
            output.push((track_byte_length / 256_u32.pow(3)) as u8);
            output.push(((track_byte_length / 256_u32.pow(2)) % 256) as u8);
            output.push(((track_byte_length / 256_u32.pow(1)) % 256) as u8);
            output.push((track_byte_length % 256) as u8);

            output.extend(track_event_bytes.iter().copied());
        }

        output
    }

    pub fn save(&self, path: String) {
        let bytes = self.to_bytes();
        match File::create(path) {
            Ok(mut file) => {
                file.write_all(bytes.as_slice());
            }
            Err(e) => {
            }
        }
    }

    pub fn get_track_count(&self) -> usize {
        self.tracks.len()
    }

    fn get_track_length(&self, track: usize) -> usize {
        match self.tracks.get(track) {
            Some(miditrack) => {
                miditrack.len()
            }
            None => {
                0
            }
        }
    }

    fn get_tick_length(&self, track: usize, tick: usize) -> usize {
        let length: usize;
        if tick >= self.get_track_length(track) {
            length = 0;
        } else {
            length = match self.tracks.get(track) {
                Some(miditrack) => {
                    miditrack.get_event_count(tick)
                }
                None => {
                    0
                }
            };
        }

        length
    }


    fn get_nth_event_id_in_tick(&self, track: usize, tick: usize, n: usize) -> Result<u64, u32> {
        match self.tracks.get(track) {
            Some(miditrack) => {
                match miditrack.ticks.get(&tick) {
                    Some(event_id_list) => {
                        match event_id_list.get(n) {
                            Some(eid) => {
                                Ok(*eid)
                            }
                            None => {
                                Err(2)
                            }
                        }
                    }
                    None => {
                        Err(1)
                    }
                }
            }
            None => {
                Err(0)
            }
        }
    }

    fn set_ppqn(&mut self, new_ppqn: u16) {
        self.ppqn = new_ppqn;
    }

    fn get_ppqn(&self) -> u16 {
        self.ppqn
    }

    fn set_format(&mut self, new_format: u16) {
        self.midi_format = new_format;
    }

    fn get_format(&self) -> u16 {
        self.midi_format
    }

    pub fn insert_event(&mut self, track: usize, tick: usize, event: Box<dyn MIDIEvent>) {
        let new_event_id = self.event_id_gen;
        self.events.insert(new_event_id, event);
        while track >= self.tracks.len() {
            self.tracks.push(MIDITrack::new());
        }


        match self.tracks.get_mut(track) {
            Some(miditrack) => {
                miditrack.insert_event(tick, new_event_id)
            }
            None => ()
        }

        self.event_id_gen += 1;
    }

    pub fn push_event(&mut self, track: usize, wait: usize, event: Box<dyn MIDIEvent>) {
        let new_event_id = self.event_id_gen;

        self.events.insert(new_event_id, event);
        while track >= self.tracks.len() {
            self.tracks.push(MIDITrack::new());
        }


        match self.tracks.get_mut(track) {
            Some(miditrack) => {
                miditrack.push_event(wait, new_event_id)
            }
            None => ()
        }

        self.event_id_gen += 1;
    }

    pub fn get_event(&self, event_id: u64) -> Option<&Box<dyn MIDIEvent>> {
        match self.events.get(&event_id) {
            Some(event) => {
                Some(event)
            }
            None => {
                None
            }
        }
    }

    pub fn get_event_property(&self, event_id: u64, arg: u8) -> Vec<u8> {
        match self.get_event(event_id) {
            Some(event) => {
                event.get_property(arg)
            }
            None => {
                Vec::new()
            }
        }

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

fn to_variable_length_bytes(number: usize) -> Vec<u8> {
    let mut output = Vec::new();
    let mut first_pass = true;
    let mut working_number = number;
    let mut tmp;
    while working_number > 0 || first_pass {
        tmp = working_number & 0x7F;
        working_number >>= 7;
        if first_pass {
            tmp |= 0x00;
        } else {
            tmp |= 0x80;
        }
        output.push(tmp as u8);
        first_pass = false;
    }
    output.reverse();

    output
}


fn process_mtrk_event(leadbyte: u8, bytes: &mut Vec<u8>, current_deltatime: &mut usize, mlo: &mut MIDI, track: usize, fallback_cmd: &mut u8) {
    let mut channel: u8;

    let mut a: u8;
    let mut b: u8;
    let mut c: u8;

    let mut n: u32;
    let mut varlength: u64;

    let mut dump: Vec<u8>;

    let mut leadnibble: u8 = leadbyte >> 4;

    match leadbyte {
        0..=7 => {
             // Implicitly a Channel Event
            bytes.push(leadbyte);
            process_mtrk_event(*fallback_cmd, bytes, current_deltatime, mlo, track, fallback_cmd);
        }
        8..=14 => {
            match leadnibble {
                8 => {
                    channel = leadbyte & 0x0F;
                    b = bytes.pop().unwrap();
                    c = bytes.pop().unwrap();
                    mlo.insert_event(track, *current_deltatime, NoteOffEvent::new(channel, b, c));
                }
                9 => {
                    channel = leadbyte & 0x0F;
                    b = bytes.pop().unwrap();
                    c = bytes.pop().unwrap();
                    // Convert fake NoteOff (NoteOn where velocity is 0) to real NoteOff
                    if c == 0 {
                        mlo.insert_event(track, *current_deltatime, NoteOffEvent::new(channel, b, c));
                    } else {
                        mlo.insert_event(track, *current_deltatime, NoteOnEvent::new(channel, b, c));
                    }
                }
                10 => {
                    channel = leadbyte & 0x0F;
                    b = bytes.pop().unwrap();
                    c = bytes.pop().unwrap();
                    mlo.insert_event(track, *current_deltatime, PolyphonicKeyPressureEvent::new(channel, b, c));
                }
                11 => {
                    channel = leadbyte & 0x0F;
                    b = bytes.pop().unwrap();
                    c = bytes.pop().unwrap();
                    mlo.insert_event(track, *current_deltatime, ControlChangeEvent::new(channel, b, c));
                }
                12 => {
                    channel = leadbyte & 0x0F;
                    b = bytes.pop().unwrap();
                    mlo.insert_event(track, *current_deltatime, ProgramChangeEvent::new(channel, b));
                }
                13 => {
                    channel = leadbyte & 0x0F;
                    b = bytes.pop().unwrap();
                    mlo.insert_event(track, *current_deltatime, ChannelPressureEvent::new(channel, b));
                }
                14 => {
                    channel = leadbyte & 0x0F;
                    b = bytes.pop().unwrap();
                    c = bytes.pop().unwrap();
                    mlo.insert_event(track, *current_deltatime, PitchWheelChangeEvent::new(channel, (c << 7 + b) as u16));
                }
                _ => {
                    //undefined behavior
                }
            }
        }
        0xF0 => {
            // System Exclusive
            dump = Vec::new();
            loop {
                match bytes.pop() {
                    Some(byte) => {
                        if byte == 0xF7 {
                            break;
                        } else {
                            dump.push(byte);
                        }
                    },
                    None => {
                        break;
                    }
                }
            }

            mlo.insert_event(track, *current_deltatime, SystemExclusiveEvent::new(dump.clone()));
        }
        0xF2 => {
            // Song Position Pointer
            b = bytes.pop().unwrap();
            c = bytes.pop().unwrap();

            let beat = (c as u16 << 7) + (b as u16);
            mlo.insert_event(track, *current_deltatime, SongPositionPointerEvent::new(beat));
        }
        0xF3 => {
            b = bytes.pop().unwrap();
            mlo.insert_event(track, *current_deltatime, SongSelectEvent::new(b & 0x7F));
        }
        0xF6 | 0xF8 | 0xFA | 0xFB | 0xFC | 0xFE => {
            // Do Nothing. These are system-realtime and shouldn't be in a file.
        }
        0xF7 => {
            varlength = get_variable_length_number(bytes);
            n = pop_n(bytes, varlength as usize);
            // TODO ADD EVENT
        }
        0xF1 | 0xF4 | 0xF5 => {
            // Undefined Behaviour
        }
        0xFF => {
            a = bytes.pop().unwrap(); // Meta Type
            varlength = get_variable_length_number(bytes);
            if (a == 0x51) {
                mlo.insert_event(track, *current_deltatime, SetTempoEvent::new(pop_n(bytes, varlength as usize)));
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
                match a {
                    0x02 => {
                        mlo.insert_event(track, *current_deltatime, CopyRightNoticeEvent::new(dump));
                    }
                    0x03 => {
                        mlo.insert_event(track, *current_deltatime, TrackNameEvent::new(dump));
                    }
                    0x04 => {
                        mlo.insert_event(track, *current_deltatime, InstrumentNameEvent::new(dump));
                    }
                    0x05 => {
                        mlo.insert_event(track, *current_deltatime, LyricEvent::new(dump));
                    }
                    0x06 => {
                        mlo.insert_event(track, *current_deltatime, MarkerEvent::new(dump));
                    }
                    0x07 => {
                        mlo.insert_event(track, *current_deltatime, CuePointEvent::new(dump));
                    }
                    0x20 => {
                        mlo.insert_event(track, *current_deltatime, ChannelPrefixEvent::new(dump[0]));
                    }
                    0x2F => {
                        mlo.insert_event(track, *current_deltatime, EndOfTrackEvent::new() );
                    }
                    0x51 => {
                    }
                    0x54 => {
                        mlo.insert_event(track, *current_deltatime, SMPTEOffsetEvent::new(dump[0], dump[1], dump[2], dump[3], dump[4]));
                    }
                    0x58 => {
                        mlo.insert_event(track, *current_deltatime, TimeSignatureEvent::new(dump[0], dump[1],dump[2], dump[3]));
                    }
                    0x59 => {
                        mlo.insert_event(track, *current_deltatime, KeySignatureEvent::new(dump[0], dump[1]));
                    }
                    0x7F => {
                        mlo.insert_event(track, *current_deltatime, SequencerSpecifcEvent::new(dump));
                    }
                    _ => {
                    }
                }
            }
        }
        _ => {
            // Undefined Behaviour
        }
    }

    if leadnibble >= 8 && leadnibble < 15 {
        *fallback_cmd = leadbyte.clone();
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
pub extern fn get_track_length(midi_ptr: *mut MIDI, track: usize) -> usize {
    let mut midi = unsafe { Box::from_raw(midi_ptr) };
    let length = midi.get_track_length(track);

    Box::into_raw(midi);

    length
}

#[no_mangle]
pub extern fn get_track_count(midi_ptr: *mut MIDI) -> usize {

    let mut midi = unsafe { Box::from_raw(midi_ptr) };

    let count = midi.tracks.len();

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

    match midi.events.get_mut(&event_id) {
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
    match midi.events.get(&event_id) {
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
    let output = match midi.tracks.get(track) {
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

    let output = match midi.tracks.get(track) {
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
