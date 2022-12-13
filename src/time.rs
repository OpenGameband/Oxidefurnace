use core::panicking::panic;
use std::fmt::Error;
use std::time::{SystemTime, UNIX_EPOCH};
use hidapi::{HidError, HidResult};
use crate::gameband::Gameband;

fn pack_time(buf: &mut [u8], offset: usize, seconds: u64) {
    let bytes = seconds.to_le_bytes();
    for i in 0..4 {
        buf[offset+i] = bytes[i];
    }
}

pub fn set_time(gameband: Gameband) -> Result<(),String> {
    let seconds = SystemTime::now().duration_since(UNIX_EPOCH).expect("It's 1969???").as_secs();

    let mut buf: [u8;9] = [0;9];
    buf[1] = 2;
    pack_time(&mut buf, 5, seconds);
    match gameband.device.write(&buf) {
        Ok(size) => {Ok(()) }
        Err(err) => {Err(err.to_string())}
    }
}
