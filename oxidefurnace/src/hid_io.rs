use crate::gameband::{Gameband, GamebandError};
use bincode::Encode;

#[derive(Encode)]
pub struct PacketHeader {
    pub(crate) hid_register: u8,
    pub(crate) command_id: u8
}

impl Gameband {
    pub fn new() -> Result<Gameband,GamebandError> {
        let api = hidapi::HidApi::new().unwrap();

        let (vid, pid) = (0x2a90, 0x0021);
        return match api.open(vid, pid) {
            Ok(dev) => {
                Ok(Gameband {
                    device: dev
                })
            }
            Err(hid_err) => {
                Err(GamebandError::HidError {error: hid_err})
            }
        }
    }

    pub fn write(&self, data: Vec<u8>, buf_size: usize, check: u8) -> Result<Vec<u8>, GamebandError>{
        let dev_write_result = self.device.write(data.as_slice());
        let written = match dev_write_result {
            Ok(w) => w,
            Err(error) => return Err(GamebandError::HidError {error}),
        };

        if written <= 0 {
            return Err(GamebandError::NothingWritten {});
        }

        self.device.set_blocking_mode(true).expect("Failed to set blocking mode");

        let mut resp: [u8;64] = [0;64];
        self.device.read(&mut resp).expect("Failed to read gameband");

        let out: &[u8] = &resp[0..buf_size];

        if out[0] != check {
            Err(GamebandError::UnexpectedReturnCode { return_code: out[0] })
        } else if out[1] != 0 {
            Err(GamebandError::UnexpectedReturnCode { return_code: out[1] })
        } else {
            Ok(out.to_vec())
        }
    }

    pub fn blind_write(&self, data: Vec<u8>, check: u8) -> Result<(), GamebandError>{
        match self.write(data, 2, check) {
            Ok(_) => Ok(()),
            Err(e) => Err(e)
        }
    }
    
   pub fn write_offset(&self, command_code: u16, data: Vec<u8>, offset: usize, data_size: usize) -> Result<(), GamebandError>{
       if data.len() < offset+ data_size {
            return Err(GamebandError::InvalidDataChunkSize {})
       }
       
       let mut buf: [u8;37] = [0;37];
       buf[1] = 6;
       
       buf[3] = command_code as u8;
       buf[4] = (command_code >> 8) as u8;

       for i in 0..data_size {
           buf[i+5] = data[offset+i];
       }
       
       self.blind_write(buf.to_vec(), 7)
   }
}