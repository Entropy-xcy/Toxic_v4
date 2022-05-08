pub mod toxic_inst;
use std::fs;
use toxic_inst::*;

#[derive(Debug)]
pub struct ToxicInstMem {
    raw: Vec<u8>,
    bus_width: u32,
}

impl ToxicInstMem {
    pub fn new(bus_width: u32) -> ToxicInstMem {
        ToxicInstMem {
            raw: vec![0u8; u32::pow(2, bus_width) as usize],
            bus_width,
        }
    }

    pub fn address_space(&self) -> u32{
        u32::pow(2, self.bus_width)
    }

    pub fn main_address(&self) -> u32{
        self.address_space() / 8u32
    }

    pub fn decode(&self, pc: u32) -> ToxicInst {
        let inst_bin: u8 = match self.raw.get(pc as usize) {
            Some(r) => *r,
            None => std::panic!("Instruction Fetch Out of Range!")
        };

        let dec = toxic_inst::ToxicInst::from_bits(inst_bin);
        match dec {
            Err(e) => std::panic!("Invalid Instruction Raw Bits: {}", inst_bin),
            Ok(r) => r
        }
    }

    pub fn load(&mut self, position: u32, prog: Vec<u8>) {
        for i in 0..(prog.len()){
            let mut elem = &mut self.raw[i + (position as usize)];
            *elem = prog[i];
        }
    }

    pub fn load_from_source(&mut self, filename: String) {
        let contents = fs::read_to_string(filename)
            .expect("Cannot Read Source file");
        let mut prog: Vec<toxic_inst::ToxicInst> = Vec::new();
        for line in contents.split("\n") {
            let inst_str = line.replace(" ", "");
            // println!("{}", inst_str);
            prog.push(match toxic_inst::ToxicInst::from_str(inst_str) {
                Ok(inst) => inst,
                Err(e) => std::panic!("{}", e)
            });
        }

        let prog_raw: Vec<u8> = prog.iter().map(|x| x.to_bits()).collect();

        self.load(self.main_address(), prog_raw);
    }

    pub fn load_from_bin(&mut self, filename: String) {

    }

    pub fn to_str(&self, pc_start: u32, pc_end: u32) -> String {
        let mut result: String = String::from("");
        for pc in pc_start..pc_end{
            result = format!("{}\n{:#06x}\t{}", result, pc, self.decode(pc).to_str());
        }
        result
    }
}

impl std::fmt::Display for ToxicInstMem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut printable: String = String::from("");

        write!(f, "{}", printable)
    }
}
