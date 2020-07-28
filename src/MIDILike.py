'''Mutable Midi Library'''
import sys
from cffi import FFI

class NoTickException(Exception):
    pass


class MIDILike:
    """Usable object. Converted from midi files.
        Events are the same midi files from simplicities sake.
    """
    SO_PATH = "/home/pent/Projects/RustMidiLib/target/debug/libRustMidiLib.so"
    def _get_track_count(self):
        return self.lib.get_track_count(self.pointer)

    def __init__(self, path):
        ffi = FFI()
        ffi.cdef("""
                typedef void* MIDILike;

                MIDILike interpret(const char*);
                uint32_t get_track_length(MIDILike, uint32_t);
                uint32_t get_active_tick_count(MIDILike, uint32_t);
                uint32_t get_nth_active_tick(MIDILike, uint32_t, uint32_t);
                uint32_t get_track_count(MIDILike);
                uint32_t get_tick_length(MIDILike, uint32_t, uint32_t);
                uint32_t get_nth_event_in_tick(MIDILike, uint32_t, uint32_t, uint32_t);
                void set_event_property(MIDILIKE, uint32_t, const char*);
                uint32_t get_event_value(MIDILike, uint32_t, uint32_t);
                """)

        self.lib = ffi.dlopen(self.SO_PATH)
        self.path = path
        fmt_path = bytes(self.path, 'utf-8')
        self.pointer = self.lib.interpret(fmt_path)

        self.tracks = []
        for i in range(self._get_track_count()):
            new_track = MIDILikeTrack(i, self)
            self.tracks.append(new_track)

    def get_tracks(self):
        return self.tracks

    def _get_track_length(self, n):
        return self.lib.get_track_length(self.pointer, n)

    def _get_active_tick_count(self, track):
        return self.lib.get_active_tick_count(self.pointer, track)

    def _get_nth_active_tick(self, track, n):
        return self.lib.get_nth_active_tick(self.pointer, track, n)

    def _get_tick_length(self, track, tick):
        return self.lib.get_tick_length(self.pointer, track, tick)

    def _get_nth_event_in_tick(self, track, tick, n):
        return self.lib.get_nth_event_in_tick(self.pointer, track, tick, n)

    def _get_events_in_tick(self, track, tick):
        return self.lib.get_events_in_tick(self.pointer, track, tick)

    def _set_event_property(self, n, somevalue):
        self.lib.set_event_property(self.pointer, n, somevalue)

    def _get_event_value(self, event_uuid, event_property):
        return self.lib.get_event_value(event_uuid, event_property)
>>>>>>> Stashed changes
    ##########################################################


class MIDILikeTrack:
    def __init__(self, track_number, midilike):
        self.track_number = track_number

        self._midilike = midilike
        self._ticks = {}

        for i in range(self._get_active_tick_count()):
            tick = self._get_nth_active_tick(i)
            self._ticks[tick] = []
            for j in range(self._get_tick_length(tick)):
                uuid = self._get_nth_event_in_tick(tick, j)
                self._ticks[tick].append(MIDIEvent(uuid, self._midilike))

    def get_ticks(self):
        return self._ticks

    def _get_nth_event_in_tick(self, tick, n):
        return self._midilike._get_nth_event_in_tick(self.track_number, tick, n)

    def _get_nth_active_tick(self, n):
        return self._midilike._get_nth_active_tick(self.track_number, n)

    def _get_tick_length(self, tick):
        return self._midilike._get_tick_length(self.track_number, tick)

    def _get_active_tick_count(self):
        return self._midilike._get_active_tick_count(self.track_number)

    def __len__(self):
        return self._midilike._get_track_length(self.track_number)

    def add_event(self, constructor, **kwargs):
        tick = 0
        if "tick" in kwargs.keys():
            tick = kwargs["tick"]
        elif "wait" in kwargs.keys():
            tick = self._get_active_tick_count() + kwargs['wait']
        else:
            raise NoTickException

        new_event = self._midilike.create_new_event(track, event_as_bytes)


