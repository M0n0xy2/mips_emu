extern crate elf;
extern crate regex;
#[macro_use]
extern crate lazy_static;

mod utils;
pub mod memory;
pub mod cpu;
pub mod instruction;
mod decoder;
mod executer;
mod syscall;
