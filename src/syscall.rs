use std::io::{self, Read, Write};

use cpu::{Cpu, Signal, PCOperation};
use utils;

pub fn call_syscall(cpu: &mut Cpu) -> Result<PCOperation, Signal> {
    let syscall_value = cpu.get_register(2);
    match syscall_value {
        1 => print_int(cpu),
        4 => print_string(cpu),
        5 => read_int(cpu),
        8 => read_string(cpu),
        10 => return Err(Signal::Exit),
        _ => panic!("Wrong syscall number"),
    }
    Ok(PCOperation::Offset(4))
}

fn print_int(cpu: &mut Cpu) {
    let i = utils::u2i(cpu.get_register(4));
    print!("{}", i);
}

fn print_string(cpu: &mut Cpu) {
    let mut addr = cpu.get_register(4);
    let mut buff = String::new();
    while cpu.memory.get_byte(addr) != 0 {
        buff.push(cpu.memory.get_byte(addr) as char);
        addr += 1;
    }
    print!("{}", buff);
    io::stdout().flush().unwrap();
}

fn read_int(cpu: &mut Cpu) {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();

    let i : i32 = line.trim().parse().unwrap();
    cpu.set_register(2, utils::i2u(i));
}

fn read_string(cpu: &mut Cpu) {
    // really not sure about this implementation
    let mut addr = cpu.get_register(4);
    let len = cpu.get_register(5) as usize;
    let stdin = io::stdin();

    for c in stdin
        .lock()
        .bytes()
        .take(len - 1)
        .map(Result::unwrap)
        .take_while(|c| *c != b'\n') {
        cpu.memory.set_byte(addr, c);
        addr += 1;
    }
    cpu.memory.set_byte(addr, 0);
}
