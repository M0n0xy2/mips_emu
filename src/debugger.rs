use std::io::{self, Write};
use std::collections::HashMap;
use runner::Runner;

pub struct Debugger {
    runner: Runner,
}

impl Debugger {
    pub fn new(runner: Runner) -> Debugger {
        Debugger {
            runner,
        }
    }

    pub fn launch(&mut self) {
        let mut cmds: HashMap<&str, commands::Command> = HashMap::new();
        cmds.insert("help", commands::help);
        cmds.insert("registers", commands::registers);
        cmds.insert("step", commands::step);
        cmds.insert("continue", commands::continue_cmd);

        loop {
            print!("dbg> ");
            io::stdout().flush().unwrap();
            let mut line = String::new();
            io::stdin().read_line(&mut line).expect("Stdin error.");
            
            let mut args_iter = line.trim().split(' ');
            let (cmd, args) = if let Some(cmd) = args_iter.next() {
                (cmd, args_iter.filter(|s| !s.is_empty()).collect::<Vec<_>>())
            } else {
                continue
            };

            if cmd == "exit" {
                break
            }

            if let Some(cmd_func) = cmds.get(cmd) {
                if let Err(err) = cmd_func(self, args) {
                    println!("Error: {}", err);
                }
            }
        }
    }
}

mod commands {
    use super::Debugger;

    pub type Command = fn(dbg: &mut Debugger, args: Vec<&str>) -> Result<(), String>;

    pub fn help(dbg: &mut Debugger, args: Vec<&str>) -> Result<(), String> {
        println!("Debugger help:");
        println!("registers - print value of all registers");
        println!("step - execute the next instruction");
        println!("continue - run the program until breakpoint/exit");
        Ok(())
    }

    pub fn registers(dbg: &mut Debugger, args: Vec<&str>) -> Result<(), String> {
        println!("pc = {:#010x}", dbg.runner.pc);
        println!("hi = {:#010x}", dbg.runner.machine.hi);
        println!("lo = {:#010x}", dbg.runner.machine.lo);
        for i in 0..32 {
            println!("${} = {:#010x}", i, dbg.runner.machine.get_register(i));
        }
        Ok(())
    }

    pub fn step(dbg: &mut Debugger, args: Vec<&str>) -> Result<(), String> {
        if !dbg.runner.step() {
            println!("Program has ended.");
        }
        Ok(())
    }

    pub fn continue_cmd(dbg: &mut Debugger, args: Vec<&str>) -> Result<(), String> {
        dbg.runner.run();
        println!("Program has ended.");
        Ok(())
    }
}
