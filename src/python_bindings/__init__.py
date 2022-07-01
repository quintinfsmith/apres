"""Mutable Midi Library"""
from __future__ import annotations
import sys
import site
import os
import platform
import time
import select
import threading
from ctypes.util import find_library
from typing import Tuple, Optional, List, Dict
from abc import ABC, abstractmethod
from cffi import FFI

class EventNotFound(Exception):
    """Raised when an event id is given that doesn't belong to any known event"""

class AlreadyInMIDI(Exception):
    """
        Raised when attempting to add an event
        to a midi that already has the associated id
    """

class NoMIDI(Exception):
    """
        Raised when attempting to call a function that requires a
        MIDI object be associated with the event, but is not
    """

class InvalidMIDIFile(Exception):
    """Raised when reading an unrecognizeable file"""

class MIDIEvent(ABC):
    """Abstract representation of a MIDI event"""
    uuid_gen: int = 0

    def __init__(self, **_kwargs):
        self.uuid = MIDIEvent.gen_uuid()

    @classmethod
    def gen_uuid(cls) -> int:
        """Create a unique id for an event"""
        cls.uuid_gen += 1
        return cls.uuid_gen

    def get_uuid(self) -> int:
        """Get the unique id of the midi event"""
        return self.uuid

    def set_uuid(self, new_uuid: int) -> None:
        """Apply an id to the event (as when it's generated from MIDIFactory"""
        MIDIEvent.uuid_gen = max(MIDIEvent.uuid_gen, new_uuid)
        self.uuid = new_uuid

    @classmethod
    def from_properties(cls, *_props) -> MIDIEvent:
        """Generate a MIDIEvent from its properties"""
        return cls()

class SequenceNumber(MIDIEvent):
    """Pythonic version of the SequenceNumber event found in MIDI spec."""
    sequence = 0
    def __bytes__(self):
        output = [
            0xFF,
            0x00,
            0x02,
            self.sequence // 256,
            self.sequence % 256
        ]
        return bytes(output)

    def __init__(self, **kwargs):
        self.sequence = kwargs['sequence']
        super().__init__(**kwargs)

    @staticmethod
    def from_properties(cls, *props):
        """Build the MIDIEvent from given list of properties"""
        sequence = 0
        for b in props[0]:
            sequence *= 256
            sequence += b

        return cls(sequence=sequence)

    def get_sequence(self):
        return self.sequence

    def set_sequence(self, sequence):
        self.sequence = sequence

class Text(MIDIEvent):
    """Pythonic version of the Text event found in MIDI spec."""

    text = ''
    def __bytes__(self):
        output = [0xFF, 0x01]
        text_bytes = self.text.encode("utf8")
        output.extend(to_variable_length(len(text_bytes)))
        return bytes(output) + text_bytes

    def __init__(self, text):
        self.text = text
        super().__init__()

    @classmethod
    def from_properties(cls, *props):
        """Build the MIDIEvent from given list of properties"""
        bytelist = bytes(props[0])
        return cls(bytelist.decode("utf8"))

    def get_text(self):
        return self.text

    def set_text(self, text):
        self.text = text


class CopyRightNotice(MIDIEvent):
    """Pythonic version of the CopyRightNotice event found in MIDI spec."""

    text = ""
    def __bytes__(self):
        output = [0xFF, 0x02]
        text_bytes = self.text.encode("utf8")
        output.extend(to_variable_length(len(text_bytes)))
        return bytes(output) + text_bytes

    def __init__(self, text):
        self.text = text
        super().__init__()

    @classmethod
    def from_properties(cls, *props):
        """Build the MIDIEvent from given list of properties"""
        bytelist = bytes(props[0])
        return cls(bytelist.decode("utf8"))

    def get_text(self):
        return self.text

    def set_text(self, text):
        self.text = text

class TrackName(MIDIEvent):
    """Pythonic version of the TrackName event found in MIDI spec."""
    name = ""
    def __bytes__(self):
        output = [0xFF, 0x03]
        text_bytes = self.name.encode("utf8")
        output.extend(to_variable_length(len(text_bytes)))
        return bytes(output) + text_bytes

    def __init__(self, name):
        self.name = name
        super().__init__()

    @classmethod
    def from_properties(cls, *props):
        """Build the MIDIEvent from given list of properties"""
        bytelist = bytes(props[0])
        return cls(bytelist.decode("utf8"))

    def get_name(self):
        return self.name

    def set_name(self, name):
        self.name = name

class InstrumentName(MIDIEvent):
    """Pythonic version of the InstrumentName event found in MIDI spec."""
    name = ""
    def __bytes__(self):
        output = [0xFF, 0x04]
        text_bytes = self.name.encode("utf8")
        output.extend(to_variable_length(len(text_bytes)))
        return bytes(output) + text_bytes

    def __init__(self, name):
        self.name = name
        super().__init__()

    @classmethod
    def from_properties(cls, *props):
        """Build the MIDIEvent from given list of properties"""
        bytelist = bytes(props[0])
        return cls(bytelist.decode("utf8"))

    def get_name(self):
        return self.name

    def set_name(self, name):
        self.name = name

class Lyric(MIDIEvent):
    """Pythonic version of the Lyric event found in MIDI spec."""
    lyric = ""
    def __bytes__(self):
        output = [0xFF, 0x05]
        text_bytes = self.lyric.encode("utf8")
        output.extend(to_variable_length(len(text_bytes)))
        return bytes(output) + text_bytes

    def __init__(self, lyric):
        self.lyric = lyric
        super().__init__()

    @classmethod
    def from_properties(cls, *props):
        """Build the MIDIEvent from given list of properties"""
        bytelist = bytes(props[0])
        return cls(bytelist.decode("utf8"))

    def get_lyric(self):
        return self.lyric

    def set_lyric(self, lyric):
        self.lyric = lyric

class Marker(MIDIEvent):
    """Pythonic version of the Marker event found in MIDI spec."""
    text = ""
    def __bytes__(self):
        output = [0xFF, 0x06]
        text_bytes = self.text.encode("utf8")
        output.extend(to_variable_length(len(text_bytes)))
        return bytes(output) + text_bytes

    def __init__(self, text):
        self.text = text
        super().__init__()

    @classmethod
    def from_properties(cls, *props):
        """Build the MIDIEvent from given list of properties"""
        bytelist = bytes(props[0])
        return cls(bytelist.decode("utf8"))

    def get_text(self):
        return self.text

    def set_text(self, text):
        self.text = text

class CuePoint(MIDIEvent):
    """Pythonic version of the CuePoint event found in MIDI spec."""
    text = ""
    def __bytes__(self):
        output = [0xFF, 0x07]
        text_bytes = self.text.encode("utf8")
        output.extend(to_variable_length(len(text_bytes)))
        return bytes(output) + text_bytes

    def __init__(self, text):
        self.text = text
        super().__init__()

    @classmethod
    def from_properties(cls, *props):
        """Build the MIDIEvent from given list of properties"""
        bytelist = bytes(props[0])
        return cls(bytelist.decode("utf8"))

    def get_text(self):
        return self.text

    def set_text(self, text):
        self.text = text

class EndOfTrack(MIDIEvent):
    """Pythonic version of the EndOfTrack event found in MIDI spec."""
    def __bytes__(self):
        return bytes([0xFF, 0x2F, 0x00])

