'''Mutable Midi Library'''
import sys
from cffi import FFI
def logg(*msg):
    with open("logg", "a") as fp:
        for m in msg:
            fp.write(str(m) + "\n")

class NoTickException(Exception):
    pass

class EventNotFound(Exception):
    pass

class AlreadyInMIDI(Exception):
    pass

class NoMIDI(Exception):
    pass

class MIDIEvent:
    def __init__(self, **kwargs):
        self.uuid = None

    def pullsync(self):
        '''update the python object with data from the rust layer'''
        pass

    def set_uuid(self, uuid):
        self.uuid = uuid

    def set_property(self, event_number, event_value):
        if self._midi:
            self._midi.set_property(self.uuid, event_number, event_value)

    def get_property(self, event_number):
        if not self._midi:
            raise NoMIDI()

        prop = self._midi._event_get_property(self.uuid, event_number)
        return prop

    def get_position(self):
        if not self._midi:
            raise NoMIDI()

        pos = self._midi._event_get_position(self.uuid)
        self._midi.event_positions[self.uuid] = pos
        return pos

    def move(self, **kwargs):
        active_track = 0
        if "track" in kwargs.keys():
            active_track = track

        if "tick" in kwargs.keys():
            active_tick = kwargs["tick"]
        elif "wait" in kwargs.keys():
            active_tick = self._track_get_length(active_track) - 1 + kwargs['wait']
        else:
            active_tick = self._track_get_length(active_track) - 1

        self._midi._event_move(self.uuid, active_track, active_tick)


class SequenceNumberEvent(MIDIEvent):
    _rust_id = 22
    sequence = 0
    def __bytes__(self):
        output = [
            0xFF,
            self.eid,
            0x02,
            self.sequence // 256,
            self.sequence % 256
        ]
        return bytes(output)

    def __init__(self, **kwargs):
        if "uuid" not in kwargs.keys():
            self.sequence = kwargs['sequence']
        super().__init__(**kwargs)

    def pullsync(self):
        self.sequence = 0
        for b in self.get_property(0):
            self.sequence *= 256
            self.sequence += b

    def get_sequence(self):
        return self.sequence

    def set_sequence(self, sequence):
        self.sequence = sequence
        self.set_property(0, bytes([sequence]))


class TextEvent(MIDIEvent):
    _rust_id = 1
    text = ''
    def __bytes__(self):
        output = [0xFF, 0x01]
        text_bytes = self.text.encode("utf8")
        output.extend(to_variable_length(len(text_bytes)))
        return bytes(output) + text_bytes

    def __init__(self, **kwargs):
        if "uuid" not in kwargs.keys():
            self.text = kwargs['text']
        super().__init__(**kwargs)

    def pullsync(self):
        text = self.get_property(0)
        bytelist = bytes(text)
        self.text = bytelist.decode("utf8")

    def get_text(self):
        return self.text

    def set_text(self, text):
        self.text = text
        self.set_property(0, self.text)


class CopyRightNoticeEvent(MIDIEvent):
    _rust_id = 2
    text = ""
    def __bytes__(self):
        output = [0xFF, 0x02]
        text_bytes = self.text.encode("utf8")
        output.extend(to_variable_length(len(text_bytes)))
        return bytes(output) + text_bytes

    def __init__(self, **kwargs):
        if "uuid" not in kwargs.keys():
            self.text = kwargs["text"]
        super().__init__(**kwargs)

    def pullsync(self):
        text = self.get_property(0)
        bytelist = bytes(text)
        self.text = bytelist.decode("utf8")

    def get_text(self):
        return self.text

    def set_text(self, text):
        self.text = text
        self.set_property(0, self.text)

class TrackNameEvent(MIDIEvent):
    _rust_id = 3
    name = ""
    def __bytes__(self):
        output = [0xFF, 0x03]
        text_bytes = self.name.encode("utf8")
        output.extend(to_variable_length(len(text_bytes)))
        return bytes(output) + text_bytes

    def __init__(self, **kwargs):
        if "uuid" not in kwargs.keys():
            self.name = kwargs["name"]
        super().__init__(**kwargs)

    def pullsync(self):
        name = self.get_property(0)
        bytelist = bytes(name)
        self.name = bytelist.decode("utf8")

    def get_name(self):
        return self.name

    def set_name(self, name):
        self.name = name
        self.set_property(0, self.name)

