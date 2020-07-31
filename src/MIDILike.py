'''Mutable Midi Library'''
import sys
from cffi import FFI

class NoTickException(Exception):
    pass

class EventNotFound(Exception):
    pass


class MIDILike:
    """Usable object. Converted from midi files.
        Events are the same midi files from simplicities sake.
    """
    SO_PATH = "/home/pent/Projects/105/target/release/libRustMidiLib.so"
    def _get_track_count(self):
        return self.lib.get_track_count(self.pointer)

    def __init__(self, path):
        self.ffi = FFI()
        self.ffi.cdef("""
            typedef void* MIDILike;

            MIDILike interpret(const char*);
            void save(MIDILike, const char*);
            uint32_t get_track_length(MIDILike, uint32_t);
            uint32_t get_active_tick_count(MIDILike, uint32_t);
            uint32_t get_track_count(MIDILike);
            uint32_t get_tick_length(MIDILike, uint32_t, uint32_t);
            uint32_t get_active_tick(MIDILike, uint32_t, uint32_t);
            uint32_t get_nth_event_in_tick(MIDILike, uint32_t, uint32_t, uint32_t);
            uint8_t* get_event_property(MIDILike, uint64_t, uint8_t);
            uint8_t get_event_property_length(MIDILike, uint64_t, uint8_t);
            void set_event_property(MIDILike, uint32_t, const char*);
            uint8_t get_event_type(MIDILike, uint64_t);
        """)

        self.lib = self.ffi.dlopen(self.SO_PATH)
        self.path = path
        fmt_path = bytes(self.path, 'utf-8')
        self.pointer = self.lib.interpret(fmt_path)
        self.events = {}
        self.tracks = []
        for i in range(self._get_track_count()):
            new_track = MIDILikeTrack(i, self)
            self.tracks.append(new_track)
            new_track.sync()

    def get_tracks(self):
        return self.tracks

    def _track_get_length(self, n):
        return self.lib.get_track_length(self.pointer, n)

    def _track_get_tick_count(self, track):
        return self.lib.get_active_tick_count(self.pointer, track)
    def _track_get_tick(self, track, n):
        return self.lib.get_active_tick(self.pointer, track, n)

    def _tick_get_event_count(self, track, tick):
        return self.lib.get_tick_length(self.pointer, track, tick)

    def _tick_get_event_id(self, track, tick, n):
        return self.lib.get_nth_event_in_tick(self.pointer, track, tick, n)

    def _tick_get_events(self, track, tick):
        return self.lib.get_events_in_tick(self.pointer, track, tick)

    def _event_get_type(self, uuid):
        return self.lib.get_event_type(self.pointer, uuid)

    def _event_set_property(self, n, somevalue):
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

        self.lib.set_event_property(self.pointer, n, somevalue)

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
        constructor = {
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
        }[type_num]

        # passing uuid will cause it to sync on init
        event = constructor(self, uuid=event_uuid)

        return event

    def save(self, path):
        fmt_path = bytes(path, 'utf-8')
        self.lib.save(MIDILike, fmt_path)

    ##########################################################

class MIDILikeTrack:
    def __init__(self, track_number, midilike):
        self.track_number = track_number

        self._midilike = midilike
        self._ticks = {}
        self.sync()

    def get_ticks(self):
        return self._ticks

    def sync(self):
        self._ticks = {}
        tick_count = self._track_get_tick_count()
        for i in range(tick_count):
            tick = self._track_get_tick(i)
            self._ticks[tick] = []
            for j in range(self._tick_get_event_count(tick)):
                uuid = self._tick_get_event_id(tick, j)
                event = self._midilike._get_event(uuid)
                self._ticks[tick].append(event)

    def _tick_get_event_id(self, tick, n):
        return self._midilike._tick_get_event_id(self.track_number, tick, n)

    def _track_get_tick(self, n):
        return self._midilike._track_get_tick(self.track_number, n)

    def _tick_get_event_count(self, tick):
        return self._midilike._tick_get_event_count(self.track_number, tick)

    def _track_get_tick_count(self):
        return self._midilike._track_get_tick_count(self.track_number)

    def __len__(self):
        return self._midilike._track_get_length(self.track_number)

    def add_event(self, constructor, **kwargs):
        tick = 0
        if "tick" in kwargs.keys():
            tick = kwargs["tick"]
        elif "wait" in kwargs.keys():
            tick = self._track_get_tick_count() + kwargs['wait']
        else:
            raise NoTickException

        new_event = self._midilike.create_new_event(track, event_as_bytes)


