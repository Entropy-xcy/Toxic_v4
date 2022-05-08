#[derive(Debug)]
pub struct ToxicDataMem {
    raw: Vec<u8>,
}

impl ToxicDataMem {
    pub fn new(bus_width: u32) -> ToxicDataMem {
        ToxicDataMem {
            raw: vec![0u8; u32::pow(2, bus_width) as usize],
        }
    }
}
