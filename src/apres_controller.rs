use super::*;
use std::fs::File;

pub struct ApresController {
    pipe: File
}

impl ApresController {
    pub fn new(path: &str) -> ApresController {
        ApresController {
            pipe: File::open(path).unwrap()
        }
    }

    fn get_next_byte(&mut self) -> u8 {
        let mut buffer = [0;1];
        loop {
            match self.pipe.read_exact(&mut buffer) {
                Ok(_success) => {
                    break;
                }
                Err(_e) => {
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
                        Some(Box::new(NoteOffEvent::new(channel, b, c)))
                    }
                    0x90 => {
                        let b = self.get_next_byte();
                        let c = self.get_next_byte();
                        if c == 0 {
                            Some(Box::new(NoteOffEvent::new(channel, b, c)))
                        } else {
                            Some(Box::new(NoteOnEvent::new(channel, b, c)))
                        }
                    }
                    0xA0 => {
                        let b = self.get_next_byte();
                        let c = self.get_next_byte();
                        Some(Box::new(AfterTouchEvent::new(channel, b, c)))
                    }
                    0xB0 => {
                        let b = self.get_next_byte();
                        let c = self.get_next_byte();
                        Some(Box::new(ControlChangeEvent::new(channel, b, c)))
                    }
                    0xC0 => {
                        let b = self.get_next_byte();
                        Some(Box::new(ProgramChangeEvent::new(channel, b)))
                    }
                    0xD0 => {
                        let b = self.get_next_byte();
                        Some(Box::new(ChannelPressureEvent::new(channel, b)))
                    }
                    0xE0 => {
                        let b = self.get_next_byte();
                        let c = self.get_next_byte();
                        Some(Box::new(PitchWheelChangeEvent::new_from_lsb_msb(channel, b, c)))
                    }
                    _ => { None }
                }
            }
            0xF0 => {
                let mut bytedump = vec![0xF0];
                loop {
                    match self.get_next_byte() {
                        0xF7 => {
                            break;
                        }
                        x => {
                            bytedump.push(x);
                        }
                    }
                }
                bytedump.push(0xF7);
                Some(Box::new(SystemExclusiveEvent::new(bytedump)))
            }
            0xF1 => {
                let b = self.get_next_byte();
                Some(Box::new(MTCQuarterFrameEvent::new(b)))
            }
            0xF2 => {
                let b = self.get_next_byte();
                let c = self.get_next_byte();
                Some(Box::new(SongPositionPointerEvent::new_from_lsb_msb(b, c)))
            }
            0xF3 => {
                let song = self.get_next_byte();
                Some(Box::new(SongSelectEvent::new(song)))
            }
            // System real-time events
            0xF6 => {
                Some(Box::new(TuneRequestEvent {}))
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
            0xF4 | 0xF5 | 0xF9 | 0xFD => {
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
        let some_text = "This is some text".to_string();
        let text_len_bytes = to_variable_length_bytes(some_text.len());

        let event = TextEvent::new(some_text.clone());
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
        let some_text = "This is some text".to_string();
        let text_len_bytes = to_variable_length_bytes(some_text.len());

        let event = CopyRightNoticeEvent::new(some_text.clone());
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
        let some_text = "Some Track Name".to_string();
        let text_len_bytes = to_variable_length_bytes(some_text.len());

        let event = TrackNameEvent::new(some_text.clone());
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
        let some_text = "Some Instrument Name".to_string();
        let text_len_bytes = to_variable_length_bytes(some_text.len());

        let event = InstrumentNameEvent::new(some_text.clone());
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
        let some_text = "Here are some Lyrics.".to_string();
        let text_len_bytes = to_variable_length_bytes(some_text.len());

        let event = LyricEvent::new(some_text.clone());
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
        let some_text = "marker text".to_string();
        let text_len_bytes = to_variable_length_bytes(some_text.len());

        let event = MarkerEvent::new(some_text.clone());
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
        let some_text = "cue point text".to_string();
        let text_len_bytes = to_variable_length_bytes(some_text.len());

        let event = CuePointEvent::new(some_text.clone());
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
        let event = EndOfTrackEvent::new();
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
        let event = SMPTEOffsetEvent::new(1,2,3,4,5);
        assert_eq!(
            event.as_bytes().as_slice(),
            [0xFF, 0x54, 0x05, 0x01, 0x02, 0x03, 0x04, 0x05]
        );
    }

    #[test]
    fn test_time_signature_event() {
        let event = TimeSignatureEvent::new(4, 4, 32, 3);
        assert_eq!(
            event.as_bytes().as_slice(),
            [0xFF, 0x58, 0x04, 0x04, 0x04, 0x20, 0x03]
        );
    }

    #[test]
    fn test_key_signature_event() {
        let event = KeySignatureEvent::new("A".to_string());
        assert_eq!(
            event.as_bytes().as_slice(),
            [0xFF, 0x59, 0x02, 0x03, 0x00]
        );
    }

    #[test]
    fn test_sequence_specific_event() {
        let event = SequencerSpecificEvent::new(vec![0x00]);
        assert_eq!(
            event.as_bytes().as_slice(),
            [0xFF, 0x7F, 0x01, 0x00]
        );
    }

    #[test]
    fn test_note_on_event() {
        let event = NoteOnEvent::new(14, 23, 33);
        assert_eq!(
            event.as_bytes().as_slice(),
            [0x9E, 0x17, 0x21]
        );
    }

    #[test]
    fn test_note_off_event() {
        let event = NoteOffEvent::new(14, 23, 33);
        assert_eq!(
            event.as_bytes().as_slice(),
            [0x8E, 0x17, 0x21]
        );
    }

    #[test]
    fn test_aftertouch_event() {
        let event = AfterTouchEvent::new(14, 23, 33);
        assert_eq!(
            event.as_bytes().as_slice(),
            [0xAE, 0x17, 0x21]
        );
    }

    #[test]
    fn test_control_change_event() {
        let event = ControlChangeEvent::new(14, 23, 33);
        assert_eq!(
            event.as_bytes().as_slice(),
            [0xBE, 0x17, 0x21]
        );
    }

    #[test]
    fn test_program_change_event() {
        let event = ProgramChangeEvent::new(14, 23);
        assert_eq!(
            event.as_bytes().as_slice(),
            [0xCE, 0x17]
        );
    }

    #[test]
    fn test_channel_pressure_event() {
        let event = ChannelPressureEvent::new(14, 23);
        assert_eq!(
            event.as_bytes().as_slice(),
            [0xDE, 0x17]
        );
    }

    #[test]
    fn test_pitchwheel_change_event() {
        let mut event = PitchWheelChangeEvent::new(14, 0.0);
        let test_cases: Vec<(f64, (u8, u8))> = vec![
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
        let event = SystemExclusiveEvent::new(vec![0,0,1,0]);
        assert_eq!(
            event.as_bytes().as_slice(),
            [0xF0, 0x00, 0x00, 0x01, 0x00, 0xF7]
        );
    }
}
