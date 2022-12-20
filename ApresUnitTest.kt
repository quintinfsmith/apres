package com.qfs.radixulous

import org.junit.Test
import org.junit.Assert.*
import com.qfs.radixulous.apres.*

fun intlist_to_bytearray(input: List<Int>): ByteArray {
    var output: MutableList<Byte> = mutableListOf()
    for (i in input) {
        output.add(i.toByte())
    }
    return output.toByteArray()
}

class ApresUnitTest {
    @Test
    fun test_initialize_load() {
        var midi_bytes = listOf(
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
        )

        val midi = MIDI.from_bytes(intlist_to_bytearray(midi_bytes))

        assertEquals(midi.count_tracks(), 1)
        assertEquals(midi.get_track_length(0), 121)
        assertEquals(midi.events.size, 16)
        assertEquals(midi.event_positions.size, 16)
    }

    @Test
    fun test_add_event() {
        val midi = MIDI()
        val on_event = midi.push_event(0,0,NoteOn(0, 64, 100))
        var off_event = midi.push_event(0, 119, NoteOff(0,64,0))
        assertEquals(1, on_event)
        assertEquals(2, off_event)
        assertEquals(2, midi.events.size)
        assertEquals(2, midi.event_positions.size)
        assertEquals(1, midi.count_tracks())
        assertEquals(120, midi.get_track_length(0))
    }

