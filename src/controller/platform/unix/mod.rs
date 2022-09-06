use std::ffi::CString;
use libc;

use crate::ApresError;

pub struct Controller {
    file_descriptor: libc::c_int,
    pub listening: bool
}

impl Controller {
    pub fn new(channel:u8, dev_id: u8) -> Result<Controller, ApresError> {
        let path = &format!("/dev/snd/midiC{}D{}", channel, dev_id);
        let path_cstring = CString::new(path.as_str()).unwrap();
        let cpath: *const libc::c_char = path_cstring.as_ptr();
        let file_descriptor = unsafe { libc::open(cpath, 0) };

        if file_descriptor == -1 {
            Err(ApresError::PathNotFound(path.to_string()))?
        }

        Ok(Controller {
            file_descriptor,
            listening: false
        })
    }

    pub fn kill(&mut self) {
        self.stop_listening();
        unsafe {
            libc::close(self.file_descriptor);
        }
    }

    pub fn poll_next_byte(&mut self) -> Option<u8> {
        let mut output = None;
        unsafe {
            let mut fds = [libc::pollfd {
                fd: self.file_descriptor,
                events: 1 as libc::c_short,
                revents: 0 as libc::c_short
            };1];

            let ready = libc::poll(
                fds.as_mut_ptr(),
                1,
                0
            );

            if ready > 0 {
                let buffer = std::mem::transmute::<&mut u8, &mut libc::c_void>(&mut 0u8);

                let count = 1;
                let bytes_read = libc::read(
                    self.file_descriptor,
                    buffer,
                    count
                );

                let value = std::mem::transmute::<&mut libc::c_void, &u8>(buffer);
                if bytes_read > -1 {
                    output = Some(*value);
                }
            }
        }

        output
    }
}


