use clap::Parser;
use std::fmt;
use std::fmt::{format, Debug, Formatter};
use std::fs::File;
use std::io::Write;
use Toxic_v4::toxic::toxic_imem::toxic_inst::ToxicInst;
use Toxic_v4::toxic::toxic_translate::toxic_asm::*;
use Toxic_v4::toxic::Toxic;
use Toxic_v4::*;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    name: String,
    #[clap(short, long)]
    output: Option<String>,
}

fn main() {
    let cli = Cli::parse();

    let input_filename = cli.name;
    let output_filename = cli.output.unwrap();
    let prog = ToxicAsm::load_prog_from_source(input_filename);
    let prog_seq = ToxicAsm::translate_program(prog);
    // println!("generated : {:?}", prog_seq);
    // println!("name: {:?}", output_filename);

    let mut outfile = File::create(output_filename).expect("Cannot Open File");
    for x in prog_seq{
        outfile.write(format!("{}\n", x.to_str()).as_bytes());
    }
}
