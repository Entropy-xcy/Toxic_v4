use Toxic_v4::toxic::Toxic;

extern crate shrust;
use shrust::{Shell, ShellIO};
use std::io::prelude::*;

fn main() {
    let toxic = Toxic::new(8);
    // toxic.imem.load_from_source(String::from("test.asm"));
    // toxic.step();
    // println!("{}", toxic.stack_to_str());
    let mut shell = Shell::new(toxic);

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


    shell.new_command("step", "Execute by One step", 0,
                      |io, toxic, s| {
                          // writeln!(io, "Executing {}", s[0]);
                          toxic.step();
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
                          writeln!(io, "{}", toxic.context_to_str(toxic.pc-2, toxic.pc + 5));
                          // v.push(s[0].to_string());
                          Ok(())
                      });

    shell.new_command("reg", "Display Registers", 0,
                      |io, toxic, s| {
                          writeln!(io, "{}", toxic.reg_to_str());
                          // v.push(s[0].to_string());
                          Ok(())
                      });

    shell.new_command("load", "Load from Assembly file", 1,
                      |io, toxic, s| {
                          let res = toxic.imem.load_from_source(String::from(s[0]),
                                                                toxic.pc);
                          match res {
                              Ok(_) => (),
                              Err(e) => {writeln!(io, "Error: {}", e); ()}
                          }

                          Ok(())
                      });

    shell.new_command("mem", "Show memory content", 2,
                      |io, toxic, s| {
                          let begin_str = s[0].trim_start_matches("0x");
                          let end_str = s[1].trim_start_matches("0x");
                          let begin_idx: u32 = match i32::from_str_radix(begin_str, 16){
                              Ok(i) => i,
                              Err(_) => {writeln!(io, "Please Enter a valid Hex Decimal Address"); 0}
                          } as u32;

                          let end_idx: u32 = match i32::from_str_radix(end_str, 16){
                              Ok(i) => i,
                              Err(_) => {writeln!(io, "Please Enter a valid Hex Decimal Address"); 0}
                          } as u32;

                          let ret = toxic.mem_to_str(begin_idx, end_idx);
                          writeln!(io, "{}", ret);

                          Ok(())
                      });

    shell.run_loop(&mut ShellIO::default());
}