class MIDIEvent:
    def __init__(self, midilike, **kwargs):
        self._midilike = midilike
        if "uuid" in kwargs.keys():
            self.uuid = kwargs["uuid"]
            self.sync()
        else:
            self.uuid = self._midilike.create_new_event(self.__repr__())

        self._midilike.events[self.uuid] = self

    def set_property(self, event_number, event_value):
        self._midilike.set_property(self.uuid, event_number, event_value)

    def get_property(self, event_number):
        prop = self._midilike._event_get_property(self.uuid, event_number)
        return prop


class SequenceNumberEvent(MIDIEvent):
    _rust_id = 22
    sequence = 0
    def __repr__(self):
        output = [
            0xFF,
            self.eid,
            0x02,
            self.sequence // 256,
            self.sequence % 256
        ]
        return bytes(output)

    def __init__(self, midilike, **kwargs):
        if "uuid" not in kwargs.keys():
            self.sequence = kwargs['sequence']
        super().__init__(midilike, **kwargs)

    def sync(self):
        self.get_sequence()

    def get_sequence(self):
        self.sequence = 0
        for b in self.get_property(0):
            self.sequence *= 256
            self.sequence += b

        return self.sequence

    def set_sequence(self, sequence):
        self.sequence = sequence
        self.set_property(0, bytes([sequence]))


class TextEvent(MIDIEvent):
    _rust_id = 1
    text = ''
    def __repr__(self):
        output = [0xFF, 0x01]
        text_bytes = self.text.encode("utf8")
        output.extend(to_variable_length(len(text_bytes)))
        return bytes(output) + text_bytes

    def __init__(self, midilike, **kwargs):
        if "uuid" not in kwargs.keys():
            self.text = kwargs['text']
        super().__init__(midilike, **kwargs)

    def sync(self):
        self.get_text()

    def get_text(self):
        text = self.get_property(0)
        bytelist = bytes(text)
        self.text = bytelist.decode("utf8")
        return self.text

    def set_text(self, text):
        self.text = text
        self.set_property(0, self.text)


class CopyRightNoticeEvent(MIDIEvent):
    _rust_id = 2
    text = ""
    def __repr__(self):
        output = [0xFF, 0x02]
        text_bytes = self.text.encode("utf8")
        output.extend(to_variable_length(len(text_bytes)))
        return bytes(output) + text_bytes

    def __init__(self, midilike, **kwargs):
        if "uuid" not in kwargs.keys():
            self.text = kwargs["text"]
        super().__init__(midilike, **kwargs)

    def sync(self):
        self.get_text()

    def get_text(self):
        text = self.get_property(0)
        bytelist = bytes(text)
        self.text = bytelist.decode("utf8")
        return self.text

    def set_text(self, text):
        self.text = text
        self.set_property(0, self.text)

class TrackNameEvent(MIDIEvent):
    _rust_id = 3
    name = ""
    def __repr__(self):
        output = [0xFF, 0x03]
        text_bytes = self.name.encode("utf8")
        output.extend(to_variable_length(len(text_bytes)))
        return bytes(output) + text_bytes

    def __init__(self, midilike, **kwargs):
        if "uuid" not in kwargs.keys():
            self.name = kwargs["name"]
        super().__init__(midilike, **kwargs)

    def sync(self):
        self.get_name()

    def get_name(self):
        name = self.get_property(0)
        bytelist = bytes(name)
        self.name = bytelist.decode("utf8")
        return self.name

    def set_name(self, name):
        self.name = name
        self.set_property(0, self.name)

class InstrumentNameEvent(MIDIEvent):
    _rust_id = 4
    name = ""
    def __repr__(self):
        output = [0xFF, 0x04]
        text_bytes = self.name.encode("utf8")
        output.extend(to_variable_length(len(text_bytes)))
        return bytes(output) + text_bytes

    def __init__(self, midilike, **kwargs):
        if "uuid" not in kwargs.keys():
            self.name = kwargs["name"]
        super().__init__(midilike, **kwargs)

    def sync(self):
        self.get_name()

    def get_name(self):
        name = self.get_property(0)
        bytelist = bytes(name)
        self.name = bytelist.decode("utf8")
        return self.name

    def set_name(self, name):
        self.name = name
        self.set_property(0, self.name)

