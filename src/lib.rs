use std::fs::File;
use std::io::prelude::*;
use std::cmp::{max, min};
use std::fmt;
use std::mem;
use std::collections::{HashMap, HashSet};

// For python Bindings
#[derive(PartialEq, Eq, Debug)]
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
    AfterTouch = 17,
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
    Reset = 33,

    BankSelect = 34,
    ModulationWheel = 35,
    BreathController = 36,
    FootPedal = 37,
    PortamentoTime = 38,
    DataEntrySlider = 39,
    Volume = 40,
    Balance = 41,
    Pan = 42,
    Expression = 43,
    EffectControl = 44,
    Slider = 45,
    HoldPedal = 46,
    Portamento = 47,
    Sustenuto = 48,
    SoftPedal = 49,
    Legato = 50,
    Hold2Pedal = 51,
    SoundVariation = 52,
    SoundTimbre = 53,
    SoundReleaseTime = 54,
    SoundAttack = 55,
    SoundBrightness = 56,
    SoundControl = 57,
    GeneralButtonOn = 58,
    GeneralButtonOff = 59,
    EffectsLevel = 60,
    TremuloLevel = 61,
    ChorusLevel = 62,
    CelesteLevel = 63,
    PhaserLevel = 64,
    DataButtonIncrement = 65,
    DataButtonDecrement = 66,
    RegisteredParameterNumber = 67,
    NonRegisteredParameterNumber = 68,
    AllControllersOff = 69,
    LocalKeyboardEnable = 70,
    LocalKeyboardDisable = 71,
    AllNotesOff = 72,
    AllSoundOff = 73,
    OmniOff = 74,
    OmniOn = 75,
    MonophonicOperation = 76,
    PolyphonicOperation = 77
}
#[derive(PartialEq, Eq)]
pub enum MIDICategory {
    Meta,
    ControlChange,
    Voice,
    RealTime
}
impl fmt::Debug for MIDICategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            MIDICategory::Meta => { "Meta" }
            MIDICategory::ControlChange => { "ControlChange" }
            MIDICategory::Voice => { "Voice" }
            MIDICategory::RealTime => { "RealTime" }
            _ => { "Unknown" }
        };
        write!(f, "{} Event", name)
    }
}
pub trait MIDIEvent {
    fn as_bytes(&self) -> Vec<u8>;
    fn get_category(&self) -> MIDICategory;

    //For FFI bindings
    fn set_property(&mut self, argument: u8, bytes: Vec<u8>);
    fn get_property(&self, argument: u8) -> Vec<u8>;
    fn get_type(&self) -> MIDIEventType;
}

pub struct SequenceNumberEvent {
    sequence: u16
}

impl SequenceNumberEvent {
    pub fn new(sequence: u16) -> Box<SequenceNumberEvent> {
        Box::new(
            SequenceNumberEvent {
                sequence: sequence
            }
        )
    }

