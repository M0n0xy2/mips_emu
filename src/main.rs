extern crate elf;
extern crate clap;
extern crate regex;
#[macro_use]
extern crate lazy_static;

use clap::{Arg, App};

mod utils;
mod memory;
mod cpu;
mod instruction;
mod decoder;
mod executer;
mod syscall;
mod debugger;

use debugger::Debugger;
use cpu::Cpu;

fn main() {
    let matches = App::new("MIPS emulator")
        .version("1.0")
        .author("Paul CACHEUX <paulcacheux@gmail.com>")
        .arg(Arg::with_name("INPUT")
             .help("Sets the input file to use.")
             .required(true)
             .index(1))
        .arg(Arg::with_name("debug")
             .help("Activate debugger.")
             .short("d")
             .long("debug"))
        .get_matches();

    let path = matches.value_of("INPUT").unwrap();
    
    let mut cpu = Cpu::new();
    let elf_file = elf::File::open_path(path).expect("Can't read elf.");

    cpu.load_elf(elf_file).unwrap();

    if matches.is_present("debug") {
        let mut debugger = Debugger::new(cpu);
        debugger.launch();
    } else {
        cpu.run(false);
    }
}
