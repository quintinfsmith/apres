#[cfg(test)]
use super::*;
use super::MIDIEvent::*;

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


        0x00, 0xFF, 0x2F, 0x00 // EOT
    ];
    match MIDI::from_bytes(midi_bytes) {
        Ok(midi) => {
            assert_eq!(midi.count_tracks(), 1);
            assert_eq!(midi.get_track_length(0), 121);
            assert_eq!(midi.events.len(), 16);
            assert_eq!(midi.event_positions.len(), 16);
        }
        Err(ApresError::InvalidBytes(bytes)) => {
            print!("{:?}", bytes);
            assert!(false);
        }
        Err(_) => ()
    }

}

#[test]
fn test_add_event() {
    let mut midi = MIDI::new();
    let on_event = midi.push_event(0, 0, NoteOn(0, 64, 100));
    let off_event = midi.push_event(0, 119, NoteOff(0, 64, 0));

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


        output_n = get_variable_length_number(expected_vector);
        assert_eq!(
            *input_number,
            output_n as usize
        );
    }



}


#[test]
fn test_sequence_number_event() {
    let mut event = SequenceNumber(1);
    assert_eq!(event.as_bytes().as_slice(), &[0xFF, 0x00, 0x02, 0x00, 0x01]);
    event = SequenceNumber(13607);
    assert_eq!(event.as_bytes().as_slice(), &[0xFF, 0x00, 0x02, 0x35, 0x27]);

}

#[test]
fn test_text_event() {
    let some_text = "This is some text".to_string();
    let text_len_bytes = to_variable_length_bytes(some_text.len());

    let event = Text(some_text.clone());
    let mut compare_vec = vec![ 0xFF, 0x01 ];
    compare_vec.extend(text_len_bytes.iter().copied());
    compare_vec.extend(some_text.as_bytes().iter().copied());
    assert_eq!(
        event.as_bytes().as_slice(),
        compare_vec.as_slice()
    );

}

#[test]
fn test_copyright_notice_event() {
    let some_text = "This is some text".to_string();
    let text_len_bytes = to_variable_length_bytes(some_text.len());

    let event = CopyRightNotice(some_text.clone());
    let mut compare_vec = vec![ 0xFF, 0x02 ];
    compare_vec.extend(text_len_bytes.iter().copied());
    compare_vec.extend(some_text.as_bytes().iter().copied());
    assert_eq!(
        event.as_bytes().as_slice(),
        compare_vec.as_slice()
    );
}

#[test]
fn test_track_name_event() {
    let some_text = "Some Track Name".to_string();
    let text_len_bytes = to_variable_length_bytes(some_text.len());

    let event = TrackName(some_text.clone());
    let mut compare_vec = vec![ 0xFF, 0x03 ];
    compare_vec.extend(text_len_bytes.iter().copied());
    compare_vec.extend(some_text.as_bytes().iter().copied());
    assert_eq!(
        event.as_bytes().as_slice(),
        compare_vec.as_slice()
    );

}

#[test]
fn test_instrument_name_event() {
    let some_text = "Some Instrument Name".to_string();
    let text_len_bytes = to_variable_length_bytes(some_text.len());

    let event = InstrumentName(some_text.clone());
    let mut compare_vec = vec![ 0xFF, 0x04 ];
    compare_vec.extend(text_len_bytes.iter().copied());
    compare_vec.extend(some_text.as_bytes().iter().copied());
    assert_eq!(
        event.as_bytes().as_slice(),
        compare_vec.as_slice()
    );
}

#[test]
fn test_lyric_event() {
    let some_text = "Here are some Lyrics.".to_string();
    let text_len_bytes = to_variable_length_bytes(some_text.len());

    let event = Lyric(some_text.clone());
    let mut compare_vec = vec![ 0xFF, 0x05 ];
    compare_vec.extend(text_len_bytes.iter().copied());
    compare_vec.extend(some_text.as_bytes().iter().copied());
    assert_eq!(
        event.as_bytes().as_slice(),
        compare_vec.as_slice()
    );
}

#[test]
fn test_marker_event() {
    let some_text = "marker text".to_string();
    let text_len_bytes = to_variable_length_bytes(some_text.len());

    let event = Marker(some_text.clone());
    let mut compare_vec = vec![ 0xFF, 0x06 ];
    compare_vec.extend(text_len_bytes.iter().copied());
    compare_vec.extend(some_text.as_bytes().iter().copied());
    assert_eq!(
        event.as_bytes().as_slice(),
        compare_vec.as_slice()
    );
}

#[test]
fn test_cue_point_event() {
    let some_text = "cue point text".to_string();
    let text_len_bytes = to_variable_length_bytes(some_text.len());

    let event = CuePoint(some_text.clone());
    let mut compare_vec = vec![ 0xFF, 0x07 ];
    compare_vec.extend(text_len_bytes.iter().copied());
    compare_vec.extend(some_text.as_bytes().iter().copied());
    assert_eq!(
        event.as_bytes().as_slice(),
        compare_vec.as_slice()
    );
}

