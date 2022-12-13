use hidapi::HidDevice;

pub struct Gameband {
    pub device: HidDevice
}

pub fn open_gameband() -> Option<Gameband>{
    let api = hidapi::HidApi::new().unwrap();

    let (vid, pid) = (0x2a90, 0x0021);
    return match api.open(vid, pid) {
        Ok(dev) => {
            Option::from(Gameband {
                device: dev
            })
        }
        Err(_) => {
            None
        }
    }
}