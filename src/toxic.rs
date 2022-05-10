pub mod toxic_imem;
pub mod toxic_dmem;

use std::fmt::{Debug};
use toxic_imem::*;
use toxic_imem::toxic_inst::*;
use toxic_dmem::*;


#[derive(Debug)]
pub struct Toxic {
    pub bus_width: u32,
    pub imem: ToxicInstMem,
    pub dmem: ToxicDataMem,
    pub pc: u32,
    pub pt: u32,
    pub sp: u32,
    pub carry: u8,
}

impl Toxic {
    pub fn new(bus_width: u32) -> Toxic {
        let imem = ToxicInstMem::new(bus_width);
        let pc_init = imem.main_address();
        Toxic {
            bus_width,
            carry: 0,
            dmem: ToxicDataMem::new(bus_width),
            imem,
            pc: pc_init,
            pt: 0,
            sp: u32::pow(2, bus_width) - 3,
        }
    }

    pub fn address_space(&self) -> u32 {
        u32::pow(2, self.bus_width)
    }

    pub fn u8_to_u4(x: u8) -> u8 {
        (x << 4) >> 4
    }

    pub fn push(&mut self, x: u8) {
        self.dmem.write(self.sp, Toxic::u8_to_u4(x));
        self.sp -= 1;
    }

    pub fn pop(&mut self) -> u8 {
        let out = self.dmem.read(self.sp + 1);
        if self.sp == (self.address_space() - 1) {} else { self.sp += 1 }
        // println!("Poping {}", out);
        Toxic::u8_to_u4(out)
    }

    pub fn i4_to_i8(i4: u8) -> i8{
        let i4 = Toxic::u8_to_u4(i4);
        if (i4 >> 3) == 0 {
            // Positive Offset
            i4 as i8
        } else {
            // Negative
            let not_plus_1 = (!i4 << 4 >> 4) + 1;
            -(not_plus_1 as i8)
        }
    }

    pub fn tos(&self) -> u8 {
        let out = self.dmem.read(self.sp + 1);
        Toxic::u8_to_u4(out)
    }

    pub fn ntos(&self) -> u8 {
        let out = self.dmem.read(self.sp + 2);
        Toxic::u8_to_u4(out)
    }

    pub fn stack_to_str(&self) -> String {
        let mut ret: String = String::from("------------\n\tDec\tBin");
        // println!("{}   {}", (self.address_space() - 1), self.sp);
        for i in self.sp + 1..=(self.address_space() - 3) {
            // println!("{} {}", i, self.sp);
            let tos_mark = if i-1 == self.sp {"*"} else {
                if i-2 == self.sp {"-"} else {""}
            };
            ret = format!("{}\n{}\t{}\t{:04b}", ret, tos_mark,
                          self.dmem.read(i), self.dmem.read(i))
        }
        ret = format!("{}\n------------", ret);
        ret
    }

    pub fn context_to_str(&self, pc_start: u32, pc_end: u32) -> String {
        let mut result: String = String::from("\tAddr\t\tInst");
        for pc in pc_start..pc_end{
            let pc_mark = if pc == self.pc {">>>"} else {""};
            result = format!("{}\n{}\t{:#06x}\t\t{}", result, pc_mark, pc, self.imem.decode(pc).to_str());
        }
        result
    }

    pub fn reg_to_str(&self) -> String{
        String::from(format!("PC:\t0x{:04x}\nPT:\t0x{:04x}\nSP:\t0x{:04x}", self.pc, self.pt, self.sp))
    }

