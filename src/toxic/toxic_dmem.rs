mod toxic_io;

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

    pub fn write(&mut self, addr: u32, content: u8) {
        let mem_loc: &mut u8 = match self.raw.get_mut(addr as usize) {
            Some(r) => r,
            None => std::panic!("Invalid Memory Location to Write: {}", addr),
        };
        // println!("Write {} to {}", content, addr);
        *mem_loc = content;
    }

    pub fn read(&self, addr: u32) -> u8 {
        let x: u8 = self.raw[addr as usize];
        // println!("Read {} from {}", x, addr);
        x
    }
}
