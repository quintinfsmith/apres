# Apres
A MIDI library<br/>
[![Travis (.com)](https://img.shields.io/travis/com/quintinfsmith/apres?style=flat-square)](https://travis-ci.com/github/quintinfsmith/apres)
[![Crates.io](https://img.shields.io/crates/d/apres?style=flat-square)](https://crates.io/crates/apres)
[![Crates.io](https://img.shields.io/crates/v/apres?style=flat-square)](https://crates.io/crates/apres)
[![GitHub](https://img.shields.io/github/license/quintinfsmith/apres?style=flat-square)](https://github.com/quintinfsmith/apres/blob/master/LICENSE)

## Installation
In Cargo.toml
```toml
[dependencies]
apres = { git = "https://github.com/quintinfsmith/apres" }
```
## Usage Examples
Load a Song
```rust
use apres::MIDI;
Create a MIDI from a file
let midi = MIDI::from_path("/path/to/file.mid");
```
Create a new MIDI
```rust
use apres::MIDI;
// Create an empty MIDI file.
let midi = MIDI::new();
```
Creating a song
```rust
use apres::MIDI;
use apres::MIDIEvent::{NoteOn, NoteOff};
// Create an empty MIDI file.
let mut midi = MIDI::new();

// Using channel 0, press midi note 64 (Middle E) on the first track (0) at the first position (0 ticks)
midi.insert_event(0, 0, NoteOn(0, 64, 100));

// Still on channel 0, release midi note 64 (Middle E) on the first track (0) one beat later (120 ticks)
midi.push_event(0, 120, NoteOn(0, 64, 100));

// Save it to a file
midi.save("beep.mid");
```
