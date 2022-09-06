use crate::ApresError;
use windows::Win32::Media::{
    MMSYSERR_NOERROR,
    MMSYSERR_BADDEVICEID,
    MMSYSERR_INVALPARAM,
    MMSYSERR_NOMEM,
    Audio
};

pub struct InstanceData {

}

pub struct Controller {
    pub listening: bool,
    header: Audio::HMIDIIN
}
type Callback = fn(hmidin: Audio::HMIDIIN, wMsg: u32, dwInstance: *mut u32, dwParam1: *mut u32, dwParam3: *mut u32);

fn event_callback(hmidin: Audio::HMIDIIN, wMsg: u32, dwInstance: *mut u32, dwParam1: *mut u32, dwParam3: *mut u32) {
    println!("DO SOMETHING");
}
impl Controller {

    pub fn new(channel:u8, dev_id: u8) -> Result<Controller, ApresError> {
        unsafe {
            let phmi = &mut Audio::HMIDIIN{
                0: 1
            };
            let mut udeviceid = dev_id as u32;
            let mut dwinstance = InstanceData {};

            let mut ec = Box::into_raw(Box::new(&event_callback));
            let mut dwi = Box::into_raw(Box::new(&dwinstance));

            let errmsg = Audio::midiInOpen(
                phmi,
                udeviceid,
                ec as usize,
                dwi as usize,
                Audio::CALLBACK_FUNCTION
            );

            match errmsg {
                MMSYSERR_NOERROR => {
                    println!("A");
                    Ok(Controller {
                        header: *phmi,
                        listening: false,
                    })
                }
                MMSYSERR_BADDEVICEID => {
                    println!("B");
                    Err(ApresError::BadDevice(dev_id))
                }
                MMSYSERR_INVALPARAM => {
                    println!("C");
                    Err(ApresError::UnknownError)
                }
                MMSYSERR_NOMEM => {
                    println!("D");
                    Err(ApresError::OutOfMemory)
                }
                _ => {
                    println!("E");
                    Err(ApresError::UnknownError)
                }
            }

        }
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


