extern crate clap;
extern crate elf;
extern crate regex;
#[macro_use]
extern crate lazy_static;
extern crate lib_mips_emu;

use clap::{Arg, App};

mod debugger;

use debugger::Debugger;
use lib_mips_emu::cpu::Cpu;

fn main() {
    let matches = App::new("MIPS emulator")
        .version("1.0")
        .author("Paul CACHEUX <paulcacheux@gmail.com>")
        .arg(Arg::with_name("INPUT")
             .help("Sets the input file to use.")
             .required_unless("debug")
             .index(1))
        .arg(Arg::with_name("debug")
             .help("Activate debugger.")
             .short("d")
             .long("debug"))
        .get_matches();

    
    let maybe_input_path = matches.value_of("INPUT");
    let mut cpu = Cpu::new();

    if matches.is_present("debug") {
        let mut debugger = Debugger::new(cpu);
        if let Some(path) = maybe_input_path {
            debugger.execute_command("load", vec![path]);
        }
        debugger.launch();
    } else {
        let path = matches.value_of("INPUT").unwrap();
        let elf_file = elf::File::open_path(path).expect("Can't read elf.");
        cpu.load_elf(elf_file).unwrap();
        cpu.continue_execution(false);
    }
}
