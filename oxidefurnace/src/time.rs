use crate::gameband::{Gameband};

fn pack_time(buf: &mut [u8], offset: usize, seconds: u32) {
    let bytes = seconds.to_le_bytes();
    for i in 0..4 {
        buf[offset+i] = bytes[i];
    }
}

const EXPECTED_RESPONSE_TIME: u8 = 3;
const TIME_PACKET_COMMAND: u8 = 2;

/// This is the implementation of time setting
impl Gameband {
    pub fn set_time(&self, seconds: u32) -> Result<(),String> {
        let mut encoded: [u8;9] = [0;9];
        encoded[1] = TIME_PACKET_COMMAND;
        pack_time(&mut encoded, 5, seconds);

        match self.write(encoded.to_vec(), 2, EXPECTED_RESPONSE_TIME) {
            Ok(_) => {
                Ok(())
            }
            Err(err) => {
                Err(format!("Unexpected response from Gameband, expected , got, {:?}", err).to_string())
            }
        }
    }
}