#[test]
fn test_end_of_track_event() {
    let event = EndOfTrack;
    assert_eq!(
        event.as_bytes().as_slice(),
        [0xFF, 0x2F, 0x00]
    );
}

#[test]
fn test_channel_prefix_event() {
    for i in std::u8::MIN .. std::u8::MAX {
        let mut event = ChannelPrefix(i as u8);
        assert_eq!(
            event.as_bytes().as_slice(),
            [0xFF, 0x20, 0x01, i]
        );
    }
}

#[test]
fn test_set_tempo_event() {
    let test_cases_bpm = vec![
        (120, 500000),
        (280, 214285),
        (1, 0x00FFFFFF),// Minimum bpm is 3.576278762788
        (60_000_000, 1)
    ];
    for (bpm, expected_uspqn) in test_cases_bpm.iter() {
        let mut event = SetTempo(*expected_uspqn);
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
    let event = SMPTEOffset(1,2,3,4,5);
    assert_eq!(
        event.as_bytes().as_slice(),
        [0xFF, 0x54, 0x05, 0x01, 0x02, 0x03, 0x04, 0x05]
    );
}

#[test]
fn test_time_signature_event() {
    let event = TimeSignature(4, 4, 32, 3);
    assert_eq!(
        event.as_bytes().as_slice(),
        [0xFF, 0x58, 0x04, 0x04, 0x04, 0x20, 0x03]
    );
}

#[test]
fn test_key_signature_event() {
    let event = KeySignature("A".to_string());
    assert_eq!(
        event.as_bytes().as_slice(),
        [0xFF, 0x59, 0x02, 0x03, 0x00]
    );
}

#[test]
fn test_sequence_specific_event() {
    let event = SystemExclusive(vec![0x00]);
    assert_eq!(
        event.as_bytes().as_slice(),
        [0xF0, 0x00, 0xF7]
    );
}

#[test]
fn test_note_on_event() {
    let event = NoteOn(14, 23, 33);
    assert_eq!(
        event.as_bytes().as_slice(),
        [0x9E, 0x17, 0x21]
    );
}

#[test]
fn test_note_off_event() {
    let event = NoteOff(14, 23, 33);
    assert_eq!(
        event.as_bytes().as_slice(),
        [0x8E, 0x17, 0x21]
    );
}

#[test]
fn test_aftertouch_event() {
    let event = AfterTouch(14, 23, 33);
    assert_eq!(
        event.as_bytes().as_slice(),
        [0xAE, 0x17, 0x21]
    );
}

#[test]
fn test_control_change_event() {
    let event = ControlChange(14, 23, 33);
    assert_eq!(
        event.as_bytes().as_slice(),
        [0xBE, 0x17, 0x21]
    );
}

#[test]
fn test_program_change_event() {
    let event = ProgramChange(14, 23);
    assert_eq!(
        event.as_bytes().as_slice(),
        [0xCE, 0x17]
    );
}

#[test]
fn test_channel_pressure_event() {
    let event = ChannelPressure(14, 23);
    assert_eq!(
        event.as_bytes().as_slice(),
        [0xDE, 0x17]
    );
}

#[test]
fn test_pitchwheel_change_event() {
    let test_cases: Vec<(f64, (u8, u8))> = vec![
        (-1.0, (0, 0)),
        (-0.5, (0x20, 0x00)),
        (0.0, (0x40, 0x00)),
        (0.5, (0x5f, 0x7F)),
        (1.0, (0x7F, 0x7F))
    ];
    for (input_value, (msb, lsb)) in test_cases.iter() {
        let event = PitchWheelChange(14, *input_value);
        match event {
            PitchWheelChange(_, v) => {
                assert_eq!(v, *input_value);
                assert_eq!(
                    event.as_bytes().as_slice(),
                    [0xEE, *lsb, *msb]
                );
            }
            _ => {
                assert!(false);
            }
        }
    }
}

#[test]
fn test_system_exclusive_event() {
    let event = SystemExclusive(vec![0,0,1,0]);
    assert_eq!(
        event.as_bytes().as_slice(),
        [0xF0, 0x00, 0x00, 0x01, 0x00, 0xF7]
    );
}

#[test]
fn test_chords() {
    assert_eq!(
        get_chord_name_from_mi_sf(0, 253),
        "Eb"
    );
    assert_eq!(
        get_chord_name_from_mi_sf(1, 7),
        "A#m"
    );
}

#[test]
fn test_aerith() {
    let midi = MIDI::from_path("/mnt/media/Audio/Midis/Nobuo Uematsu - Aerith's Theme.mid");

    match midi {
        Ok(_) => {}
        Err(ApresError::InvalidMIDIFile(_)) => {
            assert!(false, "Invalid midi file");
        }
        Err(ApresError::InvalidBytes(bytes)) => {
            assert!(false, format!("{:?}", bytes));
        }
        Err(e) => {
            panic!(e);
        }
    }
}
