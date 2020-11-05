use std::fs::File;
use std::io::prelude::*;
use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};

pub mod tests;

pub enum ApresError {
    InvalidBytes(Vec<u8>),
    EventNotFound(u64)
}


#[derive(Clone)]
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

	NoteOn(u8, u8, u8),
	NoteOff(u8, u8, u8),
	AfterTouch(u8, u8, u8),

	BankSelect(u8, u16),
	ModulationWheel(u8, u16),
	BreathController(u8, u16),
	FootPedal(u8, u16),
	PortamentoTime(u8, u16),
	DataEntrySlider(u8, u16),
	Volume(u8, u16),
	Balance(u8, u16),
	Pan(u8, u16),
	Expression(u8, u16),
	EffectControl(u8, u8, u16),
	Slider(u8, u8, u8),
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
	SoundControl(u8, u8, u8),
	GeneralButtonOn(u8, u8),
	GeneralButtonOff(u8, u8),
	EffectsLevel(u8, u8),
	TremuloLevel(u8, u8),
	ChorusLevel(u8, u8),
	CelesteLevel(u8, u8),
	PhaserLevel(u8, u8),
	DataButtonIncrement(u8),
	DataButtonDecrement(u8),
	RegisteredParameterNumber(u8, u16),
	NonRegisteredParameterNumber(u8, u16),
	AllControllersOff(u8),
	LocalKeyboardEnable(u8),
	LocalKeyboardDisable(u8),
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
                vec![0xFF, 0x59, 0x02, sf as u8, mi]
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
                gen_coarse_fine_bytes(
                    *channel,
                    *value,
                    0x00, 0x20
                )
            }

            MIDIEvent::ModulationWheel(channel, value) => {
                gen_coarse_fine_bytes(
                    *channel,
                    *value,
                    0x01, 0x21
                )
            }

            MIDIEvent::BreathController(channel, value) => {
                gen_coarse_fine_bytes(
                    *channel,
                    *value,
                    0x02, 0x22
                )
            }

            MIDIEvent::FootPedal(channel, value) => {
                gen_coarse_fine_bytes(
                    *channel,
                    *value,
                    0x04, 0x24
                )
            }

            MIDIEvent::PortamentoTime(channel, value) => {
                gen_coarse_fine_bytes(
                    *channel,
                    *value,
                    0x05, 0x25
                )
            }

            MIDIEvent::DataEntrySlider(channel, value) => {
                gen_coarse_fine_bytes(
                    *channel,
                    *value,
                    0x06, 0x26
                )
            }

            MIDIEvent::Volume(channel, value) => {
                gen_coarse_fine_bytes(
                    *channel,
                    *value,
                    0x07, 0x27
                )
            }

            MIDIEvent::Balance(channel, value) => {
                gen_coarse_fine_bytes(
                    *channel,
                    *value,
                    0x08, 0x28
                )
            }


            MIDIEvent::Pan(channel, value) => {
                gen_coarse_fine_bytes(
                    *channel,
                    *value,
                    0x0A, 0x2A
                )
            }

            MIDIEvent::Expression(channel, value) => {
                gen_coarse_fine_bytes(
                    *channel,
                    *value,
                    0x0B, 0x2B
                )
            }

            MIDIEvent::EffectControl(channel, which, value) => {
                gen_coarse_fine_bytes(
                    *channel,
                    *value,
                    0x0C + *which, 0x2C + *which
                )
            }

            MIDIEvent::Slider(channel, which, value) => {
                vec![
                    0xB | *channel,
                    0x10 + *which as u8,
                    *value
                ]
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
                    0xb0 | *channel,
                    0x41,
                    *value
                ]
            }

            MIDIEvent::Sustenuto(channel, value) => {
                vec![
                    0xb0 | *channel,
                    0x42,
                    *value
                ]
            }

            MIDIEvent::SoftPedal(channel, value) => {
                vec![
                    0xb0 | *channel,
                    0x43,
                    *value
                ]
            }

            MIDIEvent::Legato(channel, value) => {
                vec![
                    0xb0 | *channel,
                    0x44,
                    *value
                ]
            }

            MIDIEvent::Hold2Pedal(channel, value) => {
                vec![
                    0xb0 | *channel,
                    0x45,
                    *value
                ]
            }

            MIDIEvent::SoundVariation(channel, value) => {
                vec![
                    0xb0 | *channel,
                    0x46,
                    *value
                ]
            }

            MIDIEvent::SoundTimbre(channel, value) => {
                vec![
                    0xb0 | *channel,
                    0x47,
                    *value
                ]
            }

            MIDIEvent::SoundReleaseTime(channel, value) => {
                vec![
                    0xb0 | *channel,
                    0x48,
                    *value
                ]
            }

            MIDIEvent::SoundAttack(channel, value) => {
                vec![
                    0xb0 | *channel,
                    0x49,
                    *value
                ]
            }

            MIDIEvent::SoundBrightness(channel, value) => {
                vec![
                    0xb0 | *channel,
                    0x4A,
                    *value
                ]
            }

            MIDIEvent::SoundControl(channel, which, value) => {
                vec![
                    0xb0 | *channel,
                    0x4B + *which,
                    *value
                ]
            }

            MIDIEvent::GeneralButtonOn(channel, which) => {
                vec![
                    0xB0 | *channel,
                    0x50 + *which,
                    0b01000000
                ]
            }

            MIDIEvent::GeneralButtonOff(channel, which) => {
                vec![
                    0xB0 | *channel,
                    0x50 + *which,
                    0b00000000
                ]
            }

            MIDIEvent::EffectsLevel(channel, value) => {
                vec![
                    0xb0 | *channel,
                    0x5B,
                    *value
                ]
            }

            MIDIEvent::TremuloLevel(channel, value) => {
                vec![
                    0xb0 | *channel,
                    0x5C,
                    *value
                ]
            }

            MIDIEvent::ChorusLevel(channel, value) => {
                vec![
                    0xb0 | *channel,
                    0x5D,
                    *value
                ]
            }

            MIDIEvent::CelesteLevel(channel, value) => {
                vec![
                    0xb0 | *channel,
                    0x5E,
                    *value
                ]
            }

            MIDIEvent::PhaserLevel(channel, value) => {
                vec![
                    0xb0 | *channel,
                    0x5F,
                    *value
                ]
            }

            MIDIEvent::DataButtonIncrement(channel) => {
                vec![0xB0 | *channel, 0x60, 0x00]
            }

            MIDIEvent::DataButtonDecrement(channel) => {
                vec![0xB0 | *channel, 0x61, 0x00]
            }

            MIDIEvent::RegisteredParameterNumber(channel, value) => {
                gen_coarse_fine_bytes(
                    *channel,
                    *value,
                    0x65, 0x64
                )
            }

            MIDIEvent::NonRegisteredParameterNumber(channel, value) => {
                gen_coarse_fine_bytes(
                    *channel,
                    *value,
                    0x65, 0x64
                )
            }

            MIDIEvent::AllControllersOff(channel) => {
                vec![
                    0xB0 | *channel,
                    0x79, 0x00
                ]
            }

            MIDIEvent::LocalKeyboardEnable(channel) => {
                vec![
                    0xB0 | *channel,
                    0x7A, 0x40
                ]
            }

            MIDIEvent::LocalKeyboardDisable(channel) => {
                vec![
                    0xB0 | *channel,
                    0x7A, 0x00
                ]
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
                let msb: u8 = ((unsigned_value >> 7) & 0x007F) as u8;
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

            MIDIEvent::SongPositionPointer(beat) => {
                vec![
                    0xF2,
                    (*beat & 0x7F) as u8,
                    ((*beat >> 7) & 0x7F) as u8
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

        let n: u32;
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
                        let event = MIDIEvent::ControlChange(channel, controller, value);
                        output = Ok(event);
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

                let beat = ((most_significant_byte as u16) << 7) + (least_significant_byte as u16);
                let event = MIDIEvent::SongPositionPointer(beat);
                output = Ok(event);
            }

            0xF3 => {
                let song = bytes.remove(0);
                let event = MIDIEvent::SongSelect(song & 0x7F);
                output = Ok(event);
            }

            0xF1 | 0xF6 | 0xF8 | 0xFA | 0xFB | 0xFC | 0xFE => {
                // Do Nothing. These are system-realtime and shouldn't be in a file.
            }

            0xF7 => {
                varlength = get_variable_length_number(bytes);
                n = dequeue_n(bytes, varlength as usize);
                // TODO ADD EVENT
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
                        0x01 => {
                            match std::str::from_utf8(bytedump.as_slice()) {
                                Ok(textdump) => {
                                    let event = MIDIEvent::Text(textdump.to_string());
                                    output = Ok(event);
                                }
                                Err(_e) => { }
                            }
                        }
                        0x02 => {
                            match std::str::from_utf8(bytedump.as_slice()) {
                                Ok(textdump) => {
                                    let event = MIDIEvent::CopyRightNotice(textdump.to_string());
                                    output = Ok(event);
                                }
                                Err(_e) => { }
                            }
                        }
                        0x03 => {
                            match std::str::from_utf8(bytedump.as_slice()) {
                                Ok(textdump) => {
                                    let event = MIDIEvent::TrackName(textdump.to_string());
                                    output = Ok(event);
                                }
                                Err(_e) => { }
                            }
                        }
                        0x04 => {
                            match std::str::from_utf8(bytedump.as_slice()) {
                                Ok(textdump) => {
                                    let event = MIDIEvent::InstrumentName(textdump.to_string());
                                    output = Ok(event);
                                }
                                Err(_e) => { }
                            }
                        }
                        0x05 => {
                            match std::str::from_utf8(bytedump.as_slice()) {
                                Ok(textdump) => {
                                    let event = MIDIEvent::Lyric(textdump.to_string());
                                    output = Ok(event);
                                }
                                Err(_e) => { }
                            }
                        }
                        0x06 => {
                            match std::str::from_utf8(bytedump.as_slice()) {
                                Ok(textdump) => {
                                    let event = MIDIEvent::Marker(textdump.to_string());
                                    output = Ok(event);
                                }
                                Err(_e) => { }
                            }
                        }
                        0x07 => {
                            match std::str::from_utf8(bytedump.as_slice()) {
                                Ok(textdump) => {
                                    let event = MIDIEvent::CuePoint(textdump.to_string());
                                    output = Ok(event);
                                }
                                Err(_e) => { }
                            }
                        }
                        0x20 => {
                            let event = MIDIEvent::ChannelPrefix(bytedump[0]);
                            output = Ok(event);
                        }
                        0x2F => {
                            // I *think* EndOfTrack events can be safely ignored, since it has to be the last event in a track and the track knows how long it is.
                            //let event = MIDIEvent::EndOfTrack() );
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
                            // TODO: I tihnk this is supposed to be SequencerSpecific, and i got the 2 conflated. Commenting out for now.
                            //let event = MIDIEvent::SystemExclusive(bytedump);
                            //output = output = Ok(event);
                        }
                        _ => {
                        }
                    }
                }
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
/// let midi = MIDI::from_path("/path/to/file.mid");
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
    pub fn from_path(file_path: &str) -> MIDI {
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
        let bytes = &mut file_bytes.clone();
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
                if divword & 0x8000 > 0 {
                    smpte = ((((divword & 0x7F00) >> 8) as i8) as u32);
                    tpf = divword & 0x00FF;

                } else {
                    ppqn = (divword & 0x7FFF) as u16;
                }
                mlo.set_ppqn(ppqn);
                mlo.set_format(midi_format);
            } else if chunk_type == ('M' as u8, 'T' as u8, 'r' as u8, 'k' as u8) {
                current_deltatime = 0;
                track_length = dequeue_n(bytes, 4);
                sub_bytes = Vec::new();
                for _ in 0..track_length {
                    sub_bytes.push(bytes.remove(0))
                }

                while sub_bytes.len() > 0 {
                    current_deltatime += get_variable_length_number(&mut sub_bytes) as usize;
                    mlo.process_mtrk_event(&mut sub_bytes, &mut current_deltatime, current_track);
                }
                current_track += 1;
            } else {
                break;
            }
        }
        mlo
    }

    fn process_mtrk_event(&mut self, bytes: &mut Vec<u8>, current_deltatime: &mut usize, track: usize) -> Option<u64> {
        let mut output = None;

        let n: u32;
        let varlength: u64;


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

        let active_byte = self._active_byte;
        match MIDIEvent::from_bytes(bytes, active_byte) {
            Ok(event) => {
                output = Some(self.insert_event(track, *current_deltatime, event));
            }
            Err(e) => {
                //TODO: Don't surpress the error
            }
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
            Err(e) => {
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

    /// Change the track or position of an event, given it id in the MIDI
    pub fn move_event(&mut self, new_track: usize, new_tick: usize, event_id: u64) {
        self.event_positions.entry(event_id)
            .and_modify(|pair| { *pair = (new_track, new_tick); })
            .or_insert((new_track, new_tick));
    }

    /// Insert an event into the track
    pub fn insert_event(&mut self, track: usize, tick: usize, event: MIDIEvent) -> u64 {
        let new_event_id = self.event_id_gen;
        self.event_id_gen += 1;

        self.events.insert(new_event_id, event);

        self.move_event(track, tick, new_event_id);

        new_event_id
    }

    /// Insert an event after the latest event in the track
    pub fn push_event(&mut self, track: usize, wait: usize, event: MIDIEvent) -> u64 {
        let new_event_id = self.event_id_gen;

        self.events.insert(new_event_id, event);
        self.event_id_gen += 1;

        let last_tick_in_track = self.get_track_length(track) - 1;
        self.move_event(track, last_tick_in_track + wait, new_event_id);

        new_event_id
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

fn build_key_signature(mut mi: u8, mut sf: u8) -> MIDIEvent {
    let chord_name = get_chord_name_from_mi_sf(mi, sf);

    MIDIEvent::KeySignature(chord_name.to_string())
}

fn build_pitch_wheel_change(channel: u8, lsb: u8, msb: u8) -> MIDIEvent {
    let unsigned_value: f64 = (((msb as u16) << 7) + (lsb as u16)) as f64;
    let new_value: f64 = ((unsigned_value * 2_f64) as f64 / 0x3FFF as f64) - 1_f64;
    MIDIEvent::PitchWheelChange(channel, new_value)
}

fn gen_coarse_fine_bytes(channel: u8, value: u16, coarse_offset: u8, fine_offset: u8) -> Vec<u8> {
    let mut output: Vec<u8> = Vec::new();

    // MSB
    if value > 0x7F {
        output.push(0xB | channel);
        output.push(coarse_offset);
        output.push((value >> 7) as u8);
    }

    // LSB
    if value & 0x7F != 0 {
        output.push(0xB | channel);
        output.push(fine_offset);
        output.push((value & 0x7F) as u8);
    }

    output
}


fn get_mi_sf(chord_name: &str) -> (u8, i8) {
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
