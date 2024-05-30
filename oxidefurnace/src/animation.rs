use crate::gameband::{Gameband, GamebandError};
use crate::gameband_data::{Animation, GBData, pack_animation_header, pack_header};
use crate::utils::checksum;

impl Gameband {
    pub fn write_gameband_data(&self, mut data: GBData) -> Result<(), GamebandError>{
        let mut buf: Vec<u8> = Vec::with_capacity(12+animations_size(data.animations.clone()));

        let mut animation_buf: Vec<u8> = Vec::new();
        for mut a in data.animations.clone() {
            a.header.data_length = (10 * a.frames.len()) as u16;
            animation_buf.append(&mut pack_animation_header(a.header));
            for f in a.frames {
                animation_buf.append(&mut f.data.clone());
            }
        }

        data.header.animation_data_length = (animation_buf.len() / 2) as u16;
        data.header.screen_count = data.animations.len() as u8;
        buf.append(&mut pack_header(data.header));

        let sum = checksum(animation_buf.as_slice());
        println!("Checksum 0: {}", sum[0]);
        println!("Checksum 1: {}", sum[1]);

        buf.append(&mut sum[0].to_le_bytes().to_vec());
        buf.append(&mut sum[1].to_le_bytes().to_vec());

        buf.append(&mut animation_buf);


        self.write_data(buf)
    }

    pub fn set_data_length(&self, offset: u16, value: u16) -> Result<(), GamebandError>{
        let mut data: Vec<u8> = Vec::new();
        data.push(0);
        data.push(4);
        data.append(&mut offset.to_le_bytes().to_vec());
        data.append(&mut value.to_le_bytes().to_vec());

        self.blind_write(data, 5)
}

    pub fn read_chunk(&self, offset: u16) -> Result<Vec<u8>, GamebandError>{
        let mut buf: Vec<u8> = Vec::new();
        buf.push(0);
        buf.push(8);
        buf.push(0);
        buf.append(&mut offset.to_le_bytes().to_vec());

        let result = self.write(buf, 34, 9)?;
        
        return Ok(result[2..].to_vec());
    }
    
    pub fn commit(&self) -> Result<(), GamebandError> {
        let data: [u8;3] = [0, 10, 0];
            
        self.blind_write(data.to_vec(), 11)
    }
    
    
    const MAX_DATA_SIZE: usize = 2048;
    pub fn write_data(&self, mut data: Vec<u8>) -> Result<(), GamebandError> {
        let align = ((data.len() as f64 / 128.00) * 128.00) as usize;

        if align != data.len() {
            data.resize(align, 0);
        }

        if data.len() > Self::MAX_DATA_SIZE {
            return Err(GamebandError::DataTooBig {})
        }

        self.set_data_length(6144, data.len() as u16)?;

        let mut i = 0;
        while i < data.len() {

            self.write_offset((6144 + (i / 2)) as u16, data.to_vec(), i, 32)?;

            let chunk = self.read_chunk((6144 + (i / 2)) as u16)?;
            
            if !chunk.eq(&data[i..i + 32].to_vec()) {
                return Err(GamebandError::DataWriteFailure {})
            }

            i+=32;
        }


        self.commit()
    }
}

pub fn animation_size(animation: &Animation) -> usize {
    10+(animation.frames.len())
}

pub fn animations_size(animations: Vec<Animation>) -> usize {
    let mut size: usize = 0;
    for animation in animations.iter() {
        size += animation_size(animation);
    }
    return size;
}

impl Animation {
    pub fn as_data(&self) -> Vec<u8> {
        let buf: Vec<u8> = Vec::new();



        return buf;
    }
}