class MIDIEvent:
    def __init__(self, midilike):
        self._midilike = midilike
        self.uuid = self._midilike.create_new_event(self.__repr__())


class SequenceNumberEvent(MIDIEvent):
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
        self.sequence = kwards['sequence']
        super().__init__(midilike)

class TextEvent(MIDIEvent):
    text = ''
    def __repr__(self):
        output = [0xFF, 0x01]
        text_bytes = self.text.encode("utf8")
        output.extend(to_variable_length(len(text_bytes)))
        return bytes(output) + text_bytes

    def __init__(self, midilike, **kwargs):
        self.text = kwargs['text']
        super().__init__(midilike)

class CopyRightNoticeEvent(MIDIEvent):
    text = ""
    def __repr__(self):
        output = [0xFF, 0x02]
        text_bytes = self.text.encode("utf8")
        output.extend(to_variable_length(len(text_bytes)))
        return bytes(output) + text_bytes

    def __init__(self, midilike, **kwargs):
        self.text = kwargs["text"]
        super().__init__(midilike)

class TrackNameEvent(MIDIEvent):
    name = ""
    def __repr__(self):
        output = [0xFF, 0x03]
        text_bytes = self.name.encode("utf8")
        output.extend(to_variable_length(len(text_bytes)))
        return bytes(output) + text_bytes

    def __init__(self, midilike, **kwargs):
        self.name = kwargs["name"]
        super().__init__(midilike)

class InstrumentNameEvent(MIDIEvent):
    name = ""
    def __repr__(self):
        output = [0xFF, 0x04]
        text_bytes = self.name.encode("utf8")
        output.extend(to_variable_length(len(text_bytes)))
        return bytes(output) + text_bytes

    def __init__(self, midilike, **kwargs):
        self.name = kwargs["name"]
        super().__init__(midilike)

class LyricEvent(MIDIEvent):
    lyric = ""
    def __repr__(self):
        output = [0xFF, 0x05]
        text_bytes = self.lyric.encode("utf8")
        output.extend(to_variable_length(len(text_bytes)))
        return bytes(output) + text_bytes

    def __init__(self, midilike, **kwargs):
        self.lyric = kwargs["lyric"]
        super().__init__(midilike)

class MarkerEvent(MIDIEvent):
    text = ""
    def __repr__(self):
        output = [0xFF, 0x06]
        text_bytes = self.text.encode("utf8")
        output.extend(to_variable_length(len(text_bytes)))
        return bytes(output) + text_bytes

    def __init__(self, midilike, **kwargs):
        self.text = kwargs["text"]
        super().__init__(midilike)

class CuePointEvent(MIDIEvent):
    text = ""
    def __repr__(self):
        output = [0xFF, 0x07]
        text_bytes = self.text.encode("utf8")
        output.extend(to_variable_length(len(text_bytes)))
        return bytes(output) + text_bytes

    def __init__(self, midilike, **kwargs):
        self.text = kwargs["text"]
        super().__init__(midilike)

class EndOfTrackEvent(MIDIEvent):
    def __repr__(self):
        return bytes([0xFF, 0x2F, 0x00])

    def __init__(self, midilike):
        super().__init__(midilike)

class ChannelPrefixEvent(MIDIEvent):
    channel = 0
    def __repr__(self):
        return bytes([0xFF, 0x20, 0x01, self.channel])

    def __init__(self, midilike, **kwargs):
        self.channel = kwargs["channel"]
        super().__init__(midilike)

