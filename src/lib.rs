use std::fs::File;
use std::io::prelude::*;
use std::cmp::max;
use std::collections::{HashMap, HashSet};

pub mod controller;

use controller::Controller;

#[derive(Debug)]
pub enum ApresError {
    InvalidMIDIFile(String),
    InvalidBytes(Vec<u8>),
    UnknownMetaEvent(Vec<u8>),
    EventNotFound(u64),
    IllegibleString(Vec<u8>),
    PathNotFound(String),
    PipeBroken,
    TrackOutOfBounds,
    Killed,
    MissingHeader
}


#[derive(Clone, Debug, PartialEq)]
pub enum MIDIEvent {
	SequenceNumber(u16),
	Text(String),
	CopyRightNotice(String),
	TrackName(String),
	InstrumentName(String),
	Lyric(String),
	Marker(String),
	CuePoint(String),
	ChannelPrefix(u8),
    // Note: Tempo Stored in u32 but is a 3 byte value
	SetTempo(u32),
	SMPTEOffset(u8, u8, u8, u8, u8),
	TimeSignature(u8, u8, u8, u8),
	KeySignature(String),
    SequencerSpecific(Vec<u8>),

	NoteOn(u8, u8, u8),
	NoteOff(u8, u8, u8),
	AfterTouch(u8, u8, u8),

    BankSelect(u8, u8),
    BankSelectLSB(u8, u8),
    ModulationWheel(u8, u8),
    ModulationWheelLSB(u8, u8),
    BreathController(u8, u8),
    BreathControllerLSB(u8, u8),
    FootPedal(u8, u8),
    FootPedalLSB(u8, u8),
    PortamentoTime(u8, u8),
    PortamentoTimeLSB(u8, u8),
    DataEntry(u8, u8),
    DataEntryLSB(u8, u8),
    Volume(u8, u8),
    VolumeLSB(u8, u8),
    Balance(u8, u8),
    BalanceLSB(u8, u8),
    Pan(u8, u8),
    PanLSB(u8, u8),
    Expression(u8, u8),
    ExpressionLSB(u8, u8),
    EffectControl1(u8, u8),
    EffectControl1LSB(u8, u8),
    EffectControl2(u8, u8),
    EffectControl2LSB(u8, u8),
    GeneralPurpose1(u8, u8),
    GeneralPurpose1LSB(u8, u8),
    GeneralPurpose2(u8, u8),
    GeneralPurpose2LSB(u8, u8),
    GeneralPurpose3(u8, u8),
    GeneralPurpose3LSB(u8, u8),
    GeneralPurpose4(u8, u8),
    GeneralPurpose4LSB(u8, u8),
	HoldPedal(u8, u8),
	Portamento(u8, u8),
	Sustenuto(u8, u8),
	SoftPedal(u8, u8),
	Legato(u8, u8),
	Hold2Pedal(u8, u8),
	SoundVariation(u8, u8),
	SoundTimbre(u8, u8),
	SoundReleaseTime(u8, u8),
	SoundAttack(u8, u8),
	SoundBrightness(u8, u8),
	SoundControl1(u8, u8),
	SoundControl2(u8, u8),
	SoundControl3(u8, u8),
	SoundControl4(u8, u8),
	SoundControl5(u8, u8),
    GeneralPurpose5(u8, u8),
    GeneralPurpose6(u8, u8),
    GeneralPurpose7(u8, u8),
    GeneralPurpose8(u8, u8),
	EffectsLevel(u8, u8),
	TremuloLevel(u8, u8),
	ChorusLevel(u8, u8),
	CelesteLevel(u8, u8),
	PhaserLevel(u8, u8),
	DataIncrement(u8),
	DataDecrement(u8),
    RegisteredParameterNumber(u8, u8),
    RegisteredParameterNumberLSB(u8, u8),
    NonRegisteredParameterNumber(u8, u8),
    NonRegisteredParameterNumberLSB(u8, u8),
	AllControllersOff(u8),
	LocalControl(u8, u8),
	AllNotesOff(u8),
	AllSoundOff(u8),
	OmniOff(u8),
	OmniOn(u8),
	MonophonicOperation(u8, u8),
	PolyphonicOperation(u8),
	ControlChange(u8, u8, u8),

	ProgramChange(u8, u8),
	ChannelPressure(u8, u8),
	PitchWheelChange(u8, f64),
	SystemExclusive(Vec<u8>),
	MTCQuarterFrame(u8, u8),
	SongPositionPointer(u16),
	SongSelect(u8),
    TimeCode(f32, u8, u8, u8, u8),

	EndOfTrack,
    TuneRequest,
    MIDIClock,
    MIDIStart,
    MIDIContinue,
    MIDIStop,
    ActiveSense,
    Reset
}

pub trait MIDIBytes {
    fn as_bytes(&self) -> Vec<u8>;
    fn from_bytes(bytes: &mut Vec<u8>, default_byte: u8) -> Result<Self, ApresError> where Self: std::marker::Sized;
}

