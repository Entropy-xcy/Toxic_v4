use crate::toxic::toxic_imem::toxic_inst::ToxicInst;
use crate::toxic::toxic_translate::toxic_asm::ToxicAsm;
use crate::toxic::Toxic;

pub mod toxic_asm;

fn trans_pseudo(toxic_asm: ToxicAsm) -> Vec<ToxicInst> {
    match toxic_asm {
        ToxicAsm::INST(inst) => vec![inst],
        ToxicAsm::PSEUDO(pseudo_code) => vec![ToxicInst::LS],
    }
}