    pub fn set_sequence(&mut self, new_sequence: u16) {
        self.sequence = new_sequence;
    }
}
impl MIDIEvent for SequenceNumberEvent {
    fn as_bytes(&self) -> Vec<u8> {
        vec![
            0xFF, 0x00, 0x02,
            (self.sequence / 256) as u8,
            (self.sequence % 256) as u8
        ]
    }
    fn get_category(&self) -> MIDICategory {
        MIDICategory::Meta
    }
    fn set_property(&mut self, _argument: u8, bytes: Vec<u8>) {
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
impl TextEvent {
    pub fn new(text: String) -> Box<TextEvent> {
        Box::new(
            TextEvent {
                text: text
            }
        )
    }
}

impl MIDIEvent for TextEvent {
    fn as_bytes(&self) -> Vec<u8> {
        let text_bytes = self.text.as_bytes();
        let mut length_bytes = to_variable_length_bytes(text_bytes.len());

        let mut output = vec![0xFF, 0x01];
        output.extend(length_bytes.iter().copied());
        output.extend(text_bytes.iter().copied());

        output
    }
    fn get_category(&self) -> MIDICategory {
        MIDICategory::Meta
    }
    fn set_property(&mut self, _argument: u8, bytes: Vec<u8>) {
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
impl CopyRightNoticeEvent {
    pub fn new(text: String) -> Box<CopyRightNoticeEvent> {
        Box::new(
            CopyRightNoticeEvent {
                text: text
            }
        )
    }
}
impl MIDIEvent for CopyRightNoticeEvent {
    fn as_bytes(&self) -> Vec<u8> {
        let text_bytes = self.text.as_bytes();
        let mut length_bytes = to_variable_length_bytes(text_bytes.len());

        let mut output = vec![0xFF, 0x02];
        output.extend(length_bytes.iter().copied());
        output.extend(text_bytes.iter().copied());

        output
    }

    fn get_category(&self) -> MIDICategory {
        MIDICategory::Meta
    }

    fn set_property(&mut self, _argument: u8, bytes: Vec<u8>) {
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
impl TrackNameEvent {
    pub fn new(track_name: String) -> Box<TrackNameEvent> {
        Box::new(
            TrackNameEvent {
                track_name: track_name
            }
        )
    }
}
impl MIDIEvent for TrackNameEvent {
    fn as_bytes(&self) -> Vec<u8> {
        let text_bytes = self.track_name.as_bytes();
        let mut length_bytes = to_variable_length_bytes(text_bytes.len());

        let mut output = vec![0xFF, 0x03];
        output.extend(length_bytes.iter().copied());
        output.extend(text_bytes.iter().copied());

        output
    }
    fn get_category(&self) -> MIDICategory {
        MIDICategory::Meta
    }
    fn set_property(&mut self, _argument: u8, bytes: Vec<u8>) {
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

impl InstrumentNameEvent {
    pub fn new(instrument_name: String) -> Box<InstrumentNameEvent> {
        Box::new(
            InstrumentNameEvent {
                instrument_name: instrument_name
            }
        )
    }
}

impl MIDIEvent for InstrumentNameEvent {
    fn as_bytes(&self) -> Vec<u8> {
        let text_bytes = self.instrument_name.as_bytes();
        let mut length_bytes = to_variable_length_bytes(text_bytes.len());

        let mut output = vec![0xFF, 0x04];
        output.extend(length_bytes.iter().copied());
        output.extend(text_bytes.iter().copied());

        output
    }
    fn get_category(&self) -> MIDICategory {
        MIDICategory::Meta
    }
    fn set_property(&mut self, _argument: u8, bytes: Vec<u8>) {
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
impl LyricEvent {
    pub fn new(lyric: String) -> Box<LyricEvent> {
        Box::new(
            LyricEvent {
                lyric: lyric
            }
        )
    }
}
impl MIDIEvent for LyricEvent {
    fn as_bytes(&self) -> Vec<u8> {
        let text_bytes = self.lyric.as_bytes();
        let mut length_bytes = to_variable_length_bytes(text_bytes.len());

        let mut output = vec![0xFF, 0x05];
        output.extend(length_bytes.iter().copied());
        output.extend(text_bytes.iter().copied());

        output
    }
    fn get_category(&self) -> MIDICategory {
        MIDICategory::Meta
    }
    fn set_property(&mut self, _argument: u8, bytes: Vec<u8>) {
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
impl MarkerEvent {
    pub fn new(text: String) -> Box<MarkerEvent> {
        Box::new(
            MarkerEvent {
                text: text
            }
        )
    }
}
impl MIDIEvent for MarkerEvent {
    fn as_bytes(&self) -> Vec<u8> {
        let text_bytes = self.text.as_bytes();
        let mut length_bytes = to_variable_length_bytes(text_bytes.len());

        let mut output = vec![0xFF, 0x06];
        output.extend(length_bytes.iter().copied());
        output.extend(text_bytes.iter().copied());

        output
    }
    fn get_category(&self) -> MIDICategory {
        MIDICategory::Meta
    }
    fn set_property(&mut self, _argument: u8, bytes: Vec<u8>) {
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
impl CuePointEvent {
    pub fn new(text: String) -> Box<CuePointEvent> {
        Box::new(
            CuePointEvent {
                text: text
            }
        )
    }
}
impl MIDIEvent for CuePointEvent {
    fn as_bytes(&self) -> Vec<u8> {
        let text_bytes = self.text.as_bytes();
        let mut length_bytes = to_variable_length_bytes(text_bytes.len());

        let mut output = vec![0xFF, 0x07];
        output.extend(length_bytes.iter().copied());
        output.extend(text_bytes.iter().copied());

        output
    }
    fn get_category(&self) -> MIDICategory {
        MIDICategory::Meta
    }
    fn set_property(&mut self, _argument: u8, bytes: Vec<u8>) {
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
    fn as_bytes(&self) -> Vec<u8> {
        vec![0xFF, 0x2F, 0x00]
    }
    fn get_category(&self) -> MIDICategory {
        MIDICategory::Meta
    }
    fn set_property(&mut self, _argument: u8, _bytes: Vec<u8>) {
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
impl ChannelPrefixEvent {
    pub fn new(channel: u8) -> Box<ChannelPrefixEvent> {
        Box::new(
            ChannelPrefixEvent {
                channel: channel
            }
        )
    }
    pub fn set_channel(&mut self, new_channel: u8) {
        self.channel = new_channel;
    }
}
impl MIDIEvent for ChannelPrefixEvent {
    fn as_bytes(&self) -> Vec<u8> {
        vec![0xFF, 0x20, 0x01, self.channel]
    }
    fn get_category(&self) -> MIDICategory {
        MIDICategory::Meta
    }
    fn set_property(&mut self, _argument: u8, bytes: Vec<u8>) {
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
                us_per_quarter_note: min(us_per_quarter_note, 0x00FFFFFF)
            }
        )
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
    fn as_bytes(&self) -> Vec<u8> {
        vec![
            0xFF, 0x51, 0x03,
            ((self.us_per_quarter_note / 256u32.pow(2)) % 256) as u8,
            ((self.us_per_quarter_note / 256u32.pow(1)) % 256) as u8,
            (self.us_per_quarter_note % 256) as u8,
        ]
    }
    fn get_category(&self) -> MIDICategory {
        MIDICategory::Meta
    }
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
impl SMPTEOffsetEvent {
    pub fn new(hour: u8, minute: u8, second: u8, ff: u8, fr: u8) -> Box<SMPTEOffsetEvent> {
        Box::new(
            SMPTEOffsetEvent {
                hour: hour,
                minute: minute,
                second: second,
                ff: ff,
                fr: fr
            }
        )
    }
}

impl MIDIEvent for SMPTEOffsetEvent {
    fn as_bytes(&self) -> Vec<u8> {
        vec![0xFF, 0x54, 05, self.hour, self.minute, self.second, self.ff, self.fr]
    }
    fn get_category(&self) -> MIDICategory {
        MIDICategory::Meta
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
    fn as_bytes(&self) -> Vec<u8> {
        vec![0xFF, 0x58, 04, self.numerator, self.denominator, self.clocks_per_metronome, self.thirtysecondths_per_quarter]
    }
    fn get_category(&self) -> MIDICategory {
        MIDICategory::Meta
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
    fn from_mi_sf(mi: u8, sf: u8) -> Box<KeySignatureEvent> {
        let key = KeySignatureEvent::get_key_from_mi_sf(mi, sf);
        KeySignatureEvent::new(key)
    }

    fn new(key: String) -> Box<KeySignatureEvent> {
        Box::new(
            KeySignatureEvent {
                key: key
            }
        )
    }

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
    fn as_bytes(&self) -> Vec<u8> {
        let (mi, sf) = self.get_mi_sf();
        vec![0xFF, 0x59, 0x02, sf, mi]
    }
    fn get_category(&self) -> MIDICategory {
        MIDICategory::Meta
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

impl SequencerSpecificEvent {
    fn new(data: Vec<u8>) -> Box<SequencerSpecificEvent> {
        Box::new(
            SequencerSpecificEvent {
                data: data.clone()
            }
        )
    }
}

impl MIDIEvent for SequencerSpecificEvent {
    fn as_bytes(&self) -> Vec<u8> {
        let mut length_bytes = to_variable_length_bytes(self.data.len());

        let mut output = vec![0xFF, 0x7F];
        output.extend(length_bytes.iter().copied());
        output.extend(self.data.iter().copied());

        output
    }
    fn get_category(&self) -> MIDICategory {
        MIDICategory::Meta
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
                channel: channel & 0x0F,
                note: note & 0x7F,
                velocity: velocity & 0x7F
            }
        )
    }

}
impl MIDIEvent for NoteOnEvent {
    fn as_bytes(&self) -> Vec<u8> {
        vec![
            0x90 | self.channel,
            self.note,
            self.velocity
        ]
    }
    fn get_category(&self) -> MIDICategory {
        MIDICategory::Voice
    }

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
                channel: channel & 0x0F,
                note: note & 0x7F,
                velocity: velocity & 0x7F
            }
        )
    }
}
impl MIDIEvent for NoteOffEvent {
    fn as_bytes(&self) -> Vec<u8> {
        vec![
            0x80 | self.channel,
            self.note,
            self.velocity
        ]
    }
    fn get_category(&self) -> MIDICategory {
        MIDICategory::Voice
    }
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
    fn get_type(&self) -> MIDIEventType {
        MIDIEventType::NoteOff
    }
}

pub struct AfterTouchEvent {
	channel: u8,
	note: u8,
	pressure: u8
}
impl AfterTouchEvent {
    pub fn new(channel: u8, note: u8, pressure: u8) -> Box<AfterTouchEvent> {
        Box::new(
            AfterTouchEvent {
                channel: channel & 0x0F,
                note: note & 0x7F,
                pressure: pressure & 0x7F
            }
        )
    }
}
impl MIDIEvent for AfterTouchEvent {
    fn as_bytes(&self) -> Vec<u8> {
        vec![
            0xA0 | self.channel,
            self.note,
            self.pressure
        ]
    }
    fn get_category(&self) -> MIDICategory {
        MIDICategory::Voice
    }
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
    fn get_type(&self) -> MIDIEventType {
        MIDIEventType::AfterTouch
    }
}

//ControlChangeEvents
fn gen_coarse_fine_bytes(channel: u8, value: u16, coarse_offset: u8, fine_offset: u8) -> Vec<u8> {
    let mut output: Vec<u8> = Vec::new();

    // MSB
    if (value > 0x7F) {
        output.push(0xB | channel);
        output.push(coarse_offset);
        output.push((value >> 7) as u8);
    }

    // LSB
    if (value & 0x7F != 0) {
        output.push(0xB | channel);
        output.push(fine_offset);
        output.push((value & 0x7F) as u8);
    }

    output
}

pub struct BankSelectEvent {
    channel: u8,
    value: u16 // (technically u14)
}
impl BankSelectEvent {
    pub fn new(channel: u8, value: u16) -> Box<BankSelectEvent> {
        Box::new(
            BankSelectEvent {
                channel: channel,
                value: value
            }
        )
    }
}

impl MIDIEvent for BankSelectEvent {
    fn get_type(&self) -> MIDIEventType { MIDIEventType::BankSelect }
    fn get_category(&self) -> MIDICategory { MIDICategory::ControlChange }

    fn as_bytes(&self) -> Vec<u8> {
        gen_coarse_fine_bytes(
            self.channel,
            self.value,
            0x00, 0x20
        )
    }

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

pub struct ModulationWheelEvent {
    channel: u8,
    value: u16
}
impl ModulationWheelEvent {
    pub fn new(channel: u8, value: u16) -> Box<ModulationWheelEvent> {
        Box::new(
            ModulationWheelEvent {
                channel: channel,
                value: value
            }
        )
    }
}

impl MIDIEvent for ModulationWheelEvent {
    fn get_category(&self) -> MIDICategory { MIDICategory::ControlChange }
    fn get_type(&self) -> MIDIEventType { MIDIEventType::ModulationWheel }

    fn as_bytes(&self) -> Vec<u8> {
        gen_coarse_fine_bytes(
            self.channel,
            self.value,
            0x01, 0x21
        )
    }
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

pub struct BreathControllerEvent {
    channel: u8,
    value: u16 // (technically u14)
}
impl BreathControllerEvent {
    pub fn new(channel: u8, value: u16) -> Box<BreathControllerEvent> {
        Box::new(
            BreathControllerEvent {
                channel: channel,
                value: value
            }
        )
    }
}
impl MIDIEvent for BreathControllerEvent {
    fn get_category(&self) -> MIDICategory { MIDICategory::ControlChange }
    fn get_type(&self) -> MIDIEventType { MIDIEventType::BreathController }

    fn as_bytes(&self) -> Vec<u8> {
        gen_coarse_fine_bytes(
            self.channel,
            self.value,
            0x02, 0x22
        )
    }
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
pub struct FootPedalEvent {
    channel: u8,
    value: u16 // (technically u14)
}
impl FootPedalEvent {
    pub fn new(channel: u8, value: u16) -> Box<FootPedalEvent> {
        Box::new(
            FootPedalEvent {
                channel: channel,
                value: value
            }
        )
    }
}
impl MIDIEvent for FootPedalEvent {
    fn get_category(&self) -> MIDICategory { MIDICategory::ControlChange }
    fn get_type(&self) -> MIDIEventType { MIDIEventType::FootPedal }

    fn as_bytes(&self) -> Vec<u8> {
        gen_coarse_fine_bytes(
            self.channel,
            self.value,
            0x04, 0x24
        )
    }
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

pub struct PortamentoTimeEvent {
    channel: u8,
    value: u16 // (technically u14)
}
impl PortamentoTimeEvent {
    pub fn new(channel: u8, value: u16) -> Box<PortamentoTimeEvent> {
        Box::new(
            PortamentoTimeEvent {
                channel: channel,
                value: value
            }
        )
    }
}
impl MIDIEvent for PortamentoTimeEvent {
    fn get_category(&self) -> MIDICategory { MIDICategory::ControlChange }
    fn get_type(&self) -> MIDIEventType { MIDIEventType::PortamentoTime }

    fn as_bytes(&self) -> Vec<u8> {
        gen_coarse_fine_bytes(
            self.channel,
            self.value,
            0x05, 0x25
        )
    }
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

pub struct DataEntrySliderEvent {
    channel: u8,
    value: u16, // (technically u14)
    target: Option<u16> // if set, will create RPN event beforehand
}
impl DataEntrySliderEvent {
    pub fn new(channel: u8, value: u16, target: Option<u16>) -> Box<DataEntrySliderEvent> {
        Box::new(
            DataEntrySliderEvent {
                channel: channel,
                value: value,
                target: target
            }
        )
    }
}
impl MIDIEvent for DataEntrySliderEvent {
    fn get_category(&self) -> MIDICategory { MIDICategory::ControlChange }
    fn get_type(&self) -> MIDIEventType { MIDIEventType::DataEntrySlider }

    fn as_bytes(&self) -> Vec<u8> {
        let mut output: Vec<u8> = Vec::new();

        // if target is defined, include an RPN event before the DataEntrySlider event bytes
        match self.target {
            Some(target) => {
                let rpn = RegisteredParameterNumberEvent::new(self.channel, target);
                output.extend(rpn.as_bytes().iter().copied());
            }
            None => ()
        }
        let suboutput = gen_coarse_fine_bytes(
            self.channel,
            self.value,
            0x06, 0x26
        );
        output.extend(suboutput.iter().copied());

        output
    }
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

pub struct VolumeEvent {
    channel: u8,
    value: u16 // (technically u14)
}
impl VolumeEvent {
    pub fn new(channel: u8, value: u16) -> Box<VolumeEvent> {
        Box::new(
            VolumeEvent {
                channel: channel,
                value: value
            }
        )
    }
}

impl MIDIEvent for VolumeEvent {
    fn get_category(&self) -> MIDICategory { MIDICategory::ControlChange }
    fn get_type(&self) -> MIDIEventType { MIDIEventType::Volume }

    fn as_bytes(&self) -> Vec<u8> {
        gen_coarse_fine_bytes(
            self.channel,
            self.value,
            0x07, 0x27
        )
    }
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
pub struct BalanceEvent {
    channel: u8,
    value: u16 // (technically u14)
}
impl BalanceEvent {
    pub fn new(channel: u8, value: u16) -> Box<BalanceEvent> {
        Box::new(
            BalanceEvent {
                channel: channel,
                value: value
            }
        )
    }
}
impl MIDIEvent for BalanceEvent {
    fn get_category(&self) -> MIDICategory { MIDICategory::ControlChange }
    fn get_type(&self) -> MIDIEventType { MIDIEventType::Balance }

    fn as_bytes(&self) -> Vec<u8> {
        gen_coarse_fine_bytes(
            self.channel,
            self.value,
            0x08, 0x28
        )
    }

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


pub struct PanEvent {
    channel: u8,
    value: u16 // (technically u14)
}
impl PanEvent {
    pub fn new(channel: u8, value: u16) -> Box<PanEvent> {
        Box::new(
            PanEvent {
                channel: channel,
                value: value
            }
        )
    }
}
impl MIDIEvent for PanEvent {
    fn get_category(&self) -> MIDICategory { MIDICategory::ControlChange }
    fn get_type(&self) -> MIDIEventType { MIDIEventType::Pan }

    fn as_bytes(&self) -> Vec<u8> {
        gen_coarse_fine_bytes(
            self.channel,
            self.value,
            0x0A, 0x2A
        )
    }
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
pub struct ExpressionEvent {
    channel: u8,
    value: u16 // (technically u14)
}
impl ExpressionEvent {
    pub fn new(channel: u8, value: u16) -> Box<ExpressionEvent> {
        Box::new(
            ExpressionEvent {
                channel: channel,
                value: value
            }
        )
    }
}
impl MIDIEvent for ExpressionEvent {
    fn get_category(&self) -> MIDICategory { MIDICategory::ControlChange }
    fn get_type(&self) -> MIDIEventType { MIDIEventType::Expression }

    fn as_bytes(&self) -> Vec<u8> {
        gen_coarse_fine_bytes(
            self.channel,
            self.value,
            0x0B, 0x2B
        )
    }
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

pub struct EffectControlEvent {
    channel: u8,
    which: u8, // (0-1)
    value: u16 // (technically u14)
}
impl EffectControlEvent {
    pub fn new(channel: u8, which: u8, value: u16) -> Box<EffectControlEvent> {
        Box::new(
            EffectControlEvent {
                channel: channel,
                which: which,
                value: value
            }
        )
    }
}
impl MIDIEvent for EffectControlEvent {
    fn get_category(&self) -> MIDICategory { MIDICategory::ControlChange }
    fn get_type(&self) -> MIDIEventType { MIDIEventType::EffectControl }

    fn as_bytes(&self) -> Vec<u8> {
        gen_coarse_fine_bytes(
            self.channel,
            self.value,
            0x0C + self.which, 0x2C + self.which
        )
    }
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

pub struct SliderEvent {
    channel: u8,
    which: u8, // (0-3)
    value: u8 // (technically u7)
}
impl SliderEvent {
    pub fn new(channel: u8, which: u8, value: u8) -> Box<SliderEvent> {
        Box::new(
            SliderEvent {
                channel: channel,
                which: which,
                value: value
            }
        )
    }
}
impl MIDIEvent for SliderEvent {
    fn get_category(&self) -> MIDICategory { MIDICategory::ControlChange }
    fn get_type(&self) -> MIDIEventType { MIDIEventType::Slider }

    fn as_bytes(&self) -> Vec<u8> {
        vec![
            0xB | self.channel,
            0x10 + self.which as u8,
            self.value
        ]
    }
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

pub struct HoldPedalEvent {
    channel: u8,
    value: u8
}
impl HoldPedalEvent {
    pub fn new(channel: u8, value: u8) -> Box<HoldPedalEvent> {
        Box::new(
            HoldPedalEvent {
                channel: channel,
                value: value
            }
        )
    }
}
impl MIDIEvent for HoldPedalEvent {
    fn get_category(&self) -> MIDICategory { MIDICategory::ControlChange }
    fn get_type(&self) -> MIDIEventType { MIDIEventType::HoldPedal }

    fn as_bytes(&self) -> Vec<u8> {
        vec![
            0xB0 | self.channel,
            0x40,
            self.value
        ]
    }
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


pub struct PortamentoEvent {
    channel: u8,
    value: u8
}
impl PortamentoEvent {
    pub fn new(channel: u8, value: u8) -> Box<PortamentoEvent> {
        Box::new(
            PortamentoEvent {
                channel: channel,
                value: value
            }
        )
    }
}
impl MIDIEvent for PortamentoEvent {
    fn get_category(&self) -> MIDICategory { MIDICategory::ControlChange }
    fn get_type(&self) -> MIDIEventType { MIDIEventType::Portamento }

    fn as_bytes(&self) -> Vec<u8> {
        vec![
            0xb0 | self.channel,
            0x41,
            self.value
        ]
    }
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

pub struct SustenutoEvent {
    channel: u8,
    value: u8
}

impl SustenutoEvent {
    pub fn new(channel: u8, value: u8) -> Box<SustenutoEvent> {
        Box::new(
            SustenutoEvent {
                channel: channel,
                value: value
            }
        )
    }
}
impl MIDIEvent for SustenutoEvent {
    fn get_category(&self) -> MIDICategory { MIDICategory::ControlChange }
    fn get_type(&self) -> MIDIEventType { MIDIEventType::Sustenuto }

    fn as_bytes(&self) -> Vec<u8> {
        vec![
            0xb0 | self.channel,
            0x42,
            self.value
        ]
    }
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

pub struct SoftPedalEvent {
    channel: u8,
    value: u8
}
impl SoftPedalEvent {
    pub fn new(channel: u8, value: u8) -> Box<SoftPedalEvent> {
        Box::new(
            SoftPedalEvent {
                channel: channel,
                value: value
            }
        )
    }
}
impl MIDIEvent for SoftPedalEvent {
    fn get_category(&self) -> MIDICategory { MIDICategory::ControlChange }
    fn get_type(&self) -> MIDIEventType { MIDIEventType::SoftPedal }

    fn as_bytes(&self) -> Vec<u8> {
        vec![
            0xb0 | self.channel,
            0x43,
            self.value
        ]
    }
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

pub struct LegatoEvent {
    channel: u8,
    value: u8
}
impl LegatoEvent {
    pub fn new(channel: u8, value: u8) -> Box<LegatoEvent> {
        Box::new(
            LegatoEvent {
                channel: channel,
                value: value
            }
        )
    }
}
impl MIDIEvent for LegatoEvent {
    fn get_category(&self) -> MIDICategory { MIDICategory::ControlChange }
    fn get_type(&self) -> MIDIEventType { MIDIEventType::Legato }

    fn as_bytes(&self) -> Vec<u8> {
        vec![
            0xb0 | self.channel,
            0x44,
            self.value
        ]
    }
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

pub struct Hold2PedalEvent { // TODO: Bettwe name? Fade pedal?
    channel: u8,
    value: u8
}
impl Hold2PedalEvent {
    pub fn new(channel: u8, value: u8) -> Box<Hold2PedalEvent> {
        Box::new(
            Hold2PedalEvent {
                channel: channel,
                value: value
            }
        )
    }
}
impl MIDIEvent for Hold2PedalEvent {
    fn get_category(&self) -> MIDICategory { MIDICategory::ControlChange }
    fn get_type(&self) -> MIDIEventType { MIDIEventType::Hold2Pedal }

    fn as_bytes(&self) -> Vec<u8> {
        vec![
            0xb0 | self.channel,
            0x45,
            self.value
        ]
    }
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

pub struct SoundVariationEvent {
    channel: u8,
    value: u8
}
impl SoundVariationEvent {
    pub fn new(channel: u8, value: u8) -> Box<SoundVariationEvent> {
        Box::new(
            SoundVariationEvent {
                channel: channel,
                value: value
            }
        )
    }
}
impl MIDIEvent for SoundVariationEvent {
    fn get_category(&self) -> MIDICategory { MIDICategory::ControlChange }
    fn get_type(&self) -> MIDIEventType { MIDIEventType::SoundVariation }

    fn as_bytes(&self) -> Vec<u8> {
        vec![
            0xb0 | self.channel,
            0x46,
            self.value
        ]
    }
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

pub struct SoundTimbreEvent {
    channel: u8,
    value: u8
}
impl SoundTimbreEvent {
    pub fn new(channel: u8, value: u8) -> Box<SoundTimbreEvent> {
        Box::new(
            SoundTimbreEvent {
                channel: channel,
                value: value
            }
        )
    }
}
impl MIDIEvent for SoundTimbreEvent {
    fn get_category(&self) -> MIDICategory { MIDICategory::ControlChange }
    fn get_type(&self) -> MIDIEventType { MIDIEventType::SoundTimbre }

    fn as_bytes(&self) -> Vec<u8> {
        vec![
            0xb0 | self.channel,
            0x47,
            self.value
        ]
    }
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

pub struct SoundReleaseTimeEvent {
    channel: u8,
    value: u8
}
impl SoundReleaseTimeEvent {
    pub fn new(channel: u8, value: u8) -> Box<SoundReleaseTimeEvent> {
        Box::new(
            SoundReleaseTimeEvent {
                channel: channel,
                value: value
            }
        )
    }
}
impl MIDIEvent for SoundReleaseTimeEvent {
    fn get_category(&self) -> MIDICategory { MIDICategory::ControlChange }
    fn get_type(&self) -> MIDIEventType { MIDIEventType::SoundReleaseTime }

    fn as_bytes(&self) -> Vec<u8> {
        vec![
            0xb0 | self.channel,
            0x48,
            self.value
        ]
    }
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

pub struct SoundAttackEvent {
    channel: u8,
    value: u8
}
impl SoundAttackEvent {
    pub fn new(channel: u8, value: u8) -> Box<SoundAttackEvent> {
        Box::new(
            SoundAttackEvent {
                channel: channel,
                value: value
            }
        )
    }
}
impl MIDIEvent for SoundAttackEvent {
    fn get_category(&self) -> MIDICategory { MIDICategory::ControlChange }
    fn get_type(&self) -> MIDIEventType { MIDIEventType::SoundAttack }

    fn as_bytes(&self) -> Vec<u8> {
        vec![
            0xb0 | self.channel,
            0x49,
            self.value
        ]
    }
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


pub struct SoundBrightnessEvent {
    channel: u8,
    value: u8
}
impl SoundBrightnessEvent {
    pub fn new(channel: u8, value: u8) -> Box<SoundBrightnessEvent> {
        Box::new(
            SoundBrightnessEvent {
                channel: channel,
                value: value
            }
        )
    }
}

impl MIDIEvent for SoundBrightnessEvent {
    fn get_category(&self) -> MIDICategory { MIDICategory::ControlChange }
    fn get_type(&self) -> MIDIEventType { MIDIEventType::SoundBrightness }

    fn as_bytes(&self) -> Vec<u8> {
        vec![
            0xb0 | self.channel,
            0x4A,
            self.value
        ]
    }
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


pub struct SoundControlEvent {
    channel: u8,
    which: u8, // (0-4)
    value: u8
}
impl SoundControlEvent {
    pub fn new(channel: u8, which: u8, value: u8) -> Box<SoundControlEvent> {
        Box::new(
            SoundControlEvent {
                channel: channel,
                which: which,
                value: value
            }
        )
    }
}
impl MIDIEvent for SoundControlEvent {
    fn get_category(&self) -> MIDICategory { MIDICategory::ControlChange }
    fn get_type(&self) -> MIDIEventType { MIDIEventType::SoundControl }

    fn as_bytes(&self) -> Vec<u8> {
        vec![
            0xb0 | self.channel,
            0x4B + self.which,
            self.value
        ]
    }
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

pub struct GeneralButtonOnEvent {
    channel: u8,
    which: u8 //(0-3)
}
impl GeneralButtonOnEvent {
    pub fn new(channel: u8, which: u8) -> Box<GeneralButtonOnEvent> {
        Box::new(
            GeneralButtonOnEvent {
                channel: channel,
                which: which
            }
        )
    }
}
impl MIDIEvent for GeneralButtonOnEvent {
    fn get_category(&self) -> MIDICategory { MIDICategory::ControlChange }
    fn get_type(&self) -> MIDIEventType { MIDIEventType::GeneralButtonOn }

    fn as_bytes(&self) -> Vec<u8> {
        vec![
            0xB0 | self.channel,
            0x50 + self.which,
            0b01000000
        ]
    }
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


pub struct GeneralButtonOffEvent {
    channel: u8,
    which: u8 //(0-3)
}
impl GeneralButtonOffEvent {
    pub fn new(channel: u8, which: u8) -> Box<GeneralButtonOffEvent> {
        Box::new(
            GeneralButtonOffEvent {
                channel: channel,
                which: which
            }
        )
    }
}
impl MIDIEvent for GeneralButtonOffEvent {
    fn get_category(&self) -> MIDICategory { MIDICategory::ControlChange }
    fn get_type(&self) -> MIDIEventType { MIDIEventType::GeneralButtonOff }

    fn as_bytes(&self) -> Vec<u8> {
        vec![
            0xB0 | self.channel,
            0x50 + self.which,
            0b00000000
        ]
    }
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

pub struct EffectsLevelEvent {
    channel: u8,
    value: u8
}

impl EffectsLevelEvent {
    pub fn new(channel: u8, value: u8) -> Box<EffectsLevelEvent> {
        Box::new(
            EffectsLevelEvent {
                channel: channel,
                value: value
            }
        )
    }
}
impl MIDIEvent for EffectsLevelEvent {
    fn get_category(&self) -> MIDICategory { MIDICategory::ControlChange }
    fn get_type(&self) -> MIDIEventType { MIDIEventType::EffectsLevel }

    fn as_bytes(&self) -> Vec<u8> {
        vec![
            0xb0 | self.channel,
            0x5B,
            self.value
        ]
    }
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


pub struct TremuloLevelEvent {
    channel: u8,
    value: u8
}
impl TremuloLevelEvent {
    pub fn new(channel: u8, value: u8) -> Box<TremuloLevelEvent> {
        Box::new(
            TremuloLevelEvent {
                channel: channel,
                value: value
            }
        )
    }
}
impl MIDIEvent for TremuloLevelEvent {
    fn get_category(&self) -> MIDICategory { MIDICategory::ControlChange }
    fn get_type(&self) -> MIDIEventType { MIDIEventType::TremuloLevel }

    fn as_bytes(&self) -> Vec<u8> {
        vec![
            0xb0 | self.channel,
            0x5C,
            self.value
        ]
    }
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


pub struct ChorusLevelEvent {
    channel: u8,
    value: u8
}
impl ChorusLevelEvent {
    pub fn new(channel: u8, value: u8) -> Box<ChorusLevelEvent> {
        Box::new(
            ChorusLevelEvent {
                channel: channel,
                value: value
            }
        )
    }
}
impl MIDIEvent for ChorusLevelEvent {
    fn get_category(&self) -> MIDICategory { MIDICategory::ControlChange }
    fn get_type(&self) -> MIDIEventType { MIDIEventType::ChorusLevel }

    fn as_bytes(&self) -> Vec<u8> {
        vec![
            0xb0 | self.channel,
            0x5D,
            self.value
        ]
    }
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

pub struct CelesteLevelEvent {
    channel: u8,
    value: u8
}
impl CelesteLevelEvent {
    pub fn new(channel: u8, value: u8) -> Box<CelesteLevelEvent> {
        Box::new(
            CelesteLevelEvent {
                channel: channel,
                value: value
            }
        )
    }
}
impl MIDIEvent for CelesteLevelEvent {
    fn get_category(&self) -> MIDICategory { MIDICategory::ControlChange }
    fn get_type(&self) -> MIDIEventType { MIDIEventType::CelesteLevel }

    fn as_bytes(&self) -> Vec<u8> {
        vec![
            0xb0 | self.channel,
            0x5E,
            self.value
        ]
    }
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

pub struct PhaserLevelEvent {
    channel: u8,
    value: u8
}
impl PhaserLevelEvent {
    pub fn new(channel: u8, value: u8) -> Box<PhaserLevelEvent> {
        Box::new(
            PhaserLevelEvent {
                channel: channel,
                value: value
            }
        )
    }
}
impl MIDIEvent for PhaserLevelEvent {
    fn get_category(&self) -> MIDICategory { MIDICategory::ControlChange }
    fn get_type(&self) -> MIDIEventType { MIDIEventType::PhaserLevel }

    fn as_bytes(&self) -> Vec<u8> {
        vec![
            0xb0 | self.channel,
            0x5F,
            self.value
        ]
    }
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


pub struct DataButtonIncrementEvent {
    channel: u8,
    target: Option<u16> // if set, will create RPN event beforehand
}
impl DataButtonIncrementEvent {
    pub fn new(channel: u8, target: Option<u16>) -> Box<DataButtonIncrementEvent> {
        Box::new(
            DataButtonIncrementEvent {
                channel: channel,
                target: target
            }
        )
    }
}
impl MIDIEvent for DataButtonIncrementEvent {
    fn get_category(&self) -> MIDICategory { MIDICategory::ControlChange }
    fn get_type(&self) -> MIDIEventType { MIDIEventType::DataButtonIncrement }

    fn as_bytes(&self) -> Vec<u8> {
        let mut output = Vec::new();

        // if target is defined, include an RPN event before the DataEntrySlider event bytes
        match self.target {
            Some(target) => {
                let rpn = RegisteredParameterNumberEvent::new(self.channel, target);
                output.extend(rpn.as_bytes().iter().copied());
            }
            None => ()
        }

        output.push(0xB0 | self.channel);
        output.push(0x60);
        output.push(0x00);

        output
    }
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

pub struct DataButtonDecrementEvent {
    channel: u8,
    target: Option<u16> // if set, will create RPN event beforehand
}
impl DataButtonDecrementEvent {
    pub fn new(channel: u8, target: Option<u16>) -> Box<DataButtonDecrementEvent> {
        Box::new(
            DataButtonDecrementEvent {
                channel: channel,
                target: target
            }
        )
    }
}
impl MIDIEvent for DataButtonDecrementEvent {
    fn get_category(&self) -> MIDICategory { MIDICategory::ControlChange }
    fn get_type(&self) -> MIDIEventType { MIDIEventType::DataButtonDecrement }

    fn as_bytes(&self) -> Vec<u8> {
        let mut output = Vec::new();

        // if target is defined, include an RPN event before the DataEntrySlider event bytes
        match self.target {
            Some(target) => {
                let rpn = RegisteredParameterNumberEvent::new(self.channel, target);
                output.extend(rpn.as_bytes().iter().copied());
            }
            None => ()
        }

        output.push(0xB0 | self.channel);
        output.push(0x61);
        output.push(0x00);

        output
    }
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


pub struct RegisteredParameterNumberEvent {
    channel:u8,
    value: u16
}
impl RegisteredParameterNumberEvent {
    pub fn new(channel: u8, value: u16) -> Box<RegisteredParameterNumberEvent> {
        Box::new(
            RegisteredParameterNumberEvent {
                channel: channel,
                value: value
            }
        )
    }
}

impl MIDIEvent for RegisteredParameterNumberEvent {
    fn get_category(&self) -> MIDICategory { MIDICategory::ControlChange }
    fn get_type(&self) -> MIDIEventType { MIDIEventType::RegisteredParameterNumber }

    fn as_bytes(&self) -> Vec<u8> {
        gen_coarse_fine_bytes(
            self.channel,
            self.value,
            0x65, 0x64
        )
    }

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


pub struct NonRegisteredParameterNumberEvent {
    channel:u8,
    value: u16
}
impl NonRegisteredParameterNumberEvent {
    pub fn new(channel: u8, value: u16) -> Box<NonRegisteredParameterNumberEvent> {
        Box::new(
            NonRegisteredParameterNumberEvent {
                channel: channel,
                value: value
            }
        )
    }
}
impl MIDIEvent for NonRegisteredParameterNumberEvent {
    fn get_category(&self) -> MIDICategory { MIDICategory::ControlChange }
    fn get_type(&self) -> MIDIEventType { MIDIEventType::NonRegisteredParameterNumber }

    fn as_bytes(&self) -> Vec<u8> {
        gen_coarse_fine_bytes(
            self.channel,
            self.value,
            0x65, 0x64
        )
    }
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

pub struct AllControllersOffEvent {
    channel: u8
}
impl AllControllersOffEvent {
    pub fn new(channel: u8) -> Box<AllControllersOffEvent> {
        Box::new(
            AllControllersOffEvent {
                channel: channel
            }
        )
    }
}
impl MIDIEvent for AllControllersOffEvent {
    fn get_category(&self) -> MIDICategory { MIDICategory::ControlChange }
    fn get_type(&self) -> MIDIEventType { MIDIEventType::AllControllersOff }

    fn as_bytes(&self) -> Vec<u8> {
        vec![
            0xB0 | self.channel,
            0x79, 0x00
        ]
    }
    fn get_property(&self, _: u8) -> Vec<u8> {
        vec![self.channel]
    }
    fn set_property(&mut self, _: u8, bytes: Vec<u8>) {
        self.channel = bytes[0];
    }
}

pub struct LocalKeyboardEnableEvent {
    channel: u8
}
impl LocalKeyboardEnableEvent {
    pub fn new(channel: u8) -> Box<LocalKeyboardEnableEvent> {
        Box::new(
            LocalKeyboardEnableEvent {
                channel: channel
            }
        )
    }
}
impl MIDIEvent for LocalKeyboardEnableEvent {
    fn get_category(&self) -> MIDICategory { MIDICategory::ControlChange }
    fn get_type(&self) -> MIDIEventType { MIDIEventType::LocalKeyboardEnable }

    fn as_bytes(&self) -> Vec<u8> {
        vec![
            0xB0 | self.channel,
            0x7A, 0x40
        ]
    }
    fn get_property(&self, _: u8) -> Vec<u8> {
        vec![self.channel]
    }
    fn set_property(&mut self, _: u8, bytes: Vec<u8>) {
        self.channel = bytes[0];
    }
}

pub struct LocalKeyboardDisableEvent {
    channel: u8
}
impl LocalKeyboardDisableEvent {
    pub fn new(channel: u8) -> Box<LocalKeyboardDisableEvent> {
        Box::new(
            LocalKeyboardDisableEvent {
                channel: channel
            }
        )
    }
}
impl MIDIEvent for LocalKeyboardDisableEvent {
    fn get_category(&self) -> MIDICategory { MIDICategory::ControlChange }
    fn get_type(&self) -> MIDIEventType { MIDIEventType::LocalKeyboardDisable }

    fn as_bytes(&self) -> Vec<u8> {
        vec![
            0xB0 | self.channel,
            0x7A, 0x00
        ]
    }
    fn get_property(&self, _: u8) -> Vec<u8> {
        vec![self.channel]
    }
    fn set_property(&mut self, _: u8, bytes: Vec<u8>) {
        self.channel = bytes[0];
    }
}


pub struct AllNotesOffEvent {
    channel: u8
}
impl AllNotesOffEvent {
    pub fn new(channel: u8) -> Box<AllNotesOffEvent> {
        Box::new(
            AllNotesOffEvent {
                channel: channel
            }
        )
    }
}
impl MIDIEvent for AllNotesOffEvent {
    fn get_category(&self) -> MIDICategory { MIDICategory::ControlChange }
    fn get_type(&self) -> MIDIEventType { MIDIEventType::AllNotesOff }

    fn as_bytes(&self) -> Vec<u8> {
        vec![
            0xB0 | self.channel,
            0x7B, 0x00
        ]
    }
    fn get_property(&self, _: u8) -> Vec<u8> {
        vec![self.channel]
    }
    fn set_property(&mut self, _: u8, bytes: Vec<u8>) {
        self.channel = bytes[0];
    }
}

pub struct AllSoundOffEvent {
    channel: u8
}
impl AllSoundOffEvent {
    pub fn new(channel: u8) -> Box<AllSoundOffEvent> {
        Box::new(
            AllSoundOffEvent {
                channel: channel
            }
        )
    }
}
impl MIDIEvent for AllSoundOffEvent {
    fn get_category(&self) -> MIDICategory { MIDICategory::ControlChange }
    fn get_type(&self) -> MIDIEventType { MIDIEventType::AllSoundOff }

    fn as_bytes(&self) -> Vec<u8> {
        vec![
            0xB0 | self.channel,
            0x78, 0x00
        ]
    }
    fn get_property(&self, _: u8) -> Vec<u8> {
        vec![self.channel]
    }
    fn set_property(&mut self, _: u8, bytes: Vec<u8>) {
        self.channel = bytes[0];
    }
}


pub struct OmniOffEvent {
    channel: u8
}
impl OmniOffEvent {
    pub fn new(channel: u8) -> Box<OmniOffEvent> {
        Box::new(
            OmniOffEvent {
                channel: channel
            }
        )
    }
}
impl MIDIEvent for OmniOffEvent {
    fn get_category(&self) -> MIDICategory { MIDICategory::ControlChange }
    fn get_type(&self) -> MIDIEventType { MIDIEventType::OmniOff }

    fn as_bytes(&self) -> Vec<u8> {
        vec![
            0xB0 | self.channel,
            0x7C, 0x00
        ]
    }
    fn get_property(&self, _: u8) -> Vec<u8> {
        vec![self.channel]
    }
    fn set_property(&mut self, _: u8, bytes: Vec<u8>) {
        self.channel = bytes[0];
    }
}


pub struct OmniOnEvent {
    channel: u8
}
impl OmniOnEvent {
    pub fn new(channel: u8) -> Box<OmniOnEvent> {
        Box::new(
            OmniOnEvent {
                channel: channel
            }
        )
    }
}
impl MIDIEvent for OmniOnEvent {
    fn get_category(&self) -> MIDICategory { MIDICategory::ControlChange }
    fn get_type(&self) -> MIDIEventType { MIDIEventType::OmniOn }

    fn as_bytes(&self) -> Vec<u8> {
        vec![
            0xB0 | self.channel,
            0x7D, 0x00
        ]
    }
    fn get_property(&self, _: u8) -> Vec<u8> {
        vec![self.channel]
    }
    fn set_property(&mut self, _: u8, bytes: Vec<u8>) {
        self.channel = bytes[0];
    }
}


pub struct MonophonicOperationEvent {
    channel: u8,
    value: u8
}
impl MonophonicOperationEvent {
    pub fn new(channel: u8, value: u8) -> Box<MonophonicOperationEvent> {
        Box::new(
            MonophonicOperationEvent {
                channel: channel,
                value: value
            }
        )
    }
}
impl MIDIEvent for MonophonicOperationEvent {
    fn get_category(&self) -> MIDICategory { MIDICategory::ControlChange }
    fn get_type(&self) -> MIDIEventType { MIDIEventType::MonophonicOperation }

    fn as_bytes(&self) -> Vec<u8> {
        vec![
            0xB0 | self.channel, 0xFE, self.value
        ]
    }

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


pub struct PolyphonicOperationEvent {
    channel: u8
}
impl PolyphonicOperationEvent {
    pub fn new(channel: u8) -> Box<PolyphonicOperationEvent> {
        Box::new(
            PolyphonicOperationEvent {
                channel: channel
            }
        )
    }
}
impl MIDIEvent for PolyphonicOperationEvent {
    fn get_category(&self) -> MIDICategory { MIDICategory::ControlChange }
    fn get_type(&self) -> MIDIEventType { MIDIEventType::PolyphonicOperation }

    fn as_bytes(&self) -> Vec<u8> {
        vec![
            0xB0 | self.channel, 0xFF, 0
        ]
    }
    fn get_property(&self, _: u8) -> Vec<u8> {
        vec![self.channel]
    }
    fn set_property(&mut self, _: u8, bytes: Vec<u8>) {
        self.channel = bytes[0];
    }
}

// End ControlChangeEvents
pub struct ControlChangeEvent {
	channel: u8,
	controller: u8,
	value: u8
}

impl ControlChangeEvent {
    pub fn new(channel: u8, controller: u8, value:u8) -> Box<ControlChangeEvent> {
        Box::new(
            ControlChangeEvent {
                channel: channel & 0x0F,
                controller: controller & 0x7F,
                value: value & 0x7F
            }
        )
    }
}

impl MIDIEvent for ControlChangeEvent {
    fn as_bytes(&self) -> Vec<u8> {
        vec![
            0xB0 | self.channel,
            self.controller,
            self.value
        ]
    }
    fn get_category(&self) -> MIDICategory {
        MIDICategory::Voice
    }
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
                channel: channel & 0x0F,
                program: program & 0x7F
            }
        )
    }
}
impl MIDIEvent for ProgramChangeEvent {
    fn as_bytes(&self) -> Vec<u8> {
        vec![
            0xC0 | self.channel,
            self.program
        ]
    }
    fn get_category(&self) -> MIDICategory {
        MIDICategory::Voice
    }
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
                channel: channel & 0x0F,
                pressure: pressure & 0x7F
            }
        )
    }
}

impl MIDIEvent for ChannelPressureEvent {
    fn as_bytes(&self) -> Vec<u8> {
        vec![
            0xD0 | self.channel,
            self.pressure
        ]
    }
    fn get_category(&self) -> MIDICategory {
        MIDICategory::Voice
    }
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
    fn get_type(&self) -> MIDIEventType {
        MIDIEventType::ChannelPressure
    }
}

pub struct PitchWheelChangeEvent {
	channel: u8,
    value: f64
}

impl PitchWheelChangeEvent {
    pub fn new(channel: u8, value: f64) -> Box<PitchWheelChangeEvent> {
        Box::new(
            PitchWheelChangeEvent {
                channel: channel & 0x0F,
                value: value
            }
        )
    }
    pub fn new_from_lsb_msb(channel: u8, lsb: u8, msb: u8) -> Box<PitchWheelChangeEvent> {
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

    fn set_value(&mut self, value: f64) {
        self.value = value;
    }
}

impl MIDIEvent for PitchWheelChangeEvent {
    fn as_bytes(&self) -> Vec<u8> {
        let unsigned_value = self.get_unsigned_value();
        let lsb: u8 = (unsigned_value & 0x007F) as u8;
        let msb: u8 = ((unsigned_value >> 7) & 0x007F) as u8;
        vec![
            0xE0 | self.channel,
            lsb,
            msb
        ]
    }

    fn get_category(&self) -> MIDICategory {
        MIDICategory::Voice
    }
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
    fn as_bytes(&self) -> Vec<u8> {
        let mut output = vec![0xF0];
        output.extend(self.data.iter().copied());
        output.push(0xF7);
        output
    }
    fn get_category(&self) -> MIDICategory {
        MIDICategory::Voice
    }
    fn set_property(&mut self, _:u8, bytes: Vec<u8>) {
        self.data = bytes.clone()
    }

    fn get_property(&self, _argument: u8) -> Vec<u8> {
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
    fn as_bytes(&self) -> Vec<u8> {
        let mut b = 0;
        b |= self.message_type;
        b <<= 3;
        b |= self.value;

        vec![ 0xF1, b ]
    }
    fn get_category(&self) -> MIDICategory {
        MIDICategory::RealTime
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
impl SongPositionPointerEvent {
    pub fn new(beat: u16) -> Box<SongPositionPointerEvent> {
        Box::new(
            SongPositionPointerEvent {
                beat: beat
            }
        )
    }
}
impl MIDIEvent for SongPositionPointerEvent {
    fn as_bytes(&self) -> Vec<u8> {
        vec![
            0xF2,
            (self.beat & 0x7F) as u8,
            ((self.beat >> 7) & 0x7F) as u8
        ]
    }
    fn get_category(&self) -> MIDICategory {
        MIDICategory::RealTime
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
impl SongSelectEvent {
    pub fn new(song: u8) -> Box<SongSelectEvent> {
        Box::new(
            SongSelectEvent {
                song: song
            }
        )
    }
}
impl MIDIEvent for SongSelectEvent {
    fn as_bytes(&self) -> Vec<u8> {
        vec![
            0xF3,
            self.song & 0x7F
        ]
    }
    fn get_category(&self) -> MIDICategory {
        MIDICategory::RealTime
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
    fn as_bytes(&self) -> Vec<u8> {
        vec![ 0xF6 ]
    }
    fn get_category(&self) -> MIDICategory {
        MIDICategory::RealTime
    }

    fn set_property(&mut self, _: u8, _bytes: Vec<u8>) { }
    fn get_property(&self, _: u8) -> Vec<u8> {
        Vec::new()
    }

    fn get_type(&self) -> MIDIEventType {
        MIDIEventType::TuneRequest
    }
}

pub struct MIDIClockEvent { }
impl MIDIEvent for MIDIClockEvent {
    fn as_bytes(&self) -> Vec<u8> {
        vec![ 0xF8 ]
    }
    fn get_category(&self) -> MIDICategory {
        MIDICategory::RealTime
    }

    fn set_property(&mut self, _: u8, _bytes: Vec<u8>) { }
    fn get_property(&self, _: u8) -> Vec<u8> {
        Vec::new()
    }

    fn get_type(&self) -> MIDIEventType {
        MIDIEventType::MIDIClock
    }
}

pub struct MIDIStartEvent { }
impl MIDIEvent for MIDIStartEvent {
    fn as_bytes(&self) -> Vec<u8> {
        vec![ 0xFA ]
    }
    fn get_category(&self) -> MIDICategory {
        MIDICategory::RealTime
    }

    fn set_property(&mut self, _: u8, _bytes: Vec<u8>) { }
    fn get_property(&self, _: u8) -> Vec<u8> {
        Vec::new()
    }

    fn get_type(&self) -> MIDIEventType {
        MIDIEventType::MIDIStart
    }
}

pub struct MIDIContinueEvent { }
impl MIDIEvent for MIDIContinueEvent {
    fn as_bytes(&self) -> Vec<u8> {
        vec![ 0xFB ]
    }
    fn get_category(&self) -> MIDICategory {
        MIDICategory::RealTime
    }

    fn set_property(&mut self, _: u8, _bytes: Vec<u8>) { }
    fn get_property(&self, _: u8) -> Vec<u8> {
        Vec::new()
    }

    fn get_type(&self) -> MIDIEventType {
        MIDIEventType::MIDIContinue
    }
}

pub struct MIDIStopEvent { }
impl MIDIEvent for MIDIStopEvent {
    fn as_bytes(&self) -> Vec<u8> {
        vec![ 0xFC ]
    }
    fn get_category(&self) -> MIDICategory {
        MIDICategory::RealTime
    }

    fn set_property(&mut self, _: u8, _bytes: Vec<u8>) { }

    fn get_property(&self, _: u8) -> Vec<u8> {
        Vec::new()
    }

    fn get_type(&self) -> MIDIEventType {
        MIDIEventType::MIDIStop
    }
}

pub struct ActiveSenseEvent { }
impl MIDIEvent for ActiveSenseEvent {
    fn as_bytes(&self) -> Vec<u8> {
        vec![ 0xFE ]
    }
    fn get_category(&self) -> MIDICategory {
        MIDICategory::RealTime
    }

    fn set_property(&mut self, _: u8, _bytes: Vec<u8>) { }

    fn get_property(&self, _: u8) -> Vec<u8> {
        Vec::new()
    }

    fn get_type(&self) -> MIDIEventType {
        MIDIEventType::ActiveSense
    }
}

pub struct ResetEvent { }
impl MIDIEvent for ResetEvent {
    fn as_bytes(&self) -> Vec<u8> {
        vec![ 0xFF ]
    }
    fn get_category(&self) -> MIDICategory {
        MIDICategory::RealTime
    }
    fn set_property(&mut self, _: u8, _bytes: Vec<u8>) { }

    fn get_property(&self, _: u8) -> Vec<u8> {
        Vec::new()
    }

    fn get_type(&self) -> MIDIEventType {
        MIDIEventType::Reset
    }
}


/// Structural representation of MIDI.
///
/// Can represent a file or a real-time performance.
///
/// # Examples
/// ```
/// use apres::MIDI;
/// // Create a MIDI from a file
/// let midi = MIDI::from_path("/path/to/file.mid");
/// ```
/// ```
/// use apres::MIDI;
/// // Create an empty MIDI file.
/// let midi = MIDI::new();
/// ```
pub struct MIDI {
    ppqn: u16,
    midi_format: u16, // 16 because the format stores in 2 bytes, even though it only requires 2 bits (0,1,2)
    events: HashMap<u64, Box<dyn MIDIEvent>>,
    event_id_gen: u64,
    event_positions: HashMap<u64, (usize, usize)>
}


impl MIDI {
    pub fn new() -> MIDI {
        MIDI {
            event_id_gen: 1, // Reserve 0 for passing 'none' to bindings
            ppqn: 120,
            midi_format: 1,
            events: HashMap::new(),
            event_positions: HashMap::new(),
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
                    Err(_) => {
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
        let mut current_deltatime: usize;

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
                current_deltatime = 0;
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
                            mlo.process_mtrk_event(byte, &mut sub_bytes, &mut current_deltatime, current_track, &mut fallback_byte);
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

    pub fn process_mtrk_event(&mut self, leadbyte: u8, bytes: &mut Vec<u8>, current_deltatime: &mut usize, track: usize, fallback_cmd: &mut u8) -> Option<u64> {
        let mut output = None;
        let mut channel: u8;

        let mut a: u8;
        let mut b: u8;
        let mut c: u8;

        let mut n: u32;
        let mut varlength: u64;

        let mut dump: Vec<u8>;

        let mut leadnibble: u8 = leadbyte >> 4;

        match leadbyte {
            0..=0x7F => {
                 // Implicitly a Channel Event
                bytes.push(leadbyte);
                output = self.process_mtrk_event(*fallback_cmd, bytes, current_deltatime, track, fallback_cmd);
            }
            0x80..=0xEF => {
                match leadnibble {
                    0x8 => {
                        channel = leadbyte & 0x0F;
                        b = bytes.pop().unwrap();
                        c = bytes.pop().unwrap();
                        output = Some(self.insert_event(track, *current_deltatime, NoteOffEvent::new(channel, b, c)));
                    }
                    0x9 => {
                        channel = leadbyte & 0x0F;
                        b = bytes.pop().unwrap();
                        c = bytes.pop().unwrap();
                        // Convert fake NoteOff (NoteOn where velocity is 0) to real NoteOff
                        if c == 0 {
                            output = Some(self.insert_event(track, *current_deltatime, NoteOffEvent::new(channel, b, c)));
                        } else {
                            output = Some(self.insert_event(track, *current_deltatime, NoteOnEvent::new(channel, b, c)));
                        }
                    }
                    0xA => {
                        channel = leadbyte & 0x0F;
                        b = bytes.pop().unwrap();
                        c = bytes.pop().unwrap();
                        output = Some(self.insert_event(track, *current_deltatime, AfterTouchEvent::new(channel, b, c)));
                    }
                    0xB => {
                        channel = leadbyte & 0x0F;
                        b = bytes.pop().unwrap();
                        c = bytes.pop().unwrap();
                        output = Some(self.insert_event(track, *current_deltatime, ControlChangeEvent::new(channel, b, c)));
                    }
                    0xC => {
                        channel = leadbyte & 0x0F;
                        b = bytes.pop().unwrap();
                        output = Some(self.insert_event(track, *current_deltatime, ProgramChangeEvent::new(channel, b)));
                    }
                    0xD => {
                        channel = leadbyte & 0x0F;
                        b = bytes.pop().unwrap();
                        output = Some(self.insert_event(track, *current_deltatime, ChannelPressureEvent::new(channel, b)));
                    }
                    0xE => {
                        channel = leadbyte & 0x0F;
                        b = bytes.pop().unwrap();
                        c = bytes.pop().unwrap();
                        output = Some(self.insert_event(track, *current_deltatime, PitchWheelChangeEvent::new_from_lsb_msb(channel, b, c)));
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

                output = Some(self.insert_event(track, *current_deltatime, SystemExclusiveEvent::new(dump.clone())));
            }
            0xF2 => {
                // Song Position Pointer
                b = bytes.pop().unwrap();
                c = bytes.pop().unwrap();

                let beat = ((c as u16) << 7) + (b as u16);
                output = Some(self.insert_event(track, *current_deltatime, SongPositionPointerEvent::new(beat)));
            }
            0xF3 => {
                b = bytes.pop().unwrap();
                output = Some(self.insert_event(track, *current_deltatime, SongSelectEvent::new(b & 0x7F)));
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
                if a == 0x51 {
                    output = Some(self.insert_event(track, *current_deltatime, SetTempoEvent::new(pop_n(bytes, varlength as usize))));
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
                    match a {
                        0x01 => {
                            match std::str::from_utf8(dump.as_slice()) {
                                Ok(textdump) => {
                                    output = Some(self.insert_event(track, *current_deltatime, TextEvent::new(textdump.to_string())));
                                }
                                Err(e) => {}
                            };
                        }
                        0x02 => {
                            match std::str::from_utf8(dump.as_slice()) {
                                Ok(textdump) => {
                                    output = Some(self.insert_event(track, *current_deltatime, CopyRightNoticeEvent::new(textdump.to_string())));
                                }
                                Err(e) => {}
                            };
                        }
                        0x03 => {
                            match std::str::from_utf8(dump.as_slice()) {
                                Ok(textdump) => {
                                    output = Some(self.insert_event(track, *current_deltatime, TrackNameEvent::new(textdump.to_string())));
                                }
                                Err(e) => {}
                            };
                        }
                        0x04 => {
                            match std::str::from_utf8(dump.as_slice()) {
                                Ok(textdump) => {
                                    output = Some(self.insert_event(track, *current_deltatime, InstrumentNameEvent::new(textdump.to_string())));
                                }
                                Err(e) => {}
                            };
                        }
                        0x05 => {
                            match std::str::from_utf8(dump.as_slice()) {
                                Ok(textdump) => {
                                    output = Some(self.insert_event(track, *current_deltatime, LyricEvent::new(textdump.to_string())));
                                }
                                Err(e) => {}
                            };
                        }
                        0x06 => {
                            match std::str::from_utf8(dump.as_slice()) {
                                Ok(textdump) => {
                                    output = Some(self.insert_event(track, *current_deltatime, MarkerEvent::new(textdump.to_string())));
                                }
                                Err(e) => {}
                            };
                        }
                        0x07 => {
                            match std::str::from_utf8(dump.as_slice()) {
                                Ok(textdump) => {
                                    output = Some(self.insert_event(track, *current_deltatime, CuePointEvent::new(textdump.to_string())));
                                }
                                Err(e) => {}
                            };
                        }
                        0x20 => {
                            output = Some(self.insert_event(track, *current_deltatime, ChannelPrefixEvent::new(dump[0])));
                        }
                        0x2F => {
                            // I *think* EndOfTrack events can be safely ignored, since it has to be the last event in a track and the track knows how long it is.
                            //output = Some(self.insert_event(track, *current_deltatime, EndOfTrackEvent::new() ));
                        }
                        0x51 => {
                        }
                        0x54 => {
                            output = Some(self.insert_event(track, *current_deltatime, SMPTEOffsetEvent::new(dump[0], dump[1], dump[2], dump[3], dump[4])));
                        }
                        0x58 => {
                            output = Some(self.insert_event(track, *current_deltatime, TimeSignatureEvent::new(dump[0], dump[1],dump[2], dump[3])));
                        }
                        0x59 => {
                            //output = Some(self.insert_event(track, *current_deltatime, KeySignatureEvent::from_mi_sf(dump[1], dump[0])));
                        }
                        0x7F => {
                            output = Some(self.insert_event(track, *current_deltatime, SequencerSpecificEvent::new(dump)));
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

        output
    }


    fn as_bytes(&self) -> Vec<u8> {
        // First 8  bytes will always be the same
        let mut output: Vec<u8> = vec!['M' as u8, 'T' as u8, 'h' as u8, 'd' as u8, 0, 0, 0, 6];

        let format: u16 = self.get_format();
        output.push((format / 256) as u8);
        output.push((format % 256) as u8);

        let track_count: u16 = self.count_tracks() as u16;
        output.push((track_count / 256) as u8);
        output.push((track_count % 256) as u8);

        let ppqn: u16 = self.get_ppqn();
        output.push((ppqn / 256) as u8);
        output.push((ppqn % 256) as u8);

        // Tracks (MTrk)
        let mut track_event_bytes: Vec<u8>;
        let mut working_event: Box<dyn MIDIEvent>;
        let mut track_byte_length: u32;
        let tracks: Vec<Vec<(usize, u64)>> = self.get_tracks();

        for (i, ticks) in tracks.iter().enumerate() {
            output.push('M' as u8);
            output.push('T' as u8);
            output.push('r' as u8);
            output.push('k' as u8);

            track_event_bytes = Vec::new();
            for (tick_delay, eid) in ticks.iter() {
                match self.get_event(*eid) {
                    Some(working_event) => {
                        track_event_bytes.extend(to_variable_length_bytes(*tick_delay).iter().copied());
                        track_event_bytes.extend(working_event.as_bytes());
                    }
                    None => { }
                }
            }

            // Automatically handle EndOfTrackEvent Here instead of requiring it be in the MIDITrack Object
            track_event_bytes.push(0);
            track_event_bytes.extend(EndOfTrackEvent::new().as_bytes().iter().copied());

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
        let bytes = self.as_bytes();
        match File::create(path) {
            Ok(mut file) => {
                file.write_all(bytes.as_slice());
            }
            Err(e) => {
            }
        }
    }

    pub fn get_event_position(&self, event_id: u64) -> Option<&(usize, usize)> {
        self.event_positions.get(&event_id)
    }

    pub fn get_tracks(&self) -> Vec<Vec<(usize, u64)>> {
        let mut tracks = Vec::new();
        for (eid, (track, tick)) in self.event_positions.iter() {
            while tracks.len() <= *track {
                tracks.push(Vec::new())
            }
            match tracks.get_mut(*track) {
                Some(ticklist) => {
                    ticklist.push((*tick, *eid));
                }
                None => ()
            }
        }

        let mut output = Vec::new();
        let mut previous_tick;
        let mut current;
        for track in tracks.iter_mut() {
            track.sort();
            current = Vec::new();
            previous_tick = 0;
            for (current_tick, eid) in track.iter() {
                current.push((*current_tick - previous_tick, *eid));
                previous_tick = *current_tick;
            }
            output.push(current);
        }

        output
    }

    pub fn count_tracks(&self) -> usize {
        let mut used_tracks = HashSet::new();
        for (_, (current_track, __)) in self.event_positions.iter() {
            used_tracks.insert(current_track);
        }

        used_tracks.len()
    }

    pub fn count_events(&self) -> usize {
        self.event_positions.len()
    }

    pub fn get_track_length(&self, track: usize) -> usize {
        let mut highest_tick = 0;
        for (_, (current_track, test_tick)) in self.event_positions.iter() {
            highest_tick = max(highest_tick, *test_tick);
        }

        highest_tick + 1
    }

    pub fn set_ppqn(&mut self, new_ppqn: u16) {
        self.ppqn = new_ppqn;
    }

    pub fn get_ppqn(&self) -> u16 {
        self.ppqn
    }

    pub fn set_format(&mut self, new_format: u16) {
        self.midi_format = new_format;
    }

    pub fn get_format(&self) -> u16 {
        self.midi_format
    }

    pub fn move_event(&mut self, new_track: usize, new_tick: usize, event_id: u64) {
        self.event_positions.entry(event_id)
            .and_modify(|pair| { *pair = (new_track, new_tick); })
            .or_insert((new_track, new_tick));
    }

    pub fn insert_event(&mut self, track: usize, tick: usize, event: Box<dyn MIDIEvent>) -> u64 {
        let new_event_id = self.event_id_gen;
        self.event_id_gen += 1;

        self.events.insert(new_event_id, event);

        self.move_event(track, tick, new_event_id);

        new_event_id
    }

    pub fn push_event(&mut self, track: usize, wait: usize, event: Box<dyn MIDIEvent>) -> u64 {
        let new_event_id = self.event_id_gen;

        self.events.insert(new_event_id, event);
        self.event_id_gen += 1;

        let last_tick_in_track = self.get_track_length(track) - 1;
        self.move_event(track, last_tick_in_track + wait, new_event_id);

        new_event_id
    }

    pub fn get_event_mut(&mut self, event_id: u64) -> Option<&mut Box<dyn MIDIEvent>> {
        match self.events.get_mut(&event_id) {
            Some(event) => {
                Some(event)
            }
            None => {
                None
            }
        }
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

pub struct ApresController {
    event_queue: Vec<Box<dyn MIDIEvent>>,
    byte_buffer: Vec<u8>,
    pipe: File
}

impl ApresController {
    pub fn new(path: &str) -> ApresController {
        ApresController {
            event_queue: Vec::new(),
            byte_buffer: Vec::new(),
            pipe: File::open(path).unwrap()
        }
    }

    // TODO: Implment timeout
    fn get_next_byte(&mut self) -> u8 {
        let mut buffer = [0;1];
        while true {
            match self.pipe.read_exact(&mut buffer) {
                Ok(success) => {
                    break;
                }
                Err(e) => {
                }
            }
        }
        buffer[0]
    }

    // TODO: implement the rest of the relevent events (not just channel events)
    pub fn get_next(&mut self) -> Option<Box<dyn MIDIEvent>> {
        let lead_byte = self.get_next_byte();
        match lead_byte {
            0..=0x7F => {
                None
            }
            0x80..=0xEF => {
                let channel = lead_byte & 0x0F;
                match lead_byte & 0xF0 {
                    0x80 => {
                        let b = self.get_next_byte();
                        let c = self.get_next_byte();
                        Some(NoteOffEvent::new(channel, b, c))
                    }
                    0x90 => {
                        let b = self.get_next_byte();
                        let c = self.get_next_byte();
                        if c == 0 {
                            Some(NoteOffEvent::new(channel, b, c))
                        } else {
                            Some(NoteOnEvent::new(channel, b, c))
                        }
                    }
                    0xA0 => {
                        let b = self.get_next_byte();
                        let c = self.get_next_byte();
                        Some(AfterTouchEvent::new(channel, b, c))
                    }
                    0xB0 => {
                        let b = self.get_next_byte();
                        let c = self.get_next_byte();
                        Some(ControlChangeEvent::new(channel, b, c))
                    }
                    0xC0 => {
                        let b = self.get_next_byte();
                        Some(ProgramChangeEvent::new(channel, b))
                    }
                    0xD0 => {
                        let b = self.get_next_byte();
                        Some(ChannelPressureEvent::new(channel, b))
                    }
                    0xE0 => {
                        let b = self.get_next_byte();
                        let c = self.get_next_byte();
                        Some(PitchWheelChangeEvent::new_from_lsb_msb(channel, b, c))
                    }
                    _ => { None }
                }
            }
            0xF0 => {
                None
            }
            0xF2 => {
                None
            }
            0xF3 => {
                None
            }
            // System real-time events
            0xF6 => {
                let b = self.get_next_byte();
                None
            }
            0xF8 => {
                Some(Box::new(MIDIClockEvent {}))
            }
            0xFA => {
                Some(Box::new(MIDIStartEvent {}))
            }
            0xFB => {
                Some(Box::new(MIDIContinueEvent {}))
            }
            0xFC => {
                Some(Box::new(MIDIStopEvent {}))
            }
            0xFE => {
                Some(Box::new(ActiveSenseEvent {}))
            }
            // 0xFF is NOT meta for controllers
            0xFF => {
                Some(Box::new(ResetEvent {}))
            }
            // Undefined behaviour (as specified in MIDI standard)
            0xF1 | 0xF4 | 0xF5 => {
                None
            }
            // Undefined behaviour
            _ => {
                None
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
                n |= (x & 0x7F) as u64;
                if x & 0x80 == 0 {
                    break;
                }
            }
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

        if ! first_pass {
            tmp |= 0x80;
        }

        output.push(tmp as u8);
        first_pass = false;
    }
    output.reverse();

    output
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initialize_load() {
        let midi_bytes = vec![
            0x4D, 0x54, 0x68, 0x64, // MThd
            0x00, 0x00, 0x00, 0x06, // Length
            0x00, 0x01, // format = 1
            0x00, 0x01, // track count = 1
            0x00, 0x1b, // bytes in midi (excluding Mthd) = 20 expressed as variable length
            0x4D, 0x54, 0x72, 0x6B, // MTrk
            0x00, 0x00, 0x00, 92, // Length

            // EVENTS (Do not change order)

            // Meta Events
            // TimeSignature
            0x00, 0xFF, 0x58, 0x04, 0x04, 0x04, 0x24, 0x04,
            // Text ("ABC")
            0x00, 0xFF, 0x01, 0x03, 0x41, 0x42, 0x43,
            // CopyRightNotice ("ABC")
            0x00, 0xFF, 0x02, 0x03, 0x41, 0x42, 0x43,
            // TrackName ("ABC")
            0x00, 0xFF, 0x03, 0x03, 0x41, 0x42, 0x43,
            // InstrumentName ("ABC")
            0x00, 0xFF, 0x04, 0x03, 0x41, 0x42, 0x43,
            // Lyric ("ABC")
            0x00, 0xFF, 0x05, 0x03, 0x41, 0x42, 0x43,
            // Marker ("ABC")
            0x00, 0xFF, 0x06, 0x03, 0x41, 0x42, 0x43,
            // CuePoint ("ABC")
            0x00, 0xFF, 0x07, 0x03, 0x41, 0x42, 0x43,

            // Channel Prefix
            0x00, 0xFF, 0x20, 0x01, 0x00,

            // SetTempo (~210)
            0x00, 0xFF, 0x51, 0x03, 0x04, 0x5c, 0x12,

            // Channel Events
            // NoteOn
            0x00, 0x90, 0x40, 0x18,
            // AfterTouch
            0x30, 0xA0, 0x40, 0x50,
            // PitchWheel
            0x10, 0xE0, 0x00, 0x00,
            // ChannelPressure
            0x00, 0xD0, 0x2F,
            // NoteOff
            0x38, 0x80, 0x00, 0x40,


            // EOTs are ignored
            0x00, 0xFF, 0x2F, 0x00 // EOT
        ];
        let midi = MIDI::from_bytes(midi_bytes);

        assert_eq!(midi.count_tracks(), 1);
        assert_eq!(midi.get_track_length(0), 121);
        assert_eq!(midi.events.len(), 15);
        assert_eq!(midi.event_positions.len(), 15);

    }

    #[test]
    fn test_add_event() {
        let mut midi = MIDI::new();
        let on_event = midi.push_event(0, 0, NoteOnEvent::new(0, 64, 100));
        let off_event = midi.push_event(0, 119, NoteOffEvent::new(0, 64, 0));

        assert_eq!(on_event, 1);
        assert_eq!(off_event, 2);
        assert_eq!(midi.events.len(), 2);
        assert_eq!(midi.event_positions.len(), 2);
        assert_eq!(midi.count_tracks(), 1);
        assert_eq!(midi.get_track_length(0), 120);
    }

    #[test]
    fn test_variable_length_conversion() {
        let mut test_cases = vec![
            (0, vec![0]),
            (127, vec![0x7F]),
            (128, vec![0x81, 0x00]),
            (2097151, vec![0xFF, 0xFF, 0x7F])
        ];
        let mut output_vector;
        let mut output_n;
        for (input_number, expected_vector) in test_cases.iter_mut() {
            output_vector = to_variable_length_bytes(*input_number);
            assert_eq!(
                output_vector.as_slice(),
                expected_vector.as_slice()
            );


            expected_vector.reverse();
            output_n = get_variable_length_number(expected_vector);
            assert_eq!(
                *input_number,
                output_n as usize
            );
        }



    }


    #[test]
    fn test_sequence_number_event() {
        let mut event = SequenceNumberEvent::new(1);
        assert_eq!(event.as_bytes().as_slice(), &[0xFF, 0x00, 0x02, 0x00, 0x01]);
        event.set_sequence(13607);
        assert_eq!(event.as_bytes().as_slice(), &[0xFF, 0x00, 0x02, 0x35, 0x27]);

        assert_eq!(event.get_category(), MIDICategory::Meta);
        assert_eq!(event.get_type(), MIDIEventType::SequenceNumber);
    }

    #[test]
    fn test_text_event() {
        let mut some_text = "This is some text".to_string();
        let mut text_len_bytes = to_variable_length_bytes(some_text.len());

        let mut event = TextEvent::new(some_text.clone());
        let mut compare_vec = vec![ 0xFF, 0x01 ];
        compare_vec.extend(text_len_bytes.iter().copied());
        compare_vec.extend(some_text.as_bytes().iter().copied());
        assert_eq!(
            event.as_bytes().as_slice(),
            compare_vec.as_slice()
        );

        assert_eq!(event.get_category(), MIDICategory::Meta);
        assert_eq!(event.get_type(), MIDIEventType::Text);
    }

    #[test]
    fn test_copyright_notice_event() {
        let mut some_text = "This is some text".to_string();
        let mut text_len_bytes = to_variable_length_bytes(some_text.len());

        let mut event = CopyRightNoticeEvent::new(some_text.clone());
        let mut compare_vec = vec![ 0xFF, 0x02 ];
        compare_vec.extend(text_len_bytes.iter().copied());
        compare_vec.extend(some_text.as_bytes().iter().copied());
        assert_eq!(
            event.as_bytes().as_slice(),
            compare_vec.as_slice()
        );
        assert_eq!(event.get_category(), MIDICategory::Meta);
        assert_eq!(event.get_type(), MIDIEventType::CopyRightNotice);
    }

    #[test]
    fn test_track_name_event() {
        let mut some_text = "Some Track Name".to_string();
        let mut text_len_bytes = to_variable_length_bytes(some_text.len());

        let mut event = TrackNameEvent::new(some_text.clone());
        let mut compare_vec = vec![ 0xFF, 0x03 ];
        compare_vec.extend(text_len_bytes.iter().copied());
        compare_vec.extend(some_text.as_bytes().iter().copied());
        assert_eq!(
            event.as_bytes().as_slice(),
            compare_vec.as_slice()
        );

        assert_eq!(event.get_category(), MIDICategory::Meta);
        assert_eq!(event.get_type(), MIDIEventType::TrackName);
    }

    #[test]
    fn test_instrument_name_event() {
        let mut some_text = "Some Instrument Name".to_string();
        let mut text_len_bytes = to_variable_length_bytes(some_text.len());

        let mut event = InstrumentNameEvent::new(some_text.clone());
        let mut compare_vec = vec![ 0xFF, 0x04 ];
        compare_vec.extend(text_len_bytes.iter().copied());
        compare_vec.extend(some_text.as_bytes().iter().copied());
        assert_eq!(
            event.as_bytes().as_slice(),
            compare_vec.as_slice()
        );
        assert_eq!(event.get_category(), MIDICategory::Meta);
        assert_eq!(event.get_type(), MIDIEventType::InstrumentName);
    }

    #[test]
    fn test_lyric_event() {
        let mut some_text = "Here are some Lyrics.".to_string();
        let mut text_len_bytes = to_variable_length_bytes(some_text.len());

        let mut event = LyricEvent::new(some_text.clone());
        let mut compare_vec = vec![ 0xFF, 0x05 ];
        compare_vec.extend(text_len_bytes.iter().copied());
        compare_vec.extend(some_text.as_bytes().iter().copied());
        assert_eq!(
            event.as_bytes().as_slice(),
            compare_vec.as_slice()
        );
        assert_eq!(event.get_category(), MIDICategory::Meta);
        assert_eq!(event.get_type(), MIDIEventType::Lyric);
    }

    #[test]
    fn test_marker_event() {
        let mut some_text = "marker text".to_string();
        let mut text_len_bytes = to_variable_length_bytes(some_text.len());

        let mut event = MarkerEvent::new(some_text.clone());
        let mut compare_vec = vec![ 0xFF, 0x06 ];
        compare_vec.extend(text_len_bytes.iter().copied());
        compare_vec.extend(some_text.as_bytes().iter().copied());
        assert_eq!(
            event.as_bytes().as_slice(),
            compare_vec.as_slice()
        );
        assert_eq!(event.get_category(), MIDICategory::Meta);
        assert_eq!(event.get_type(), MIDIEventType::Marker);
    }

    #[test]
    fn test_cue_point_event() {
        let mut some_text = "cue point text".to_string();
        let mut text_len_bytes = to_variable_length_bytes(some_text.len());

        let mut event = CuePointEvent::new(some_text.clone());
        let mut compare_vec = vec![ 0xFF, 0x07 ];
        compare_vec.extend(text_len_bytes.iter().copied());
        compare_vec.extend(some_text.as_bytes().iter().copied());
        assert_eq!(
            event.as_bytes().as_slice(),
            compare_vec.as_slice()
        );
        assert_eq!(event.get_category(), MIDICategory::Meta);
        assert_eq!(event.get_type(), MIDIEventType::CuePoint);
    }

    #[test]
    fn test_end_of_track_event() {
        let mut event = EndOfTrackEvent::new();
        assert_eq!(
            event.as_bytes().as_slice(),
            [0xFF, 0x2F, 0x00]
        );
        assert_eq!(event.get_category(), MIDICategory::Meta);
        assert_eq!(event.get_type(), MIDIEventType::EndOfTrack);
    }

    #[test]
    fn test_channel_prefix_event() {
        let mut event = ChannelPrefixEvent::new(0);
        for i in std::u8::MIN .. std::u8::MAX {
            event.set_channel(i as u8);
            assert_eq!(
                event.as_bytes().as_slice(),
                [0xFF, 0x20, 0x01, i]
            );
        }
    }

    #[test]
    fn test_set_tempo_event() {
        let mut event = SetTempoEvent::new(500000);
        let test_cases_bpm = vec![
            (120, 500000),
            (280, 214285),
            (1, 0x00FFFFFF),// Minimum bpm is 3.576278762788
            (60_000_000, 1)
        ];
        for (bpm, expected_uspqn) in test_cases_bpm.iter() {
            event.set_bpm(*bpm as f64);
            assert_eq!(*expected_uspqn, event.get_uspqn());
            assert_eq!(
                event.as_bytes().as_slice(),
                [
                    0xFF, 0x51, 0x03,
                    ((*expected_uspqn / 256u32.pow(2)) % 256) as u8,
                    ((*expected_uspqn / 256u32.pow(1)) % 256) as u8,
                    (*expected_uspqn % 256) as u8
                ]
            );
        }
    }

    #[test]
    fn test_smpte_offset_event() {
        let mut event = SMPTEOffsetEvent::new(1,2,3,4,5);
        assert_eq!(
            event.as_bytes().as_slice(),
            [0xFF, 0x54, 0x05, 0x01, 0x02, 0x03, 0x04, 0x05]
        );
    }

    #[test]
    fn test_time_signature_event() {
        let mut event = TimeSignatureEvent::new(4, 4, 32, 3);
        assert_eq!(
            event.as_bytes().as_slice(),
            [0xFF, 0x58, 0x04, 0x04, 0x04, 0x20, 0x03]
        );
    }

    #[test]
    fn test_key_signature_event() {
        let mut event = KeySignatureEvent::new("A".to_string());
        assert_eq!(
            event.as_bytes().as_slice(),
            [0xFF, 0x59, 0x02, 0x03, 0x00]
        );
    }

    #[test]
    fn test_sequence_specific_event() {
        let mut event = SequencerSpecificEvent::new(vec![0x00]);
        assert_eq!(
            event.as_bytes().as_slice(),
            [0xFF, 0x7F, 0x01, 0x00]
        );
    }

    #[test]
    fn test_note_on_event() {
        let mut event = NoteOnEvent::new(14, 23, 33);
        assert_eq!(
            event.as_bytes().as_slice(),
            [0x9E, 0x17, 0x21]
        );
    }

    #[test]
    fn test_note_off_event() {
        let mut event = NoteOffEvent::new(14, 23, 33);
        assert_eq!(
            event.as_bytes().as_slice(),
            [0x8E, 0x17, 0x21]
        );
    }

    #[test]
    fn test_aftertouch_event() {
        let mut event = AfterTouchEvent::new(14, 23, 33);
        assert_eq!(
            event.as_bytes().as_slice(),
            [0xAE, 0x17, 0x21]
        );
    }

    #[test]
    fn test_control_change_event() {
        let mut event = ControlChangeEvent::new(14, 23, 33);
        assert_eq!(
            event.as_bytes().as_slice(),
            [0xBE, 0x17, 0x21]
        );
    }

    #[test]
    fn test_program_change_event() {
        let mut event = ProgramChangeEvent::new(14, 23);
        assert_eq!(
            event.as_bytes().as_slice(),
            [0xCE, 0x17]
        );
    }

    #[test]
    fn test_channel_pressure_event() {
        let mut event = ChannelPressureEvent::new(14, 23);
        assert_eq!(
            event.as_bytes().as_slice(),
            [0xDE, 0x17]
        );
    }

    #[test]
    fn test_pitchwheel_change_event() {
        let mut event = PitchWheelChangeEvent::new(14, 0.0);
        let mut test_cases: Vec<(f64, (u8, u8))> = vec![
            (-1.0, (0, 0)),
            (-0.5, (0x20, 0x00)),
            (0.0, (0x40, 0x00)),
            (0.5, (0x5f, 0x7F)),
            (1.0, (0x7F, 0x7F))
        ];
        for (input_value, (msb, lsb)) in test_cases.iter() {
            event.set_value(*input_value);
            assert_eq!(
                event.as_bytes().as_slice(),
                [0xEE, *lsb, *msb]
            );
        }
    }

    #[test]
    fn test_system_exclusive_event() {
        let mut event = SystemExclusiveEvent::new(vec![0,0,1,0]);
        assert_eq!(
            event.as_bytes().as_slice(),
            [0xF0, 0x00, 0x00, 0x01, 0x00, 0xF7]
        );
    }
}