class SetTempoEvent(MIDIEvent):
    us_per_quarter_note = 500000
    def __repr__(self):
        return bytes([
            0xFF, 0x51, 0x03,
            (self.us_per_quarter_note // (256 ** 2)) % 256,
            (self.us_per_quarter_note // 256) % 256,
            self.us_per_quarter_note % 256
        ])

    def __init__(self, midilike, **kwargs):
        if "uspqn" in kwargs.keys():
            self.us_per_quarter_note = kwargs['uspqn']
        elif "bpm" in kwargs.keys():
            bpm = kwargs["bpm"]
            self.us_per_quarter_note = 60000000 // bpm

        super().__init__(midilike)

class SMPTEOffsetEvent(MIDIEvent)
    def __repr__(self):
        return bytes([
            0xFF, 0x54, 0x05,
            self.hour, self.minute, self.second,
            self.ff, self.fr
        ])

    def __init__(self, midilike, **kwargs):
        self.hour = kwargs["hour"]
        self.minute = kwargs["minute"]
        self.second = kwargs["second"]
        self.ff = kwargs["ff"]
        self.fr = kwargs["fr"]
        super().__init__(midilike)

class TimeSignatureEvent(MIDIEvent):
    def __repr__(self):
        return bytes([
            0xFF, 0x58, 0x04,
            self.numerator, self.denominator,
            self.clocks_per_metronome,
            self.thirtysecondths_per_quarter
        ])

    def __init__(self, midilike, **kwargs):
        self.numerator = kwargs["numerator"]
        self.denominator = kwargs["denominator"]
        self.clocks_per_metronome = kwargs["cpm"]
        self.thirtysecondths_per_quarter = kwargs["tspqn"]
        super().__init__(midilike)

class KeySignatureEvent(MIDIEvent):
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
        self.key = kwargs["key"]
        super().__init__(midilike)

class SequencerEvent(MIDIEvent):
    data = b''
    def __repr__(self):
        output = [0xFF, 0x7F]
        data_length = len(self.data)
        output.extend(to_variable_length(data_length))
        return bytes(output) + self.data

    def __init__(self, midilike, **kwargs):
        self.data = kwargs["data"]
        super().__init__(midilike)

class NoteOnEvent(MIDIEvent):
    def __repr__(self):
        return bytes([
            0x90 & self.channel,
            self.note,
            self.velocity
        ])
    def __init__(self, midilike, **kwargs):
        self.channel = kwargs["channel"]
        self.note = kwargs["note"]
        self.velocity = kwargs["velocity"]
        super().__init__(midilike)

class NoteOffEvent(MIDIEvent):
    def __repr__(self):
        return bytes([
            0x80 & self.channel,
            self.note,
            self.velocity
        ])

    def __init__(self, midilike, **kwargs):
        self.channel = kwargs["channel"]
        self.note = kwargs["note"]
        self.velocity = kwargs["velocity"]
        super().__init__(midilike)

class PolyphonicKeyPressureEvent(MIDIEvent):
    def __repr__(self):
        return bytes([
            0xA0 & self.channel,
            self.note,
            self.pressure
        ])
    def __init__(self, midilike, **kwargs):
        self.channel = kwargs["channel"]
        self.note = kwargs["note"]
        self.pressure = kwargs["pressure"]
        super().__init__(midilike)

class ControlChangeEvent(MIDIEvent):
    def __repr__(self):
        return bytes([
            0xB0 & self.channel,
            self.controller,
            self.value
        ])

    def __init__(self, midilike, **kwargs):
        self.channel = kwargs["channel"]
        self.controller = kwargs["controller"]
        self.value = kwargs["value"]
        super().__init__(midilike)

class ProgramChangeEvent(MIDIEvent):
    def __repr__(self):
        return bytes([
            0xC0 & self.channel,
            self.program
        ])

    def __init__(self, midilike, **kwargs):
        self.channel = kwargs["channel"]
        self.program = kwargs["program"]
        super().__init__(midilike)

class ChannelPressureEvent(MIDIEvent):
    def __repr__(self):
        return bytes([
            0xD0 & self.channel,
            self.pressure
        ])

    def __init__(self, midilike, **kwargs):
        self.channel = kwargs["channel"]
        self.pressure = kwargs["pressure"]
        super().__init__(midilike)

class PitchWheelChangeEvent(MIDIEvent):
    def __repr__(self):
        return bytes([(0xE0 | self.channel), least, most])

    def __init__(self, midilike, **kwargs):
        self.channel = kwargs["channel"]
        self.least = kwargs["least"]
        self.most = kwargs["most"]
        super().__init__(midilike)

ml = MIDILike(sys.argv[1])
print(ml.tracks[0]._ticks.keys())
