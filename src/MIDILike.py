'''Mutable Midi Library'''
import sys
from cffi import FFI

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
                """)
        self.lib = ffi.dlopen(self.SO_PATH)
        self.path = path
        fmt_path = bytes(self.path, 'utf-8')
        self.pointer = self.lib.interpret(fmt_path)

        self.tracks = []
        for i in range(self._get_track_count()):
            new_track = MIDILikeTrack(i, self)
            self.tracks.append(new_track)


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


class MIDIEvent:
    def __init__(self, uuid, midilike):
        self._midilike = midilike
        self.uuid = uuid



ml = MIDILike(sys.argv[1])
print(ml.tracks[0]._ticks.keys())