impl MIDIBytes for MIDIEvent {
    fn as_bytes(&self) -> Vec<u8> {
        match self {
            MIDIEvent::SequenceNumber(sequence) => {
                vec![
                    0xFF, 0x00, 0x02,
                    (sequence / 256) as u8,
                    (sequence % 256) as u8
                ]
            }

            MIDIEvent::Text(text) => {
                let text_bytes = text.as_bytes();
                let length_bytes = to_variable_length_bytes(text_bytes.len());

                let mut output = vec![0xFF, 0x01];
                output.extend(length_bytes.iter().copied());
                output.extend(text_bytes.iter().copied());

                output
            }

            MIDIEvent::CopyRightNotice(text) => {
                let text_bytes = text.as_bytes();
                let length_bytes = to_variable_length_bytes(text_bytes.len());

                let mut output = vec![0xFF, 0x02];
                output.extend(length_bytes.iter().copied());
                output.extend(text_bytes.iter().copied());

                output
            }

            MIDIEvent::TrackName(track_name) => {
                let text_bytes = track_name.as_bytes();
                let length_bytes = to_variable_length_bytes(text_bytes.len());

                let mut output = vec![0xFF, 0x03];
                output.extend(length_bytes.iter().copied());
                output.extend(text_bytes.iter().copied());

                output
            }

            MIDIEvent::InstrumentName(instrument_name) => {
                let text_bytes = instrument_name.as_bytes();
                let length_bytes = to_variable_length_bytes(text_bytes.len());

                let mut output = vec![0xFF, 0x04];
                output.extend(length_bytes.iter().copied());
                output.extend(text_bytes.iter().copied());

                output
            }

            MIDIEvent::Lyric(lyric) => {
                let text_bytes = lyric.as_bytes();
                let length_bytes = to_variable_length_bytes(text_bytes.len());

                let mut output = vec![0xFF, 0x05];
                output.extend(length_bytes.iter().copied());
                output.extend(text_bytes.iter().copied());

                output
            }

            MIDIEvent::Marker(text) => {
                let text_bytes = text.as_bytes();
                let length_bytes = to_variable_length_bytes(text_bytes.len());

                let mut output = vec![0xFF, 0x06];
                output.extend(length_bytes.iter().copied());
                output.extend(text_bytes.iter().copied());

                output
            }

            MIDIEvent::CuePoint(text) => {
                let text_bytes = text.as_bytes();
                let length_bytes = to_variable_length_bytes(text_bytes.len());

                let mut output = vec![0xFF, 0x07];
                output.extend(length_bytes.iter().copied());
                output.extend(text_bytes.iter().copied());

                output
            }

            MIDIEvent::EndOfTrack => {
                vec![0xFF, 0x2F, 0x00]
            }

            MIDIEvent::ChannelPrefix(channel) => {
                vec![0xFF, 0x20, 0x01, *channel]
            }

            MIDIEvent::SetTempo(us_per_quarter_note) => {
                vec![
                    0xFF, 0x51, 0x03,
                    ((*us_per_quarter_note / 256u32.pow(2)) % 256) as u8,
                    ((*us_per_quarter_note / 256u32.pow(1)) % 256) as u8,
                    (*us_per_quarter_note % 256) as u8,
                ]
            }

            //TODO: Figure out what ff/fr are, u16 for now
            MIDIEvent::SMPTEOffset(hour, minute, second, ff, fr) => {
                vec![0xFF, 0x54, 05, *hour, *minute, *second, *ff, *fr]
            }

            MIDIEvent::TimeSignature(numerator, denominator, clocks_per_metronome, thirtysecondths_per_quarter) => {
                vec![0xFF, 0x58, 04, *numerator, *denominator, *clocks_per_metronome, *thirtysecondths_per_quarter]
            }

            MIDIEvent::KeySignature(string) => {
                let (mi, sf) = get_mi_sf(string);
                vec![0xFF, 0x59, 0x02, sf, mi]
            }

            MIDIEvent::SequencerSpecific(data) => {
                let mut output: Vec<u8> = vec![0xFF, 0x7F];
                output.push(data.len() as u8); // Data length is limited to 1 byte
                output.extend(data.iter().copied());
                output
            }

            MIDIEvent::NoteOn(channel, note, velocity) => {
                vec![
                    0x90 | *channel,
                    *note,
                    *velocity
                ]
            }

            MIDIEvent::NoteOff(channel, note, velocity) => {
                vec![
                    0x80 | *channel,
                    *note,
                    *velocity
                ]
            }

            MIDIEvent::AfterTouch(channel, note, pressure) => {
                vec![
                    0xA0 | *channel,
                     *note,
                     *pressure
                ]
            }

            MIDIEvent::BankSelect(channel, value) => {
                vec![ 0xB0 | *channel, 0x00, *value ]
            }
            MIDIEvent::BankSelectLSB(channel, value) => {
                vec![ 0xB0 | *channel, 0x20, *value ]
            }

            MIDIEvent::ModulationWheel(channel, value) => {
                vec![ 0xB0 | *channel, 0x01, *value ]
            }
            MIDIEvent::ModulationWheelLSB(channel, value) => {
                vec![ 0xB0 | *channel, 0x21, *value ]
            }

            MIDIEvent::BreathController(channel, value) => {
                vec![ 0xB0 | *channel, 0x02, *value ]
            }
            MIDIEvent::BreathControllerLSB(channel, value) => {
                vec![ 0xB0 | *channel, 0x22, *value ]
            }

            MIDIEvent::FootPedal(channel, value) => {
                vec![ 0xB0 | *channel, 0x04, *value ]
            }
            MIDIEvent::FootPedalLSB(channel, value) => {
                vec![ 0xB0 | *channel, 0x24, *value ]
            }

            MIDIEvent::PortamentoTime(channel, value) => {
                vec![ 0xB0 | *channel, 0x05, *value ]
            }
            MIDIEvent::PortamentoTimeLSB(channel, value) => {
                vec![ 0xB0 | *channel, 0x25, *value ]
            }

            MIDIEvent::DataEntry(channel, value) => {
                vec![ 0xB0 | *channel, 0x06, *value ]
            }
            MIDIEvent::DataEntryLSB(channel, value) => {
                vec![ 0xB0 | *channel, 0x26, *value ]
            }

            MIDIEvent::Volume(channel, value) => {
                vec![ 0xB0 | *channel, 0x07, *value ]
            }
            MIDIEvent::VolumeLSB(channel, value) => {
                vec![ 0xB0 | *channel, 0x27, *value ]
            }

            MIDIEvent::Balance(channel, value) => {
                vec![ 0xB0 | *channel, 0x08, *value ]
            }
            MIDIEvent::BalanceLSB(channel, value) => {
                vec![ 0xB0 | *channel, 0x28, *value ]
            }

            MIDIEvent::Pan(channel, value) => {
                vec![ 0xB0 | *channel, 0x0A, *value ]
            }
            MIDIEvent::PanLSB(channel, value) => {
                vec![ 0xB0 | *channel, 0x2A, *value ]
            }

            MIDIEvent::Expression(channel, value) => {
                vec![ 0xB0 | *channel, 0x0B, *value ]
            }
            MIDIEvent::ExpressionLSB(channel, value) => {
                vec![ 0xB0 | *channel, 0x2B, *value ]
            }

            MIDIEvent::EffectControl1(channel, value) => {
                vec![ 0xB0 | *channel, 0x0C, *value ]
            }
            MIDIEvent::EffectControl1LSB(channel, value) => {
                vec![ 0xB0 | *channel, 0x2C, *value ]
            }
            MIDIEvent::EffectControl2(channel, value) => {
                vec![ 0xB0 | *channel, 0x0D, *value ]
            }
            MIDIEvent::EffectControl2LSB(channel, value) => {
                vec![ 0xB0 | *channel, 0x2D, *value ]
            }

            MIDIEvent::GeneralPurpose1(channel, value) => {
                vec![ 0xB0 | *channel, 0x10, *value ]
            }
            MIDIEvent::GeneralPurpose1LSB(channel, value) => {
                vec![ 0xB0 | *channel, 0x30, *value ]
            }
            MIDIEvent::GeneralPurpose2(channel, value) => {
                vec![ 0xB0 | *channel, 0x11, *value ]
            }
            MIDIEvent::GeneralPurpose2LSB(channel, value) => {
                vec![ 0xB0 | *channel, 0x31, *value ]
            }
            MIDIEvent::GeneralPurpose3(channel, value) => {
                vec![ 0xB0 | *channel, 0x12, *value ]
            }
            MIDIEvent::GeneralPurpose3LSB(channel, value) => {
                vec![ 0xB0 | *channel, 0x32, *value ]
            }
            MIDIEvent::GeneralPurpose4(channel, value) => {
                vec![ 0xB0 | *channel, 0x13, *value ]
            }
            MIDIEvent::GeneralPurpose4LSB(channel, value) => {
                vec![ 0xB0 | *channel, 0x33, *value ]
            }
            MIDIEvent::GeneralPurpose5(channel, value) => {
                vec![ 0xB0 | *channel, 0x50, *value]
            }
            MIDIEvent::GeneralPurpose6(channel, value) => {
                vec![ 0xB0 | *channel, 0x51, *value]
            }
            MIDIEvent::GeneralPurpose7(channel, value) => {
                vec![ 0xB0 | *channel, 0x52, *value]
            }
            MIDIEvent::GeneralPurpose8(channel, value) => {
                vec![ 0xB0 | *channel, 0x53, *value]
            }

            MIDIEvent::HoldPedal(channel, value) => {
                vec![
                    0xB0 | *channel,
                    0x40,
                    *value
                ]
            }

            MIDIEvent::Portamento(channel, value) => {
                vec![
                    0xB0 | *channel,
                    0x41,
                    *value
                ]
            }

            MIDIEvent::Sustenuto(channel, value) => {
                vec![
                    0xB0 | *channel,
                    0x42,
                    *value
                ]
            }

            MIDIEvent::SoftPedal(channel, value) => {
                vec![
                    0xB0 | *channel,
                    0x43,
                    *value
                ]
            }

            MIDIEvent::Legato(channel, value) => {
                vec![
                    0xB0 | *channel,
                    0x44,
                    *value
                ]
            }

            MIDIEvent::Hold2Pedal(channel, value) => {
                vec![
                    0xB0 | *channel,
                    0x45,
                    *value
                ]
            }

            MIDIEvent::SoundVariation(channel, value) => {
                vec![
                    0xB0 | *channel,
                    0x46,
                    *value
                ]
            }

            MIDIEvent::SoundTimbre(channel, value) => {
                vec![
                    0xB0 | *channel,
                    0x47,
                    *value
                ]
            }

            MIDIEvent::SoundReleaseTime(channel, value) => {
                vec![
                    0xB0 | *channel,
                    0x48,
                    *value
                ]
            }

            MIDIEvent::SoundAttack(channel, value) => {
                vec![
                    0xB0 | *channel,
                    0x49,
                    *value
                ]
            }

            MIDIEvent::SoundBrightness(channel, value) => {
                vec![
                    0xB0 | *channel,
                    0x4A,
                    *value
                ]
            }

            MIDIEvent::SoundControl1(channel, value) => {
                vec![ 0xB0 | *channel, 0x4B, *value ]
            }
            MIDIEvent::SoundControl2(channel, value) => {
                vec![ 0xB0 | *channel, 0x4C, *value ]
            }
            MIDIEvent::SoundControl3(channel, value) => {
                vec![ 0xB0 | *channel, 0x4D, *value ]
            }
            MIDIEvent::SoundControl4(channel, value) => {
                vec![ 0xB0 | *channel, 0x4E, *value ]
            }
            MIDIEvent::SoundControl5(channel, value) => {
                vec![ 0xB0 | *channel, 0x4F, *value ]
            }

            MIDIEvent::EffectsLevel(channel, value) => {
                vec![
                    0xB0 | *channel,
                    0x5B,
                    *value
                ]
            }

            MIDIEvent::TremuloLevel(channel, value) => {
                vec![
                    0xB0 | *channel,
                    0x5C,
                    *value
                ]
            }

            MIDIEvent::ChorusLevel(channel, value) => {
                vec![
                    0xB0 | *channel,
                    0x5D,
                    *value
                ]
            }

            MIDIEvent::CelesteLevel(channel, value) => {
                vec![
                    0xB0 | *channel,
                    0x5E,
                    *value
                ]
            }

            MIDIEvent::PhaserLevel(channel, value) => {
                vec![
                    0xB0 | *channel,
                    0x5F,
                    *value
                ]
            }

            MIDIEvent::DataIncrement(channel) => {
                vec![0xB0 | *channel, 0x60, 0x00]
            }

            MIDIEvent::DataDecrement(channel) => {
                vec![0xB0 | *channel, 0x61, 0x00]
            }

            MIDIEvent::NonRegisteredParameterNumber(channel, value) => {
                vec![ 0xB0 | *channel, 0x63, *value ]
            }
            MIDIEvent::NonRegisteredParameterNumberLSB(channel, value) => {
                vec![ 0xB0 | *channel, 0x62, *value ]
            }
            MIDIEvent::RegisteredParameterNumber(channel, value) => {
                vec![ 0xB0 | *channel, 0x65, *value ]
            }
            MIDIEvent::RegisteredParameterNumberLSB(channel, value) => {
                vec![ 0xB0 | *channel, 0x64, *value ]
            }


            MIDIEvent::AllControllersOff(channel) => {
                vec![
                    0xB0 | *channel,
                    0x79, 0x00
                ]
            }

            MIDIEvent::LocalControl(channel, value) => {
                vec![ 0xB0 | *channel, 0x7A, *value ]
            }

            MIDIEvent::AllNotesOff(channel) => {
                vec![
                    0xB0 | *channel,
                    0x7B, 0x00
                ]
            }

            MIDIEvent::AllSoundOff(channel) => {
                vec![
                    0xB0 | *channel,
                    0x78, 0x00
                ]
            }

            MIDIEvent::OmniOff(channel) => {
                vec![
                    0xB0 | *channel,
                    0x7C, 0x00
                ]
            }

            MIDIEvent::OmniOn(channel) => {
                vec![
                    0xB0 | *channel,
                    0x7D, 0x00
                ]
            }

            MIDIEvent::MonophonicOperation(channel, value) => {
                vec![
                    0xB0 | *channel, 0xFE, *value
                ]
            }

            MIDIEvent::PolyphonicOperation(channel) => {
                vec![
                    0xB0 | *channel, 0xFF, 0
                ]
            }

            MIDIEvent::ControlChange(channel, controller, value) => {
                vec![
                    0xB0 | *channel,
                    *controller,
                    *value
                ]
            }

            MIDIEvent::ProgramChange(channel, program) => {
                vec![
                    0xC0 | *channel,
                    *program
                ]
            }

            MIDIEvent::ChannelPressure(channel, pressure) => {
                vec![
                    0xD0 | *channel,
                    *pressure
                ]
            }

            MIDIEvent::PitchWheelChange(channel, value) => {
                let unsigned_value = get_pitchwheel_value(*value);
                let lsb: u8 = (unsigned_value & 0x007F) as u8;
                let msb: u8 = ((unsigned_value >> 8) & 0x007F) as u8;
                vec![
                    0xE0 | *channel,
                    lsb,
                    msb
                ]
            }

            MIDIEvent::SystemExclusive(data) => {
                let mut output = vec![0xF0];
                output.extend(data.iter().copied());
                output.push(0xF7);
                output
            }

            MIDIEvent::MTCQuarterFrame(message_type, values) => {
                let mut b = 0;
                b |= *message_type;
                b <<= 3;
                b |= *values;

                vec![ 0xF1, b ]
            }

            MIDIEvent::TimeCode(rate, hour, minute, second, frame) => {
                let coded_rate = {
                    if *rate == 24.0 {
                        0
                    } else if *rate == 25.0 {
                        1
                    } else if *rate == 27.97 {
                        2
                    } else if *rate == 30.0 {
                        3
                    } else {
                        3
                    }
                };

                let first_byte: u8 = (coded_rate << 5) + hour;
                vec![0xF1, first_byte, *minute, *second, *frame]
            }

            MIDIEvent::SongPositionPointer(beat) => {
                vec![
                    0xF2,
                    (*beat & 0x7F) as u8,
                    ((*beat >> 8) & 0x7F) as u8
                ]
            }

            MIDIEvent::SongSelect(song) => {
                vec![
                    0xF3,
                    *song & 0x7F
                ]
            }

            MIDIEvent::TuneRequest => {
                vec![ 0xF6 ]
            }
            MIDIEvent::MIDIClock => {
                vec![ 0xF8 ]
            }
            MIDIEvent::MIDIStart => {
                vec![ 0xFA ]
            }
            MIDIEvent::MIDIContinue => {
                vec![ 0xFB ]
            }
            MIDIEvent::MIDIStop => {
                vec![ 0xFC ]
            }
            MIDIEvent::ActiveSense => {
                vec![ 0xFE ]
            }
            MIDIEvent::Reset => {
                vec![ 0xFF ]
            }
        }
    }

