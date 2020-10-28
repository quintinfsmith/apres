# Apres
A MIDI library
## Installation
In Cargo.toml
```
[dependencies]
apres = { git = "https://github.com/quintinfsmith/apres" }
```
## Usage Examples
Load a Song
```
use apres::MIDI;
Create a MIDI from a file
let midi = MIDI::from_path("/path/to/file.mid");
```
Create a new MIDI
```
use apres::MIDI;
// Create an empty MIDI file.
let midi = MIDI::new();
```
Creating a song
```
use apres::{MIDI, NoteOnEvent, NoteOffEvent};
// Create an empty MIDI file.
let mut midi = MIDI::new();

// Using channel 0, press midi note 64 (Middle E) on the first track (0) at the first position (0 ticks)
midi.insert_event(0, 0, Box::new(NoteOnEvent::new(0, 64, 100)));

// Still on channel 0, release midi note 64 (Middle E) on the first track (0) one beat later (120 ticks)
midi.push_event(0, 120, Box::new(NoteOnEvent::new(0, 64, 100)));

// Save it to a file
midi.save("beep.mid");
```
