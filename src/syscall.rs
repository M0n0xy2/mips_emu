use std::io::{self, Read, Write};

use machine::Machine;
use instruction::PCOperation;
use utils;

pub fn call_syscall(machine: &mut Machine) -> PCOperation {
    let syscall_value = machine.get_register(2);
    match syscall_value {
        1 => print_int(machine),
        4 => print_string(machine),
        5 => read_int(machine),
        8 => read_string(machine),
        10 => return PCOperation::Exit,
        _ => panic!("Wrong syscall number"),
    }
    PCOperation::Offset(4)
}

fn print_int(machine: &mut Machine) {
    let i = utils::u2i(machine.get_register(4));
    print!("{}", i);
}

fn print_string(machine: &mut Machine) {
    let mut addr = machine.get_register(4);
    let mut buff = String::new();
    while machine.data[addr as usize] != 0 {
        buff.push(machine.data[addr as usize] as char);
        addr += 1;
    }
    print!("{}", buff);
    io::stdout().flush().unwrap();
}

fn read_int(machine: &mut Machine) {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();

    let i : i32 = line.trim().parse().unwrap();
    machine.set_register(2, utils::i2u(i));
}

fn read_string(machine: &mut Machine) {
    // really not sure about this implementation
    let mut addr = machine.get_register(4) as usize;
    let len = machine.get_register(5) as usize;
    let stdin = io::stdin();

    for c in stdin
        .lock()
        .bytes()
        .take(len - 1)
        .map(Result::unwrap)
        .take_while(|c| *c != b'\n') {
        machine.data[addr] = c; 
        addr += 1;
    }
    machine.data[addr] = 0;
}
