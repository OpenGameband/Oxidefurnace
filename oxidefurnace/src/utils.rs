// this is ugly and could probably do with being streamlined
/// A direct rust port of the checksum algorithm used by the original gameband
pub fn checksum(data: &[u8]) -> [u16;2]{
    let mut cs1: u32 = 0;
    let mut cs2: u32 = 0;

    for i in 0 .. data.len(){
        let byte  = data[i];
        cs1 = (cs1 + (byte as u32 & 255)) % 255;
        cs2 = (cs2 + cs1) % 255;
    }

    return [cs1 as u16, cs2 as u16];
}

//replace later with checksum
// pub fn TESTcheck(data: &[u8]) -> [u16;2]{
//     let mut cs: [u32;2] = [0;2];
//     for i in 0 .. data.len(){
//         let byte  = data[i];
//         cs[0] = (cs[1] + (byte as u32 & 255)) % 255;
//         cs[1] = (cs[1] + cs[0]) % 255;
//     }
//
//     return cs;
// }

pub fn align_value(value: usize, multiplier: usize) -> usize {
    (value as f64/ multiplier as f64).ceil() as usize * multiplier
}