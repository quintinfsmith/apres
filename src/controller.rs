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
                        let controller = self.get_next_byte()?;
                        let value = self.get_next_byte()?;
                        channel = lead_byte & 0x0F;
                        match controller {
                            0x00 => {
                                Ok(MIDIEvent::BankSelect(channel, Some(value), None))
                            }
                            0x20 => {
                                Ok(MIDIEvent::BankSelect(channel, None, Some(value)))
                            }
                            0x01 => {
                                Ok(MIDIEvent::ModulationWheel(channel, Some(value), None))
                            }
                            0x21 => {
                                Ok(MIDIEvent::ModulationWheel(channel, None, Some(value)))
                            }
                            0x02 => {
                                Ok(MIDIEvent::BreathController(channel, Some(value), None))
							}
                            0x22 => {
                                Ok(MIDIEvent::BreathController(channel, None, Some(value)))
							}
                            0x04 => {
                                Ok(MIDIEvent::FootPedal(channel, Some(value), None))
							}
                            0x24 => {
                                Ok(MIDIEvent::FootPedal(channel, None, Some(value)))
							}
                            0x05 => {
                                Ok(MIDIEvent::PortamentoTime(channel, Some(value), None))
							}
                            0x25 => {
                                Ok(MIDIEvent::PortamentoTime(channel, None, Some(value)))
							}
                            0x06 => {
                                Ok(MIDIEvent::DataEntrySlider(channel, Some(value), None))
							}
                            0x26 => {
                                Ok(MIDIEvent::DataEntrySlider(channel, None, Some(value)))
							}
                            0x07 => {
                                Ok(MIDIEvent::Volume(channel, Some(value), None))
							}
                            0x27 => {
                                Ok(MIDIEvent::Volume(channel, None, Some(value)))
							}
                            0x08 => {
                                Ok(MIDIEvent::Balance(channel, Some(value), None))
							}
                            0x28 => {
                                Ok(MIDIEvent::Balance(channel, None, Some(value)))
							}
                            0x0A => {
                                Ok(MIDIEvent::Pan(channel, Some(value), None))
							}
                            0x2A => {
                                Ok(MIDIEvent::Pan(channel, None, Some(value)))
							}
                            0x0B => {
                                Ok(MIDIEvent::Expression(channel, Some(value), None))
							}
                            0x2B => {
                                Ok(MIDIEvent::Expression(channel, None, Some(value)))
							}
                            0x0C => {
                                Ok(MIDIEvent::EffectControl(channel, 0, Some(value), None))
							}
                            0x2C => {
                                Ok(MIDIEvent::EffectControl(channel, 0, None, Some(value)))
							}
                            0x0D => {
                                Ok(MIDIEvent::EffectControl(channel, 1, Some(value), None))
							}
                            0x2D => {
                                Ok(MIDIEvent::EffectControl(channel, 1, None, Some(value)))
							}

                            0x10 => {
                                Ok(MIDIEvent::GeneralPurpose(channel, 0, Some(value), None))
							}
                            0x30 => {
                                Ok(MIDIEvent::GeneralPurpose(channel, 0, None, Some(value)))
							}
                            0x11 => {
                                Ok(MIDIEvent::GeneralPurpose(channel, 1, Some(value), None))
							}
                            0x31 => {
                                Ok(MIDIEvent::GeneralPurpose(channel, 1, None, Some(value)))
							}
                            0x12 => {
                                Ok(MIDIEvent::GeneralPurpose(channel, 2, Some(value), None))
							}
                            0x32 => {
                                Ok(MIDIEvent::GeneralPurpose(channel, 2, None, Some(value)))
							}
                            0x13 => {
                                Ok(MIDIEvent::GeneralPurpose(channel, 3, Some(value), None))
							}
                            0x33 => {
                                Ok(MIDIEvent::GeneralPurpose(channel, 3, None, Some(value)))
							}
                            0x40 => {
                                Ok(MIDIEvent::HoldPedal(channel, value))
                            }
                            0x41 => {
                                Ok(MIDIEvent::Portamento(channel, value))
                            }
                            0x42 => {
                                Ok(MIDIEvent::Sustenuto(channel, value))
                            }
                            0x43 => {
                                Ok(MIDIEvent::SoftPedal(channel, value))
                            }
                            0x44 => {
                                Ok(MIDIEvent::Legato(channel, value))
                            }
                            0x45 => {
                                Ok(MIDIEvent::Hold2Pedal(channel, value))
                            }
                            0x46 => {
                                Ok(MIDIEvent::SoundVariation(channel, value))
                            }
                            0x47 => {
                                Ok(MIDIEvent::SoundTimbre(channel, value))
                            }
                            0x48 => {
                                Ok(MIDIEvent::SoundReleaseTime(channel, value))
                            }
                            0x49 => {
                                Ok(MIDIEvent::SoundAttack(channel, value))
                            }
                            0x4A => {
                                Ok(MIDIEvent::SoundBrightness(channel, value))
                            }
                            0x4B => {
                                Ok(MIDIEvent::SoundControl(channel, 0, value))
                            }
                            0x4C => {
                                Ok(MIDIEvent::SoundControl(channel, 1, value))
                            }
                            0x4D => {
                                Ok(MIDIEvent::SoundControl(channel, 2, value))
                            }
                            0x4E => {
                                Ok(MIDIEvent::SoundControl(channel, 3, value))
                            }
                            0x4F => {
                                Ok(MIDIEvent::SoundControl(channel, 4, value))
                            }
                            0x50 => {
                                if value == 0 {
                                    Ok(MIDIEvent::GeneralButtonOff(channel, 0))
                                } else {
                                    Ok(MIDIEvent::GeneralButtonOn(channel, 0))
                                }
                            }
                            0x51 => {
                                if value == 0 {
                                    Ok(MIDIEvent::GeneralButtonOff(channel, 1))
                                } else {
                                    Ok(MIDIEvent::GeneralButtonOn(channel, 1))
                                }
                            }
                            0x52 => {
                                if value == 0 {
                                    Ok(MIDIEvent::GeneralButtonOff(channel, 2))
                                } else {
                                    Ok(MIDIEvent::GeneralButtonOn(channel, 2))
                                }
                            }
                            0x53 => {
                                if value == 0 {
                                    Ok(MIDIEvent::GeneralButtonOff(channel, 3))
                                } else {
                                    Ok(MIDIEvent::GeneralButtonOn(channel, 3))
                                }
                            }

                            0x5B => {
                                Ok(MIDIEvent::EffectsLevel(channel, value))
                            }

                            0x5C => {
                                Ok(MIDIEvent::TremuloLevel(channel, value))
                            }

                            0x5D => {
                                Ok(MIDIEvent::ChorusLevel(channel, value))
                            }
                            0x5E => {
                                Ok(MIDIEvent::CelesteLevel(channel, value))
                            }

                            0x5F => {
                                Ok(MIDIEvent::PhaserLevel(channel, value))
                            }

                            0x60 => {
                                Ok(MIDIEvent::DataIncrement(channel))
                            }

                            0x61 => {
                                Ok(MIDIEvent::DataDecrement(channel))
                            }
                            0x62 => {
                                Ok(MIDIEvent::NonRegisteredParameterNumber(channel, None, Some(value)))
                            }

                            0x63 => {
                                Ok(MIDIEvent::NonRegisteredParameterNumber(channel, Some(value), None))
                            }

                            0x64 => {
                                Ok(MIDIEvent::RegisteredParameterNumber(channel, None, Some(value)))
                            }
                            0x65 => {
                                Ok(MIDIEvent::RegisteredParameterNumber(channel, Some(value), None))
                            }
                            0x78 => {
                                Ok(MIDIEvent::AllSoundOff(channel))
                            }
                            0x79 => {
                                Ok(MIDIEvent::AllControllersOff(channel))
                            }
                            0x7A => {
                                if value == 0x40 {
                                    Ok(MIDIEvent::LocalKeyboardEnable(channel))
                                } else {
                                    Ok(MIDIEvent::LocalKeyboardDisable(channel))
                                }
                            }
                            0x7B => {
                                Ok(MIDIEvent::AllNotesOff(channel))
                            }
                            0x7C => {
                                Ok(MIDIEvent::OmniOff(channel))
                            }
                            0x7D => {
                                Ok(MIDIEvent::OmniOn(channel))
                            }
                            0xFE => {
                                Ok(MIDIEvent::MonophonicOperation(channel, value))
                            }
                            0xFF => {
                                Ok(MIDIEvent::PolyphonicOperation(channel))
                            }
                            _ => {
                                Ok(MIDIEvent::ControlChange(channel, controller, value))
                            }
                        }
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


