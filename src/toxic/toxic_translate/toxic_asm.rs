use crate::toxic::toxic_imem::toxic_inst::ToxicInst;
use crate::toxic::toxic_imem::toxic_inst::ToxicInst::*;
use crate::toxic::toxic_translate::toxic_asm::ToxicAsm::*;
use crate::toxic::toxic_translate::toxic_asm::ToxicPseudoOpcode::*;
use std::fs;
use std::result::Result::{Err, Ok};
use clap::arg;
use crate::toxic::toxic_translate::trans_pseudo;

#[derive(Debug)]
pub enum ToxicPseudoOpcode {
    IMM4,
    HALT,
}

impl ToxicPseudoOpcode {
    fn from_str(s: String) -> Result<ToxicPseudoOpcode, String> {
        match s.as_str() {
            "IMM4" => Ok(IMM4),
            _ => Err(format!("Undefined Pseudo Code {}", s).to_string()),
        }
    }
}

#[derive(Debug)]
pub struct ToxicPseudo {
    opcode: ToxicPseudoOpcode,
    arguments: Vec<String>,
}

impl ToxicPseudo {
    fn imm4_gen(imm: u8) -> Result<Vec<ToxicInst>, String> {
        match imm {
            0 => Ok(vec![P0]),
            1 => Ok(vec![P1]),
            2 => Ok(vec![P1, LS]),
            3 => Ok(vec![P1, LS, P1, ADD]),
            4 => Ok(vec![P1, LS, LS]),
            5 => Ok(vec![P1, LS, LS, P1, ADD]),
            6 => Ok(vec![P1, LS, P1, ADD, LS]),
            7 => Ok(vec![P1, LS, LS, LS, NOT]),
            8 => Ok(vec![P1, LS, LS, LS]),
            9 => Ok(vec![P1, LS, LS, LS, P1, ADD]),
            10 => Ok(vec![P1, LS, LS, P1, ADD, LS]),
            11 => Ok(vec![P1, LS, LS, LS, NOT]),
            12 => Ok(vec![P1, LS, P1, ADD, LS, LS]),
            13 => Ok(vec![P1, LS, NOT]),
            14 => Ok(vec![P1, NOT]),
            15 => Ok(vec![P0, NOT]),
            _ => Err("Internal Error: Cannot convert not u4 to imm".to_string()),
        }
    }
}

#[derive(Debug)]
pub enum ToxicAsm {
    INST(ToxicInst),
    PSEUDO(ToxicPseudo),
}

impl ToxicAsm {
    pub fn from_str(str: String) -> Result<ToxicAsm, String> {
        // let str_smt = Box::new(str);
        match ToxicInst::from_str(str.clone()) {
            Ok(inst) => Ok(INST(inst)),
            Err(_) => {
                let str_lst = Vec::from_iter(str.split(" ").map(String::from));
                // println!("{:?}", str_lst);
                let opcode = ToxicPseudoOpcode::from_str(str_lst.get(0).unwrap().to_owned())
                    .expect("Failed to Parse Pseudo code");
                let pseudo = ToxicPseudo {
                    opcode,
                    arguments: str_lst[1..].to_vec(),
                };
                Ok(PSEUDO(pseudo))
            }
        }
    }

    pub fn load_prog_from_source(filename: String) -> Vec<ToxicAsm> {
        let contents = fs::read_to_string(filename).expect("Cannot Read Source file");
        let mut prog: Vec<ToxicAsm> = Vec::new();
        for line in contents.split("\n") {
            // let inst_str = line.replace(" ", "");
            if line.is_empty() {
                continue;
            }
            let inst_str = line.to_string();
            // println!("{}", inst_str);
            prog.push(match ToxicAsm::from_str(inst_str) {
                Ok(inst) => inst,
                Err(e) => std::panic!("{}", e),
            });
        }

        prog
    }

    fn translate_pseudo(pseudo: ToxicPseudo) -> Result<Vec<ToxicInst>, String> {
        match pseudo.opcode {
            IMM4 => {
                let imm_str: String = pseudo.arguments.get(0).unwrap().to_owned();
                let imm_val = imm_str.parse::<u8>().unwrap();
                ToxicPseudo::imm4_gen(imm_val)
            }
            HALT => Ok(vec![P0]),
        }
    }

    fn translate(asm: ToxicAsm) -> Result<Vec<ToxicInst>, String> {
        match asm {
            INST(i) => Ok(vec![i]),
            PSEUDO(pseudo) => ToxicAsm::translate_pseudo(pseudo)
            }
    }

    pub fn translate_program(prog: Vec<ToxicAsm>) -> Vec<ToxicInst> {
        let mut prog_inst: Vec<Vec<ToxicInst>> = Vec::new();
        for x in prog {
            match ToxicAsm::translate(x) {
                Ok(inst_seq) => prog_inst.push(inst_seq),
                Err(err) => panic!("Parsing Program Error: {}", err)
            }
        }
        // println!("{:?}", prog_inst);
        let mut prog_unravel: Vec<ToxicInst> = Vec::new();
        for x in prog_inst{
            prog_unravel.extend(x);
        }

        prog_unravel
    }
}