class ChannelPrefix(MIDIEvent):
    """Pythonic version of the ChannelPrefix event found in MIDI spec."""

    channel = 0
    def __bytes__(self):
        return bytes([0xFF, 0x20, 0x01, self.channel])

    def __init__(self, **kwargs):
        self.channel = kwargs.get("channel", 0)
        super().__init__(**kwargs)

    @classmethod
    def from_properties(cls, *props):
        """Build the MIDIEvent from given list of properties"""
        return cls(channel=props[0][0])

    def get_channel(self):
        return self.channel

    def set_channel(self, channel):
        self.channel = channel

class SetTempo(MIDIEvent):
    """Pythonic version of the SetTempo event found in MIDI spec."""
    us_per_quarter_note = 500000
    def __bytes__(self):
        return bytes([
            0xFF, 0x51, 0x03,
            (self.us_per_quarter_note // (256 ** 2)) % 256,
            (self.us_per_quarter_note // 256) % 256,
            self.us_per_quarter_note % 256
        ])

    def __init__(self, value):
        """Assume low values actually denote a bpm rather than microseconds per quarter note"""
        self.us_per_quarter_note = value

        super().__init__()

    @classmethod
    def from_bpm(cls, bpm):
        return cls(60000000 // bpm)

    @classmethod
    def from_properties(cls, *props):
        """Build the MIDIEvent from given list of properties"""
        us_per_quarter_note = 0
        for n in props[0]:
            us_per_quarter_note *= 256
            us_per_quarter_note += n

        return cls(us_per_quarter_note)


    def get_bpm(self):
        usperqn = self.get_us_per_quarter_note()
        if usperqn:
            output = 60000000 / usperqn
        else:
            output = 0

        return output

    def get_us_per_quarter_note(self):
        return self.us_per_quarter_note

    def set_bpm(self, bpm):
        if not bpm:
            uspqn = 0
        else:
            uspqn = 60000000 // bpm

        self.set_us_per_quarter_note(uspqn)

    def set_us_per_quarter_note(self, uspqn):
        self.us_per_quarter_note = uspqn

class SMPTEOffset(MIDIEvent):
    """Pythonic version of the SMPTEOffset event found in MIDI spec."""
    def __bytes__(self):
        return bytes([
            0xFF, 0x54, 0x05,
            self.hour, self.minute, self.second,
            self.ff, self.fr
        ])

    def __init__(self, **kwargs):
        self.hour = kwargs.get("hour", 0)
        self.minute = kwargs.get("minute", 0)
        self.second = kwargs.get("second", 0)
        self.ff = kwargs.get("ff", 0)
        self.fr = kwargs.get("fr", 0)
        super().__init__(**kwargs)

    @classmethod
    def from_properties(cls, *props):
        """Build the MIDIEvent from given list of properties"""
        return cls(
            hour = props[0][0],
            minute = props[1][0],
            second = props[2][0],
            ff = props[3][0],
            fr = props[4][0]
        )

    def get_hour(self):
        return self.hour

    def get_minute(self):
        return self.minute

    def get_second(self):
        return self.second

    def get_ff(self):
        return self.ff

    def get_fr(self):
        return self.fr

    def set_hour(self, hour):
        self.hour = hour

    def set_minute(self, minute):
        self.minute = minute

    def set_second(self, second):
        self.second = second

    def set_ff(self, ff):
        self.ff = ff

    def set_fr(self, fr):
        self.fr = fr


class TimeSignature(MIDIEvent):
    """Pythonic version of the TimeSignature event found in MIDI spec."""
    def __bytes__(self):
        return bytes([
            0xFF, 0x58, 0x04,
            self.numerator, self.denominator,
            self.clocks_per_metronome,
            self.thirtysecondths_per_quarter
        ])

    def __init__(self, **kwargs):
        self.numerator = kwargs.get("numerator", 4)
        self.denominator = kwargs.get("denominator", 2)
        self.clocks_per_metronome = kwargs.get("cpm", 0)
        self.thirtysecondths_per_quarter = kwargs.get("tspqn", 0)
        super().__init__(**kwargs)

    @classmethod
    def from_properties(cls, *props):
        """Build the MIDIEvent from given list of properties"""
        return cls(
            numerator = props[0][0],
            denominator = props[1][0],
            clocks_per_metronome = props[2][0],
            thirtysecondths_per_quarter = props[3][0]
        )

    def get_numerator(self):
        return self.numerator

    def get_denominator(self):
        return self.denominator

    def get_clocks_per_metronome(self):
        return self.clocks_per_metronome

    def get_thirtysecondths_per_quarter_note(self):
        return self.thirtysecondths_per_quarter

    def set_numerator(self, numerator):
        self.numerator = numerator

    def set_denominator(self, denominator):
        self.denominator = denominator

    def set_clocks_per_metronome(self, cpm):
        self.clocks_per_metronome = cpm

    def set_thirtysecondths_per_quarter_note(self, tspqn):
        self.thirtysecondths_per_quarter = tspqn


class KeySignature(MIDIEvent):
    """Pythonic version of the KeySignature event found in MIDI spec."""
    misf_map = {
        "A": (0, 3),
        "A#": (0, 8 | 2),
        "Bb": (0, 8 | 2),
        "B": (0, 5),
        "C": (0, 0),
        "C#": (0, 7),
        "Db": (0, 7),
        "D": (0, 2),
        "D#": (0, 8 | 3),
        "Eb": (0, 8 | 3),
        "E": (0, 4),
        "F": (0, 8 | 1),
        "F#": (0, 6),
        "Gb": (0, 6),
        "Am": (1, 0),
        "A#m": (1, 7),
        "Bbm": (1, 7),
        "Bm": (1, 2),
        "Cm": (1, 8 | 3),
        "C#m": (1, 4),
        "Dbm": (1, 4),
        "Dm": (1, 8 | 1),
        "D#m": (1, 6),
        "Ebm": (1, 6),
        "Em": (1, 1),
        "Fm": (1, 8 | 4),
        "F#m": (1, 3),
        "Gbm": (1, 3),
        "Gm": (1, 8 | 2)
    }

    def __bytes__(self):
        mi, sf = self.misf_map[self.key]
        return bytes([0xFF, 0x59, 0x02, sf, mi])

    def __init__(self, **kwargs):
        self.key = kwargs.get("key", "C")
        super().__init__(**kwargs)

    @classmethod
    def from_mi_sf(cls, mi, sf):
        key = "C"
        for chordname, pair in cls.misf_map.items():
            if pair == (mi, sf):
                key = chordname
                break
        return cls(key=key)

    @classmethod
    def from_properties(cls, *props):
        """Build the MIDIEvent from given list of properties"""
        return cls.from_mi_sf(props[0], props[1])

    def get_key(self):
        return self.key

    def set_key(self, key):
        self.key = key


class Sequencer(MIDIEvent):
    """Pythonic version of the Sequencer event found in MIDI spec."""
    data = b''
    def __bytes__(self):
        output = [0xFF, 0x7F]
        data_length = len(self.data)
        output.extend(to_variable_length(data_length))
        return bytes(output) + self.data

    def __init__(self, data):
        self.data = data
        super().__init__()

    @classmethod
    def from_properties(cls, *props):
        """Build the MIDIEvent from given list of properties"""
        return cls(bytes(props[0]))

    def get_data(self):
        return self.data

    def set_data(self, data):
        self.data = data


class NoteOn(MIDIEvent):
    """Pythonic version of the NoteOn event found in MIDI spec."""
    def __bytes__(self):
        return bytes([
            0x90 | self.channel,
            self.note,
            self.velocity
        ])
    def __init__(self, **kwargs):
        self.note = kwargs["note"]
        self.channel = kwargs.get("channel", 0)
        self.velocity = kwargs.get("velocity", 64)
        super().__init__(**kwargs)

    @classmethod
    def from_properties(cls, *props):
        """Build the MIDIEvent from given list of properties"""
        return cls(
            channel = props[0][0],
            note = props[1][0],
            velocity = props[2][0]
        )

    def get_channel(self):
        return self.channel

    def get_note(self):
        return self.note

    def get_velocity(self):
        return self.velocity

    def set_channel(self, channel):
        self.channel = channel

    def set_note(self, note):
        self.note = note

    def set_velocity(self, velocity):
        self.velocity = velocity

class NoteOff(MIDIEvent):
    """Pythonic version of the NoteOff event found in MIDI spec."""
    def __bytes__(self):
        return bytes([
            0x80 | self.channel,
            self.note,
            self.velocity
        ])

    def __init__(self, **kwargs):
        self.note = kwargs["note"]
        self.channel = kwargs.get("channel", 0)
        self.velocity = kwargs.get("velocity", 64)
        super().__init__(**kwargs)

    @classmethod
    def from_properties(cls, *props):
        """Build the MIDIEvent from given list of properties"""
        return cls(
            channel = props[0][0],
            note = props[1][0],
            velocity = props[2][0]
        )

    def get_channel(self):
        return self.channel

    def get_note(self):
        return self.note

    def get_velocity(self):
        return self.velocity

    def set_channel(self, channel):
        self.channel = channel

    def set_note(self, note):
        self.note = note

    def set_velocity(self, velocity):
        self.velocity = velocity

class PolyphonicKeyPressure(MIDIEvent):
    """Pythonic version of the PolyphonicKeyPressure event found in MIDI spec."""

    def __bytes__(self):
        return bytes([
            0xA0 | self.channel,
            self.note,
            self.pressure
        ])

    def __init__(self, **kwargs):
        self.note = kwargs["note"]
        self.pressure = kwargs["pressure"]
        self.channel = kwargs.get("channel", 0)
        super().__init__(**kwargs)

    @classmethod
    def from_properties(cls, *props):
        """Build the MIDIEvent from given list of properties"""
        return cls(
            channel = props[0][0],
            note = props[1][0],
            pressure = props[2][0]
        )

    def get_channel(self):
        return self.channel

    def get_note(self):
        return self.note

    def get_pressure(self):
        return self.pressure

    def set_channel(self, channel):
        self.channel = channel

    def set_note(self, note):
        self.note = note

    def set_pressure(self, pressure):
        self.pressure = pressure

class ControlChange(MIDIEvent):
    """Pythonic version of the ControlChange event found in MIDI spec."""

    def __bytes__(self):
        return bytes([
            0xB0 | self.channel,
            self.get_controller(),
            self.get_value()
        ])

    def __init__(self, controller, value, **kwargs):
        self.controller = controller
        self.value = value
        self.channel = kwargs.get("channel", 0)
        super().__init__(**kwargs)

    @classmethod
    def from_properties(cls, *props):
        """Build the MIDIEvent from given list of properties"""
        return cls(
            value = props[2][0],
            channel = props[0][0],
            controller = props[1][0]
        )

    def get_channel(self):
        return self.channel

    def get_controller(self):
        return self.controller

    def get_value(self):
        return self.value

    def set_channel(self, channel):
        self.channel = channel

    def set_controller(self, controller):
        self.controller = controller

    def set_value(self, value):
        self.value = value


class VariableControlChange(ControlChange):
    CONTROL_BYTE = 0

    def get_controller(self):
        '''Get *constant* control byte value'''
        return self.__class__.CONTROL_BYTE

    def __init__(self, value, **kwargs):
        super().__init__(self.get_controller(), value, **kwargs)

    @classmethod
    def from_properties(cls, *props):
        """Build the MIDIEvent from given list of properties"""
        return cls(
            props[1][0],
            channel = props[0][0]
        )

class InvariableControlChange(VariableControlChange):
    def get_value(self):
        '''
            Get 0. Invariable ControlChanges are invariable
            and have a constant value of 0
        '''
        return 0

    def __init__(self, **kwargs):
        super().__init__(0, **kwargs)

    @classmethod
    def from_properties(cls, *props):
        """Build the MIDIEvent from given list of properties"""
        return cls(
            channel = props[0][0]
        )

class HoldPedal(VariableControlChange):
    """Pythonic version of the HoldPedal event found in MIDI spec."""
    CONTROL_BYTE = 0x40
class Portamento(VariableControlChange):
    """Pythonic version of the Portamento event found in MIDI spec."""
    CONTROL_BYTE = 0x41
class Sustenuto(VariableControlChange):
    """Pythonic version of the Sustenuto event found in MIDI spec."""
    CONTROL_BYTE = 0x42
class SoftPedal(VariableControlChange):
    """Pythonic version of the SoftPedal event found in MIDI spec."""
    CONTROL_BYTE = 0x43
class Legato(VariableControlChange):
    """Pythonic version of the Legato event found in MIDI spec."""
    CONTROL_BYTE = 0x44
class Hold2Pedal(VariableControlChange):
    """Pythonic version of the Hold2Pedal event found in MIDI spec."""
    CONTROL_BYTE = 0x45
class SoundVariation(VariableControlChange):
    """Pythonic version of the SoundVariation event found in MIDI spec."""
    CONTROL_BYTE = 0x46
class SoundTimbre(VariableControlChange):
    """Pythonic version of the SoundTimbre event found in MIDI spec."""
    CONTROL_BYTE = 0x47
class SoundReleaseTime(VariableControlChange):
    """Pythonic version of the SoundReleaseTime event found in MIDI spec."""
    CONTROL_BYTE = 0x48
class SoundAttack(VariableControlChange):
    """Pythonic version of the SoundAttack event found in MIDI spec."""
    CONTROL_BYTE = 0x49
class SoundBrightness(VariableControlChange):
    """Pythonic version of the SoundBrightness event found in MIDI spec."""
    CONTROL_BYTE = 0x4A
class SoundControl1(VariableControlChange):
    """Pythonic version of the SoundControl1 event found in MIDI spec."""
    CONTROL_BYTE = 0x4B
class SoundControl2(VariableControlChange):
    """Pythonic version of the SoundControl2 event found in MIDI spec."""
    CONTROL_BYTE = 0x4C
class SoundControl3(VariableControlChange):
    """Pythonic version of the SoundControl3 event found in MIDI spec."""
    CONTROL_BYTE = 0x4D
class SoundControl4(VariableControlChange):
    """Pythonic version of the SoundControl4 event found in MIDI spec."""
    CONTROL_BYTE = 0x4E
class SoundControl5(VariableControlChange):
    """Pythonic version of the SoundControl5 event found in MIDI spec."""
    CONTROL_BYTE = 0x4F


class EffectsLevel(VariableControlChange):
    """Pythonic version of the EffectsLevel event found in MIDI spec."""
    CONTROL_BYTE = 0x5B
class TremuloLevel(VariableControlChange):
    """Pythonic version of the TremuloLevel event found in MIDI spec."""
    CONTROL_BYTE = 0x5C
class ChorusLevel(VariableControlChange):
    """Pythonic version of the ChorusLevel event found in MIDI spec."""
    CONTROL_BYTE = 0x5D
class CelesteLevel(VariableControlChange):
    """Pythonic version of the CelesteLevel event found in MIDI spec."""
    CONTROL_BYTE = 0x5E
class PhaserLevel(VariableControlChange):
    """Pythonic version of the PhaserLevel event found in MIDI spec."""
    CONTROL_BYTE = 0x5F
class LocalControl(VariableControlChange):
    """Pythonic version of the LocalControl event found in MIDI spec."""
    CONTROL_BYTE = 0x7A
class MonophonicOperation(VariableControlChange):
    """Pythonic version of the MonophonicOperation event found in MIDI spec."""
    CONTROL_BYTE = 0xFE

class DataIncrement(InvariableControlChange):
    """Pythonic version of the DataIncrement event found in MIDI spec."""
    CONTROL_BYTE = 0x60
class DataDecrement(InvariableControlChange):
    """Pythonic version of the DataDecrement event found in MIDI spec."""
    CONTROL_BYTE = 0x61
class AllControllersOff(InvariableControlChange):
    """Pythonic version of the AllControllersOff event found in MIDI spec."""
    CONTROL_BYTE = 0x79

class AllNotesOff(InvariableControlChange):
    """Pythonic version of the AllNotesOff event found in MIDI spec."""
    CONTROL_BYTE = 0x7B
class AllSoundOff(InvariableControlChange):
    """Pythonic version of the AllSoundOff event found in MIDI spec."""
    CONTROL_BYTE = 0x78
class OmniOff(InvariableControlChange):
    """Pythonic version of the OmniOff event found in MIDI spec."""
    CONTROL_BYTE = 0x7C
class OmniOn(InvariableControlChange):
    """Pythonic version of the OmniOn event found in MIDI spec."""
    CONTROL_BYTE = 0x7D
class PolyphonicOperation(InvariableControlChange):
    """Pythonic version of the PolyphonicOperation event found in MIDI spec."""
    CONTROL_BYTE = 0xFF

class BankSelect(VariableControlChange):
    """Pythonic version of the BankSelect event found in MIDI spec."""
    CONTROL_BYTE = 0x00
class BankSelectLSB(VariableControlChange):
    """Pythonic version of the BankSelectLSB event found in MIDI spec."""
    CONTROL_BYTE = 0x20
class ModulationWheel(VariableControlChange):
    """Pythonic version of the ModulationWheel event found in MIDI spec."""
    CONTROL_BYTE = 0x01
class ModulationWheelLSB(VariableControlChange):
    """Pythonic version of the ModulationWheelLSB event found in MIDI spec."""
    CONTROL_BYTE = 0x21
class BreathController(VariableControlChange):
    """Pythonic version of the BreathController event found in MIDI spec."""
    CONTROL_BYTE = 0x02
class BreathControllerLSB(VariableControlChange):
    """Pythonic version of the BreathControllerLSB event found in MIDI spec."""
    CONTROL_BYTE = 0x22
class FootPedal(VariableControlChange):
    """Pythonic version of the FootPedal event found in MIDI spec."""
    CONTROL_BYTE = 0x04
class FootPedalLSB(VariableControlChange):
    """Pythonic version of the FootPedalLSB event found in MIDI spec."""
    CONTROL_BYTE = 0x24
class PortamentoTime(VariableControlChange):
    """Pythonic version of the PortamentoTime event found in MIDI spec."""
    CONTROL_BYTE = 0x05
class PortamentoTimeLSB(VariableControlChange):
    """Pythonic version of the PortamentoTimeLSB event found in MIDI spec."""
    CONTROL_BYTE = 0x25
class DataEntry(VariableControlChange):
    """Pythonic version of the DataEntry event found in MIDI spec."""
    CONTROL_BYTE = 0x06
class DataEntryLSB(VariableControlChange):
    """Pythonic version of the DataEntryLSB event found in MIDI spec."""
    CONTROL_BYTE = 0x26
class Volume(VariableControlChange):
    """Pythonic version of the Volume event found in MIDI spec."""
    CONTROL_BYTE = 0x07
class VolumeLSB(VariableControlChange):
    """Pythonic version of the VolumeLSB event found in MIDI spec."""
    CONTROL_BYTE = 0x27
class Balance(VariableControlChange):
    """Pythonic version of the Balance event found in MIDI spec."""
    CONTROL_BYTE = 0x08
class BalanceLSB(VariableControlChange):
    """Pythonic version of the BalanceLSB event found in MIDI spec."""
    CONTROL_BYTE = 0x28
class Pan(VariableControlChange):
    """Pythonic version of the Pan event found in MIDI spec."""
    CONTROL_BYTE = 0x0A
class PanLSB(VariableControlChange):
    """Pythonic version of the PanLSB event found in MIDI spec."""
    CONTROL_BYTE = 0x2A
class Expression(VariableControlChange):
    """Pythonic version of the Expression event found in MIDI spec."""
    CONTROL_BYTE = 0x0B
class ExpressionLSB(VariableControlChange):
    """Pythonic version of the ExpressionLSB event found in MIDI spec."""
    CONTROL_BYTE = 0x2B

class NonRegisteredParameterNumber(VariableControlChange):
    """Pythonic version of the NonRegisteredParameterNumber event found in MIDI spec."""
    CONTROL_BYTE = 0x63
class NonRegisteredParameterNumberLSB(VariableControlChange):
    """Pythonic version of the NonRegisteredParameterNumberLSB event found in MIDI spec."""
    CONTROL_BYTE = 0x62
class RegisteredParameterNumber(VariableControlChange):
    """Pythonic version of the RegisteredParameterNumber event found in MIDI spec."""
    CONTROL_BYTE = 0x65
class RegisteredParameterNumberLSB(VariableControlChange):
    """Pythonic version of the RegisteredParameterNumberLSB event found in MIDI spec."""
    CONTROL_BYTE = 0x64

class EffectControl1(VariableControlChange):
    """Pythonic version of the EffectControl1 event found in MIDI spec."""
    CONTROL_BYTE = 0x0C
class EffectControl1LSB(VariableControlChange):
    """Pythonic version of the EffectControl1LSB event found in MIDI spec."""
    CONTROL_BYTE = 0x2C
class EffectControl2(VariableControlChange):
    """Pythonic version of the EffectControl2 event found in MIDI spec."""
    CONTROL_BYTE = 0x0D
class EffectControl2LSB(VariableControlChange):
    """Pythonic version of the EffectControl2LSB event found in MIDI spec."""
    CONTROL_BYTE = 0x2D
class GeneralPurpose1(VariableControlChange):
    """Pythonic version of the GeneralPurpose1 event found in MIDI spec."""
    CONTROL_BYTE = 0x10
class GeneralPurpose1LSB(VariableControlChange):
    """Pythonic version of the GeneralPurpose1LSB event found in MIDI spec."""
    CONTROL_BYTE = 0x30
class GeneralPurpose2(VariableControlChange):
    """Pythonic version of the GeneralPurpose2 event found in MIDI spec."""
    CONTROL_BYTE = 0x11
class GeneralPurpose2LSB(VariableControlChange):
    """Pythonic version of the GeneralPurpose2LSB event found in MIDI spec."""
    CONTROL_BYTE = 0x31
class GeneralPurpose3(VariableControlChange):
    """Pythonic version of the GeneralPurpose3 event found in MIDI spec."""
    CONTROL_BYTE = 0x12
class GeneralPurpose3LSB(VariableControlChange):
    """Pythonic version of the GeneralPurpose3LSB event found in MIDI spec."""
    CONTROL_BYTE = 0x32
class GeneralPurpose4(VariableControlChange):
    """Pythonic version of the GeneralPurpose4 event found in MIDI spec."""
    CONTROL_BYTE = 0x13
class GeneralPurpose4LSB(VariableControlChange):
    """Pythonic version of the GeneralPurpose4LSB event found in MIDI spec."""
    CONTROL_BYTE = 0x33
class GeneralPurpose5(VariableControlChange):
    """Pythonic version of the GeneralPurpose5 event found in MIDI spec."""
    CONTROL_BYTE = 0x50
class GeneralPurpose6(VariableControlChange):
    """Pythonic version of the GeneralPurpose6 event found in MIDI spec."""
    CONTROL_BYTE = 0x51
class GeneralPurpose7(VariableControlChange):
    """Pythonic version of the GeneralPurpose7 event found in MIDI spec."""
    CONTROL_BYTE = 0x52
class GeneralPurpose8(VariableControlChange):
    """Pythonic version of the GeneralPurpose8 event found in MIDI spec."""
    CONTROL_BYTE = 0x53

class ProgramChange(MIDIEvent):
    """Pythonic version of the ProgramChange event found in MIDI spec."""
    def __bytes__(self):
        return bytes([
            0xC0 | self.channel,
            self.program
        ])

    def __init__(self, program, **kwargs):
        self.program = program
        self.channel = kwargs.get("channel", 0)
        super().__init__(**kwargs)

    @classmethod
    def from_properties(cls, *props):
        """Build the MIDIEvent from given list of properties"""
        return cls(
            props[1][0],
            channel = props[0][0]
        )

    def get_channel(self):
        return self.channel

    def get_program(self):
        return self.program

    def set_channel(self, channel):
        self.channel = channel

    def set_program(self, program):
        self.program = program


class ChannelPressure(MIDIEvent):
    """Pythonic version of the ChannelPressure event found in MIDI spec."""
    def __bytes__(self):
        return bytes([
            0xD0 | self.channel,
            self.pressure
        ])

    def __init__(self, pressure, **kwargs):
        self.pressure = pressure
        self.channel = kwargs.get("channel", 0)
        super().__init__(**kwargs)

    @classmethod
    def from_properties(cls, *props):
        """Build the MIDIEvent from given list of properties"""
        return cls(
            props[1][0],
            channel = props[0][0]
        )

    def get_channel(self):
        return self.channel

    def get_pressure(self):
        return self.pressure

    def set_channel(self, channel):
        self.channel = channel

    def set_pressure(self, pressure):
        self.pressure = pressure

# TODO: Store as signed integer, set 0x2000 to == 0
class PitchWheelChange(MIDIEvent):
    """Pythonic version of the PitchWheelChange event found in MIDI spec."""
    """
        NOTE: value is stored as float from [-1, 1]
    """


    def __bytes__(self):
        unsigned_value = self.get_unsigned_value()
        least = unsigned_value & 0x007F
        most = (unsigned_value >> 8) & 0x007F
        return bytes([(0xE0 | self.channel), least, most])

    def __init__(self, value, **kwargs):
        self.value = value
        self.channel = kwargs.get("channel", 0)
        super().__init__(**kwargs)

    @classmethod
    def from_properties(cls, *props):
        """Build the MIDIEvent from given list of properties"""
        cls.channel = props[0][0]

        prop = props[1]
        unsigned_value = (prop[0] * 256) + prop[1]
        value = ((unsigned_value * 2) / 0x3FFF) - 1

        return cls(
            value,
            channel = props[0][0]
        )

    def get_channel(self):
        return self.channel

    def get_value(self):
        return self.value

    def set_channel(self, channel):
        self.channel = channel

    def set_value(self, value):
        self.value = value

    def get_unsigned_value(self):
        """ get value as integer in range (0, 0x3FFF) """
        if self.value == 0:
            output = 0x2000
        else:
            output = int(((self.value + 1) * 0x3FFF) // 2)
        return output

class SystemExclusive(MIDIEvent):
    """Pythonic version of the SystemExclusive event found in MIDI spec."""
    data = b''
    def __bytes__(self):
        output = [0xF0]
        for b in self.data:
            output.append(b)
        output.append(0xF7)
        return bytes(output)

    def __init__(self, data):
        self.data = data
        super().__init__()

    @classmethod
    def from_properties(cls, *props):
        """Build the MIDIEvent from given list of properties"""
        return cls(bytes(props[0]))

    def get_data(self):
        return self.data

    def set_data(self, new_data):
        self.data = new_data

class MTCQuarterFrame(MIDIEvent):
    """Pythonic version of the MTCQuarterFrame event found in MIDI spec."""
    time_code = 0
    def __bytes__(self):
        return bytes([0xF1, self.time_code])

    def __init__(self, time_code):
        self.time_code = time_code & 0xFF
        super().__init__(**kwargs)

    @classmethod
    def from_properties(cls, *props):
        """Build the MIDIEvent from given list of properties"""
        return cls(props[0][0] & 0xFF)

    def get_time_code(self):
        return self.time_code

class SongPositionPointer(MIDIEvent):
    """Pythonic version of the SongPositionPointer event found in MIDI spec."""

    def __bytes__(self):
        least = self.beat & 0x007F
        most = (self.beat >> 8) & 0x007F
        return bytes([0xF2, least, most])

    def __init__(self, beat):
        self.beat = beat
        super().__init__()

    @classmethod
    def from_properties(cls, *props):
        """Build the MIDIEvent from given list of properties"""
        prop = props[0]
        return cls((prop[0] * 256) + prop[1])

    def get_beat(self):
        return self.beat

    def set_beat(self, beat):
        self.beat = beat

class SongSelect(MIDIEvent):
    """Pythonic version of the SongSelect event found in MIDI spec."""
    def __bytes__(self):
        return bytes([0xF3, self.song & 0xFF])

    def __init__(self, song):
        self.song = song
        super().__init__()

    @classmethod
    def from_properties(cls, *props):
        """Build the MIDIEvent from given list of properties"""
        return cls(props[0][0])

    def get_song(self):
        return self.song

    def set_song(self, song):
        self.song = song

class TuneRequest(MIDIEvent):
    """Pythonic version of the TuneRequest event found in MIDI spec."""
    def __bytes__(self):
        return bytes([0xF6])

class MIDIClock(MIDIEvent):
    """Pythonic version of the MIDIClock event found in MIDI spec."""
    def __bytes__(self):
        return bytes([0xF8])

class MIDIStart(MIDIEvent):
    """Pythonic version of the MIDIStart event found in MIDI spec."""
    def __bytes__(self):
        return bytes([0xFA])

class MIDIContinue(MIDIEvent):
    """Pythonic version of the MIDIContinue event found in MIDI spec."""
    def __bytes__(self):
        return bytes([0xFB])

class MIDIStop(MIDIEvent):
    """Pythonic version of the MIDIStop event found in MIDI spec."""
    def __bytes__(self):
        return bytes([0xFC])

class ActiveSense(MIDIEvent):
    """Pythonic version of the ActiveSense event found in MIDI spec."""
    def __bytes__(self):
        return bytes([0xFE])

class Reset(MIDIEvent):
    """Pythonic version of the Reset event found in MIDI spec."""
    def __bytes__(self):
        return bytes([0xFF])

class TimeCode(MIDIEvent):
    """Pythonic version of the TimeCode event found in MIDI spec."""
    def __init__(self, **kwargs):
        self.rate = kwargs["rate"]
        self.hour = kwargs["hour"]
        self.minute = kwargs["minute"]
        self.second = kwargs["second"]
        self.frame = kwargs["frame"]
        super().__init__(**kwargs)

    @classmethod
    def from_properties(cls, *props):
        """Build the MIDIEvent from given list of properties"""
        return cls(
            rate = props[0][0],
            hour = props[1][0],
            minute = props[2][0],
            second = props[3][0],
            frame = props[4][0]
        )

    def __bytes__(self):
        return bytes([
            (self.rate << 5) + self.hour,
            self.minute & 0x3F,
            self.second & 0x3F,
            self.frame & 0x1F
        ])

class MIDI:
    """Usable object. Converted from midi files."""

    @staticmethod
    def load(path) -> MIDI:
        """Load a MIDI file"""
        return MIDIFactory.load(path)

    def save(self, path):
        """Save a MIDI file"""
        return MIDIFactory.save(path, self)

    def __init__(self, **kwargs):
        self.events = {}
        self.event_positions = {}
        self.ppqn = kwargs.get('ppqn', 120)
        self.format = kwargs.get('format', 1)

    def get_all_events(self) -> List[Tuple[int, MIDIEvent]]:
        """Get sorted list of events in midi events"""
        event_list = []
        for event_id, (_, tick) in self.event_positions.items():
            event_list.append((tick, event_id))
        event_list.sort()

        output = []
        for tick, event_id in event_list:
            output.append((tick, self.events[event_id]))

        return output

    def get_track_events(self) -> Dict[int, List[Tuple[int, MIDIEvent]]]:
        """Get all events in midi, grouped by track"""
        event_list = []
        for event_id, (track, tick) in self.event_positions.items():
            event_list.append((track, tick, event_id))
        event_list.sort()

        output = {}
        for track, tick, event_id in event_list:
            if track not in output:
                output[track] = []
            output[track].append((tick, self.events[event_id]))

        return output

    def add_event(self, event, **kwargs):
        """ Add Midi Event to the Midi """

        active_track = kwargs.get('track', 0)
        if "tick" in kwargs:
            active_tick = kwargs["tick"]
        elif "wait" in kwargs:
            active_tick = self.track_get_length(active_track) - 1 + kwargs['wait']
        else:
            active_tick = self.track_get_length(active_track) - 1

        self.events[event.get_uuid()] = event
        self.place_event(event, active_track, active_tick)

    def track_get_length(self, track_number: int) -> int:
        """Get number of ticks in a track"""
        max_tick = 0
        for track, tick in self.event_positions:
            if track != track_number:
                continue
            max_tick = max(tick, max_tick)

        return max_tick


    def place_event(self, event: MIDIEvent, track: int, tick: int) -> None:
        """Put a MIDIEvent at a specific position in the piece"""
        self.events[event.get_uuid()] = event
        self.event_positions[event.get_uuid()] = (track, tick)


class MIDIFactory:
    """Acts as a wrapper for the cffi bridge. Used to load and save midi files"""

    ffi = FFI()
    ffi.cdef("""
        typedef void* MIDI;

        MIDI interpret(const char*);
        MIDI new();
        void save(MIDI, const char*);
        uint32_t get_track_length(MIDI, uint8_t);
        uint32_t count_tracks(MIDI);
        uint32_t count_events(MIDI);

        uint8_t get_event_property_count(MIDI, uint64_t);
        uint8_t* get_event_property(MIDI, uint64_t, uint8_t);
        uint8_t get_event_property_length(MIDI, uint64_t, uint8_t);
        uint8_t get_event_type(MIDI, uint64_t);
        uint64_t create_event(MIDI, uint8_t, uint64_t, const uint8_t*, uint8_t);

        void replace_event(MIDI, uint64_t, const uint8_t*, uint8_t);
        void set_event_position(MIDI, uint64_t, uint8_t, uint64_t);

        uint64_t get_event_tick(MIDI, uint64_t);
        uint8_t get_event_track(MIDI, uint64_t);

        void set_ppqn(MIDI, uint16_t);
        void set_format(MIDI, uint16_t);
        uint16_t get_ppqn(MIDI);
    """)

    file_directory =  __file__[0:__file__.rfind("/") + 1]
    lib_path = f"{file_directory}libapres_manylinux2014_{platform.machine()}.so"
    lib = ffi.dlopen(lib_path)

    event_constructors = {
        1: Text,
        2: CopyRightNotice,
        3: TrackName,
        4: InstrumentName,
        5: Lyric,
        6: Marker,
        7: CuePoint,
        8: EndOfTrack,
        9: ChannelPrefix,
        10: SetTempo,
        11: SMPTEOffset,
        12: TimeSignature,
        13: KeySignature,
        14: Sequencer,
        23: SystemExclusive,

        15: NoteOn,
        16: NoteOff,
        17: PolyphonicKeyPressure,
        18: ControlChange,
        58: HoldPedal,
        59: Portamento,
        60: Sustenuto,
        61: SoftPedal,
        62: Legato,
        63: Hold2Pedal,
        64: SoundVariation,
        65: SoundTimbre,
        66: SoundReleaseTime,
        67: SoundAttack,
        68: SoundBrightness,
        69: SoundControl1,
        70: SoundControl2,
        71: SoundControl3,
        72: SoundControl4,
        73: SoundControl5,
        86: EffectsLevel,
        87: TremuloLevel,
        88: ChorusLevel,
        89: CelesteLevel,
        90: PhaserLevel,
        103: MonophonicOperation,
        91: DataIncrement,
        92: DataDecrement,
        98: LocalControl,
        97: AllControllersOff,
        99: AllNotesOff,
        100: AllSoundOff,
        101: OmniOff,
        102: OmniOn,
        104: PolyphonicOperation,
        34: BankSelect,
        35: BankSelectLSB,
        36: ModulationWheel,
        37: ModulationWheelLSB,
        38: BreathController,
        39: BreathControllerLSB,
        40: FootPedal,
        41: FootPedalLSB,
        42: PortamentoTime,
        43: PortamentoTimeLSB,
        44: DataEntry,
        45: DataEntryLSB,
        46: Volume,
        47: VolumeLSB,
        48: Balance,
        49: BalanceLSB,
        50: Pan,
        51: PanLSB,
        52: Expression,
        53: ExpressionLSB,
        95: NonRegisteredParameterNumber,
        96: NonRegisteredParameterNumberLSB,
        93: RegisteredParameterNumber,
        94: RegisteredParameterNumberLSB,

        54: EffectControl1,
        55: EffectControl1LSB,
        56: EffectControl2,
        57: EffectControl2LSB,
        74: GeneralPurpose1,
        75: GeneralPurpose1LSB,
        76: GeneralPurpose2,
        77: GeneralPurpose2LSB,
        78: GeneralPurpose3,
        79: GeneralPurpose3LSB,
        80: GeneralPurpose4,
        81: GeneralPurpose4LSB,
        82: GeneralPurpose5,
        83: GeneralPurpose6,
        84: GeneralPurpose7,
        85: GeneralPurpose8,

        19: ProgramChange,
        20: ChannelPressure,
        21: PitchWheelChange,
        22: SequenceNumber
    }

    @classmethod
    def save(cls, midi: MIDI, path: str) -> None:
        """Save the midi to a file"""
        pointer = cls.lib.new()
        cls.lib.set_ppqn(pointer, midi.get_ppqn())
        cls.lib.set_format(pointer, midi.get_format())

        for track, ticks in midi.get_track_events():
            for tick, event in ticks:
                byte_rep = bytes(event)
                cls.lib.create_event(track, tick, byte_rep, len(byte_rep))

        fmt_path = bytes(path, 'utf-8')
        cls.lib.save(pointer, fmt_path)

    @classmethod
    def event_get_properties(cls, pointer, event_uuid):
        """ Get an event's properties via the CFFI bridge"""
        count = cls.lib.get_event_property_count(pointer, event_uuid)
        props = []
        for i in range(count):
            props.append(cls.event_get_property(pointer, event_uuid, i))

        return props

    @classmethod
    def event_get_property(cls, pointer, event_uuid, event_property):
        """ Get an event's property via the CFFI bridge"""
        length = cls.lib.get_event_property_length(pointer, event_uuid, event_property)
        bufferlist = bytearray(length)
        array_pointer = cls.lib.get_event_property(pointer, event_uuid, event_property)
        cls.ffi.memmove(bufferlist, array_pointer, length)
        return bufferlist

    @classmethod
    def load(cls, path: str):
        """Load a MIDI from a path"""
        midi = MIDI()

        fmt_path = bytes(path, 'utf-8')
        pointer = cls.lib.interpret(fmt_path)

        midi.ppqn = cls.lib.get_ppqn(pointer)

        #Kludge: using ppqn == 0  to indicate a bad Midi
        if midi.ppqn == 0:
            raise InvalidMIDIFile()

        # 0 is reserved, but eids are generated in order.
        # So we don't need to query every individual active id at this point
        for eid in range(1, cls.lib.count_events(pointer)):
            type_num = cls.lib.get_event_type(pointer, eid)
            if type_num == 0:
                raise EventNotFound()

            constructor = cls.event_constructors[type_num]
            props = cls.event_get_properties(pointer, eid)
            event = constructor.from_properties(*props)
            event.set_uuid(eid)

            tick = cls.lib.get_event_tick(pointer, eid) - 1
            track = cls.lib.get_event_track(pointer, eid) - 1

            midi.add_event(event, track=track, tick=tick)
        return midi


class PipeClosed(Exception):
    """Error Thrown when the midi device pipe is closed or disconnected"""

class MIDIController:
    """Read Input from Midi Device"""
    CONTROL_EVENT_MAP = {
        HoldPedal.CONTROL_BYTE: HoldPedal,
        Portamento.CONTROL_BYTE: Portamento,
        Sustenuto.CONTROL_BYTE: Sustenuto,
        SoftPedal.CONTROL_BYTE: SoftPedal,
        Legato.CONTROL_BYTE: Legato,
        Hold2Pedal.CONTROL_BYTE: Hold2Pedal,
        SoundVariation.CONTROL_BYTE: SoundVariation,
        SoundTimbre.CONTROL_BYTE: SoundTimbre,
        SoundReleaseTime.CONTROL_BYTE: SoundReleaseTime,
        SoundAttack.CONTROL_BYTE: SoundAttack,
        SoundBrightness.CONTROL_BYTE: SoundBrightness,
        SoundControl1.CONTROL_BYTE: SoundControl1,
        SoundControl2.CONTROL_BYTE: SoundControl2,
        SoundControl3.CONTROL_BYTE: SoundControl3,
        SoundControl4.CONTROL_BYTE: SoundControl4,
        SoundControl5.CONTROL_BYTE: SoundControl5,
        EffectsLevel.CONTROL_BYTE: EffectsLevel,
        TremuloLevel.CONTROL_BYTE: TremuloLevel,
        ChorusLevel.CONTROL_BYTE: ChorusLevel,
        CelesteLevel.CONTROL_BYTE: CelesteLevel,
        PhaserLevel.CONTROL_BYTE: PhaserLevel,
        LocalControl.CONTROL_BYTE: LocalControl,
        MonophonicOperation.CONTROL_BYTE: MonophonicOperation,
        BankSelect.CONTROL_BYTE: BankSelect,
        BankSelectLSB.CONTROL_BYTE: BankSelectLSB,
        ModulationWheel.CONTROL_BYTE: ModulationWheel,
        ModulationWheelLSB.CONTROL_BYTE: ModulationWheelLSB,
        BreathController.CONTROL_BYTE: BreathController,
        BreathControllerLSB.CONTROL_BYTE: BreathControllerLSB,
        FootPedal.CONTROL_BYTE: FootPedal,
        FootPedalLSB.CONTROL_BYTE: FootPedalLSB,
        PortamentoTime.CONTROL_BYTE: PortamentoTime,
        PortamentoTimeLSB.CONTROL_BYTE: PortamentoTimeLSB,
        DataEntry.CONTROL_BYTE: DataEntry,
        DataEntryLSB.CONTROL_BYTE: DataEntryLSB,
        Volume.CONTROL_BYTE: Volume,
        VolumeLSB.CONTROL_BYTE: VolumeLSB,
        Balance.CONTROL_BYTE: Balance,
        BalanceLSB.CONTROL_BYTE: BalanceLSB,
        Pan.CONTROL_BYTE: Pan,
        PanLSB.CONTROL_BYTE: PanLSB,
        Expression.CONTROL_BYTE: Expression,
        ExpressionLSB.CONTROL_BYTE: ExpressionLSB,
        NonRegisteredParameterNumber.CONTROL_BYTE: NonRegisteredParameterNumber,
        NonRegisteredParameterNumberLSB.CONTROL_BYTE: NonRegisteredParameterNumberLSB,
        RegisteredParameterNumber.CONTROL_BYTE: RegisteredParameterNumber,
        RegisteredParameterNumberLSB.CONTROL_BYTE: RegisteredParameterNumberLSB,
        EffectControl1.CONTROL_BYTE: EffectControl1,
        EffectControl1LSB.CONTROL_BYTE: EffectControl1LSB,
        EffectControl2.CONTROL_BYTE: EffectControl2,
        EffectControl2LSB.CONTROL_BYTE: EffectControl2LSB,
        GeneralPurpose1.CONTROL_BYTE: GeneralPurpose1,
        GeneralPurpose1LSB.CONTROL_BYTE: GeneralPurpose1LSB,
        GeneralPurpose2.CONTROL_BYTE: GeneralPurpose2,
        GeneralPurpose2LSB.CONTROL_BYTE: GeneralPurpose2LSB,
        GeneralPurpose3.CONTROL_BYTE: GeneralPurpose3,
        GeneralPurpose3LSB.CONTROL_BYTE: GeneralPurpose3LSB,
        GeneralPurpose4.CONTROL_BYTE: GeneralPurpose4,
        GeneralPurpose4LSB.CONTROL_BYTE: GeneralPurpose4LSB,
        GeneralPurpose5.CONTROL_BYTE: GeneralPurpose5,
        GeneralPurpose6.CONTROL_BYTE: GeneralPurpose6,
        GeneralPurpose7.CONTROL_BYTE: GeneralPurpose7,
        GeneralPurpose8.CONTROL_BYTE: GeneralPurpose8,
        # Invariable ControlChanges
        DataIncrement.CONTROL_BYTE: DataIncrement,
        DataDecrement.CONTROL_BYTE: DataDecrement,
        AllControllersOff.CONTROL_BYTE: AllControllersOff,
        AllNotesOff.CONTROL_BYTE: AllNotesOff,
        AllSoundOff.CONTROL_BYTE: AllSoundOff,
        OmniOff.CONTROL_BYTE: OmniOff,
        OmniOn.CONTROL_BYTE: OmniOn,
        PolyphonicOperation.CONTROL_BYTE: PolyphonicOperation
    }
    def __init__(self, default_path=""):
        self.pipe = None
        self.listening = False
        self.midipath = None
        self.connect(default_path)
        self.hook_map = {}
        self.event_queue = []

    def set_hook(self, event_type, hook):
        self.hook_map[event_type] = hook

    def is_connected(self):
        """Check if pipe is open and ready to be read"""
        return bool(self.pipe)

    def connect(self, path):
        """Attempt to open a pipe to the path specified"""
        if not self.pipe:
            self.pipe = open(path, 'rb')
            self.midipath = path

    def disconnect(self):
        """Close the pipe."""
        try:
            if self.pipe is not None:
                self.pipe.close()
        except Exception as e:
            raise e

        self.pipe = None
        self.midipath = ''

    def close(self):
        """Tear down this midi controller"""
        self.disconnect()

    def get_next_byte(self):
        """Attempt to read next byte from pipe"""
        output = None
        while output is None:
            try:
                ready, _, __ = select.select([self.pipe], [], [], 0)
            except TypeError:
                ready = []
            except ValueError:
                ready = []

            if self.is_connected():
                if self.pipe in ready:
                    try:
                        output = os.read(self.pipe.fileno(), 1)
                        if output:
                            output = output[0]
                        else:
                            continue
                    except ValueError:
                        continue

                else:
                    # wait for input
                    time.sleep(.01)
            else:
                raise PipeClosed()

        return output

    def listen(self):
        """Listen to the midi device for incoming bits. Process them and call their hooks."""
        self.listening = True
        pq_thread = threading.Thread(target=self._process_queue)
        pq_thread.start()
        while self.listening:
            try:
                event = self.get_next_event()
            except PipeClosed:
                self.listening = False
                event = None

            if not event:
                continue

            #FIXME: Massive kludge so I don't have to put in a shit tonne of bioler-plate, empty functions.
            hookname = "hook_" + str(type(event).__name__)
            if hookname in dir(self):
                self.event_queue.append((self.__getattribute__(hookname), event))

    def _process_queue(self):
        while self.listening:
            try:
                hook, event = self.event_queue.pop(0)
                hook(event)
            except IndexError:
                time.sleep(.01)

    def get_next_event(self):
        """Read Midi Input Device until relevant event is found"""
        lead_byte = self.get_next_byte()

        output = None
        if lead_byte & 0xF0 == 0x80:
            channel = lead_byte & 0x0F
            note = self.get_next_byte()
            velocity = self.get_next_byte()
            output = NoteOff(channel=channel, note=note, velocity=velocity)

        elif lead_byte & 0xF0 == 0x90:
            channel = lead_byte & 0x0F
            note = self.get_next_byte()
            velocity = self.get_next_byte()
            if velocity == 0:
                output = NoteOff(channel=channel, note=note, velocity=0)
            else:
                output = NoteOn(channel=channel, note=note, velocity=velocity)

        elif lead_byte & 0xF0 == 0xA0:
            channel = lead_byte & 0x0F
            note = self.get_next_byte()
            velocity = self.get_next_byte()
            output = PolyphonicKeyPressure(channel=channel, note=note, velocity=velocity)

        elif lead_byte & 0xF0 == 0xB0:
            channel = lead_byte & 0x0F
            controller = self.get_next_byte()
            constructor = MIDIController.CONTROL_EVENT_MAP.get(controller, ControlChange)

            value = self.get_next_byte()
            output = constructor(
                channel=channel,
                controller=controller,
                value=value
            )

        elif lead_byte & 0xF0 == 0xC0:
            channel = lead_byte & 0x0F
            new_program = self.get_next_byte()
            output = ProgramChange(channel=channel, program=new_program)

        elif lead_byte & 0xF0 == 0xD0:
            channel = lead_byte & 0x0F
            pressure = self.get_next_byte()
            output = ChannelPressure(channel=channel, pressure=pressure)

        elif lead_byte & 0xF0 == 0xE0:
            channel = lead_byte & 0x0F

            lsb = self.get_next_byte()
            msb = self.get_next_byte()

            unsigned = (msb << 8) + (lsb & 0x7F)
            value = ((0x3FFF * unsigned) - 2) / 2

            output = PitchWheelChange(channel=channel, value=value)

        elif lead_byte == 0xF0:
            # System Exclusive
            bytedump = []

            byte = self.get_next_byte()
            while byte != 0xF7:
                bytedump.append(byte)
                byte = self.get_next_byte()

            output = SystemExclusive(bytedump)

            # Time Code
        elif lead_byte == 0xF1:
            byte_a = self.get_next_byte()
            coded_rate = byte_a >> 5
            if coded_rate == 0:
                rate = 24
            elif coded_rate == 1:
                rate = 25
            elif coded_rate == 2:
                rate = 29.97
            else:
                rate = 30

            hour = byte_a & 0x1F
            minute = self.get_next_byte() & 0x3F
            second = self.get_next_byte() & 0x3F
            frame = self.get_next_byte() & 0x1F

            output = TimeCode(rate=rate, hour=hour, minute=minute, second=second, frame=frame)

        elif lead_byte == 0xF2:
            least_significant_byte = self.get_next_byte()
            most_significant_byte = self.get_next_byte()
            beat = (most_significant_byte << 8) + least_significant_byte
            output = SongPositionPointer(beat=beat)

        elif lead_byte == 0xF3:
            song = self.get_next_byte()
            output = SongSelect(song & 0x7F)

        elif lead_byte == 0xF6:
            output = TuneRequest()

        elif lead_byte == 0xF7:
            # Real Time SysEx
            for _ in range(self.get_next_byte()):
                byte = self.get_next_byte()
                bytedump.push(byte)

            output = SystemExclusive(bytedump)

        # Clock
        elif lead_byte == 0xF8:
            output = MIDIClock()
        # Start
        elif lead_byte == 0xFA:
            output = MIDIStart()
        # Continue
        elif lead_byte == 0xFB:
            output = MIDIContinue()
        #Stop
        elif lead_byte == 0xFC:
            output = MIDIStop()
        #Active Sensing
        elif lead_byte == 0xFE:
            output = ActiveSense()
        # System Reset
        elif lead_byte == 0xFF:
            output = Reset()

        return output

def to_variable_length(number):
    """Convert a number to midi's variable-length format"""
    output = []
    is_first_pass = True
    working_number = number
    while working_number > 0 or is_first_pass:
        tmp = working_number & 0x7F
        working_number >>= 7
        if not is_first_pass:
            tmp |= 0x80
        else:
            is_first_pass = False
        output.append(tmp)
    return bytes(output[::-1])
