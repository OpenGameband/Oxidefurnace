use hidapi::{HidDevice, HidError};

pub struct Gameband {
    pub device: HidDevice
}

struct Timezone {
    current_offset: u64,
    next_offset: u64,
    change_time: u64,
}

#[derive(Debug)]
pub enum GamebandError {
    HidError { error: HidError },
    UnexpectedReturnCode { return_code: u8 },
    InvalidWordSize {},
    InvalidDataChunkSize {},
    DataTooBig {},
    NothingWritten {},
    DataWriteFailure {},
}