class InstrumentNameEvent(MIDIEvent):
    _rust_id = 4
    name = ""
    def __bytes__(self):
        output = [0xFF, 0x04]
        text_bytes = self.name.encode("utf8")
        output.extend(to_variable_length(len(text_bytes)))
        return bytes(output) + text_bytes

    def __init__(self, **kwargs):
        if "uuid" not in kwargs.keys():
            self.name = kwargs["name"]
        super().__init__(**kwargs)

    def pullsync(self):
        name = self.get_property(0)
        bytelist = bytes(name)
        self.name = bytelist.decode("utf8")

    def get_name(self):
        return self.name

    def set_name(self, name):
        self.name = name
        self.set_property(0, self.name)

class LyricEvent(MIDIEvent):
    _rust_id = 5
    lyric = ""
    def __bytes__(self):
        output = [0xFF, 0x05]
        text_bytes = self.lyric.encode("utf8")
        output.extend(to_variable_length(len(text_bytes)))
        return bytes(output) + text_bytes

    def __init__(self, **kwargs):
        if "uuid" not in kwargs.keys():
            self.lyric = kwargs["lyric"]
        super().__init__(**kwargs)

    def pullsync(self):
        lyric = self.get_property(0)
        bytelist = bytes(lyric)
        self.lyric = bytelist.decode("utf8")

    def get_lyric(self):
        return self.lyric

    def set_lyric(self, lyric):
        self.lyric = lyric
        self.set_property(0, self.lyric)

class MarkerEvent(MIDIEvent):
    _rust_id = 6
    text = ""
    def __bytes__(self):
        output = [0xFF, 0x06]
        text_bytes = self.text.encode("utf8")
        output.extend(to_variable_length(len(text_bytes)))
        return bytes(output) + text_bytes

    def __init__(self, **kwargs):
        if "uuid" not in kwargs.keys():
            self.text = kwargs["text"]
        super().__init__(**kwargs)

    def pullsync(self):
        text = self.get_property(0)
        bytelist = bytes(text)
        self.text = bytelist.decode("utf8")

    def get_text(self):
        return self.text

    def set_text(self, text):
        self.text = text
        self.set_property(0, self.text)

class CuePointEvent(MIDIEvent):
    _rust_id = 7
    text = ""
    def __bytes__(self):
        output = [0xFF, 0x07]
        text_bytes = self.text.encode("utf8")
        output.extend(to_variable_length(len(text_bytes)))
        return bytes(output) + text_bytes

    def __init__(self, **kwargs):
        if "uuid" not in kwargs.keys():
            self.text = kwargs["text"]
        super().__init__(**kwargs)

    def pullsync(self):
        text = self.get_property(0)
        bytelist = bytes(text)
        self.text = bytelist.decode("utf8")

    def get_text(self):
        return self.text

    def set_text(self, text):
        self.text = text
        self.set_property(0, self.text)

class EndOfTrackEvent(MIDIEvent):
    _rust_id = 8
    def __bytes__(self):
        return bytes([0xFF, 0x2F, 0x00])

    def pullsync(self):
        pass

    def __init__(self, **kwargs):
        super().__init__(**kwargs)

class ChannelPrefixEvent(MIDIEvent):
    _rust_id = 9
    channel = 0
    def __bytes__(self):
        return bytes([0xFF, 0x20, 0x01, self.channel])

    def __init__(self, **kwargs):
        if "uuid" not in kwargs.keys():
            self.channel = kwargs["channel"]
        super().__init__(**kwargs)

    def pullsync(self):
        channel = self.get_property(0)
        self.channel = channel[0]

    def get_channel(self):
        return self.channel

    def set_channel(self, channel):
        self.channel = channel
        self.set_property(0, self.channel)

