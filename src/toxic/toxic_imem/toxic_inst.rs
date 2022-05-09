use std::fmt;
use std::fmt::{format, Debug, Formatter};


pub enum ToxicInst {
    LS,
    RS,
    NOT,
    BR,
    P0,
    P1,
    CMP,
    GET,
    ADD,
    AND,
    OR,
    PUT,
    ADR,
    OFF,
    SP,
    PC,
}

impl ToxicInst {
    pub fn from_bits(bits: u8) -> Result<ToxicInst, String> {
        match bits {
            0 => Ok(ToxicInst::LS),
            1 => Ok(ToxicInst::RS),
            2 => Ok(ToxicInst::BR),
            3 => Ok(ToxicInst::NOT),
            4 => Ok(ToxicInst::P0),
            5 => Ok(ToxicInst::P1),
            6 => Ok(ToxicInst::GET),
            7 => Ok(ToxicInst::CMP),
            8 => Ok(ToxicInst::ADR),
            9 => Ok(ToxicInst::OFF),
            10 => Ok(ToxicInst::PC),
            11 => Ok(ToxicInst::SP),
            12 => Ok(ToxicInst::ADD),
            13 => Ok(ToxicInst::AND),
            14 => Ok(ToxicInst::PUT),
            15 => Ok(ToxicInst::OR),
            _ => Err(format!("Invalid Instruction Bits: {}", bits)),
        }
    }

    pub fn from_str(inst_str: String) -> Result<ToxicInst, String> {
        match inst_str.as_str() {
            "LS" => Ok(ToxicInst::LS),
            "RS" => Ok(ToxicInst::RS),
            "BR" => Ok(ToxicInst::BR),
            "NOT" => Ok(ToxicInst::NOT),
            "P0" => Ok(ToxicInst::P0),
            "P1" => Ok(ToxicInst::P1),
            "GET" => Ok(ToxicInst::GET),
            "CMP" => Ok(ToxicInst::CMP),
            "ADR" => Ok(ToxicInst::ADR),
            "OFF" => Ok(ToxicInst::OFF),
            "PC" => Ok(ToxicInst::PC),
            "SP" => Ok(ToxicInst::SP),
            "ADD" => Ok(ToxicInst::ADD),
            "AND" => Ok(ToxicInst::AND),
            "PUT" => Ok(ToxicInst::PUT),
            "OR" => Ok(ToxicInst::OR),
            _ => Err(format!("Invalid Instruction: {}", inst_str)),
        }
    }

    pub fn to_str(&self) -> String {
        match self {
            ToxicInst::LS => String::from("LS"),
            ToxicInst::RS => String::from("RS"),
            ToxicInst::BR => String::from("BR"),
            ToxicInst::NOT => String::from("NOT"),
            ToxicInst::P0 => String::from("P0"),
            ToxicInst::P1 => String::from("P1 "),
            ToxicInst::GET => String::from("GET"),
            ToxicInst::CMP => String::from("CMP"),
            ToxicInst::ADR => String::from("ADR"),
            ToxicInst::OFF => String::from("OFF"),
            ToxicInst::PC => String::from("PC"),
            ToxicInst::SP => String::from("SP"),
            ToxicInst::ADD => String::from("ADD"),
            ToxicInst::AND => String::from("AND"),
            ToxicInst::PUT => String::from("PUT"),
            ToxicInst::OR => String::from("OR"),
        }
    }

    pub fn to_bits(&self) -> u8 {
        match self {
            ToxicInst::LS => 0,
            ToxicInst::RS => 1,
            ToxicInst::BR => 2,
            ToxicInst::NOT => 3,
            ToxicInst::P0 => 4,
            ToxicInst::P1 => 5,
            ToxicInst::GET => 6,
            ToxicInst::CMP => 7,
            ToxicInst::ADR => 8,
            ToxicInst::OFF => 9,
            ToxicInst::PC => 10,
            ToxicInst::SP => 11,
            ToxicInst::ADD => 12,
            ToxicInst::AND => 13,
            ToxicInst::PUT => 14,
            ToxicInst::OR => 15,
        }
    }
}

impl std::fmt::Display for ToxicInst {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let printable = self.to_str();
        write!(f, "{}", printable)
    }
}
