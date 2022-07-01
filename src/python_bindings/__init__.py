'''Mutable Midi Library'''
from __future__ import annotations
import sys
import site
import os
import platform
import time
import select
import threading
from ctypes.util import find_library
from typing import Tuple, Optional
from abc import ABC, abstractmethod
from cffi import FFI

class EventNotFound(Exception):
    '''Raised when an event id is given that doesn't belong to any known event'''

class AlreadyInMIDI(Exception):
    '''
        Raised when attempting to add an event
        to a midi that already has the associated id
    '''

class NoMIDI(Exception):
    '''
        Raised when attempting to call a function that requires a
        MIDI object be associated with the event, but is not
    '''

class InvalidMIDIFile(Exception):
    '''Raised when reading an unrecognizeable file'''

class MIDIEvent(ABC):
    '''Abstract representation of a MIDI event'''
    uuid_gen = 0

    def __init__(self, **_kwargs):
        self.uuid = MIDIEvent.gen_uuid()

    @classmethod
    def gen_uuid(self):
        '''Create a unique id for an event'''
        self.uuid_gen += 1
        return self.uuid_gen

    def get_uuid(self):
        '''Get the unique id of the midi event'''
        return self.uuid

    def set_uuid(self, new_uuid: int):
        '''Apply an id to the event (as when it's generated from MIDIFactory'''
        MIDIEvent.uuid_gen = max(MIDIEvent.uuid_gen, new_uuid)
        self.uuid = new_uuid

    @classmethod
    def from_properties(cls, *_props):
        '''Generate a MIDIEvent from its properties'''
        return cls()

class SequenceNumber(MIDIEvent):
    @staticmethod
    def get_rust_id():
        """Get the event code defined in .rs"""
        return 22
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
    @staticmethod
    def get_rust_id():
        """Get the event code defined in .rs"""
        return 1

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
        bytelist = bytes(props[0])
        return cls(bytelist.decode("utf8"))

    def get_text(self):
        return self.text

    def set_text(self, text):
        self.text = text


class CopyRightNotice(MIDIEvent):
    @staticmethod
    def get_rust_id():
        """Get the event code defined in .rs"""
        return 2

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
        bytelist = bytes(props[0])
        return cls(bytelist.decode("utf8"))

    def get_text(self):
        return self.text

    def set_text(self, text):
        self.text = text

class TrackName(MIDIEvent):
    @staticmethod
    def get_rust_id():
        """Get the event code defined in .rs"""
        return 3

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
        bytelist = bytes(props[0])
        return cls(bytelist.decode("utf8"))

    def get_name(self):
        return self.name

    def set_name(self, name):
        self.name = name

class InstrumentName(MIDIEvent):
    @staticmethod
    def get_rust_id():
        """Get the event code defined in .rs"""
        return 4
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
        bytelist = bytes(props[0])
        return cls(bytelist.decode("utf8"))

    def get_name(self):
        return self.name

    def set_name(self, name):
        self.name = name

class Lyric(MIDIEvent):
    @staticmethod
    def get_rust_id():
        """Get the event code defined in .rs"""
        return 5
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
        bytelist = bytes(props[0])
        return cls(bytelist.decode("utf8"))

    def get_lyric(self):
        return self.lyric

    def set_lyric(self, lyric):
        self.lyric = lyric

class Marker(MIDIEvent):
    @staticmethod
    def get_rust_id():
        """Get the event code defined in .rs"""
        return 6
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
        bytelist = bytes(props[0])
        return cls(bytelist.decode("utf8"))

    def get_text(self):
        return self.text

    def set_text(self, text):
        self.text = text

class CuePoint(MIDIEvent):
    @staticmethod
    def get_rust_id():
        """Get the event code defined in .rs"""
        return 7
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
        bytelist = bytes(props[0])
        return cls(bytelist.decode("utf8"))

    def get_text(self):
        return self.text

    def set_text(self, text):
        self.text = text

class EndOfTrack(MIDIEvent):
    @staticmethod
    def get_rust_id():
        """Get the event code defined in .rs"""
        return 8
    def __bytes__(self):
        return bytes([0xFF, 0x2F, 0x00])

class ChannelPrefix(MIDIEvent):
    @staticmethod
    def get_rust_id():
        """Get the event code defined in .rs"""
        return 9

    channel = 0
    def __bytes__(self):
        return bytes([0xFF, 0x20, 0x01, self.channel])

    def __init__(self, **kwargs):
        self.channel = kwargs.get("channel", 0)
        super().__init__(**kwargs)

    @classmethod
    def from_properties(cls, *props):
        return cls(channel=props[0][0])

    def get_channel(self):
        return self.channel

    def set_channel(self, channel):
        self.channel = channel