class SetTempoEvent(MIDIEvent):
    _rust_id = 10
    us_per_quarter_note = 500000
    def __bytes__(self):
        return bytes([
            0xFF, 0x51, 0x03,
            (self.us_per_quarter_note // (256 ** 2)) % 256,
            (self.us_per_quarter_note // 256) % 256,
            self.us_per_quarter_note % 256
        ])

    def __init__(self, **kwargs):
        if "uuid" not in kwargs.keys():
            if "uspqn" in kwargs.keys():
                self.us_per_quarter_note = kwargs['uspqn']
            elif "bpm" in kwargs.keys():
                bpm = kwargs["bpm"]
                if not bpm:
                    self.us_per_quarter_note = 0
                else:
                    self.us_per_quarter_note = 60000000 // bpm

        super().__init__(**kwargs)

    def pullsync(self):
        self.us_per_quarter_note = 0
        for n in self.get_property(0):
            self.us_per_quarter_note *= 256
            self.us_per_quarter_note += n


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
        self.set_property(0, self.us_per_quarter_note)

class SMPTEOffsetEvent(MIDIEvent):
    _rust_id = 11
    def __bytes__(self):
        return bytes([
            0xFF, 0x54, 0x05,
            self.hour, self.minute, self.second,
            self.ff, self.fr
        ])

    def __init__(self, **kwargs):
        if "uuid" not in kwargs.keys():
            self.hour = kwargs["hour"]
            self.minute = kwargs["minute"]
            self.second = kwargs["second"]
            self.ff = kwargs["ff"]
            self.fr = kwargs["fr"]
        super().__init__(**kwargs)

    def pullsync(self):
        self.hour = self.get_property(0)[0]
        self.minute = self.get_property(1)[0]
        self.second = self.get_property(2)[0]
        self.ff = self.get_property(3)[0]
        self.fr = self.get_property(4)[0]

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
        self.set_property(0, hour)

    def set_minute(self, minute):
        self.minute = minute
        self.set_property(1, minute)

    def set_second(self, second):
        self.second = second
        self.set_property(2, second)

    def set_ff(self, ff):
        self.ff = ff
        self.set_property(3, ff)

    def set_fr(self, fr):
        self.fr = fr
        self.set_property(4, fr)


class TimeSignatureEvent(MIDIEvent):
    _rust_id = 12
    def __bytes__(self):
        return bytes([
            0xFF, 0x58, 0x04,
            self.numerator, self.denominator,
            self.clocks_per_metronome,
            self.thirtysecondths_per_quarter
        ])

    def __init__(self, **kwargs):
        if "uuid" not in kwargs.keys():
            self.numerator = kwargs["numerator"]
            self.denominator = kwargs["denominator"]
            self.clocks_per_metronome = kwargs["cpm"]
            self.thirtysecondths_per_quarter = kwargs["tspqn"]
        super().__init__(**kwargs)

    def pullsync(self):
        self.numerator = self.get_property(0)[0]
        self.denominator = self.get_property(1)[0]
        self.clocks_per_metronome = self.get_property(3)[0]
        self.thirtysecondths_per_quarter = self.get_property(4)[0]

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
        self.set_property(0, numerator)

    def set_denominator(self, denominator):
        self.denominator = denominator
        self.set_property(1, denominator)

    def set_clocks_per_metronome(self, cpm):
        self.clocks_per_metronome = cpm
        self.set_property(3, cpm)

    def set_thirtysecondths_per_quarter_note(self, tspqn):
        self.thirtysecondths_per_quarter_note = tspqn
        self.set_property(4, tspqn)


class KeySignatureEvent(MIDIEvent):
    _rust_id = 13
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
        if "uuid" not in kwargs.keys():
            self.key = kwargs["key"]
        super().__init__(**kwargs)

    def pullsync(self):
        self.key = bytes(self.get_property(0)).decode("utf8")

    def get_key(self):
        return self.key

    def set_key(self, key):
        self.key = key
        self.set_property(0, key)


class SequencerEvent(MIDIEvent):
    _rust_id = 14
    data = b''
    def __bytes__(self):
        output = [0xFF, 0x7F]
        data_length = len(self.data)
        output.extend(to_variable_length(data_length))
        return bytes(output) + self.data

    def __init__(self, **kwargs):
        if "uuid" not in kwargs.keys():
            self.data = kwargs["data"]
        super().__init__(**kwargs)

    def pullsync(self):
        self.data = bytes(self.get_property(0))

    def get_data(self):
        return self.data

    def set_data(self, data):
        self.data = data
        self.set_property(0, self.data)


class NoteOnEvent(MIDIEvent):
    _rust_id = 15
    def __bytes__(self):
        return bytes([
            0x90 | self.channel,
            self.note,
            self.velocity
        ])
    def __init__(self, **kwargs):
        if not "uuid" in kwargs.keys():
            self.channel = kwargs["channel"]
            self.note = kwargs["note"]
            self.velocity = kwargs["velocity"]
        super().__init__(**kwargs)

    def pullsync(self):
        prop = self.get_property(0)
        self.channel = prop[0]
        self.note = self.get_property(1)[0]
        self.velocity = self.get_property(2)[0]

    def get_channel(self):
        return self.channel

    def get_note(self):
        return self.note

    def get_velocity(self):
        return self.velocity

    def set_channel(self, channel):
        self.channel = channel
        self.set_property(0, channel)

    def set_note(self, note):
        self.note = note
        self.set_property(1, note)

    def set_velocity(self, velocity):
        self.velocity = velocity
        self.set_property(2, velocity)

class NoteOffEvent(MIDIEvent):
    _rust_id = 16
    def __bytes__(self):
        return bytes([
            0x80 | self.channel,
            self.note,
            self.velocity
        ])

    def __init__(self, **kwargs):
        if "uuid" not in kwargs.keys():
            self.channel = kwargs["channel"]
            self.note = kwargs["note"]
            self.velocity = kwargs["velocity"]
        super().__init__(**kwargs)

    def pullsync(self):
        self.channel = self.get_property(0)[0]
        self.note = self.get_property(1)[0]
        self.velocity = self.get_property(2)[0]

    def get_channel(self):
        return self.channel

    def get_note(self):
        return self.note

    def get_velocity(self):
        return self.velocity

    def set_channel(self, channel):
        self.channel = channel
        self.set_property(0, channel)

    def set_note(self, note):
        self.note = note
        self.set_property(1, note)

    def set_velocity(self, velocity):
        self.velocity = velocity
        self.set_property(2, velocity)

class PolyphonicKeyPressureEvent(MIDIEvent):
    _rust_id = 17
    def __bytes__(self):
        return bytes([
            0xA0 | self.channel,
            self.note,
            self.pressure
        ])
    def __init__(self, **kwargs):
        if "uuid" not in kwargs.keys():
            self.channel = kwargs["channel"]
            self.note = kwargs["note"]
            self.pressure = kwargs["pressure"]
        super().__init__(**kwargs)

    def pullsync(self):
        self.channel = self.get_property(0)[0]
        self.note = self.get_property(1)[0]
        self.pressure = self.get_property(2)[0]

    def get_channel(self):
        return self.channel

    def get_note(self):
        return self.note

    def get_pressure(self):
        return self.pressure

    def set_channel(self, channel):
        self.channel = channel
        self.set_property(0, channel)

    def set_note(self, note):
        self.note = note
        self.set_property(1, note)

    def set_pressure(self, pressure):
        self.pressure = pressure
        self.set_property(2, pressure)

class ControlChangeEvent(MIDIEvent):
    _rust_id = 18
    def __bytes__(self):
        return bytes([
            0xB0 | self.channel,
            self.controller,
            self.value
        ])

    def __init__(self, **kwargs):
        if "uuid" not in kwargs.keys():
            self.channel = kwargs["channel"]
            self.controller = kwargs["controller"]
            self.value = kwargs["value"]
        super().__init__(**kwargs)

    def pullsync(self):
        self.value = self.get_property(2)[0]
        self.channel = self.get_property(0)[0]
        self.controller = self.get_property(1)[0]

    def get_channel(self):
        return self.channel

    def get_controller(self):
        return self.controller

    def get_value(self):
        return self.value

    def set_channel(self, channel):
        self.channel = channel
        self.set_property(0, channel)

    def set_controller(self, controller):
        self.controller = controller
        self.set_property(1, controller)

    def set_value(self, value):
        self.value = value
        self.set_property(2, value)

class ProgramChangeEvent(MIDIEvent):
    _rust_id = 19
    def __bytes__(self):
        return bytes([
            0xC0 | self.channel,
            self.program
        ])

    def __init__(self, **kwargs):
        if "uuid" not in kwargs.keys():
            self.channel = kwargs["channel"]
            self.program = kwargs["program"]
        super().__init__(**kwargs)

    def pullsync(self):
        self.channel = self.get_property(0)[0]
        self.program = self.get_property(1)[0]

    def get_channel(self):
        return self.channel

    def get_program(self):
        return self.program

    def set_channel(self, channel):
        self.channel = channel
        self.set_property(0, channel)

    def set_program(self, program):
        self.program = program
        self.set_property(1, program)


class ChannelPressureEvent(MIDIEvent):
    _rust_id = 20
    def __bytes__(self):
        return bytes([
            0xD0 | self.channel,
            self.pressure
        ])

    def __init__(self, **kwargs):
        if "uuid" not in kwargs.keys():
            self.channel = kwargs["channel"]
            self.pressure = kwargs["pressure"]
        super().__init__(**kwargs)

    def pullsync(self):
        self.channel = self.get_property(0)[0]
        self.pressure = self.get_property(1)[0]

    def get_channel(self):
        return self.channel

    def get_pressure(self):
        return self.pressure

    def set_channel(self, channel):
        self.channel = channel
        self.set_property(0, channel)

    def set_pressure(self, pressure):
        self.pressure = pressure
        self.set_property(1, pressure)

# TODO: Store as signed integer, set 0x2000 to == 0
class PitchWheelChangeEvent(MIDIEvent):
    '''
        NOTE: value is stored as float from [-1, 1]
    '''

    _rust_id = 21
    def __bytes__(self):
        unsigned_value = self.get_unsigned_value()
        least = unsigned_value & 0x007F
        most = (unsigned_value >> 7) & 0x007F
        return bytes([(0xE0 | self.channel), least, most])

    def __init__(self, **kwargs):
        if "uuid" not in kwargs.keys():
            self.channel = kwargs["channel"]
            self.value = kwargs["value"]
        super().__init__(**kwargs)

    def pullsync(self):
        self.channel = self.get_property(0)[0]

        prop = self.get_property(1)
        unsigned_value = (prop[0] * 256) + prop[1]
        self.value = ((unsigned_value * 2) / 0x3FFF) - 1

    def get_channel(self):
        return self.channel

    def get_value(self):
        return self.value

    def set_channel(self, channel):
        self.channel = channel
        self.set_property(0, channel)

    def set_value(self, value):
        self.value = value
        self.set_property(1, self.get_unsigned_value())

    def get_unsigned_value(self):
        ''' get value as integer in range (0, 0x3FFF) '''
        return int((self.value + 1) * (2 / 0x3FFF))

class SystemExclusiveEvent(MIDIEvent):
    _rust_id = 23
    data = b''
    def __bytes__(self):
        output = [0xF0]
        for b in self.data:
            output.append(b)
        output.append(0xF7)
        return bytes(output)

    def __init__(self, **kwargs):
        if "uuid" not in kwargs.keys():
            self.data = kwargs["data"]

        super().__init__(**kwargs)

    def pullsync(self):
        self.data = bytes(self.get_property(0))

    def get_data(self):
        return self.data

    def set_data(self, new_data):
        self.data = new_data
        self.set_property(0, new_data)

class MTCQuarterFrameEvent(MIDIEvent):
    _rust_id = 24
    time_code = 0
    def __bytes__(self):
        return bytes([0xF1, time_code])

    def __init__(self, **kwargs):
        if "uuid" not in kwargs.keys():
            self.time_code = kwargs["time_code"] & 0xFF
        super().__init__(kwargs)

    def pullsync(self):
        self.time_code = self.get_property(0)[0] & 0xFF

    def get_time_code(self):
        return self.time_code

class SongPositionPointerEvent(MIDIEvent):
    _rust_id = 25
    def __bytes__(self):
        least = self.beat & 0x7F
        most = (self.beat >> 7) & 0x7F
        return bytes([0xF2, least, most])

    def __init__(self, **kwargs):
        if "uuid" not in kwargs.keys():
            self.beat = kwargs["beat"]
        super().__init__(**kwargs)

    def pullsync(self):
        prop = self.get_property(0)
        self.beat = (prop[0] * 256) + prop[1]

    def get_beat(self):
        return self.beat

    def set_beat(self, beat):
        self.beat = beat
        self.set_property(1, beat)

class SongSelectEvent(MIDIEvent):
    _rust_id = 26
    def __bytes__(self):
        return bytes([0xF3, self.song & 0xFF])

    def __init__(self, **kwargs):
        if "uuid" not in kwrags.keys():
            self.song = kwargs["song"]
        super().__init__(**kwargs)

    def pullsync(self):
        self.song = self.get_property(0)[0]

    def get_song(self):
        return self.song

    def set_song(self, song):
        self.song = song
        self.set_property(0, song)

class TuneRequestEvent(MIDIEvent):
    _rust_id = 27
    def __bytes__(self):
        return bytes([0xF6])

class MIDIClockEvent(MIDIEvent):
    _rust_id = 28
    def __bytes__(self):
        return bytes([0xF8])

class MIDIStartEvent(MIDIEvent):
    _rust_id = 29
    def __bytes__(self):
        return bytes([0xFA])

class MIDIContinueEvent(MIDIEvent):
    _rust_id = 30
    def __bytes__(self):
        return bytes([0xFB])

class MIDIStopEvent(MIDIEvent):
    _rust_id = 31
    def __bytes__(self):
        return bytes([0xFC])

class ActiveSenseEvent(MIDIEvent):
    _rust_id = 32
    def __bytes__(self):
        return bytes([0xFE])

class ResetEvent(MIDIEvent):
    _rust_id = 33
    def __bytes__(self):
        return bytes([0xFF])

class MIDI:
    """Usable object. Converted from midi files.
        Events are the same midi files from simplicities sake.
    """
    SO_PATH = "/usr/lib/libmidas.so"

    event_constructors = {
        TextEvent._rust_id: TextEvent,
        CopyRightNoticeEvent._rust_id: CopyRightNoticeEvent,
        TrackNameEvent._rust_id: TrackNameEvent,
        InstrumentNameEvent._rust_id: InstrumentNameEvent,
        LyricEvent._rust_id: LyricEvent,
        MarkerEvent._rust_id: MarkerEvent,
        CuePointEvent._rust_id: CuePointEvent,
        EndOfTrackEvent._rust_id: EndOfTrackEvent,
        ChannelPrefixEvent._rust_id: ChannelPrefixEvent,
        SetTempoEvent._rust_id: SetTempoEvent,
        SMPTEOffsetEvent._rust_id: SMPTEOffsetEvent,
        TimeSignatureEvent._rust_id: TimeSignatureEvent,
        KeySignatureEvent._rust_id: KeySignatureEvent,
        SequencerEvent._rust_id: SequencerEvent,

        NoteOnEvent._rust_id: NoteOnEvent,
        NoteOffEvent._rust_id: NoteOffEvent,
        PolyphonicKeyPressureEvent._rust_id: PolyphonicKeyPressureEvent,
        ControlChangeEvent._rust_id: ControlChangeEvent,
        ProgramChangeEvent._rust_id: ProgramChangeEvent,
        ChannelPressureEvent._rust_id: ChannelPressureEvent,
        PitchWheelChangeEvent._rust_id: PitchWheelChangeEvent,
        SequenceNumberEvent._rust_id: SequenceNumberEvent
    }

    def _get_track_count(self):
        return self.lib.get_track_count(self.pointer)

    def __init__(self, path=""):
        self.ffi = FFI()
        self.ffi.cdef("""
            typedef void* MIDI;

            MIDI interpret(const char*);
            MIDI new();
            void save(MIDI, const char*);
            uint32_t get_track_length(MIDI, uint8_t);
            uint32_t count_tracks(MIDI);
            uint64_t count_events(MIDI);

            uint8_t* get_event_property(MIDI, uint64_t, uint8_t);
            uint8_t get_event_property_length(MIDI, uint64_t, uint8_t);
            void set_event_property(MIDI, uint64_t, uint32_t, const char*);
            uint8_t get_event_type(MIDI, uint64_t);
            uint64_t create_event(MIDI, uint8_t, uint64_t, const uint8_t*, uint8_t);
            void set_event_position(MIDI, uint64_t, uint8_t, uint64_t);
            uint64_t get_event_tick(MIDI, uint64_t);
            uint8_t get_event_track(MIDI, uint64_t);

            void set_ppqn(MIDI, uint16_t);
            uint16_t get_ppqn(MIDI);
        """)
        self.lib = self.ffi.dlopen(self.SO_PATH)
        self.events = {}
        self.event_positions = {}
        self.ppqn = 120

        if path:
            self.path = path
            fmt_path = bytes(self.path, 'utf-8')
            self.pointer = self.lib.interpret(fmt_path)
            self.ppqn = self.lib.get_ppqn(self.pointer)

            # 0 is reserved, but eids are generated in order.
            # So we don't need to query every individual active id at this point
    
            for eid in range(1, self.lib.count_events(self.pointer)):
                event = self._get_event(eid)
                self.events[eid] = event

        else:
            self.path = ''
            self.pointer = self.lib.new()

    def get_all_events(self):
        event_list = []
        for event_id, (track, tick) in self.event_positions.items():
            event_list.append((tick, event_id))
        event_list.sort()

        output = []
        for tick, event_id in event_list:
            output.append((tick, self.events[event_id]))

        return output

    def add_event(self, event, **kwargs):
        if event.uuid:
            raise AlreadyInMIDI()

        active_track = 0
        if 'track' in kwargs.keys():
            active_track = kwargs['track']

        if "tick" in kwargs.keys():
            active_tick = kwargs["tick"]
        elif "wait" in kwargs.keys():
            active_tick = self._track_get_length(active_track) - 1 + kwargs['wait']
        else:
            active_tick = self._track_get_length(active_track) - 1

        event_uuid = self._pushsync_event(event, active_track, active_tick)

        event.set_uuid(event_uuid)
        event._midi = self

        self.events[event_uuid] = event
        self.event_positions[event_uuid] = (active_track, active_tick)

    def _pushsync_event(self, event, track, tick):
        orig_bytes = bytes(event)
        event_id = self.lib.create_event(self.pointer, track, tick, orig_bytes, len(orig_bytes))
        return event_id

    def _event_get_position(self, eid):
        tick = self.lib.get_event_tick(self.pointer, eid) - 1
        track = self.lib.get_event_track(self.pointer, eid) - 1
        return (track, tick)

    def _track_get_length(self, track_number):
        return self.lib.get_track_length(self.pointer, track_number)

    def _event_get_type(self, event_uuid):
        return self.lib.get_event_type(self.pointer, event_uuid)

    def _event_set_property(self, uuid, index, somevalue):
        class_name = somevalue.__class__.__name__
        if class_name == "bytes":
            pass
        elif class_name == "list":
            somevalue = bytes(somevalue)
        elif class_name == "str":
            somevalue = somevalue.encode("utf8")
        elif class_name == "int":
            working_bytes = []
            i = 0
            while somevalue > 0 or not i:
                working_byte = somevalue % 256
                working_bytes.insert(0, working_byte)
                somevalue //= 256
                i += 1
            somevalue = working_bytes

        self.lib.set_event_property(self.pointer, uuid, index, somevalue)

    def _event_get_property(self, event_uuid, event_property):
        length = self.lib.get_event_property_length(self.pointer, event_uuid, event_property)
        bufferlist = bytearray(length)
        array_pointer = self.lib.get_event_property(self.pointer, event_uuid, event_property)
        self.ffi.memmove(bufferlist, array_pointer, length)
        return bufferlist

    def _get_event(self, event_uuid):
        type_num = self._event_get_type(event_uuid)
        if type_num == 0:
            raise EventNotFound()
        constructor = self.event_constructors[type_num]

        # passing uuid will cause it to sync on init
        event = constructor(uuid=event_uuid)
        event._midi = self
        event.set_uuid(event_uuid)
        event.pullsync()
        event.get_position()


        return event

    def save(self, path):
        fmt_path = bytes(path, 'utf-8')
        self.lib.save(self.pointer, fmt_path)

    def _event_move(self, event_uuid, new_track, new_tick):
        self.lib.move_event(self.pointer, event_uuid, new_track, new_tick)


    ##########################################################

ml = MIDI()
for x in range(6):
    note_on = NoteOnEvent(channel=0, velocity=64, note= 64 + (x * 2))
    note_off = NoteOffEvent(channel=0, velocity=64, note= 64 + (x * 2))
    ml.add_event(note_on, track=0, wait=0)
    ml.add_event(note_off, track=0, wait=40)
ml.save("/home/pent/test2.mid")

#ml = MIDI(sys.argv[1])
#print(ml.tracks[0]._ticks.keys())