    fn from_bytes(bytes: &mut Vec<u8>, default_byte: u8) -> Result<MIDIEvent, ApresError> {
        let mut output = Err(ApresError::InvalidBytes(bytes.clone()));

        let varlength: u64;
        let leadbyte = bytes.remove(0);

        match leadbyte {
            0..=0x7F => {
                bytes.insert(0, leadbyte);
                bytes.insert(0, default_byte);
                output = MIDIEvent::from_bytes(bytes, default_byte);
            }

            0x80..=0xEF => {
                let channel: u8;
                let leadnibble: u8 = leadbyte >> 4;
                match leadnibble {
                    0x8 => {
                        channel = leadbyte & 0x0F;
                        let note = bytes.remove(0);
                        let velocity = bytes.remove(0);
                        let event = MIDIEvent::NoteOff(channel, note, velocity);
                        output = Ok(event);
                    }
                    0x9 => {
                        channel = leadbyte & 0x0F;
                        let note = bytes.remove(0);
                        let velocity = bytes.remove(0);
                        // Convert fake NoteOff (NoteOn where velocity is 0) to real NoteOff
                        let event = if velocity == 0 {
                            MIDIEvent::NoteOff(channel, note, velocity)
                        } else {
                            MIDIEvent::NoteOn(channel, note, velocity)
                        };

                        output = Ok(event);
                    }
                    0xA => {
                        channel = leadbyte & 0x0F;
                        let note = bytes.remove(0);
                        let velocity = bytes.remove(0);
                        let event = MIDIEvent::AfterTouch(channel, note, velocity);
                        output = Ok(event);
                    }
                    0xB => {
                        channel = leadbyte & 0x0F;
                        let controller = bytes.remove(0);
                        let value = bytes.remove(0);
                        output = match controller {
                            0x00 => {
                                Ok(MIDIEvent::BankSelect(channel, value))
                            }
                            0x20 => {
                                Ok(MIDIEvent::BankSelectLSB(channel, value))
                            }
                            0x01 => {
                                Ok(MIDIEvent::ModulationWheel(channel, value))
                            }
                            0x21 => {
                                Ok(MIDIEvent::ModulationWheelLSB(channel, value))
                            }
                            0x02 => {
                                Ok(MIDIEvent::BreathController(channel, value))
                            }
                            0x22 => {
                                Ok(MIDIEvent::BreathControllerLSB(channel, value))
                            }
                            0x04 => {
                                Ok(MIDIEvent::FootPedal(channel, value))
                            }
                            0x24 => {
                                Ok(MIDIEvent::FootPedalLSB(channel, value))
                            }
                            0x05 => {
                                Ok(MIDIEvent::PortamentoTime(channel, value))
                            }
                            0x25 => {
                                Ok(MIDIEvent::PortamentoTimeLSB(channel, value))
                            }
                            0x06 => {
                                Ok(MIDIEvent::DataEntry(channel, value))
                            }
                            0x26 => {
                                Ok(MIDIEvent::DataEntryLSB(channel, value))
                            }
                            0x07 => {
                                Ok(MIDIEvent::Volume(channel, value))
                            }
                            0x27 => {
                                Ok(MIDIEvent::VolumeLSB(channel, value))
                            }
                            0x08 => {
                                Ok(MIDIEvent::Balance(channel, value))
                            }
                            0x28 => {
                                Ok(MIDIEvent::BalanceLSB(channel, value))
                            }
                            0x0A => {
                                Ok(MIDIEvent::Pan(channel, value))
                            }
                            0x2A => {
                                Ok(MIDIEvent::PanLSB(channel, value))
                            }
                            0x0B => {
                                Ok(MIDIEvent::Expression(channel, value))
                            }
                            0x2B => {
                                Ok(MIDIEvent::ExpressionLSB(channel, value))
                            }
                            0x0C => {
                                Ok(MIDIEvent::EffectControl1(channel, value))
                            }
                            0x2C => {
                                Ok(MIDIEvent::EffectControl1LSB(channel, value))
                            }
                            0x0D => {
                                Ok(MIDIEvent::EffectControl2(channel, value))
                            }
                            0x2D => {
                                Ok(MIDIEvent::EffectControl2LSB(channel, value))
                            }

                            0x10 => {
                                Ok(MIDIEvent::GeneralPurpose1(channel, value))
                            }
                            0x30 => {
                                Ok(MIDIEvent::GeneralPurpose1LSB(channel, value))
                            }
                            0x11 => {
                                Ok(MIDIEvent::GeneralPurpose2(channel, value))
                            }
                            0x31 => {
                                Ok(MIDIEvent::GeneralPurpose2LSB(channel, value))
                            }
                            0x12 => {
                                Ok(MIDIEvent::GeneralPurpose3(channel, value))
                            }
                            0x32 => {
                                Ok(MIDIEvent::GeneralPurpose3LSB(channel, value))
                            }
                            0x13 => {
                                Ok(MIDIEvent::GeneralPurpose4(channel, value))
                            }
                            0x33 => {
                                Ok(MIDIEvent::GeneralPurpose4LSB(channel, value))
                            }
                            0x40 => {
                                Ok(MIDIEvent::HoldPedal(channel, value))
                            }
                            0x41 => {
                                Ok(MIDIEvent::Portamento(channel, value))
                            }
                            0x42 => {
                                Ok(MIDIEvent::Sustenuto(channel, value))
                            }
                            0x43 => {
                                Ok(MIDIEvent::SoftPedal(channel, value))
                            }
                            0x44 => {
                                Ok(MIDIEvent::Legato(channel, value))
                            }
                            0x45 => {
                                Ok(MIDIEvent::Hold2Pedal(channel, value))
                            }
                            0x46 => {
                                Ok(MIDIEvent::SoundVariation(channel, value))
                            }
                            0x47 => {
                                Ok(MIDIEvent::SoundTimbre(channel, value))
                            }
                            0x48 => {
                                Ok(MIDIEvent::SoundReleaseTime(channel, value))
                            }
                            0x49 => {
                                Ok(MIDIEvent::SoundAttack(channel, value))
                            }
                            0x4A => {
                                Ok(MIDIEvent::SoundBrightness(channel, value))
                            }
                            0x4B => {
                                Ok(MIDIEvent::SoundControl1(channel, value))
                            }
                            0x4C => {
                                Ok(MIDIEvent::SoundControl2(channel, value))
                            }
                            0x4D => {
                                Ok(MIDIEvent::SoundControl3(channel, value))
                            }
                            0x4E => {
                                Ok(MIDIEvent::SoundControl4(channel, value))
                            }
                            0x4F => {
                                Ok(MIDIEvent::SoundControl5(channel, value))
                            }
                            0x50 => {
                                Ok(MIDIEvent::GeneralPurpose5(channel, value))
                            }
                            0x51 => {
                                Ok(MIDIEvent::GeneralPurpose6(channel, value))
                            }
                            0x52 => {
                                Ok(MIDIEvent::GeneralPurpose7(channel, value))
                            }
                            0x53 => {
                                Ok(MIDIEvent::GeneralPurpose8(channel, value))
                            }

                            0x5B => {
                                Ok(MIDIEvent::EffectsLevel(channel, value))
                            }

                            0x5C => {
                                Ok(MIDIEvent::TremuloLevel(channel, value))
                            }

                            0x5D => {
                                Ok(MIDIEvent::ChorusLevel(channel, value))
                            }
                            0x5E => {
                                Ok(MIDIEvent::CelesteLevel(channel, value))
                            }

                            0x5F => {
                                Ok(MIDIEvent::PhaserLevel(channel, value))
                            }

                            0x60 => {
                                Ok(MIDIEvent::DataIncrement(channel))
                            }

                            0x61 => {
                                Ok(MIDIEvent::DataDecrement(channel))
                            }
                            0x62 => {
                                Ok(MIDIEvent::NonRegisteredParameterNumberLSB(channel, value))
                            }

                            0x63 => {
                                Ok(MIDIEvent::NonRegisteredParameterNumber(channel, value))
                            }

                            0x64 => {
                                Ok(MIDIEvent::RegisteredParameterNumberLSB(channel, value))
                            }
                            0x65 => {
                                Ok(MIDIEvent::RegisteredParameterNumber(channel, value))
                            }
                            0x78 => {
                                Ok(MIDIEvent::AllSoundOff(channel))
                            }
                            0x79 => {
                                Ok(MIDIEvent::AllControllersOff(channel))
                            }
                            0x7A => {
                                Ok(MIDIEvent::LocalControl(channel, value))
                            }
                            0x7B => {
                                Ok(MIDIEvent::AllNotesOff(channel))
                            }
                            0x7C => {
                                Ok(MIDIEvent::OmniOff(channel))
                            }
                            0x7D => {
                                Ok(MIDIEvent::OmniOn(channel))
                            }
                            0xFE => {
                                Ok(MIDIEvent::MonophonicOperation(channel, value))
                            }
                            0xFF => {
                                Ok(MIDIEvent::PolyphonicOperation(channel))
                            }
                            _ => {
                                Ok(MIDIEvent::ControlChange(channel, controller, value))
                            }
                        }
                    }
                    0xC => {
                        channel = leadbyte & 0x0F;
                        let new_program = bytes.remove(0);
                        let event = MIDIEvent::ProgramChange(channel, new_program);
                        output = Ok(event);
                    }
                    0xD => {
                        channel = leadbyte & 0x0F;
                        let pressure = bytes.remove(0);
                        let event = MIDIEvent::ChannelPressure(channel, pressure);
                        output = Ok(event);
                    }
                    0xE => {
                        channel = leadbyte & 0x0F;
                        let least_significant_byte = bytes.remove(0);
                        let most_significant_byte = bytes.remove(0);
                        let event = build_pitch_wheel_change(channel, least_significant_byte, most_significant_byte);
                        output = Ok(event);
                    }
                    _ => {
                    }
                }
            }

            0xF0 => {
                // System Exclusive
                let mut bytedump = Vec::new();
                loop {
                    let byte = bytes.remove(0);
                    if byte == 0xF7 {
                        break;
                    } else {
                        bytedump.push(byte);
                    }
                }

                let event = MIDIEvent::SystemExclusive(bytedump);
                output = Ok(event);
            }

            0xF2 => {
                // Song Position Pointer
                let least_significant_byte = bytes.remove(0);
                let most_significant_byte = bytes.remove(0);

                let beat = ((most_significant_byte as u16) << 8) + (least_significant_byte as u16);
                let event = MIDIEvent::SongPositionPointer(beat);
                output = Ok(event);
            }

            0xF3 => {
                let song = bytes.remove(0);
                let event = MIDIEvent::SongSelect(song & 0x7F);
                output = Ok(event);
            }

            0xFF => {
                let meta_byte = bytes.remove(0); // Meta Type
                varlength = get_variable_length_number(bytes);
                if meta_byte == 0x51 {
                    let event = MIDIEvent::SetTempo(dequeue_n(bytes, varlength as usize));
                    output = Ok(event);
                } else {
                    let mut bytedump = Vec::new();
                    for _ in 0..varlength {
                        bytedump.push(bytes.remove(0));
                    }
                    match meta_byte {
                        0x00 => {
                            let event = MIDIEvent::SequenceNumber((bytedump[0] as u16 * 256) + bytedump[1] as u16);
                            output = Ok(event);
                        }

                        0x01 => {
                            match std::str::from_utf8(bytedump.as_slice()) {
                                Ok(textdump) => {
                                    let event = MIDIEvent::Text(textdump.to_string());
                                    output = Ok(event);
                                }
                                Err(_e) => {
                                    output = Err(ApresError::IllegibleString(bytedump.clone()));
                                }
                            }
                        }
                        0x02 => {
                            match std::str::from_utf8(bytedump.as_slice()) {
                                Ok(textdump) => {
                                    let event = MIDIEvent::CopyRightNotice(textdump.to_string());
                                    output = Ok(event);
                                }
                                Err(_e) => {
                                    output = Err(ApresError::IllegibleString(bytedump.clone()));
                                }
                            }
                        }
                        0x03 => {
                            match std::str::from_utf8(bytedump.as_slice()) {
                                Ok(textdump) => {
                                    let event = MIDIEvent::TrackName(textdump.to_string());
                                    output = Ok(event);
                                }
                                Err(_e) => {
                                    output = Err(ApresError::IllegibleString(bytedump.clone()));
                                }
                            }
                        }
                        0x04 => {
                            match std::str::from_utf8(bytedump.as_slice()) {
                                Ok(textdump) => {
                                    let event = MIDIEvent::InstrumentName(textdump.to_string());
                                    output = Ok(event);
                                }
                                Err(_e) => {
                                    output = Err(ApresError::IllegibleString(bytedump.clone()));
                                }
                            }
                        }
                        0x05 => {
                            match std::str::from_utf8(bytedump.as_slice()) {
                                Ok(textdump) => {
                                    let event = MIDIEvent::Lyric(textdump.to_string());
                                    output = Ok(event);
                                }
                                Err(_e) => {
                                    output = Err(ApresError::IllegibleString(bytedump.clone()));
                                }
                            }
                        }
                        0x06 => {
                            match std::str::from_utf8(bytedump.as_slice()) {
                                Ok(textdump) => {
                                    let event = MIDIEvent::Marker(textdump.to_string());
                                    output = Ok(event);
                                }
                                Err(_e) => {
                                    output = Err(ApresError::IllegibleString(bytedump.clone()));
                                }
                            }
                        }
                        0x07 => {
                            match std::str::from_utf8(bytedump.as_slice()) {
                                Ok(textdump) => {
                                    let event = MIDIEvent::CuePoint(textdump.to_string());
                                    output = Ok(event);
                                }
                                Err(_e) => {
                                    output = Err(ApresError::IllegibleString(bytedump.clone()));
                                }
                            }
                        }
                        0x20 => {
                            let event = MIDIEvent::ChannelPrefix(bytedump[0]);
                            output = Ok(event);
                        }
                        0x2F => {
                            output = Ok(MIDIEvent::EndOfTrack);
                        }
                        0x51 => {
                        }
                        0x54 => {
                            let event = MIDIEvent::SMPTEOffset(bytedump[0], bytedump[1], bytedump[2], bytedump[3], bytedump[4]);
                            output = Ok(event);
                        }
                        0x58 => {
                            let event = MIDIEvent::TimeSignature(bytedump[0], bytedump[1], bytedump[2], bytedump[3]);
                            output = Ok(event);
                        }
                        0x59 => {
                            let event = build_key_signature(bytedump[1], bytedump[0]);
                            output = Ok(event);
                        }
                        0x7F => {
                            let event = MIDIEvent::SequencerSpecific(bytedump[3..].to_vec());

                            output = Ok(event);
                        }
                        _ => {
                            output = Err(ApresError::UnknownMetaEvent(bytes.to_vec()));
                        }
                    }
                }
            }

            0xF1 | 0xF6 | 0xF8 | 0xFA | 0xFB | 0xFC | 0xFE | 0xF7 => {
                // Do Nothing. These are system-realtime and shouldn't be in a file.
            }

            0xF4 | 0xF5 | 0xF9 | 0xFD => {
                // Undefined Behaviour
            }
        }

        output
    }
}

