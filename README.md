# Apres bindings
Python bindings for the Apres MIDI library<br/>
![PyPI - Downloads](https://img.shields.io/pypi/dw/apres?style=flat-square)
![PyPI - License](https://img.shields.io/pypi/l/apres?style=flat-square)
![PyPI](https://img.shields.io/pypi/v/apres?style=flat-square)

## Installation
Can be installed through pip
```
pip install apres
```
## Usage Examples
Load a Song
```python
from apres import MIDI
midi = MIDI("/path/to/file.mid")
```

Create a new MIDI
```python
from apres import MIDI
midi = MIDI()
```

Creating a song
```python
from apres import MIDI, NoteOnEvent, NoteOffEvent

# Create an empty MIDI file.
midi = MIDI()

# Press midi note 64 (Middle E) on the first track (0) at the first position (0 ticks)
midi.add_event(NoteOnEvent(channel=0, note=64, velocity=100), tick=0, track=0)

# Release midi note 64 (Middle E) on the first track (0) one beat later (120 ticks)
midi.add_event(NoteOffEvent(channel=0, note=64), wait=120, track=0)

# Save it to a file
midi.save("beep.mid")
```
