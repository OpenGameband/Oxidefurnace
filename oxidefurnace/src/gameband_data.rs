/// Contains the data that sets the Gameband timezone, animations, and orientation
#[derive(Clone)]
pub struct GamebandHeader {
    pub timezone: u8,
    pub alt_timezone: u8,
    pub tz_change: u32,
    pub orientation: u8,
    pub transition_frame_duration: u8,
    pub screen_count: u8,
    pub animation_data_length: u16,
    pub checksum_0: u8,
    pub checksum_1: u8,
}

/// Data about the animation/screen
#[derive(Clone)]
pub struct AnimationHeader {
    pub screen_type: u8,
    pub pause_mode: u8,
    pub pause_duration: u16,
    pub frame_duration: u16,
    pub animation_type: u8,
    pub data_length: u16,
}

#[derive(Clone)]
pub struct Animation {
    pub header: AnimationHeader,
    pub frames: Vec<Frame>
}

#[derive(Clone)]
pub struct Frame {
    pub data: Vec<u8>
}

#[derive(Clone)]
pub struct GBData {
    pub header: GamebandHeader,
    pub animations: Vec<Animation>
}

pub fn pack_header(header: GamebandHeader) -> Vec<u8> {
    let mut buf: Vec<u8> = Vec::new();
    buf.push(header.timezone);
    buf.push(0);
    buf.push(header.alt_timezone);
    buf.push(0);
    buf.append(&mut u32_to_bytes(header.tz_change));
    buf.push(header.orientation);
    buf.push(0);
    buf.push(header.transition_frame_duration);
    buf.push(0);
    buf.push(header.screen_count);
    buf.push(0);
    buf.append(&mut u16_to_bytes(header.animation_data_length));
    return buf;
}

pub fn pack_animation_header(header: AnimationHeader) -> Vec<u8> {
    let mut buf: Vec<u8> = Vec::new();
    buf.push(header.screen_type);
    buf.push(0);
    buf.push(header.pause_mode);
    buf.push(0);
    buf.append(&mut u16_to_bytes(header.pause_duration));
    buf.append(&mut u16_to_bytes(header.frame_duration));
    buf.push(header.animation_type);
    buf.push(0);
    buf.append(&mut u16_to_bytes(header.data_length));
    return buf;
}

fn u16_to_bytes(i: u16) -> Vec<u8> {
    i.to_le_bytes().to_vec()
}

fn u32_to_bytes(i: u32) -> Vec<u8> {
    i.to_le_bytes().to_vec()
}