class SetTempo(MIDIEvent):
    @staticmethod
    def get_rust_id():
        """Get the event code defined in .rs"""
        return 10
    us_per_quarter_note = 500000
    def __bytes__(self):
        return bytes([
            0xFF, 0x51, 0x03,
            (self.us_per_quarter_note // (256 ** 2)) % 256,
            (self.us_per_quarter_note // 256) % 256,
            self.us_per_quarter_note % 256
        ])

    def __init__(self, value):
        '''Assume low values actually denote a bpm rather than microseconds per quarter note'''
        self.us_per_quarter_note = value

        super().__init__()

    @classmethod
    def from_bpm(cls, bpm):
        return cls(60000000 // bpm)

    @classmethod
    def from_properties(cls, *props):
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
    @staticmethod
    def get_rust_id():
        """Get the event code defined in .rs"""
        return 11
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
    @staticmethod
    def get_rust_id():
        """Get the event code defined in .rs"""
        return 12
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
    @staticmethod
    def get_rust_id():
        """Get the event code defined in .rs"""
        return 13
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
        return cls.from_mi_sf(props[0], props[1])

    def get_key(self):
        return self.key

    def set_key(self, key):
        self.key = key


class Sequencer(MIDIEvent):
    @staticmethod
    def get_rust_id():
        """Get the event code defined in .rs"""
        return 14
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
        return cls(bytes(props[0]))

    def get_data(self):
        return self.data

    def set_data(self, data):
        self.data = data


class NoteOn(MIDIEvent):
    @staticmethod
    def get_rust_id():
        """Get the event code defined in .rs"""
        return 15
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
    @staticmethod
    def get_rust_id():
        """Get the event code defined in .rs"""
        return 16
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
    @staticmethod
    def get_rust_id():
        """Get the event code defined in .rs"""
        return 17

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
    @staticmethod
    def get_rust_id():
        """Get the event code defined in .rs"""
        return 18

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
        return self.CONTROL_BYTE

    def __init__(self, value, **kwargs):
        super().__init__(self.get_controller(), value, **kwargs)

    @classmethod
    def from_properties(cls, *props):
        return cls(
            props[1][0],
            channel = props[0][0]
        )

class InvariableControlChange(VariableControlChange):
    def get_value(self):
        return 0

    def __init__(self, **kwargs):
        super().__init__(0, **kwargs)

    @classmethod
    def from_properties(cls, *props):
        return cls(
            channel = props[0][0]
        )

class HoldPedal(VariableControlChange):
    @staticmethod
    def get_rust_id():
        """Get the event code defined in .rs"""
        return 58
    CONTROL_BYTE = 0x40
class Portamento(VariableControlChange):
    @staticmethod
    def get_rust_id():
        """Get the event code defined in .rs"""
        return 59
    CONTROL_BYTE = 0x41
class Sustenuto(VariableControlChange):
    @staticmethod
    def get_rust_id():
        """Get the event code defined in .rs"""
        return 60
    CONTROL_BYTE = 0x42
class SoftPedal(VariableControlChange):
    @staticmethod
    def get_rust_id():
        """Get the event code defined in .rs"""
        return 61
    CONTROL_BYTE = 0x43
class Legato(VariableControlChange):
    @staticmethod
    def get_rust_id():
        """Get the event code defined in .rs"""
        return 62
    CONTROL_BYTE = 0x44
class Hold2Pedal(VariableControlChange):
    @staticmethod
    def get_rust_id():
        """Get the event code defined in .rs"""
        return 63
    CONTROL_BYTE = 0x45
class SoundVariation(VariableControlChange):
    @staticmethod
    def get_rust_id():
        """Get the event code defined in .rs"""
        return 64
    CONTROL_BYTE = 0x46
class SoundTimbre(VariableControlChange):
    @staticmethod
    def get_rust_id():
        """Get the event code defined in .rs"""
        return 65
    CONTROL_BYTE = 0x47
class SoundReleaseTime(VariableControlChange):
    @staticmethod
    def get_rust_id():
        """Get the event code defined in .rs"""
        return 66
    CONTROL_BYTE = 0x48
class SoundAttack(VariableControlChange):
    @staticmethod
    def get_rust_id():
        """Get the event code defined in .rs"""
        return 67
    CONTROL_BYTE = 0x49
class SoundBrightness(VariableControlChange):
    @staticmethod
    def get_rust_id():
        """Get the event code defined in .rs"""
        return 68
    CONTROL_BYTE = 0x4A
class SoundControl1(VariableControlChange):
    @staticmethod
    def get_rust_id():
        """Get the event code defined in .rs"""
        return 69
    CONTROL_BYTE = 0x4B
class SoundControl2(VariableControlChange):
    @staticmethod
    def get_rust_id():
        """Get the event code defined in .rs"""
        return 70
    CONTROL_BYTE = 0x4C
class SoundControl3(VariableControlChange):
    @staticmethod
    def get_rust_id():
        """Get the event code defined in .rs"""
        return 71
    CONTROL_BYTE = 0x4D
class SoundControl4(VariableControlChange):
    @staticmethod
    def get_rust_id():
        """Get the event code defined in .rs"""
        return 72
    CONTROL_BYTE = 0x4E
class SoundControl5(VariableControlChange):
    @staticmethod
    def get_rust_id():
        """Get the event code defined in .rs"""
        return 73
    CONTROL_BYTE = 0x4F


class EffectsLevel(VariableControlChange):
    @staticmethod
    def get_rust_id():
        """Get the event code defined in .rs"""
        return 86
    CONTROL_BYTE = 0x5B
class TremuloLevel(VariableControlChange):
    @staticmethod
    def get_rust_id():
        """Get the event code defined in .rs"""
        return 87
    CONTROL_BYTE = 0x5C
class ChorusLevel(VariableControlChange):
    @staticmethod
    def get_rust_id():
        """Get the event code defined in .rs"""
        return 88
    CONTROL_BYTE = 0x5D
class CelesteLevel(VariableControlChange):
    @staticmethod
    def get_rust_id():
        """Get the event code defined in .rs"""
        return 89
    CONTROL_BYTE = 0x5E
class PhaserLevel(VariableControlChange):
    @staticmethod
    def get_rust_id():
        """Get the event code defined in .rs"""
        return 90
    CONTROL_BYTE = 0x5F
class LocalControl(VariableControlChange):
    @staticmethod
    def get_rust_id():
        """Get the event code defined in .rs"""
        return 98
    CONTROL_BYTE = 0x7A
class MonophonicOperation(VariableControlChange):
    @staticmethod
    def get_rust_id():
        """Get the event code defined in .rs"""
        return 103
    CONTROL_BYTE = 0xFE

class DataIncrement(InvariableControlChange):
    @staticmethod
    def get_rust_id():
        """Get the event code defined in .rs"""
        return 91
    CONTROL_BYTE = 0x60
class DataDecrement(InvariableControlChange):
    @staticmethod
    def get_rust_id():
        """Get the event code defined in .rs"""
        return 92
    CONTROL_BYTE = 0x61
class AllControllersOff(InvariableControlChange):
    @staticmethod
    def get_rust_id():
        """Get the event code defined in .rs"""
        return 97
    CONTROL_BYTE = 0x79

class AllNotesOff(InvariableControlChange):
    @staticmethod
    def get_rust_id():
        """Get the event code defined in .rs"""
        return 99
    CONTROL_BYTE = 0x7B
class AllSoundOff(InvariableControlChange):
    @staticmethod
    def get_rust_id():
        """Get the event code defined in .rs"""
        return 100
    CONTROL_BYTE = 0x78
class OmniOff(InvariableControlChange):
    @staticmethod
    def get_rust_id():
        """Get the event code defined in .rs"""
        return 101
    CONTROL_BYTE = 0x7C
class OmniOn(InvariableControlChange):
    @staticmethod
    def get_rust_id():
        """Get the event code defined in .rs"""
        return 102
    CONTROL_BYTE = 0x7D
class PolyphonicOperation(InvariableControlChange):
    @staticmethod
    def get_rust_id():
        """Get the event code defined in .rs"""
        return 104
    CONTROL_BYTE = 0xFF

class BankSelect(VariableControlChange):
    @staticmethod
    def get_rust_id():
        """Get the event code defined in .rs"""
        return 34
    CONTROL_BYTE = 0x00
class BankSelectLSB(VariableControlChange):
    @staticmethod
    def get_rust_id():
        """Get the event code defined in .rs"""
        return 35
    CONTROL_BYTE = 0x20
class ModulationWheel(VariableControlChange):
    @staticmethod
    def get_rust_id():
        """Get the event code defined in .rs"""
        return 36
    CONTROL_BYTE = 0x01
class ModulationWheelLSB(VariableControlChange):
    @staticmethod
    def get_rust_id():
        """Get the event code defined in .rs"""
        return 37
    CONTROL_BYTE = 0x21
class BreathController(VariableControlChange):
    @staticmethod
    def get_rust_id():
        """Get the event code defined in .rs"""
        return 38
    CONTROL_BYTE = 0x02
class BreathControllerLSB(VariableControlChange):
    @staticmethod
    def get_rust_id():
        """Get the event code defined in .rs"""
        return 39
    CONTROL_BYTE = 0x22
class FootPedal(VariableControlChange):
    @staticmethod
    def get_rust_id():
        """Get the event code defined in .rs"""
        return 40
    CONTROL_BYTE = 0x04
class FootPedalLSB(VariableControlChange):
    @staticmethod
    def get_rust_id():
        """Get the event code defined in .rs"""
        return 41
    CONTROL_BYTE = 0x24
class PortamentoTime(VariableControlChange):
    @staticmethod
    def get_rust_id():
        """Get the event code defined in .rs"""
        return 42
    CONTROL_BYTE = 0x05
class PortamentoTimeLSB(VariableControlChange):
    @staticmethod
    def get_rust_id():
        """Get the event code defined in .rs"""
        return 43
    CONTROL_BYTE = 0x25
class DataEntry(VariableControlChange):
    @staticmethod
    def get_rust_id():
        """Get the event code defined in .rs"""
        return 44
    CONTROL_BYTE = 0x06
class DataEntryLSB(VariableControlChange):
    @staticmethod
    def get_rust_id():
        """Get the event code defined in .rs"""
        return 45
    CONTROL_BYTE = 0x26
class Volume(VariableControlChange):
    @staticmethod
    def get_rust_id():
        """Get the event code defined in .rs"""
        return 46
    CONTROL_BYTE = 0x07
class VolumeLSB(VariableControlChange):
    @staticmethod
    def get_rust_id():
        """Get the event code defined in .rs"""
        return 47
    CONTROL_BYTE = 0x27
class Balance(VariableControlChange):
    @staticmethod
    def get_rust_id():
        """Get the event code defined in .rs"""
        return 48
    CONTROL_BYTE = 0x08
class BalanceLSB(VariableControlChange):
    @staticmethod
    def get_rust_id():
        """Get the event code defined in .rs"""
        return 49
    CONTROL_BYTE = 0x28
class Pan(VariableControlChange):
    @staticmethod
    def get_rust_id():
        """Get the event code defined in .rs"""
        return 50
    CONTROL_BYTE = 0x0A
class PanLSB(VariableControlChange):
    @staticmethod
    def get_rust_id():
        """Get the event code defined in .rs"""
        return 51
    CONTROL_BYTE = 0x2A
class Expression(VariableControlChange):
    @staticmethod
    def get_rust_id():
        """Get the event code defined in .rs"""
        return 52
    CONTROL_BYTE = 0x0B
class ExpressionLSB(VariableControlChange):
    @staticmethod
    def get_rust_id():
        """Get the event code defined in .rs"""
        return 53
    CONTROL_BYTE = 0x2B

class NonRegisteredParameterNumber(VariableControlChange):
    @staticmethod
    def get_rust_id():
        """Get the event code defined in .rs"""
        return 95
    CONTROL_BYTE = 0x63
class NonRegisteredParameterNumberLSB(VariableControlChange):
    @staticmethod
    def get_rust_id():
        """Get the event code defined in .rs"""
        return 96
    CONTROL_BYTE = 0x62
class RegisteredParameterNumber(VariableControlChange):
    @staticmethod
    def get_rust_id():
        """Get the event code defined in .rs"""
        return 93
    CONTROL_BYTE = 0x65
class RegisteredParameterNumberLSB(VariableControlChange):
    @staticmethod
    def get_rust_id():
        """Get the event code defined in .rs"""
        return 94
    CONTROL_BYTE = 0x64

class EffectControl1(VariableControlChange):
    @staticmethod
    def get_rust_id():
        """Get the event code defined in .rs"""
        return 54
    CONTROL_BYTE = 0x0C
class EffectControl1LSB(VariableControlChange):
    @staticmethod
    def get_rust_id():
        """Get the event code defined in .rs"""
        return 55
    CONTROL_BYTE = 0x2C
class EffectControl2(VariableControlChange):
    @staticmethod
    def get_rust_id():
        """Get the event code defined in .rs"""
        return 56
    CONTROL_BYTE = 0x0D
class EffectControl2LSB(VariableControlChange):
    @staticmethod
    def get_rust_id():
        """Get the event code defined in .rs"""
        return 57
    CONTROL_BYTE = 0x2D
class GeneralPurpose1(VariableControlChange):
    @staticmethod
    def get_rust_id():
        """Get the event code defined in .rs"""
        return 74
    CONTROL_BYTE = 0x10
class GeneralPurpose1LSB(VariableControlChange):
    @staticmethod
    def get_rust_id():
        """Get the event code defined in .rs"""
        return 75
    CONTROL_BYTE = 0x30
class GeneralPurpose2(VariableControlChange):
    @staticmethod
    def get_rust_id():
        """Get the event code defined in .rs"""
        return 76
    CONTROL_BYTE = 0x11
class GeneralPurpose2LSB(VariableControlChange):
    @staticmethod
    def get_rust_id():
        """Get the event code defined in .rs"""
        return 77
    CONTROL_BYTE = 0x31
class GeneralPurpose3(VariableControlChange):
    @staticmethod
    def get_rust_id():
        """Get the event code defined in .rs"""
        return 78
    CONTROL_BYTE = 0x12
class GeneralPurpose3LSB(VariableControlChange):
    @staticmethod
    def get_rust_id():
        """Get the event code defined in .rs"""
        return 79
    CONTROL_BYTE = 0x32
class GeneralPurpose4(VariableControlChange):
    @staticmethod
    def get_rust_id():
        """Get the event code defined in .rs"""
        return 80
    CONTROL_BYTE = 0x13
class GeneralPurpose4LSB(VariableControlChange):
    @staticmethod
    def get_rust_id():
        """Get the event code defined in .rs"""
        return 81
    CONTROL_BYTE = 0x33
class GeneralPurpose5(VariableControlChange):
    @staticmethod
    def get_rust_id():
        """Get the event code defined in .rs"""
        return 82
    CONTROL_BYTE = 0x50
class GeneralPurpose6(VariableControlChange):
    @staticmethod
    def get_rust_id():
        """Get the event code defined in .rs"""
        return 83
    CONTROL_BYTE = 0x51
class GeneralPurpose7(VariableControlChange):
    @staticmethod
    def get_rust_id():
        """Get the event code defined in .rs"""
        return 84
    CONTROL_BYTE = 0x52
class GeneralPurpose8(VariableControlChange):
    @staticmethod
    def get_rust_id():
        """Get the event code defined in .rs"""
        return 85
    CONTROL_BYTE = 0x53

class ProgramChange(MIDIEvent):
    @staticmethod
    def get_rust_id():
        """Get the event code defined in .rs"""
        return 19
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
    @staticmethod
    def get_rust_id():
        """Get the event code defined in .rs"""
        return 20
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
    '''
        NOTE: value is stored as float from [-1, 1]
    '''

    @staticmethod
    def get_rust_id():
        """Get the event code defined in .rs"""
        return 21

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
        ''' get value as integer in range (0, 0x3FFF) '''
        if self.value == 0:
            output = 0x2000
        else:
            output = int(((self.value + 1) * 0x3FFF) // 2)
        return output

class SystemExclusive(MIDIEvent):
    @staticmethod
    def get_rust_id():
        """Get the event code defined in .rs"""
        return 23
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
        return cls(bytes(props[0]))

    def get_data(self):
        return self.data

    def set_data(self, new_data):
        self.data = new_data

class MTCQuarterFrame(MIDIEvent):
    @staticmethod
    def get_rust_id():
        """Get the event code defined in .rs"""
        return 24
    time_code = 0
    def __bytes__(self):
        return bytes([0xF1, self.time_code])

    def __init__(self, time_code):
        self.time_code = time_code & 0xFF
        super().__init__(**kwargs)

    @classmethod
    def from_properties(cls, *props):
        return cls(props[0][0] & 0xFF)

    def get_time_code(self):
        return self.time_code

class SongPositionPointer(MIDIEvent):
    @staticmethod
    def get_rust_id():
        """Get the event code defined in .rs"""
        return 25

    def __bytes__(self):
        least = self.beat & 0x007F
        most = (self.beat >> 8) & 0x007F
        return bytes([0xF2, least, most])

    def __init__(self, beat):
        self.beat = beat
        super().__init__()

    @classmethod
    def from_properties(cls, *props):
        prop = props[0]
        return cls((prop[0] * 256) + prop[1])

    def get_beat(self):
        return self.beat

    def set_beat(self, beat):
        self.beat = beat

class SongSelect(MIDIEvent):
    @staticmethod
    def get_rust_id():
        """Get the event code defined in .rs"""
        return 26
    def __bytes__(self):
        return bytes([0xF3, self.song & 0xFF])

    def __init__(self, song):
        self.song = song
        super().__init__()

    @classmethod
    def from_properties(cls, *props):
        return cls(props[0][0])

    def get_song(self):
        return self.song

    def set_song(self, song):
        self.song = song

class TuneRequest(MIDIEvent):
    @staticmethod
    def get_rust_id():
        """Get the event code defined in .rs"""
        return 27
    def __bytes__(self):
        return bytes([0xF6])

class MIDIClock(MIDIEvent):
    @staticmethod
    def get_rust_id():
        """Get the event code defined in .rs"""
        return 28
    def __bytes__(self):
        return bytes([0xF8])

class MIDIStart(MIDIEvent):
    @staticmethod
    def get_rust_id():
        """Get the event code defined in .rs"""
        return 29
    def __bytes__(self):
        return bytes([0xFA])

class MIDIContinue(MIDIEvent):
    @staticmethod
    def get_rust_id():
        """Get the event code defined in .rs"""
        return 30
    def __bytes__(self):
        return bytes([0xFB])

class MIDIStop(MIDIEvent):
    @staticmethod
    def get_rust_id():
        """Get the event code defined in .rs"""
        return 31
    def __bytes__(self):
        return bytes([0xFC])

class ActiveSense(MIDIEvent):
    @staticmethod
    def get_rust_id():
        """Get the event code defined in .rs"""
        return 32
    def __bytes__(self):
        return bytes([0xFE])

class Reset(MIDIEvent):
    @staticmethod
    def get_rust_id():
        """Get the event code defined in .rs"""
        return 33
    def __bytes__(self):
        return bytes([0xFF])

class TimeCode(MIDIEvent):
    @staticmethod
    def get_rust_id():
        """Get the event code defined in .rs"""
        return 105
    def __init__(self, **kwargs):
        self.rate = kwargs["rate"]
        self.hour = kwargs["hour"]
        self.minute = kwargs["minute"]
        self.second = kwargs["second"]
        self.frame = kwargs["frame"]
        super().__init__(**kwargs)

    @classmethod
    def from_properties(cls, *props):
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
        return MIDIFactory.load(path)

    def save(self, path):
        return MIDIFactory.save(path, self)

    def __init__(self, **kwargs):
        self.events = {}
        self.event_positions = {}
        self.ppqn = kwargs.get('ppqn', 120)
        self.format = kwargs.get('format', 1)

    def get_all_events(self):
        '''Get sorted list of events in midi events'''
        event_list = []
        for event_id, (_, tick) in self.event_positions.items():
            event_list.append((tick, event_id))
        event_list.sort()

        output = []
        for tick, event_id in event_list:
            output.append((tick, self.events[event_id]))

        return output

    def get_track_events(self):
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
        ''' Add Midi Event to the Midi '''

        active_track = kwargs.get('track', 0)
        if "tick" in kwargs:
            active_tick = kwargs["tick"]
        elif "wait" in kwargs:
            active_tick = self.track_get_length(active_track) - 1 + kwargs['wait']
        else:
            active_tick = self.track_get_length(active_track) - 1

        self.events[event.get_uuid()] = event
        self.place_event(event, active_track, active_tick)

    def track_get_length(self, track_number):
        max_tick = 0
        for track, tick in self.event_positions:
            if track != track_number:
                continue
            max_tick = max(tick, max_tick)

        return max_tick


    def place_event(self, event, track, tick):
        self.event_positions[event.get_uuid()] = (track, tick)


class MIDIFactory:
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

    lib_path = __file__[0:__file__.rfind("/") + 1] + "libapres_manylinux2014_" + platform.machine() + ".so"
    lib = ffi.dlopen(lib_path)

    event_constructors = {
        Text.get_rust_id(): Text,
        CopyRightNotice.get_rust_id(): CopyRightNotice,
        TrackName.get_rust_id(): TrackName,
        InstrumentName.get_rust_id(): InstrumentName,
        Lyric.get_rust_id(): Lyric,
        Marker.get_rust_id(): Marker,
        CuePoint.get_rust_id(): CuePoint,
        EndOfTrack.get_rust_id(): EndOfTrack,
        ChannelPrefix.get_rust_id(): ChannelPrefix,
        SetTempo.get_rust_id(): SetTempo,
        SMPTEOffset.get_rust_id(): SMPTEOffset,
        TimeSignature.get_rust_id(): TimeSignature,
        KeySignature.get_rust_id(): KeySignature,
        Sequencer.get_rust_id(): Sequencer,
        SystemExclusive.get_rust_id(): SystemExclusive,

        NoteOn.get_rust_id(): NoteOn,
        NoteOff.get_rust_id(): NoteOff,
        PolyphonicKeyPressure.get_rust_id(): PolyphonicKeyPressure,
        ControlChange.get_rust_id(): ControlChange,
        HoldPedal.get_rust_id(): HoldPedal,
        Portamento.get_rust_id(): Portamento,
        Sustenuto.get_rust_id(): Sustenuto,
        SoftPedal.get_rust_id(): SoftPedal,
        Legato.get_rust_id(): Legato,
        Hold2Pedal.get_rust_id(): Hold2Pedal,
        SoundVariation.get_rust_id(): SoundVariation,
        SoundTimbre.get_rust_id(): SoundTimbre,
        SoundReleaseTime.get_rust_id(): SoundReleaseTime,
        SoundAttack.get_rust_id(): SoundAttack,
        SoundBrightness.get_rust_id(): SoundBrightness,
        SoundControl1.get_rust_id(): SoundControl1,
        SoundControl2.get_rust_id(): SoundControl2,
        SoundControl3.get_rust_id(): SoundControl3,
        SoundControl4.get_rust_id(): SoundControl4,
        SoundControl5.get_rust_id(): SoundControl5,
        EffectsLevel.get_rust_id(): EffectsLevel,
        TremuloLevel.get_rust_id(): TremuloLevel,
        ChorusLevel.get_rust_id(): ChorusLevel,
        CelesteLevel.get_rust_id(): CelesteLevel,
        PhaserLevel.get_rust_id(): PhaserLevel,
        MonophonicOperation.get_rust_id(): MonophonicOperation,
        DataIncrement.get_rust_id(): DataIncrement,
        DataDecrement.get_rust_id(): DataDecrement,
        LocalControl.get_rust_id(): LocalControl,
        AllControllersOff.get_rust_id(): AllControllersOff,
        AllNotesOff.get_rust_id(): AllNotesOff,
        AllSoundOff.get_rust_id(): AllSoundOff,
        OmniOff.get_rust_id(): OmniOff,
        OmniOn.get_rust_id(): OmniOn,
        PolyphonicOperation.get_rust_id(): PolyphonicOperation,
        BankSelect.get_rust_id(): BankSelect,
        BankSelectLSB.get_rust_id(): BankSelectLSB,
        ModulationWheel.get_rust_id(): ModulationWheel,
        ModulationWheelLSB.get_rust_id(): ModulationWheelLSB,
        BreathController.get_rust_id(): BreathController,
        BreathControllerLSB.get_rust_id(): BreathControllerLSB,
        FootPedal.get_rust_id(): FootPedal,
        FootPedalLSB.get_rust_id(): FootPedalLSB,
        PortamentoTime.get_rust_id(): PortamentoTime,
        PortamentoTimeLSB.get_rust_id(): PortamentoTimeLSB,
        DataEntry.get_rust_id(): DataEntry,
        DataEntryLSB.get_rust_id(): DataEntryLSB,
        Volume.get_rust_id(): Volume,
        VolumeLSB.get_rust_id(): VolumeLSB,
        Balance.get_rust_id(): Balance,
        BalanceLSB.get_rust_id(): BalanceLSB,
        Pan.get_rust_id(): Pan,
        PanLSB.get_rust_id(): PanLSB,
        Expression.get_rust_id(): Expression,
        ExpressionLSB.get_rust_id(): ExpressionLSB,
        NonRegisteredParameterNumber.get_rust_id(): NonRegisteredParameterNumber,
        NonRegisteredParameterNumberLSB.get_rust_id(): NonRegisteredParameterNumberLSB,
        RegisteredParameterNumber.get_rust_id(): RegisteredParameterNumber,
        RegisteredParameterNumberLSB.get_rust_id(): RegisteredParameterNumberLSB,

        EffectControl1.get_rust_id(): EffectControl1,
        EffectControl1LSB.get_rust_id(): EffectControl1LSB,
        EffectControl2.get_rust_id(): EffectControl2,
        EffectControl2LSB.get_rust_id(): EffectControl2LSB,
        GeneralPurpose1.get_rust_id(): GeneralPurpose1,
        GeneralPurpose1LSB.get_rust_id(): GeneralPurpose1LSB,
        GeneralPurpose2.get_rust_id(): GeneralPurpose2,
        GeneralPurpose2LSB.get_rust_id(): GeneralPurpose2LSB,
        GeneralPurpose3.get_rust_id(): GeneralPurpose3,
        GeneralPurpose3LSB.get_rust_id(): GeneralPurpose3LSB,
        GeneralPurpose4.get_rust_id(): GeneralPurpose4,
        GeneralPurpose4LSB.get_rust_id(): GeneralPurpose4LSB,
        GeneralPurpose5.get_rust_id(): GeneralPurpose5,
        GeneralPurpose6.get_rust_id(): GeneralPurpose6,
        GeneralPurpose7.get_rust_id(): GeneralPurpose7,
        GeneralPurpose8.get_rust_id(): GeneralPurpose8,

        ProgramChange.get_rust_id(): ProgramChange,
        ChannelPressure.get_rust_id(): ChannelPressure,
        PitchWheelChange.get_rust_id(): PitchWheelChange,
        SequenceNumber.get_rust_id(): SequenceNumber
    }

    @classmethod
    def save(cls, midi, path):
        '''Save the midi to a file'''
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
        count = cls.lib.get_event_property_count(pointer, event_uuid)
        props = []
        for i in range(count):
            props.append(cls.event_get_property(pointer, event_uuid, i))

        return props

    @classmethod
    def event_get_property(cls, pointer, event_uuid, event_property):
        length = cls.lib.get_event_property_length(pointer, event_uuid, event_property)
        bufferlist = bytearray(length)
        array_pointer = cls.lib.get_event_property(pointer, event_uuid, event_property)
        cls.ffi.memmove(bufferlist, array_pointer, length)
        return bufferlist

    @classmethod
    def load(cls, path):
        '''Load a MIDI from a path'''
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
    '''Error Thrown when the midi device pipe is closed or disconnected'''

class MIDIController:
    '''Read Input from Midi Device'''
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
        '''Check if pipe is open and ready to be read'''
        return bool(self.pipe)

    def connect(self, path):
        '''Attempt to open a pipe to the path specified'''
        if not self.pipe:
            self.pipe = open(path, 'rb')
            self.midipath = path

    def disconnect(self):
        '''Close the pipe.'''
        try:
            if self.pipe is not None:
                self.pipe.close()
        except Exception as e:
            raise e

        self.pipe = None
        self.midipath = ''

    def close(self):
        '''Tear down this midi controller'''
        self.disconnect()

    def get_next_byte(self):
        '''Attempt to read next byte from pipe'''
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
            except:
                time.sleep(.01)

    def get_next_event(self):
        '''Read Midi Input Device until relevant event is found'''
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
            if controller == HoldPedal.CONTROL_BYTE:
                constructor = HoldPedal
            elif controller == Portamento.CONTROL_BYTE:
                constructor = Portamento
            elif controller == Sustenuto.CONTROL_BYTE:
                constructor = Sustenuto
            elif controller == SoftPedal.CONTROL_BYTE:
                constructor = SoftPedal
            elif controller == Legato.CONTROL_BYTE:
                constructor = Legato
            elif controller == Hold2Pedal.CONTROL_BYTE:
                constructor = Hold2Pedal
            elif controller == SoundVariation.CONTROL_BYTE:
                constructor = SoundVariation
            elif controller == SoundTimbre.CONTROL_BYTE:
                constructor = SoundTimbre
            elif controller == SoundReleaseTime.CONTROL_BYTE:
                constructor = SoundReleaseTime
            elif controller == SoundAttack.CONTROL_BYTE:
                constructor = SoundAttack
            elif controller == SoundBrightness.CONTROL_BYTE:
                constructor = SoundBrightness
            elif controller == SoundControl1.CONTROL_BYTE:
                constructor = SoundControl1
            elif controller == SoundControl2.CONTROL_BYTE:
                constructor = SoundControl2
            elif controller == SoundControl3.CONTROL_BYTE:
                constructor = SoundControl3
            elif controller == SoundControl4.CONTROL_BYTE:
                constructor = SoundControl4
            elif controller == SoundControl5.CONTROL_BYTE:
                constructor = SoundControl5
            elif controller == EffectsLevel.CONTROL_BYTE:
                constructor = EffectsLevel
            elif controller == TremuloLevel.CONTROL_BYTE:
                constructor = TremuloLevel
            elif controller == ChorusLevel.CONTROL_BYTE:
                constructor = ChorusLevel
            elif controller == CelesteLevel.CONTROL_BYTE:
                constructor = CelesteLevel
            elif controller == PhaserLevel.CONTROL_BYTE:
                constructor = PhaserLevel
            elif controller == LocalControl.CONTROL_BYTE:
                constructor = LocalControl
            elif controller == MonophonicOperation.CONTROL_BYTE:
                constructor = MonophonicOperation
            elif controller == BankSelect.CONTROL_BYTE:
                constructor = BankSelect
            elif controller == BankSelectLSB.CONTROL_BYTE:
                constructor = BankSelectLSB
            elif controller == ModulationWheel.CONTROL_BYTE:
                constructor = ModulationWheel
            elif controller == ModulationWheelLSB.CONTROL_BYTE:
                constructor = ModulationWheelLSB
            elif controller == BreathController.CONTROL_BYTE:
                constructor = BreathController
            elif controller == BreathControllerLSB.CONTROL_BYTE:
                constructor = BreathControllerLSB
            elif controller == FootPedal.CONTROL_BYTE:
                constructor = FootPedal
            elif controller == FootPedalLSB.CONTROL_BYTE:
                constructor = FootPedalLSB
            elif controller == PortamentoTime.CONTROL_BYTE:
                constructor = PortamentoTime
            elif controller == PortamentoTimeLSB.CONTROL_BYTE:
                constructor = PortamentoTimeLSB
            elif controller == DataEntry.CONTROL_BYTE:
                constructor = DataEntry
            elif controller == DataEntryLSB.CONTROL_BYTE:
                constructor = DataEntryLSB
            elif controller == Volume.CONTROL_BYTE:
                constructor = Volume
            elif controller == VolumeLSB.CONTROL_BYTE:
                constructor = VolumeLSB
            elif controller == Balance.CONTROL_BYTE:
                constructor = Balance
            elif controller == BalanceLSB.CONTROL_BYTE:
                constructor = BalanceLSB
            elif controller == Pan.CONTROL_BYTE:
                constructor = Pan
            elif controller == PanLSB.CONTROL_BYTE:
                constructor = PanLSB
            elif controller == Expression.CONTROL_BYTE:
                constructor = Expression
            elif controller == ExpressionLSB.CONTROL_BYTE:
                constructor = ExpressionLSB
            elif controller == NonRegisteredParameterNumber.CONTROL_BYTE:
                constructor = NonRegisteredParameterNumber
            elif controller == NonRegisteredParameterNumberLSB.CONTROL_BYTE:
                constructor = NonRegisteredParameterNumberLSB
            elif controller == RegisteredParameterNumber.CONTROL_BYTE:
                constructor = RegisteredParameterNumber
            elif controller == RegisteredParameterNumberLSB.CONTROL_BYTE:
                constructor = RegisteredParameterNumberLSB
            elif controller == EffectControl1.CONTROL_BYTE:
                constructor = EffectControl1
            elif controller == EffectControl1LSB.CONTROL_BYTE:
                constructor = EffectControl1LSB
            elif controller == EffectControl2.CONTROL_BYTE:
                constructor = EffectControl2
            elif controller == EffectControl2LSB.CONTROL_BYTE:
                constructor = EffectControl2LSB
            elif controller == GeneralPurpose1.CONTROL_BYTE:
                constructor = GeneralPurpose1
            elif controller == GeneralPurpose1LSB.CONTROL_BYTE:
                constructor = GeneralPurpose1LSB
            elif controller == GeneralPurpose2.CONTROL_BYTE:
                constructor = GeneralPurpose2
            elif controller == GeneralPurpose2LSB.CONTROL_BYTE:
                constructor = GeneralPurpose2LSB
            elif controller == GeneralPurpose3.CONTROL_BYTE:
                constructor = GeneralPurpose3
            elif controller == GeneralPurpose3LSB.CONTROL_BYTE:
                constructor = GeneralPurpose3LSB
            elif controller == GeneralPurpose4.CONTROL_BYTE:
                constructor = GeneralPurpose4
            elif controller == GeneralPurpose4LSB.CONTROL_BYTE:
                constructor = GeneralPurpose4LSB
            elif controller == GeneralPurpose5.CONTROL_BYTE:
                constructor = GeneralPurpose5
            elif controller == GeneralPurpose6.CONTROL_BYTE:
                constructor = GeneralPurpose6
            elif controller == GeneralPurpose7.CONTROL_BYTE:
                constructor = GeneralPurpose7
            elif controller == GeneralPurpose8.CONTROL_BYTE:
                constructor = GeneralPurpose8
            # Invariable ControlChanges
            elif controller == DataIncrement.CONTROL_BYTE:
                constructor = DataIncrement
            elif controller == DataDecrement.CONTROL_BYTE:
                constructor = DataDecrement
            elif controller == AllControllersOff.CONTROL_BYTE:
                constructor = AllControllersOff
            elif controller == AllNotesOff.CONTROL_BYTE:
                constructor = AllNotesOff
            elif controller == AllSoundOff.CONTROL_BYTE:
                constructor = AllSoundOff
            elif controller == OmniOff.CONTROL_BYTE:
                constructor = OmniOff
            elif controller == OmniOn.CONTROL_BYTE:
                constructor = OmniOn
            elif controller == PolyphonicOperation.CONTROL_BYTE:
                constructor = PolyphonicOperation
            else:
                constructor = ControlChange

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