/// Structural representation of MIDI.
///
/// Can represent a file or a real-time performance.
///
/// # Examples
/// Load a Song
/// ```
/// use apres::MIDI;
/// // Create a MIDI from a file
/// match MIDI::from_path("/path/to/file.mid") {
///     Ok(midi) => {
///     }
///     Err(_) => {
///     }
///}
/// ```
/// Create a new MIDI
/// ```
/// use apres::MIDI;
/// // Create an empty MIDI file.
/// let midi = MIDI::new();
/// ```
/// Creating a song
/// ```
/// use apres::MIDI;
/// use apres::MIDIEvent::{NoteOff, NoteOn};
/// // Create an empty MIDI file.
/// let mut midi = MIDI::new();
///
/// // Press midi note 64 (Middle E) on the first track (0) at the first position (0 ticks)
/// midi.insert_event(0, 0, NoteOn(64, 100, 100));
///
/// // Release midi note 64 (Middle E) on the first track (0) one beat later (120 ticks)
/// midi.push_event(0, 120, NoteOn(64, 100, 100));
///
/// // Save it to a file
/// midi.save("beep.mid");
/// ```
#[derive(Debug)]
pub struct MIDI {
    ppqn: u16,
    midi_format: u16, // 16 because the format stores in 2 bytes, even though it only requires 2 bits (0,1,2)
    events: HashMap<u64, MIDIEvent>,
    event_id_gen: u64,
    event_positions: HashMap<u64, (usize, usize)>,

