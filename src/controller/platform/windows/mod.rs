use crate::ApresError;
use windows::Win32::Media::{
    MMSYSERR_NOERROR,
    MMSYSERR_BADDEVICEID,
    MMSYSERR_INVALPARAM,
    MMSYSERR_NOMEM,
    Audio
};

pub struct Controller {
    pub listening: bool,
    header: Audio::HMIDIIN
}

impl Controller {
    pub fn new(channel:u8, dev_id: u8) -> Result<Controller, ApresError> {
        let mut phmi: *mut Audio::HMIDIIN;

        let mut udeviceid = devi_id as u32;
        let dwcallback = 0;
        let dwinstance = 0;

        // NOTE: there is a  CALLBACK_THREAD flag here.
        // In the future it may be beneficial to use that.
        // But for now, for consistency's sake, we use no callback.
        let fdwopen: u32 = Audio::CALLBACK_NULL;

        unsafe {
            let errmsg = Audio::midiInOpen(
                phmi,
                udeviceid,
                dwcallback,
                dwinstance,
                fdwopen
            );

            match errmsg {
                MMSYSERR_NOERROR => {
                }
                MMSYSERR_BADDEVICEID => {
                }
                MMSYSERR_INVALPARAM => {
                }
                MMSYSERR_NOMEM => {
                }
                _ => {}
            }
        }

        Ok(Controller (
            header: phmi,
            listening: false,
        ))
    }

    pub fn kill(&mut self) {
        self.stop_listening();
        unsafe {
            Audio::midiInClose(self.header);
        }
    }

    pub fn poll_next_byte(&mut self) -> Option<u8> {
        Some(0)
    }
}