class LyricEvent(MIDIEvent):
    _rust_id = 5
    lyric = ""
    def __repr__(self):
        output = [0xFF, 0x05]
        text_bytes = self.lyric.encode("utf8")
        output.extend(to_variable_length(len(text_bytes)))
        return bytes(output) + text_bytes

    def __init__(self, midilike, **kwargs):
        if "uuid" not in kwargs.keys():
            self.lyric = kwargs["lyric"]
        super().__init__(midilike, **kwargs)

    def sync(self):
        self.get_lyric()

    def get_lyric(self):
        lyric = self.get_property(0)
        bytelist = bytes(lyric)
        self.lyric = bytelist.decode("utf8")
        return self.lyric

    def set_lyric(self, lyric):
        self.lyric = lyric
        self.set_property(0, self.lyric)

class MarkerEvent(MIDIEvent):
    _rust_id = 6
    text = ""
    def __repr__(self):
        output = [0xFF, 0x06]
        text_bytes = self.text.encode("utf8")
        output.extend(to_variable_length(len(text_bytes)))
        return bytes(output) + text_bytes

    def __init__(self, midilike, **kwargs):
        if "uuid" not in kwargs.keys():
            self.text = kwargs["text"]
        super().__init__(midilike, **kwargs)

    def sync(self):
        self.get_text()

    def get_text(self):
        text = self.get_property(0)
        bytelist = bytes(text)
        self.text = bytelist.decode("utf8")
        return self.text

    def set_text(self, text):
        self.text = text
        self.set_property(0, self.text)

class CuePointEvent(MIDIEvent):
    _rust_id = 7
    text = ""
    def __repr__(self):
        output = [0xFF, 0x07]
        text_bytes = self.text.encode("utf8")
        output.extend(to_variable_length(len(text_bytes)))
        return bytes(output) + text_bytes

    def __init__(self, midilike, **kwargs):
        if "uuid" not in kwargs.keys():
            self.text = kwargs["text"]
        super().__init__(midilike, **kwargs)

    def sync(self):
        self.get_text()

    def get_text(self):
        text = self.get_property(0)
        bytelist = bytes(text)
        self.text = bytelist.decode("utf8")
        return self.text

    def set_text(self, text):
        self.text = text
        self.set_property(0, self.text)

class EndOfTrackEvent(MIDIEvent):
    _rust_id = 8
    def __repr__(self):
        return bytes([0xFF, 0x2F, 0x00])

    def sync(self):
        pass

    def __init__(self, midilike, **kwargs):
        super().__init__(midilike, **kwargs)

class ChannelPrefixEvent(MIDIEvent):
    _rust_id = 9
    channel = 0
    def __repr__(self):
        return bytes([0xFF, 0x20, 0x01, self.channel])

    def __init__(self, midilike, **kwargs):
        if "uuid" not in kwargs.keys():
            self.channel = kwargs["channel"]
        super().__init__(midilike, **kwargs)

    def sync(self):
        self.get_channel()

    def get_channel(self):
        channel = self.get_property(0)
        self.channel = channel[0]
        return self.channel

    def set_channel(self, channel):
        self.channel = channel
        self.set_property(0, self.channel)