    pub fn step(&mut self) {
        let inst = self.imem.decode(self.pc);
        println!("PC: {} Inst: {}", self.pc, inst);
        match inst {
            ToxicInst::LS => {
                let tos = self.pop();
                self.push(Toxic::u8_to_u4(tos * 2));
            }
            ToxicInst::RS => {
                let tos = self.pop();
                self.push(Toxic::u8_to_u4(tos / 2));
            }
            ToxicInst::BR => {
                if (self.tos() << 7 >> 7) == 1 {
                    self.pc = self.pt
                } else {}
            },
            ToxicInst::NOT => {
                let tos = self.pop();
                self.push(Toxic::u8_to_u4(!tos));
            }
            ToxicInst::P0 => self.push(0),
            ToxicInst::P1 => self.push(1),
            ToxicInst::GET => {
                let x = self.dmem.read(self.pt);
                self.push(x);
            },
            ToxicInst::CMP => {
                let a = self.tos();
                let b = self.ntos();
                // println!("{} {}", a, b);
                let gt = (a > b) as u8;
                let eq = (a == b) as u8;
                let lt = (a < b) as u8;
                println!("{} {} {} {} {}", a, b, lt, gt, eq);
                let y: u8 = (self.carry * 8) + lt * 4 + gt *2 + eq;
                assert!(y < 16);
                self.push(Toxic::u8_to_u4(y));
            }
            ToxicInst::ADR => {
                // First Step: Shift pt to left by 4 position and make sure not overflowing
                self.pt = (self.pt << 4) % self.address_space();
                // Second Step: Add PT with popped value
                self.pt += self.pop() as u32;
            },
            ToxicInst::OFF => {
                let offset = Toxic::i4_to_i8(self.pop());
                if offset.is_negative(){
                    self.pt += !(offset.wrapping_abs() as u32) + 1;
                } else{
                    self.pt += offset as u32;
                }
                self.pt = self.pt << (32-self.bus_width) >> (32 - self.bus_width);
            },
            ToxicInst::PC => {
                self.pt = self.pc;
            },
            ToxicInst::SP => {
                self.pt = self.sp + 1;
            },
            ToxicInst::ADD => {
                let a = self.pop();
                let b = self.pop();
                // println!("{} {}", a, b);
                let result_truncated = Toxic::u8_to_u4(a + b);
                let result = a + b;
                self.carry = if result > result_truncated { 1 } else { 0 };
                self.push(result_truncated);
            }
            ToxicInst::AND => {
                let a = self.pop();
                let b = self.pop();
                // println!("{} {}", a, b);
                self.push(Toxic::u8_to_u4(a & b));
            }
            ToxicInst::PUT => {
                let x = self.pop();
                self.dmem.write(self.pt, x)
            },
            ToxicInst::OR => {
                let a = self.pop();
                let b = self.pop();
                // println!("{} {}", a, b);
                self.push(Toxic::u8_to_u4(a | b));
            }
        };

        self.pc += 1;
    }

    pub fn step_inst_str(&mut self, inst_str: String) -> Result<(), String>{
        let inst_bin = ToxicInst::from_str(inst_str);
        match inst_bin {
            Ok(ib) => {
                self.imem.load(self.pc, vec![ib.to_bits()]);
                self.step();
                Ok(())
            }
            Err(e) => Err(e)
        }
    }

    pub fn mem_to_str(&self, begin_addr: u32, end_addr:u32) -> String {
        const ELEM_PER_LINE: u32 = 8;
        let mut ret = "Addr\t|\t".to_string();

        for i in 0..ELEM_PER_LINE{
            ret = format!("{}+{}\t", ret, i);
        }

        let begin_addr = (begin_addr / ELEM_PER_LINE) * ELEM_PER_LINE;
        let end_addr = if end_addr % ELEM_PER_LINE == 0 {end_addr} else
            {(end_addr / ELEM_PER_LINE + 1) * ELEM_PER_LINE};

        for addr in begin_addr..end_addr{
            let mark = if addr == self.pt {"*".to_string()} else {
                if addr == self.sp {"^".to_string()}
                else {"".to_string()}
            };
            ret = if addr % ELEM_PER_LINE == 0 {format!("{}\n{:#06x}\t|\t{:04b}{}", ret, addr, self.dmem.read(addr), mark)}
            else {format!("{}\t{:04b}{}", ret, self.dmem.read(addr), mark)}
        }
        ret
    }
}
