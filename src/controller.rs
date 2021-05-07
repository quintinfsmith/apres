use super::*;
use std::fs::File;
use super::{MIDIEvent, ApresError};
use std::time::{Duration, Instant};

pub struct Controller {
    pipe: File,
    listening: bool
}
type Callback<T> = fn(&mut Controller, &mut T, &MIDIEvent);

impl Controller {
    pub fn new(path: &str) -> Result<Controller, ApresError> {
        match File::open(path) {
            Ok(pipe) => {
                Ok(Controller {
                    pipe,
                    listening: false
                })
            }
            Err(e) => {
               Err(ApresError::PathNotFound(path.to_string()))
            }
        }
    }

    pub fn listen<T>(&mut self, context: &mut T, callback: Callback<T>) -> Result<(), ApresError> {
        self.listening = true;

        let start_time = Instant::now();
        let ignore_time = Duration::new(0, 100_000_000);

        while self.listening {
            match self.get_next() {
                Ok(event) => {
                    // Fixme: Kludge to prevent pre-existing events from firing
                    if start_time.elapsed() > ignore_time {
                        callback(self, context, &event);
                    }
                }
                Err(e) => {
                    self.listening = false;
                    Err(e)?;
                }
            }
        }

        Ok(())
    }

    pub fn kill(&mut self) {
        self.listening = false;
    }

    fn get_next_byte(&mut self) -> Result<u8, ApresError> {
        let mut buffer = [0;1];
        loop {
            match self.pipe.read_exact(&mut buffer) {
                Ok(_success) => {
                    break;
                }
                Err(_e) => {
                    Err(ApresError::PipeBroken)?;
                }
            }
        }
        Ok(buffer[0])
    }

    fn get_next(&mut self) -> Result<MIDIEvent, ApresError> {
        let n: u32;
        let varlength: u64;

        let lead_byte = self.get_next_byte()?;
        match lead_byte {
            0..=0x7F => {
                //bytes.insert(0, lead_byte);
                //bytes.insert(0, default_byte);
                //output = MIDIEvent::from_bytes(bytes, default_byte);
                Err(ApresError::InvalidBytes(vec![lead_byte]))
            }

            0x80..=0xEF => {
                let channel: u8;
                let lead_nibble: u8 = lead_byte >> 4;
                match lead_nibble {
                    0x8 => {
                        channel = lead_byte & 0x0F;
                        let note = self.get_next_byte()?;
                        let velocity = self.get_next_byte()?;
                        Ok(MIDIEvent::NoteOff(channel, note, velocity))
                    }
                    0x9 => {
                        channel = lead_byte & 0x0F;
                        let note = self.get_next_byte()?;
                        let velocity = self.get_next_byte()?;
                        // Convert fake NoteOff (NoteOn where velocity is 0) to real NoteOff
                        if velocity == 0 {
                            Ok(MIDIEvent::NoteOff(channel, note, velocity))
                        } else {
                            Ok(MIDIEvent::NoteOn(channel, note, velocity))
                        }
                    }
                    0xA => {
                        channel = lead_byte & 0x0F;
                        let note = self.get_next_byte()?;
                        let velocity = self.get_next_byte()?;
                        Ok(MIDIEvent::AfterTouch(channel, note, velocity))
                    }
                    0xB => {
                        channel = lead_byte & 0x0F;
                        let controller = self.get_next_byte()?;
                        let value = self.get_next_byte()?;
                        Ok(MIDIEvent::ControlChange(channel, controller, value))
                    }
                    0xC => {
                        channel = lead_byte & 0x0F;
                        let new_program = self.get_next_byte()?;
                        Ok(MIDIEvent::ProgramChange(channel, new_program))
                    }
                    0xD => {
                        channel = lead_byte & 0x0F;
                        let pressure = self.get_next_byte()?;
                        Ok(MIDIEvent::ChannelPressure(channel, pressure))
                    }
                    0xE => {
                        channel = lead_byte & 0x0F;
                        let least_significant_byte = self.get_next_byte()?;
                        let most_significant_byte = self.get_next_byte()?;
                        Ok(build_pitch_wheel_change(channel, least_significant_byte, most_significant_byte))
                    }
                    _ => {
                        Err(ApresError::InvalidBytes(vec![lead_byte]))
                    }
                }
            }

            0xF0 => {
                // System Exclusive
                let mut bytedump = Vec::new();
                loop {
                    let byte = self.get_next_byte()?;
                    if byte == 0xF7 {
                        break;
                    } else {
                        bytedump.push(byte);
                    }
                }

                Ok(MIDIEvent::SystemExclusive(bytedump))
            }

            // Time Code
            0xF1 => {
                let byte_a = self.get_next_byte()?;
                let rate = match (byte_a >> 5) & 0x3 {
                    0 => {
                        24.0
                    }
                    1 => {
                        25.0
                    }
                    2 => {
                        29.97
                    }
                    3 => {
                        30.0
                    }
                    _ => {
                        30.0
                    }
                };
                let hour = byte_a & 0x1F;
                let minute = self.get_next_byte()? & 0x3F;
                let second = self.get_next_byte()? & 0x3F;
                let frame = self.get_next_byte()? & 0x1F;

                Ok(MIDIEvent::TimeCode(rate, hour, minute, second, frame))
            }
            0xF2 => {
                let least_significant_byte = self.get_next_byte()?;
                let most_significant_byte = self.get_next_byte()?;

                let beat = ((most_significant_byte as u16) << 7) + (least_significant_byte as u16);
                Ok(MIDIEvent::SongPositionPointer(beat))
            }

            0xF3 => {
                let song = self.get_next_byte()?;
                Ok(MIDIEvent::SongSelect(song & 0x7F))
            }

            0xF6 => {
                Ok(MIDIEvent::TuneRequest)
            }

            0xF7 => {
                // Real Time SysEx
                let mut bytedump = Vec::new();
                let length = self.get_next_byte()?;
                for _ in 0 .. length {
                    let byte = self.get_next_byte()?;
                    bytedump.push(byte);
                }

                Ok(MIDIEvent::SystemExclusive(bytedump))
            }

            // Clock
            0xF8 => {
                Ok(MIDIEvent::MIDIClock)
            }
            // Start
            0xFA => {
                Ok(MIDIEvent::MIDIStart)
            }
            // Continue
            0xFB => {
                Ok(MIDIEvent::MIDIContinue)
            }
            //Stop
            0xFC => {
                Ok(MIDIEvent::MIDIStop)
            }
            //Active Sensing
            0xFE => {
                Ok(MIDIEvent::ActiveSense)
            }
            // System Reset
            0xFF => {
                Ok(MIDIEvent::Reset)
            }
            // Undefined Behaviour
            0xF4 | 0xF5 | 0xF9 | 0xFD => {
                Err(ApresError::InvalidBytes(vec![lead_byte]))
            }
        }
    }
}


