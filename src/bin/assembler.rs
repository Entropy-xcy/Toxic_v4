use std::fmt;
use std::fmt::{format, Debug, Formatter};
use Toxic_v4::*;
use Toxic_v4::toxic::toxic_imem::toxic_inst::ToxicInst;
use Toxic_v4::toxic::Toxic;

fn main() {
    let mut toxic = Toxic::new(8);
    toxic.imem.init_from_source(String::from("test.asm"));
    println!("{}", toxic.imem.to_str(toxic.imem.main_address(),
                                     toxic.imem.main_address() + 10));
}