    @Test
    fun test_variable_length_conversion() {
        var test_cases = listOf(
            Pair(0, listOf(0.toByte())),
            Pair(127, listOf(0x7F.toByte())),
            Pair(128, listOf(0x81.toByte(), 0x00.toByte())),
            Pair(2097151, listOf(0xFF.toByte(), 0xFF.toByte(), 0x7F.toByte()))
        )

        for ((input_number, expectedlist) in test_cases) {
            var output_bytes = to_variable_length_bytes(input_number)
            assertEquals(expectedlist, output_bytes)

            var output_n = get_variable_length_number(expectedlist.toMutableList())
            assertEquals(input_number, output_n)
        }
    }

}
//#[test]
//fn test_sequence_number_event() {
//    let mut event = SequenceNumber(1);
//    assert_eq!(event.as_bytes().as_slice(), &[0xFF, 0x00, 0x02, 0x00, 0x01]);
//    event = SequenceNumber(13607);
//    assert_eq!(event.as_bytes().as_slice(), &[0xFF, 0x00, 0x02, 0x35, 0x27]);
//
//}
//
//#[test]
//fn test_text_event() {
//    let some_text = "This is some text".to_string();
//    let text_len_bytes = to_variable_length_bytes(some_text.len());
//
//    let event = Text(some_text.clone());
//    let mut compare_vec = vec![ 0xFF, 0x01 ];
//    compare_vec.extend(text_len_bytes.iter().copied());
//    compare_vec.extend(some_text.as_bytes().iter().copied());
//    assert_eq!(
//        event.as_bytes().as_slice(),
//        compare_vec.as_slice()
//    );
//
//}
//
//#[test]
//fn test_copyright_notice_event() {
//    let some_text = "This is some text".to_string();
//    let text_len_bytes = to_variable_length_bytes(some_text.len());
//
//    let event = CopyRightNotice(some_text.clone());
//    let mut compare_vec = vec![ 0xFF, 0x02 ];
//    compare_vec.extend(text_len_bytes.iter().copied());
//    compare_vec.extend(some_text.as_bytes().iter().copied());
//    assert_eq!(
//        event.as_bytes().as_slice(),
//        compare_vec.as_slice()
//    );
//}
//
//#[test]
//fn test_track_name_event() {
//    let some_text = "Some Track Name".to_string();
//    let text_len_bytes = to_variable_length_bytes(some_text.len());
//
//    let event = TrackName(some_text.clone());
//    let mut compare_vec = vec![ 0xFF, 0x03 ];
//    compare_vec.extend(text_len_bytes.iter().copied());
//    compare_vec.extend(some_text.as_bytes().iter().copied());
//    assert_eq!(
//        event.as_bytes().as_slice(),
//        compare_vec.as_slice()
//    );
//
//}
//
//#[test]
//fn test_instrument_name_event() {
//    let some_text = "Some Instrument Name".to_string();
//    let text_len_bytes = to_variable_length_bytes(some_text.len());
//
//    let event = InstrumentName(some_text.clone());
//    let mut compare_vec = vec![ 0xFF, 0x04 ];
//    compare_vec.extend(text_len_bytes.iter().copied());
//    compare_vec.extend(some_text.as_bytes().iter().copied());
//    assert_eq!(
//        event.as_bytes().as_slice(),
//        compare_vec.as_slice()
//    );
//}
//
//#[test]
//fn test_lyric_event() {
//    let some_text = "Here are some Lyrics.".to_string();
//    let text_len_bytes = to_variable_length_bytes(some_text.len());
//
//    let event = Lyric(some_text.clone());
//    let mut compare_vec = vec![ 0xFF, 0x05 ];
//    compare_vec.extend(text_len_bytes.iter().copied());
//    compare_vec.extend(some_text.as_bytes().iter().copied());
//    assert_eq!(
//        event.as_bytes().as_slice(),
//        compare_vec.as_slice()
//    );
//}
//
//#[test]
//fn test_marker_event() {
//    let some_text = "marker text".to_string();
//    let text_len_bytes = to_variable_length_bytes(some_text.len());
//
//    let event = Marker(some_text.clone());
//    let mut compare_vec = vec![ 0xFF, 0x06 ];
//    compare_vec.extend(text_len_bytes.iter().copied());
//    compare_vec.extend(some_text.as_bytes().iter().copied());
//    assert_eq!(
//        event.as_bytes().as_slice(),
//        compare_vec.as_slice()
//    );
//}
//
//#[test]
//fn test_cue_point_event() {
//    let some_text = "cue point text".to_string();
//    let text_len_bytes = to_variable_length_bytes(some_text.len());
//
//    let event = CuePoint(some_text.clone());
//    let mut compare_vec = vec![ 0xFF, 0x07 ];
//    compare_vec.extend(text_len_bytes.iter().copied());
//    compare_vec.extend(some_text.as_bytes().iter().copied());
//    assert_eq!(
//        event.as_bytes().as_slice(),
//        compare_vec.as_slice()
//    );
//}
//
//#[test]
//fn test_end_of_track_event() {
//    let event = EndOfTrack;
//    assert_eq!(
//        event.as_bytes().as_slice(),
//        [0xFF, 0x2F, 0x00]
//    );
//}
//
//#[test]
//fn test_channel_prefix_event() {
//    for i in std::u8::MIN .. std::u8::MAX {
//        let mut event = ChannelPrefix(i as u8);
//        assert_eq!(
//            event.as_bytes().as_slice(),
//            [0xFF, 0x20, 0x01, i]
//        );
//    }
//}
//
//#[test]
//fn test_set_tempo_event() {
//    let test_cases_bpm = vec![
//        (120, 500000),
//        (280, 214285),
//        (1, 0x00FFFFFF),// Minimum bpm is 3.576278762788
//        (60_000_000, 1)
//    ];
//    for (bpm, expected_uspqn) in test_cases_bpm.iter() {
//        let mut event = SetTempo(*expected_uspqn);
//        assert_eq!(
//            event.as_bytes().as_slice(),
//            [
//                0xFF, 0x51, 0x03,
//                ((*expected_uspqn / 256u32.pow(2)) % 256) as u8,
//                ((*expected_uspqn / 256u32.pow(1)) % 256) as u8,
//                (*expected_uspqn % 256) as u8
//            ]
//        );
//    }
//}
//
//#[test]
//fn test_smpte_offset_event() {
//    let event = SMPTEOffset(1,2,3,4,5);
//    assert_eq!(
//        event.as_bytes().as_slice(),
//        [0xFF, 0x54, 0x05, 0x01, 0x02, 0x03, 0x04, 0x05]
//    );
//}
//
//#[test]
//fn test_time_signature_event() {
//    let event = TimeSignature(4, 4, 32, 3);
//    assert_eq!(
//        event.as_bytes().as_slice(),
//        [0xFF, 0x58, 0x04, 0x04, 0x04, 0x20, 0x03]
//    );
//}
//
//#[test]
//fn test_key_signature_event() {
//    let event = KeySignature("A".to_string());
//    assert_eq!(
//        event.as_bytes().as_slice(),
//        [0xFF, 0x59, 0x02, 0x03, 0x00]
//    );
//}
//
//#[test]
//fn test_sequence_specific_event() {
//    let event = SystemExclusive(vec![0x00]);
//    assert_eq!(
//        event.as_bytes().as_slice(),
//        [0xF0, 0x00, 0xF7]
//    );
//}
//
//#[test]
//fn test_note_on_event() {
//    let event = NoteOn(14, 23, 33);
//    assert_eq!(
//        event.as_bytes().as_slice(),
//        [0x9E, 0x17, 0x21]
//    );
//}
//
//#[test]
//fn test_note_off_event() {
//    let event = NoteOff(14, 23, 33);
//    assert_eq!(
//        event.as_bytes().as_slice(),
//        [0x8E, 0x17, 0x21]
//    );
//}
//
//#[test]
//fn test_aftertouch_event() {
//    let event = AfterTouch(14, 23, 33);
//    assert_eq!(
//        event.as_bytes().as_slice(),
//        [0xAE, 0x17, 0x21]
//    );
//}
//
//#[test]
//fn test_program_change_event() {
//    let event = ProgramChange(14, 23);
//    assert_eq!(
//        event.as_bytes().as_slice(),
//        [0xCE, 0x17]
//    );
//}
//
//#[test]
//fn test_channel_pressure_event() {
//    let event = ChannelPressure(14, 23);
//    assert_eq!(
//        event.as_bytes().as_slice(),
//        [0xDE, 0x17]
//    );
//}
//
//#[test]
//fn test_pitchwheel_change_event() {
//    let test_cases: Vec<(f64, (u8, u8))> = vec![
//        (-1.0, (0, 0)),
//        (-0.5, (0x10, 0x00)),
//        (0.0, (0x20, 0x00)),
//        (0.5, (0x2F, 0x7F)),
//        (1.0, (0x3F, 0x7F))
//    ];
//    for (input_value, (msb, lsb)) in test_cases.iter() {
//        let event = PitchWheelChange(14, *input_value);
//        match event {
//            PitchWheelChange(_, v) => {
//                assert_eq!(v, *input_value);
//                assert_eq!(
//                    event.as_bytes().as_slice(),
//                    [0xEE, *lsb, *msb]
//                );
//            }
//            _ => {
//                assert!(false);
//            }
//        }
//    }
//}
//
//#[test]
//fn test_system_exclusive_event() {
//    let event = SystemExclusive(vec![0,0,1,0]);
//    assert_eq!(
//        event.as_bytes().as_slice(),
//        [0xF0, 0x00, 0x00, 0x01, 0x00, 0xF7]
//    );
//}
//
//
//#[test]
//fn test_control_change_events() {
//    let mut event: MIDIEvent;
//    let channel = 1;
//    let value = 25;
//
//    event = ControlChange(0x0E, 0x17, 0x21);
//    assert_eq!( event.as_bytes().as_slice(), [0xBE, 0x17, 0x21]);
//    event = BankSelect(channel, value);
//    assert_eq!(event.as_bytes().as_slice(), [0xB0 | channel, 0x00, value]);
//    event = BankSelectLSB(channel, value);
//    assert_eq!(event.as_bytes().as_slice(), [0xB0 | channel, 0x20, value]);
//    event = ModulationWheel(channel, value);
//    assert_eq!(event.as_bytes().as_slice(), [0xB0 | channel, 0x01, value]);
//    event = ModulationWheelLSB(channel, value);
//    assert_eq!(event.as_bytes().as_slice(), [0xB0 | channel, 0x21, value]);
//    event = BreathController(channel, value);
//    assert_eq!(event.as_bytes().as_slice(), [0xB0 | channel, 0x02, value]);
//    event = BreathControllerLSB(channel, value);
//    assert_eq!(event.as_bytes().as_slice(), [0xB0 | channel, 0x22, value]);
//    event = FootPedal(channel, value);
//    assert_eq!(event.as_bytes().as_slice(), [0xB0 | channel, 0x04, value]);
//    event = FootPedalLSB(channel, value);
//    assert_eq!(event.as_bytes().as_slice(), [0xB0 | channel, 0x24, value]);
//    event = PortamentoTime(channel, value);
//    assert_eq!(event.as_bytes().as_slice(), [0xB0 | channel, 0x05, value]);
//    event = PortamentoTimeLSB(channel, value);
//    assert_eq!(event.as_bytes().as_slice(), [0xB0 | channel, 0x25, value]);
//    event = DataEntry(channel, value);
//    assert_eq!(event.as_bytes().as_slice(), [0xB0 | channel, 0x06, value]);
//    event = DataEntryLSB(channel, value);
//    assert_eq!(event.as_bytes().as_slice(), [0xB0 | channel, 0x26, value]);
//    event = Volume(channel, value);
//    assert_eq!(event.as_bytes().as_slice(), [0xB0 | channel, 0x07, value]);
//    event = VolumeLSB(channel, value);
//    assert_eq!(event.as_bytes().as_slice(), [0xB0 | channel, 0x27, value]);
//    event = Balance(channel, value);
//    assert_eq!(event.as_bytes().as_slice(), [0xB0 | channel, 0x08, value]);
//    event = BalanceLSB(channel, value);
//    assert_eq!(event.as_bytes().as_slice(), [0xB0 | channel, 0x28, value]);
//    event = Pan(channel, value);
//    assert_eq!(event.as_bytes().as_slice(), [0xB0 | channel, 0x0A, value]);
//    event = PanLSB(channel, value);
//    assert_eq!(event.as_bytes().as_slice(), [0xB0 | channel, 0x2A, value]);
//    event = Expression(channel, value);
//    assert_eq!(event.as_bytes().as_slice(), [0xB0 | channel, 0x0B, value]);
//    event = ExpressionLSB(channel, value);
//    assert_eq!(event.as_bytes().as_slice(), [0xB0 | channel, 0x2B, value]);
//    event = EffectControl1(channel, value);
//    assert_eq!(event.as_bytes().as_slice(), [0xB0 | channel, 0x0C, value]);
//    event = EffectControl1LSB(channel, value);
//    assert_eq!(event.as_bytes().as_slice(), [0xB0 | channel, 0x2C, value]);
//    event = EffectControl2(channel, value);
//    assert_eq!(event.as_bytes().as_slice(), [0xB0 | channel, 0x0D, value]);
//    event = EffectControl2LSB(channel, value);
//    assert_eq!(event.as_bytes().as_slice(), [0xB0 | channel, 0x2D, value]);
//    event = GeneralPurpose1(channel, value);
//    assert_eq!(event.as_bytes().as_slice(), [0xB0 | channel, 0x10, value]);
//    event = GeneralPurpose1LSB(channel, value);
//    assert_eq!(event.as_bytes().as_slice(), [0xB0 | channel, 0x30, value]);
//    event = GeneralPurpose2(channel, value);
//    assert_eq!(event.as_bytes().as_slice(), [0xB0 | channel, 0x11, value]);
//    event = GeneralPurpose2LSB(channel, value);
//    assert_eq!(event.as_bytes().as_slice(), [0xB0 | channel, 0x31, value]);
//    event = GeneralPurpose3(channel, value);
//    assert_eq!(event.as_bytes().as_slice(), [0xB0 | channel, 0x12, value]);
//    event = GeneralPurpose3LSB(channel, value);
//    assert_eq!(event.as_bytes().as_slice(), [0xB0 | channel, 0x32, value]);
//    event = GeneralPurpose4(channel, value);
//    assert_eq!(event.as_bytes().as_slice(), [0xB0 | channel, 0x13, value]);
//    event = GeneralPurpose4LSB(channel, value);
//    assert_eq!(event.as_bytes().as_slice(), [0xB0 | channel, 0x33, value]);
//    event = HoldPedal(channel, value);
//    assert_eq!(event.as_bytes().as_slice(), [0xB0 | channel, 0x40, value]);
//    event = Portamento(channel, value);
//    assert_eq!(event.as_bytes().as_slice(), [0xB0 | channel, 0x41, value]);
//    event = Sustenuto(channel, value);
//    assert_eq!(event.as_bytes().as_slice(), [0xB0 | channel, 0x42, value]);
//    event = SoftPedal(channel, value);
//    assert_eq!(event.as_bytes().as_slice(), [0xB0 | channel, 0x43, value]);
//    event = Legato(channel, value);
//    assert_eq!(event.as_bytes().as_slice(), [0xB0 | channel, 0x44, value]);
//    event = Hold2Pedal(channel, value);
//    assert_eq!(event.as_bytes().as_slice(), [0xB0 | channel, 0x45, value]);
//    event = SoundVariation(channel, value);
//    assert_eq!(event.as_bytes().as_slice(), [0xB0 | channel, 0x46, value]);
//    event = SoundTimbre(channel, value);
//    assert_eq!(event.as_bytes().as_slice(), [0xB0 | channel, 0x47, value]);
//    event = SoundReleaseTime(channel, value);
//    assert_eq!(event.as_bytes().as_slice(), [0xB0 | channel, 0x48, value]);
//    event = SoundAttack(channel, value);
//    assert_eq!(event.as_bytes().as_slice(), [0xB0 | channel, 0x49, value]);
//    event = SoundBrightness(channel, value);
//    assert_eq!(event.as_bytes().as_slice(), [0xB0 | channel, 0x4A, value]);
//    event = SoundControl1(channel, value);
//    assert_eq!(event.as_bytes().as_slice(), [0xB0 | channel, 0x4B, value]);
//    event = SoundControl2(channel, value);
//    assert_eq!(event.as_bytes().as_slice(), [0xB0 | channel, 0x4C, value]);
//    event = SoundControl3(channel, value);
//    assert_eq!(event.as_bytes().as_slice(), [0xB0 | channel, 0x4D, value]);
//    event = SoundControl4(channel, value);
//    assert_eq!(event.as_bytes().as_slice(), [0xB0 | channel, 0x4E, value]);
//    event = SoundControl5(channel, value);
//    assert_eq!(event.as_bytes().as_slice(), [0xB0 | channel, 0x4F, value]);
//    event = GeneralPurpose5(channel, value);
//    assert_eq!(event.as_bytes().as_slice(), [0xB0 | channel, 0x50, value]);
//    event = GeneralPurpose6(channel, value);
//    assert_eq!(event.as_bytes().as_slice(), [0xB0 | channel, 0x51, value]);
//    event = GeneralPurpose7(channel, value);
//    assert_eq!(event.as_bytes().as_slice(), [0xB0 | channel, 0x52, value]);
//    event = GeneralPurpose8(channel, value);
//    assert_eq!(event.as_bytes().as_slice(), [0xB0 | channel, 0x53, value]);
//    event = EffectsLevel(channel, value);
//    assert_eq!(event.as_bytes().as_slice(), [0xB0 | channel, 0x5B, value]);
//    event = TremuloLevel(channel, value);
//    assert_eq!(event.as_bytes().as_slice(), [0xB0 | channel, 0x5C, value]);
//    event = ChorusLevel(channel, value);
//    assert_eq!(event.as_bytes().as_slice(), [0xB0 | channel, 0x5D, value]);
//    event = CelesteLevel(channel, value);
//    assert_eq!(event.as_bytes().as_slice(), [0xB0 | channel, 0x5E, value]);
//    event = PhaserLevel(channel, value);
//    assert_eq!(event.as_bytes().as_slice(), [0xB0 | channel, 0x5F, value]);
//    event = RegisteredParameterNumber(channel, value);
//    assert_eq!(event.as_bytes().as_slice(), [0xB0 | channel, 0x65, value]);
//    event = RegisteredParameterNumberLSB(channel, value);
//    assert_eq!(event.as_bytes().as_slice(), [0xB0 | channel, 0x64, value]);
//    event = NonRegisteredParameterNumber(channel, value);
//    assert_eq!(event.as_bytes().as_slice(), [0xB0 | channel, 0x63, value]);
//    event = NonRegisteredParameterNumberLSB(channel, value);
//    assert_eq!(event.as_bytes().as_slice(), [0xB0 | channel, 0x62, value]);
//    event = LocalControl(channel, value);
//    assert_eq!(event.as_bytes().as_slice(), [0xB0 | channel, 0x7A, value]);
//    event = MonophonicOperation(channel, value);
//    assert_eq!(event.as_bytes().as_slice(), [0xB0 | channel, 0xFE, value]);
//
//    event = DataIncrement(channel);
//    assert_eq!(event.as_bytes().as_slice(), [0xB0 | channel, 0x60, 0]);
//    event = DataDecrement(channel);
//    assert_eq!(event.as_bytes().as_slice(), [0xB0 | channel, 0x61, 0]);
//    event = PolyphonicOperation(channel);
//    assert_eq!(event.as_bytes().as_slice(), [0xB0 | channel, 0xFF, 0]);
//    event = AllSoundOff(channel);
//    assert_eq!(event.as_bytes().as_slice(), [0xB0 | channel, 0x78, 0]);
//    event = AllControllersOff(channel);
//    assert_eq!(event.as_bytes().as_slice(), [0xB0 | channel, 0x79, 0]);
//    event = AllNotesOff(channel);
//    assert_eq!(event.as_bytes().as_slice(), [0xB0 | channel, 0x7B, 0]);
//    event = OmniOff(channel);
//    assert_eq!(event.as_bytes().as_slice(), [0xB0 | channel, 0x7C, 0]);
//    event = OmniOn(channel);
//    assert_eq!(event.as_bytes().as_slice(), [0xB0 | channel, 0x7D, 0]);
//}
//
//#[test]
//fn test_chords() {
//    assert_eq!(
//        get_chord_name_from_mi_sf(0, 253),
//        "Eb"
//    );
//    assert_eq!(
//        get_chord_name_from_mi_sf(1, 7),
//        "A#m"
//    );
//}
//