    _active_byte: u8 // Only used when reading in a .mid
}


impl MIDI {
    /// Construct a new, empty MIDI
    pub fn new() -> MIDI {
        MIDI {
            event_id_gen: 1, // Reserve 0 for passing 'none' to bindings
            ppqn: 120,
            midi_format: 1,
            events: HashMap::new(),
            event_positions: HashMap::new(),
            _active_byte: 0x90
        }
    }

    /// Construct a new MIDI from a .mid file
    pub fn from_path(file_path: &str) -> Result<MIDI, ApresError> {
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
            Err(_e) => {
                Err(ApresError::InvalidMIDIFile(file_path.to_string()))?;
            }
        }

        match MIDI::from_bytes(midibytes) {
            Ok(midi_ob) => {
                Ok(midi_ob)
            }
            Err(e) => {
                Err(e)
        //        Err(ApresError::InvalidMIDIFile(file_path.to_string()))
            }
        }
    }

    fn from_bytes(file_bytes: Vec<u8>) -> Result<MIDI, ApresError> {
        let bytes = &mut file_bytes.clone();
        let mut mlo: MIDI = MIDI::new();
        let mut sub_bytes: Vec<u8>;
        let mut chunkcount: HashMap<(u8, u8,u8, u8), u16> = HashMap::new();
        let mut current_track: usize = 0;
        let mut current_deltatime: usize;

        let mut chunk_type: (u8, u8, u8, u8);


        // TODO: These Probably don't need to be 32
        let mut divword: u32;
        //let mut smpte: u32;
        //let mut tpf: u32;
        let mut midi_format: u16;

        let mut track_length: u32;

        let mut found_header = false;

        let mut ppqn: u16 = 120;
        while bytes.len() > 0 {
            chunk_type = (
                bytes.remove(0),
                bytes.remove(0),
                bytes.remove(0),
                bytes.remove(0)
            );

            let val = chunkcount.entry(chunk_type).or_insert(0);
            *val += 1;

            if chunk_type == ('M' as u8, 'T' as u8, 'h' as u8, 'd' as u8) {
                dequeue_n(bytes, 4); // Get Size
                midi_format = dequeue_n(bytes, 2) as u16; // Midi Format
                dequeue_n(bytes, 2); // Get Number of tracks
                divword = dequeue_n(bytes, 2);

                // TODO: handle divword > 0x8000
                if divword & 0x8000 > 0 {
                    //smpte = (((divword & 0x7F00) >> 8) as i8) as u32;
                    //tpf = divword & 0x00FF;

                } else {
                    ppqn = (divword & 0x7FFF) as u16;
                }
                mlo.set_ppqn(ppqn);
                mlo.set_format(midi_format);
                found_header = true;
            } else if chunk_type == ('M' as u8, 'T' as u8, 'r' as u8, 'k' as u8) {
                if ! found_header {
                    Err(ApresError::MissingHeader)?;
                }
                current_deltatime = 0;
                track_length = dequeue_n(bytes, 4);
                sub_bytes = Vec::new();
                for _ in 0..track_length {
                    sub_bytes.push(bytes.remove(0))
                }

                while sub_bytes.len() > 0 {
                    current_deltatime += get_variable_length_number(&mut sub_bytes) as usize;
                    match mlo.process_mtrk_event(&mut sub_bytes, &mut current_deltatime, current_track) {
                        Ok(_) => {
                            Ok(())
                        }
                        Err(ApresError::UnknownMetaEvent(_bytes)) => {
                            Ok(())
                        }
                        Err(ApresError::IllegibleString(_bytes)) => {
                            Ok(())
                        }
                        Err(e) => {
                            Err(e)
                        }
                    }?;
                }
                current_track += 1;
            } else {
                Err(ApresError::InvalidBytes(bytes.clone()))?;
            }
        }

        Ok(mlo)
    }

    fn process_mtrk_event(&mut self, bytes: &mut Vec<u8>, current_deltatime: &mut usize, track: usize) -> Result<u64, ApresError> {
        match bytes.first() {
            Some(status_byte) => {
                match status_byte {
                    0x80..=0xEF => {
                        self._active_byte = *status_byte;
                    }
                    _ => ()
                }
            }
            None => ()
        }

        let event = MIDIEvent::from_bytes(bytes, self._active_byte)?;

        self.insert_event(track, *current_deltatime, event)
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
        let mut track_byte_length: u32;
        let tracks: Vec<Vec<(usize, u64)>> = self.get_tracks();

        for ticks in tracks.iter() {
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
            track_event_bytes.extend(MIDIEvent::EndOfTrack.as_bytes().iter().copied());

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

    /// Save the MIDI Object to a file
    pub fn save(&self, path: &str) {
        let bytes = self.as_bytes();
        match File::create(path) {
            Ok(mut file) => {
                file.write_all(bytes.as_slice());
            }
            Err(_e) => {
            }
        }
    }

    /// Get the track and tick of an event, given its id
    pub fn get_event_position(&self, event_id: u64) -> Option<&(usize, usize)> {
        self.event_positions.get(&event_id)
    }

    /// Get a list of tracks, each populated by lists of event ids.
    /// Each list in each track represents a 'tick', so it could be empty
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
        for (_, (_current_track, test_tick)) in self.event_positions.iter() {
            if track == *_current_track {
                highest_tick = max(highest_tick, *test_tick);
            }
        }

        highest_tick + 1
    }

    /// Set Pulses Per Quarter Note
    pub fn set_ppqn(&mut self, new_ppqn: u16) {
        self.ppqn = new_ppqn;
    }

    /// Get Pulses Per Quarter Note
    pub fn get_ppqn(&self) -> u16 {
        self.ppqn
    }

    pub fn set_format(&mut self, new_format: u16) {
        self.midi_format = new_format;
    }

    pub fn get_format(&self) -> u16 {
        self.midi_format
    }

    /// Change the track or position of an event, given it id in the MIDI
    pub fn move_event(&mut self, new_track: usize, new_tick: usize, event_id: u64) {
        self.event_positions.entry(event_id)
            .and_modify(|pair| { *pair = (new_track, new_tick); })
            .or_insert((new_track, new_tick));
    }

    /// Insert an event into the track
    pub fn insert_event(&mut self, track: usize, tick: usize, event: MIDIEvent) -> Result<u64, ApresError> {
        let result;
        if track > 15 {
            result = Err(ApresError::TrackOutOfBounds);
        } else {
            let new_event_id = self.event_id_gen;
            self.event_id_gen += 1;

            self.events.insert(new_event_id, event);

            self.move_event(track, tick, new_event_id);

            result = Ok(new_event_id);
        }

        result
    }

    /// Insert an event after the latest event in the track
    pub fn push_event(&mut self, track: usize, wait: usize, event: MIDIEvent) -> Result<u64, ApresError> {
        let result;
        if track > 15 {
            result = Err(ApresError::TrackOutOfBounds);
        } else {
            let new_event_id = self.event_id_gen;

            self.events.insert(new_event_id, event);
            self.event_id_gen += 1;

            let last_tick_in_track = self.get_track_length(track) - 1;
            self.move_event(track, last_tick_in_track + wait, new_event_id);

            result = Ok(new_event_id);
        }

        result
    }

    pub fn get_event(&self, event_id: u64) -> Option<MIDIEvent> {
        match self.events.get(&event_id) {
            Some(event) => {
                Some(event.clone())
            }
            None => {
                None
            }
        }
    }

    pub fn replace_event(&mut self, event_id: u64, new_midi_event: MIDIEvent) -> Result<(), ApresError> {
        if self.events.contains_key(&event_id) {
            self.events.entry(event_id)
                .and_modify(|e| *e = new_midi_event);
            Ok(())
        } else {
            Err(ApresError::EventNotFound(event_id))
        }
    }
}