class SetTempoEvent(MIDIEvent):
    _rust_id = 10
    us_per_quarter_note = 500000
    def __repr__(self):
        return bytes([
            0xFF, 0x51, 0x03,
            (self.us_per_quarter_note // (256 ** 2)) % 256,
            (self.us_per_quarter_note // 256) % 256,
            self.us_per_quarter_note % 256
        ])

    def __init__(self, midilike, **kwargs):
        if "uuid" not in kwargs.keys():
            if "uspqn" in kwargs.keys():
                self.us_per_quarter_note = kwargs['uspqn']
            elif "bpm" in kwargs.keys():
                bpm = kwargs["bpm"]
                self.us_per_quarter_note = 60000000 // bpm

        super().__init__(midilike, **kwargs)

    def sync(self):
        self.get_us_per_quarter_note()

    def get_bpm(self):
        return 60000000 / self.get_us_per_quarter_note()

    def get_us_per_quarter_note(self):
        usqpn = self.get_property(0)
        self.us_per_quarter_note = 0

        for n in self.get_property(0):
            self.us_per_quarter_note *= 256
            self.us_per_quarter_note += n

        return self.us_per_quarter_note

    def set_bpm(self, bpm):
        self.set_us_per_quarter_note(60000000 // bpm)

    def set_us_per_quarter_note(self, uspqn):
        self.us_per_quarter_note = uspqn
        self.set_property(0, self.us_per_quarter_note)

class SMPTEOffsetEvent(MIDIEvent):
    _rust_id = 11
    def __repr__(self):
        return bytes([
            0xFF, 0x54, 0x05,
            self.hour, self.minute, self.second,
            self.ff, self.fr
        ])

    def __init__(self, midilike, **kwargs):
        if "uuid" not in kwargs.keys():
            self.hour = kwargs["hour"]
            self.minute = kwargs["minute"]
            self.second = kwargs["second"]
            self.ff = kwargs["ff"]
            self.fr = kwargs["fr"]
        super().__init__(midilike, **kwargs)

    def sync(self):
        self.get_hour()
        self.get_minute()
        self.get_second()
        self.get_ff()
        self.get_fr()

    def get_hour(self):
        self.hour = self.get_property(0)[0]
        return self.hour

    def get_minute(self):
        self.minute = self.get_property(1)[0]
        return self.minute

    def get_second(self):
        self.second = self.get_property(2)[0]
        return self.second

    def get_ff(self):
        self.ff = self.get_property(3)[0]
        return self.ff

    def get_fr(self):
        self.fr = self.get_property(4)[0]
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
    def __repr__(self):
        return bytes([
            0xFF, 0x58, 0x04,
            self.numerator, self.denominator,
            self.clocks_per_metronome,
            self.thirtysecondths_per_quarter
        ])

    def __init__(self, midilike, **kwargs):
        if "uuid" not in kwargs.keys():
            self.numerator = kwargs["numerator"]
            self.denominator = kwargs["denominator"]
            self.clocks_per_metronome = kwargs["cpm"]
            self.thirtysecondths_per_quarter = kwargs["tspqn"]
        super().__init__(midilike, **kwargs)

    def sync(self):
        self.get_numerator()
        self.get_denominator()
        self.get_clocks_per_metronome()
        self.get_thirtysecondths_per_quarter_note()

    def get_numerator(self):
        self.numerator = self.get_property(0)[0]
        return self.numerator

    def get_denominator(self):
        self.denominator = self.get_property(1)[0]
        return self.denominator

    def get_clocks_per_metronome(self):
        self.clocks_per_metronome = self.get_property(3)[0]
        return self.clocks_per_metronome

    def get_thirtysecondths_per_quarter_note(self):
        self.thirtysecondths_per_quarter = self.get_property(4)[0]
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

    def __repr__(self):
        mi, sf = self.misf_map[self.key]
        return bytes([0xFF, 0x59, 0x02, sf, mi])

    def __init__(self, midilike, **kwargs):
        if "uuid" not in kwargs.keys():
            self.key = kwargs["key"]
        super().__init__(midilike, **kwargs)

    def sync(self):
        self.get_key()

    def get_key(self):
        self.key = bytes(self.get_property(0)).decode("utf8")
        return self.key

    def set_key(self, key):
        self.key = key
        self.set_property(0, key)


class SequencerEvent(MIDIEvent):
    _rust_id = 14
    data = b''
    def __repr__(self):
        output = [0xFF, 0x7F]
        data_length = len(self.data)
        output.extend(to_variable_length(data_length))
        return bytes(output) + self.data

    def __init__(self, midilike, **kwargs):
        if "uuid" not in kwargs.keys():
            self.data = kwargs["data"]
        super().__init__(midilike, **kwargs)

    def get_data(self):
        self.data = bytes(self.get_property(0))
        return self.data

    def set_data(self, data):
        self.data = data
        self.set_property(0, self.data)


class NoteOnEvent(MIDIEvent):
    _rust_id = 15
    def __repr__(self):
        return bytes([
            0x90 & self.channel,
            self.note,
            self.velocity
        ])
    def __init__(self, midilike, **kwargs):
        if "uuid" not in kwargs.keys():
            self.channel = kwargs["channel"]
            self.note = kwargs["note"]
            self.velocity = kwargs["velocity"]
        super().__init__(midilike, **kwargs)

    def sync(self):
        self.get_channel()
        self.get_note()
        self.get_velocity()

    def get_channel(self):
        prop = self.get_property(0)
        self.channel = prop[0]
        return self.channel

    def get_note(self):
        self.note = self.get_property(1)[0]
        return self.note

    def get_velocity(self):
        self.velocity = self.get_property(2)[0]
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
    def __repr__(self):
        return bytes([
            0x80 & self.channel,
            self.note,
            self.velocity
        ])

    def __init__(self, midilike, **kwargs):
        if "uuid" not in kwargs.keys():
            self.channel = kwargs["channel"]
            self.note = kwargs["note"]
            self.velocity = kwargs["velocity"]
        super().__init__(midilike, **kwargs)
    def sync(self):
        self.get_channel()
        self.get_note()
        self.get_velocity()

    def get_channel(self):
        self.channel = self.get_property(0)[0]
        return self.channel

    def get_note(self):
        self.note = self.get_property(1)[0]
        return self.note

    def get_velocity(self):
        self.velocity = self.get_property(2)[0]
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
    def __repr__(self):
        return bytes([
            0xA0 & self.channel,
            self.note,
            self.pressure
        ])
    def __init__(self, midilike, **kwargs):
        if "uuid" not in kwargs.keys():
            self.channel = kwargs["channel"]
            self.note = kwargs["note"]
            self.pressure = kwargs["pressure"]
        super().__init__(midilike, **kwargs)

    def sync(self):
        self.get_channel()
        self.get_note()
        self.get_pressure()

    def get_channel(self):
        self.channel = self.get_property(0)[0]
        return self.channel

    def get_note(self):
        self.note = self.get_property(1)[0]
        return self.note

    def get_pressure(self):
        self.pressure = self.get_property(2)[0]
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
    def __repr__(self):
        return bytes([
            0xB0 & self.channel,
            self.controller,
            self.value
        ])

    def __init__(self, midilike, **kwargs):
        if "uuid" not in kwargs.keys():
            self.channel = kwargs["channel"]
            self.controller = kwargs["controller"]
            self.value = kwargs["value"]
        super().__init__(midilike, **kwargs)

    def sync(self):
        self.get_channel()
        self.get_controller()
        self.get_value()

    def get_channel(self):
        self.channel = self.get_property(0)[0]
        return self.channel

    def get_controller(self):
        self.controller = self.get_property(1)[0]
        return self.controller

    def get_value(self):
        self.value = self.get_property(2)[0]
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
    def __repr__(self):
        return bytes([
            0xC0 & self.channel,
            self.program
        ])

    def __init__(self, midilike, **kwargs):
        if "uuid" not in kwargs.keys():
            self.channel = kwargs["channel"]
            self.program = kwargs["program"]
        super().__init__(midilike, **kwargs)

    def sync(self):
        self.get_channel()
        self.get_program()

    def get_channel(self):
        self.channel = self.get_property(0)[0]
        return self.channel

    def get_program(self):
        self.program = self.get_property(1)[0]
        return self.program

    def set_channel(self, channel):
        self.channel = channel
        self.set_property(0, channel)

    def set_program(self, program):
        self.program = program
        self.set_property(1, program)


class ChannelPressureEvent(MIDIEvent):
    _rust_id = 20
    def __repr__(self):
        return bytes([
            0xD0 & self.channel,
            self.pressure
        ])

    def __init__(self, midilike, **kwargs):
        if "uuid" not in kwargs.keys():
            self.channel = kwargs["channel"]
            self.pressure = kwargs["pressure"]
        super().__init__(midilike, **kwargs)

    def sync(self):
        self.get_channel()
        self.get_pressure()

    def get_channel(self):
        self.channel = self.get_property(0)[0]
        return self.channel

    def get_pressure(self):
        self.pressure = self.get_property(1)[0]
        return self.pressure

    def set_channel(self, channel):
        self.channel = channel
        self.set_property(0, channel)

    def set_pressure(self, pressure):
        self.pressure = pressure
        self.set_property(1, pressure)

class PitchWheelChangeEvent(MIDIEvent):
    _rust_id = 21
    def __repr__(self):
        least = self.value & 0x7F
        most = (self.value >> 7) & 0x7F
        return bytes([(0xE0 | self.channel), least, most])

    def __init__(self, midilike, **kwargs):
        if "uuid" not in kwargs.keys():
            self.channel = kwargs["channel"]
            self.value = kwargs["value"]
        super().__init__(midilike, **kwargs)

    def sync(self):
        self.get_channel()
        self.get_value()

    def get_channel(self):
        self.channel = self.get_property(0)[0]
        return self.channel

    def get_value(self):
        prop = self.get_property(1)
        self.value = (prop[0] * 256) + prop[1]
        return self.value

    def set_channel(self, channel):
        self.channel = channel
        self.set_property(0, channel)

    def set_value(self, value):
        self.value = value
        self.set_property(1, value)

class SystemExclusiveEvent(MIDIEvent):
    _rust_id = 23
    data = b''
    def __repr__(self):
        output = [0xF0]
        for b in self.data:
            output.append(b)
        output.append(0xF7)
        return bytes(output)

    def __init__(self, midilike, **kwargs):
        if "uuid" not in kwargs.keys():
            self.data = kwargs["data"]

        super().__init__(midilike, **kwargs)

    def sync(self):
        self.get_data()

    def get_data(self):
        self.data = bytes(self.get_property(0))
        return self.data

class MTCQuarterFrameEvent(MIDIEvent):
    _rust_id = 24
    time_code = 0
    def __repr__(self):
        return bytes([0xF1, time_code])

    def __init__(self, midilike, **kwargs):
        if "uuid" not in kwargs.keys():
            self.time_code = kwargs["time_code"] & 0xFF
        super().__init__(midilike, kwargs)

    def sync(self):
        self.get_time_code()

    def get_time_code(self):
        self.time_code = self.get_property(0)[0] & 0xFF
        return self.time_code

class SongPositionPointerEvent(MIDIEvent):
    _rust_id = 25
    def __repr__(self):
        least = self.beat & 0x7F
        most = (self.beat >> 7) & 0x7F
        return bytes([0xF2, least, most])

    def __init__(self, midilike, **kwargs):
        if "uuid" not in kwargs.keys():
            self.beat = kwargs["beat"]
        super().__init__(midilike, **kwargs)

    def sync(self):
        self.get_beat()

    def get_beat(self):
        prop = self.get_property(0)
        self.beat = (prop[0] * 256) + prop[1]
        return self.beat

    def set_beat(self, beat):
        self.beat = beat
        self.set_property(1, beat)

class SongSelectEvent(MIDIEvent):
    _rust_id = 26
    def __repr__(self):
        return bytes([0xF3, self.song& 0xFF])

    def __init__(self, midilike, **kwargs):
        if "uuid" not in kwrags.keys():
            self.song = kwargs["song"]
        super().__init__(midilike, **kwargs)

    def sync(self):
        self.get_song()

    def get_song(self):
        self.song = self.get_property(0)[0]
        return self.song

    def set_song(self, song):
        self.song = song
        self.set_property(0, song)

class TuneRequestEvent(MIDIEvent):
    _rust_id = 27
    def __repr__(self):
        return bytes([0xF6])

class MIDIClockEvent(MIDIEvent):
    _rust_id = 28
    def __repr__(self):
        return bytes([0xF8])

class MIDIStartEvent(MIDIEvent):
    _rust_id = 29
    def __repr__(self):
        return bytes([0xFA])

class MIDIContinueEvent(MIDIEvent):
    _rust_id = 30
    def __repr__(self):
        return bytes([0xFB])

class MIDIStopEvent(MIDIEvent):
    _rust_id = 31
    def __repr__(self):
        return bytes([0xFC])

class ActiveSenseEvent(MIDIEvent):
    _rust_id = 32
    def __repr__(self):
        return bytes([0xFE])

class ResetEvent(MIDIEvent):
    _rust_id = 33
    def __repr__(self):
        return bytes([0xFF])


ml = MIDILike(sys.argv[1])
#print(ml.tracks[0]._ticks.keys())
