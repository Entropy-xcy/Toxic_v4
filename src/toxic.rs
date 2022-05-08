pub mod toxic_imem;
pub mod toxic_dmem;

use std::fmt::{Debug, Display};
use toxic_imem::*;
use toxic_imem::toxic_inst::*;
use toxic_dmem::*;


#[derive(Debug)]
pub struct Toxic {
    pub imem: ToxicInstMem,
    pub dmem: ToxicDataMem,
    pub pc: u32,
    pub pt: u32,
    pub sp: u32,
}

impl Toxic {
    pub fn new(bus_width: u32) -> Toxic {
        let imem = ToxicInstMem::new(bus_width);
        let pc_init = imem.main_address();
        Toxic {
            dmem: ToxicDataMem::new(bus_width),
            imem,
            pc: 0,
            pt: pc_init,
            sp: 0,
        }
    }
}