pub fn listen<T>(device_id: u8, context: &mut T, callback: fn(&mut Controller, &mut T, &MIDIEvent) -> ()) -> Result<(), ApresError> {
    match Controller::new(device_id) {
        Ok(mut controller) => {
            controller.listen(context, callback)
        }
        Err(e) => {
            Err(e)
        }
    }
}

fn dequeue_n(bytes: &mut Vec<u8>, n: usize) -> u32 {
    let mut tn: u32 = 0;
    for _ in 0..n {
        tn *= 256;
        let x = bytes.remove(0);
        tn += x as u32;
    }
    tn
}

fn get_variable_length_number(bytes: &mut Vec<u8>) -> u64 {
    let mut n = 0u64;

    loop {
        n <<= 7;
        let x = bytes.remove(0);
        n |= (x & 0x7F) as u64;
        if x & 0x80 == 0 {
            break;
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

// input a number between (-1, 1), get an unsigned value with 0x2000 as midpoint
pub fn get_pitchwheel_value(n: f64) -> u16 {
    if n < 0_f64 {
        ((1_f64 + n) * (0x2000 as f64)) as u16
    } else if n > 0_f64 {
        (n * (0x1FFF as f64)) as u16 + 0x2000
    } else {
        0x2000
    }
}

fn build_key_signature(mi: u8, sf: u8) -> MIDIEvent {
    let chord_name = get_chord_name_from_mi_sf(mi, sf);

    MIDIEvent::KeySignature(chord_name.to_string())
}

fn build_pitch_wheel_change(channel: u8, lsb: u8, msb: u8) -> MIDIEvent {
    let unsigned_value: f64 = (((msb as u16) << 8) + (lsb as u16)) as f64;
    let new_value: f64 = ((unsigned_value * 2_f64) as f64 / 0x3FFF as f64) - 1_f64;
    MIDIEvent::PitchWheelChange(channel, new_value)
}

pub fn get_mi_sf(chord_name: &str) -> (u8, u8) {
    match chord_name {
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

fn get_chord_name_from_mi_sf(mi: u8, sf: u8) -> String {
    let map = vec![
        vec![
            "Cb", "Gb", "Db", "Ab",
            "Eb", "Bb", "F",
            "C", "G", "D", "A",
            "E", "B", "F#", "C#",
        ],
        vec![
            "Abm", "Ebm", "Bbm", "Fm",
            "Cm", "Gm", "Dm",
            "Am", "Em", "Bm", "F#m",
            "C#m", "G#m", "D#m", "A#m",
        ]
    ];

    map[mi as usize][((sf as i8) + 7) as usize].to_string()
}
