use std::fmt;
use std::fmt::{format, Debug, Formatter};
use Toxic_v4::*;
use Toxic_v4::toxic::toxic_imem::toxic_inst::ToxicInst;
use Toxic_v4::toxic::Toxic;

extern crate shrust;
use shrust::{Shell, ShellIO};
use std::io::prelude::*;

fn main() {
    let mut toxic = Toxic::new(8);
    // toxic.imem.load_from_source(String::from("test.asm"));
    // toxic.step();
    // println!("{}", toxic.stack_to_str());
    let mut shell = Shell::new(toxic);
    shell.new_command_noargs("hello", "Say 'hello' to the world", |io, _| {
        writeln!(io, "Hello World !!!")?;
        Ok(())
    });

    shell.new_command("exec", "Execute Command", 1,
                      |io, toxic, s| {
                          // writeln!(io, "Executing {}", s[0]);
                          let res = toxic.step_inst_str(String::from(s[0]));
                          match res {
                              Ok(_) => (),
                              Err(e) => {writeln!(io, "Error: {}", e); ()}
                          }
                          Ok(())
    });

    shell.new_command("stack", "Display stack content", 0,
                      |io, toxic, s| {
                          writeln!(io, "{}", toxic.stack_to_str());
                          // v.push(s[0].to_string());
                          Ok(())
                      });

    shell.new_command("context", "Display Program Context", 0,
                      |io, toxic, s| {
                          writeln!(io, "{}", toxic.context_to_str(toxic.pc-2, toxic.pc + 10));
                          // v.push(s[0].to_string());
                          Ok(())
                      });

    shell.new_command("reg", "Display Registers", 0,
                      |io, toxic, s| {
                          writeln!(io, "{}", toxic.reg_to_str());
                          // v.push(s[0].to_string());
                          Ok(())
                      });

    shell.run_loop(&mut ShellIO::default());
}
