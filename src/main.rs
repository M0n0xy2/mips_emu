extern crate elf;
extern crate clap;

use clap::{Arg, App};

mod utils;
mod memory;
mod machine;
mod runner;
mod instruction;
mod decoder;
mod executer;
mod syscall;
mod debugger;

use runner::Runner;
use debugger::Debugger;

fn main() {
    let matches = App::new("MIPS emulator")
        .version("1.0")
        .author("Paul CACHEUX <paulcacheux@gmail.com>")
        .arg(Arg::with_name("INPUT")
             .help("Sets the input file to use.")
             .required(true)
             .index(1))
        .arg(Arg::with_name("mode")
             .help("Choose the mode (run, debug). Defaults to run.")
             .short("m")
             .long("mode")
             .takes_value(true)
             .possible_values(&["run", "debug"]))
        .get_matches();

    let path = matches.value_of("INPUT").unwrap();
    
    let (machine, entry) = machine::from_elf(path).expect("Can't read elf.");
    let mut runner = Runner::new(machine, entry);

    match matches.value_of("mode") {
        Some("debug") => {
            let mut debugger = Debugger::new(runner);
            debugger.launch();
        },
        _ => {
            runner.run();
        }
    }
}
