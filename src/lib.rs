extern crate elf;
extern crate regex;
#[macro_use]
extern crate lazy_static;

pub mod utils;
pub mod memory;
pub mod cpu;
pub mod instruction;
pub mod decoder;
pub mod executer;
pub mod syscall;
pub mod debugger;
