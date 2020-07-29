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

// For python Bindings
enum MIDIEventType {
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
    PitchWheelChange = 21
}

trait MIDIEvent {
    fn get_bytes(&self) -> Vec<u8>;
    fn get_eid(&self) -> u8;
    fn is_meta(&self) -> bool;
    fn set_property(&mut self, argument: u8, bytes: Vec<u8>);
    fn get_property(&self, argument: u8) -> Vec<u8>;
    fn get_type(&self) -> MIDIEventType;
}

impl MIDIEvent { }

struct SequenceNumberEvent {
    sequence: u16
}

impl MIDIEvent for SequenceNumberEvent {
    fn get_bytes(&self) -> Vec<u8> {
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

struct TextEvent {
    text: String
}
impl MIDIEvent for TextEvent {
    fn get_bytes(&self) -> Vec<u8> {
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
struct CopyRightNoticeEvent {
    text: String
}
impl MIDIEvent for CopyRightNoticeEvent {
    fn get_bytes(&self) -> Vec<u8> {
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
struct TrackNameEvent {
    track_name: String
}
impl MIDIEvent for TrackNameEvent {
    fn get_bytes(&self) -> Vec<u8> {
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
struct InstrumentNameEvent {
    instrument_name: String
}
impl MIDIEvent for InstrumentNameEvent {
    fn get_bytes(&self) -> Vec<u8> {
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
struct LyricEvent {
    lyric: String
}
impl MIDIEvent for LyricEvent {
    fn get_bytes(&self) -> Vec<u8> {
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
struct MarkerEvent {
    text: String
}
impl MIDIEvent for MarkerEvent {
    fn get_bytes(&self) -> Vec<u8> {
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
struct CuePointEvent {
    text: String
}
impl MIDIEvent for CuePointEvent {
    fn get_bytes(&self) -> Vec<u8> {
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

struct EndOfTrackEvent { }
impl MIDIEvent for EndOfTrackEvent {
    fn get_bytes(&self) -> Vec<u8> {
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

struct ChannelPrefixEvent {
    channel: u8
}
impl MIDIEvent for ChannelPrefixEvent {
    fn get_bytes(&self) -> Vec<u8> {
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

struct SetTempoEvent {
    // Note: Stored in u32 but is a 3 byte value
    us_per_quarter_note: u32
}
impl SetTempoEvent {
    fn new(us_per_quarter_note: u32) -> SetTempoEvent {
        SetTempoEvent {
            us_per_quarter_note: us_per_quarter_note
        }
    }
    fn newbox(us_per_quarter_note: u32) -> Box<SetTempoEvent> {
        Box::new(SetTempoEvent::new(us_per_quarter_note))
    }
}
impl MIDIEvent for SetTempoEvent {
    fn get_bytes(&self) -> Vec<u8> {
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
struct SMPTEOffsetEvent {
    hour: u8,
    minute: u8,
    second: u8,
    ff: u8,
    fr: u8
}
impl MIDIEvent for SMPTEOffsetEvent {
    fn get_bytes(&self) -> Vec<u8> {
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

struct TimeSignatureEvent {
    numerator: u8,
    denominator: u8,
    clocks_per_metronome: u8,
    thirtysecondths_per_quarter: u8
}
impl TimeSignatureEvent {
    fn new(numerator: u8, denominator: u8, cpm: u8, tspq: u8) -> TimeSignatureEvent {
        TimeSignatureEvent {
            numerator: numerator,
            denominator: denominator,
            clocks_per_metronome: cpm,
            thirtysecondths_per_quarter: tspq
        }
    }

    fn newbox(numerator: u8, denominator: u8, cpm: u8, tspq: u8) -> Box<TimeSignatureEvent> {
        Box::new(TimeSignatureEvent::new(numerator, denominator, cpm, tspq))
    }
}
impl MIDIEvent for TimeSignatureEvent {
    fn get_bytes(&self) -> Vec<u8> {
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
struct KeySignatureEvent {
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
    fn get_bytes(&self) -> Vec<u8> {
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

struct SequencerSpecificEvent {
    data: Vec<u8>
}
impl MIDIEvent for SequencerSpecificEvent {
    fn get_bytes(&self) -> Vec<u8> {
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
struct NoteOnEvent {
    channel: u8,
    note: u8,
    velocity: u8
}

impl NoteOnEvent {
    fn new(channel: u8, note: u8, velocity: u8) -> NoteOnEvent {
        NoteOnEvent {
            channel: channel,
            note: note,
            velocity: velocity
        }
    }

    fn newbox(channel: u8, note: u8, velocity: u8) -> Box<NoteOnEvent> {
        Box::new(NoteOnEvent::new(channel, note, velocity))
    }
}
impl MIDIEvent for NoteOnEvent {
    fn get_bytes(&self) -> Vec<u8> {
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

struct NoteOffEvent{
	channel: u8,
	note: u8,
	velocity: u8
}
impl NoteOffEvent {
    fn new(channel: u8, note: u8, velocity: u8) -> NoteOffEvent {
        NoteOffEvent {
            channel: channel,
            note: note,
            velocity: velocity
        }
    }
    fn newbox(channel: u8, note: u8, velocity: u8) -> Box<NoteOffEvent> {
        Box::new(NoteOffEvent::new(channel, note, velocity))
    }
}
impl MIDIEvent for NoteOffEvent {
    fn get_bytes(&self) -> Vec<u8> {
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

struct PolyphonicKeyPressureEvent {
	channel: u8,
	note: u8,
	pressure: u8
}
impl PolyphonicKeyPressureEvent {
    fn new(channel: u8, note: u8, pressure: u8) -> PolyphonicKeyPressureEvent {
        PolyphonicKeyPressureEvent {
            channel: channel,
            note: note,
            pressure: pressure
        }
    }
    fn newbox(channel: u8, note: u8, pressure: u8) -> Box<PolyphonicKeyPressureEvent> {
        Box::new(PolyphonicKeyPressureEvent::new(channel, note, pressure))
    }
}
impl MIDIEvent for PolyphonicKeyPressureEvent {
    fn get_bytes(&self) -> Vec<u8> {
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

struct ControlChangeEvent {
	channel: u8,
	controller: u8,
	value: u8
}
impl ControlChangeEvent {
    fn new(channel: u8, controller: u8, value:u8) -> ControlChangeEvent {
        ControlChangeEvent {
            channel: channel,
            controller: controller,
            value: value
        }
    }
    fn newbox(channel: u8, controller: u8, value: u8) -> Box<ControlChangeEvent> {
        Box::new(ControlChangeEvent::new(channel, controller, value))
    }
}
impl MIDIEvent for ControlChangeEvent {
    fn get_bytes(&self) -> Vec<u8> {
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

struct ProgramChangeEvent {
    channel: u8,
    program: u8,
}
impl ProgramChangeEvent {
    fn new(channel: u8, program: u8) -> ProgramChangeEvent {
        ProgramChangeEvent {
            channel: channel,
            program: program
        }
    }
    fn newbox(channel: u8, program: u8) -> Box<ProgramChangeEvent> {
        Box::new(ProgramChangeEvent::new(channel, program))
    }
}
impl MIDIEvent for ProgramChangeEvent {
    fn get_bytes(&self) -> Vec<u8> {
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

struct ChannelPressureEvent {
    channel: u8,
    pressure: u8
}
impl ChannelPressureEvent {
    fn new(channel: u8, pressure: u8) -> ChannelPressureEvent {
        ChannelPressureEvent {
            channel: channel,
            pressure: pressure
        }
    }
    fn newbox(channel: u8, pressure: u8) -> Box<ChannelPressureEvent> {
        Box::new(ChannelPressureEvent::new(channel, pressure))
    }
}

impl MIDIEvent for ChannelPressureEvent {
    fn get_bytes(&self) -> Vec<u8> {
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

struct PitchWheelChangeEvent {
	channel: u8,
	least: u8,
	most: u8
}

impl PitchWheelChangeEvent {
    fn new(channel: u8, least: u8, most: u8) -> PitchWheelChangeEvent {
        PitchWheelChangeEvent {
            channel: channel,
            least: least,
            most: most
        }
    }
    fn newbox(channel: u8, least: u8, most: u8) -> Box<PitchWheelChangeEvent> {
        Box::new(PitchWheelChangeEvent::new(channel, least, most))
    }
}

impl MIDIEvent for PitchWheelChangeEvent {
    fn get_bytes(&self) -> Vec<u8> {
        vec![
            0xE0 | self.channel,
            self.least,
            self.most
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
                self.least = bytes[0];
            }
            2 => {
                self.most = bytes[0];
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
                self.least
            }
            2 => {
                self.most
            }
            _ => {
                0
            }
        };

        vec![output]
    }
    fn get_type(&self) -> MIDIEventType {
        MIDIEventType::PitchWheelChange
    }
}

pub struct MIDILike {
    ppqn: u32,
    midi_format: u16, // 16 because the format stores in 2 bytes, even though it only requires 2 bits (0,1,2)
    tracks: Vec<HashMap<usize, Vec<u64>>>, // Outer Vector is list of track, not every tick in a track has an event, some have many
    events: HashMap<u64, Box<dyn MIDIEvent>>,
    event_id_gen: u64
}

impl MIDILike {
    pub fn new() -> MIDILike {
        MIDILike {
            event_id_gen: 0,
            ppqn: 120,
            midi_format: 1,
            tracks: Vec::new(),
            events: HashMap::new()
        }
    }

    pub fn from_path(file_path: String) -> MIDILike {
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

        MIDILike::from_bytes(midibytes)
    }

    fn from_bytes(file_bytes: Vec<u8>) -> MIDILike {
        let mut bytes = &mut file_bytes.clone();
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

    pub fn get_track_count(&self) -> usize {
        self.tracks.len()
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

    pub fn get_active_tick_count(&self, track: usize) -> usize {
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

    fn add_event(&mut self, track: usize, tick: usize, event: Box<dyn MIDIEvent>) {
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
        println!("AAA");
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
            mlo.add_event(track, *current_deltatime, NoteOffEvent::newbox(channel, b, c));
        } else if leadnibble == 9 {
            mlo.add_event(track, *current_deltatime, NoteOnEvent::newbox(channel, b, c));
        } else if leadnibble == 10 {
            mlo.add_event(track, *current_deltatime, PolyphonicKeyPressureEvent::newbox(channel, b, c));

        } else if leadnibble == 11 {
            mlo.add_event(track, *current_deltatime, ControlChangeEvent::newbox(channel, b, c));
        } else if leadnibble == 14 {
            mlo.add_event(track, *current_deltatime, PitchWheelChangeEvent::newbox(channel, b, c));
        }

    } else if leadnibble == 12 || leadnibble == 13 {
    // ProgramChange/ChannelPressure
        channel = leadbyte & 0x0F;
        b = bytes.pop().unwrap();
        if leadnibble == 12 {
            mlo.add_event(track, *current_deltatime, ProgramChangeEvent::newbox(channel, b));
        } else if leadnibble == 13 {
            mlo.add_event(track, *current_deltatime, ChannelPressureEvent::newbox(channel, b));
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
            mlo.add_event(track, *current_deltatime, SetTempoEvent::newbox(pop_n(bytes, varlength as usize)));
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
                mlo.add_event(track, *current_deltatime, Box::new(EndOfTrackEvent {}) );
            } else if a == 0x51 {
            } else if a == 0x58 {
                mlo.add_event(track, *current_deltatime, TimeSignatureEvent::newbox(dump[0], dump[1],dump[2], dump[3]));
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

#[no_mangle]
pub extern fn interpret(path: *const c_char) -> *mut MIDILike {
    let cstr_path = unsafe {
        CStr::from_ptr(path)
    };

    let mut bytes: Vec<u8> = Vec::new();
    let clean_path = cstr_path.to_str().expect("Not a valid UTF-8 string");
    let midilike = MIDILike::from_path(clean_path.to_string());
   Box::into_raw(Box::new( midilike ))
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

#[no_mangle]
pub extern fn set_event_property(midilike_ptr: *mut MIDILike, event_id: u64, argument: u8, value: *const c_char) {
    let mut midilike = unsafe { Box::from_raw(midilike_ptr) };

    let cstr_value = unsafe {
        CStr::from_ptr(value)
    };
    let value_vector = cstr_value.to_bytes().to_vec();

    match midilike.events.get_mut(&event_id) {
        Some(midievent) => {
            midievent.set_property(argument, value_vector.clone());
        }
        None => ()
    };

    Box::into_raw(midilike);
}

#[no_mangle]
pub extern fn get_event_property(midilike_ptr: *mut MIDILike, event_id: u64, argument: u8) -> Vec<u8> {
    println!("SS");
    let mut midilike = unsafe { Box::from_raw(midilike_ptr) };
    println!("SSxx");
    let mut value = midilike.get_event_property(event_id, argument);
    println!("---");

    Box::into_raw(midilike);

    Vec::new()
}

#[no_mangle]
pub extern fn get_event_type(midilike_ptr: *mut MIDILike, event_id: u64) -> u32 {
    let mut midilike = unsafe { Box::from_raw(midilike_ptr) };

    let mut output = 0;
    match midilike.events.get_mut(&event_id) {
        Some(midievent) => {
            output = midievent.get_type() as u32;
        }
        None => ()
    };

    Box::into_raw(midilike);

    output
}

