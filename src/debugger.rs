use std::io::{self, Write};
use std::collections::HashMap;
use std::path::Path;

use elf;

use lib_mips_emu::cpu::Cpu;

pub struct Debugger {
    cpu: Cpu,
    log: bool,
    saved_cpu: Option<Cpu>,
}

impl Debugger {
    pub fn new(cpu: Cpu) -> Debugger {
        Debugger {
           cpu,
           log: false,
           saved_cpu: None
        }
    }

    pub fn launch(&mut self) {
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

            self.execute_command(cmd, args);
        }
    }

    pub fn execute_command(&mut self, cmd: &str, args: Vec<&str>) {
        let mut cmds: HashMap<&str, commands::Command> = HashMap::new();
        cmds.insert("help", commands::help);
        cmds.insert("load", commands::load);
        cmds.insert("restart", commands::restart);
        cmds.insert("registers", commands::registers);
        cmds.insert("r", commands::registers);
        cmds.insert("step", commands::step);
        cmds.insert("s", commands::step);
        cmds.insert("continue", commands::continue_cmd);
        cmds.insert("c", commands::continue_cmd);
        cmds.insert("print", commands::print);
        cmds.insert("log", commands::log);
        cmds.insert("breakpoint", commands::breakpoint);
        cmds.insert("b", commands::breakpoint);

        if let Some(cmd_func) = cmds.get(cmd) {
            if let Err(err) = cmd_func(self, args) {
                println!("Error: {}", err);
            }
        }
    }

    pub fn load<P: AsRef<Path>>(&mut self, path: P) -> Result<(), String> {
        let elf_file = elf::File::open_path(path).expect("Can't read elf.");
        self.cpu.load_elf(elf_file)?;
        self.saved_cpu = Some(self.cpu.clone());
        Ok(())
    }
}

mod commands {
    use std::collections::HashMap;
    use regex::Regex;
    use super::Debugger;
    use lib_mips_emu::cpu::Signal;

    macro_rules! expect_n_args {
        ($n:expr, $args:expr) => {
            let n = $n;
            let len = $args.len();
            if len != n {
                return Err(format!(
                    "Expected {} argument{} (given {})",
                    n,
                    if n > 1 { "s" } else { "" },
                    len
                ));
            }
        }
    }

    macro_rules! expect_max_n_args {
        ($n:expr, $args:expr) => {
            let n = $n;
            let len = $args.len();
            if len > n {
                return Err(format!(
                    "Expected at most {} argument{} (given {})",
                    n,
                    if n > 1 { "s" } else { "" },
                    len
                ));
            }
        }
    }

    lazy_static! {
        static ref REGISTER_DIRECT_REGEX: Regex = Regex::new(r"\$([0-9]{1,2})").unwrap();
        static ref REGISTER_ALIAS_REGEX: Regex = Regex::new(r"\$([a-zA-Z0-9]+)").unwrap();
        static ref MEMORY_REGEX: Regex = Regex::new(r"0x([a-fA-F0-9]{0,8})").unwrap();
        static ref REGISTER_ALIASES: HashMap<&'static str, u32> = {
            let mut map = HashMap::new();
            map.insert("zero", 0u32);
            map.insert("at", 1);
            map.insert("v0", 2);
            map.insert("v1", 3);
            map.insert("a0", 4);
            map.insert("a1", 5);
            map.insert("a2", 6);
            map.insert("a3", 7);
            map.insert("t0", 8);
            map.insert("t1", 9);
            map.insert("t2", 10);
            map.insert("t3", 11);
            map.insert("t4", 12);
            map.insert("t5", 13);
            map.insert("t6", 14);
            map.insert("t7", 15);
            map.insert("s0", 16);
            map.insert("s1", 17);
            map.insert("s2", 18);
            map.insert("s3", 19);
            map.insert("s4", 20);
            map.insert("s5", 21);
            map.insert("s6", 22);
            map.insert("s7", 23);
            map.insert("t8", 24);
            map.insert("t9", 25);
            map.insert("k0", 26);
            map.insert("k1", 27);
            map.insert("gp", 28);
            map.insert("sp", 29);
            map.insert("fp", 30);
            map.insert("ra", 31);
            map
        };
    }

    pub type Command = fn(dbg: &mut Debugger, args: Vec<&str>) -> Result<(), String>;

    pub fn help(_: &mut Debugger, _: Vec<&str>) -> Result<(), String> {
        println!("Debugger help:");
        println!("  load <path> - load elf file");
        println!("  restart - restart the current program");
        println!("  r[egisters] - print value of all registers");
        println!("  s[tep] - execute the next instruction");
        println!("  c[ontinue] - run the program until breakpoint/exit");
        println!("  b[reakpoint] - list breakpoints");
        println!("  b[reakpoint] 0xXXXXXXXX - add/remove breakpoint");
        println!("  print $XX - print register");
        println!("  print 0xXXXXXXXX - print memory byre");
        println!("  log [on|off] - (de)activate the execution logging");
        Ok(())
    }

    pub fn load(dbg: &mut Debugger, args: Vec<&str>) -> Result<(), String> {
        expect_n_args!(1, args);

        dbg.load(args[0])
    }

    pub fn restart(dbg: &mut Debugger, args: Vec<&str>) -> Result<(), String> {
        expect_n_args!(0, args);

        if let Some(cpu) = dbg.saved_cpu.clone() {
            dbg.cpu = cpu;
            Ok(())
        } else {
            Err("Restart impossible. No file has ever been loaded.".to_string())
        }
    }

    pub fn registers(dbg: &mut Debugger, args: Vec<&str>) -> Result<(), String> {
        expect_n_args!(0, args);

        println!("pc = {:#010x}", dbg.cpu.pc);
        println!("hi = {:#010x}", dbg.cpu.hi);
        println!("lo = {:#010x}", dbg.cpu.lo);
        for i in 0..32 {
            println!("${} = {:#010x}", i, dbg.cpu.get_register(i));
        }
        Ok(())
    }

    pub fn step(dbg: &mut Debugger, args: Vec<&str>) -> Result<(), String> {
        expect_max_n_args!(1, args);

        let n = if args.len() == 0 {
            1
        } else {
            if let Ok(n) = args[0].parse::<usize>() {
                n
            } else {
                return Err(format!("Can't parse {}.", args[0]));
            }
        };

        for _ in 0..n {
            if let Some(signal) = dbg.cpu.run(true, dbg.log) {
                println!("{}", signal);
                match signal {
                    Signal::Exit => break,
                    _ => {}
                }
            }
        }
        
        Ok(())
    }

    pub fn continue_cmd(dbg: &mut Debugger, args: Vec<&str>) -> Result<(), String> {
        expect_n_args!(0, args);

        if let Some(signal) = dbg.cpu.run(false, dbg.log) {
            println!("{}", signal);
        }
        Ok(())
    }

    pub fn breakpoint(dbg: &mut Debugger, args: Vec<&str>) -> Result<(), String> {
        expect_max_n_args!(1, args);

        if args.len() == 0 {
            println!("breakpoints:");
            for bp in &dbg.cpu.breakpoints {
                println!("  {}", bp);
            }
        } else {
            if let Some(capt) = MEMORY_REGEX.captures(args[0]) {
                let pc = u32::from_str_radix(&capt[1], 16).unwrap();
                dbg.cpu.add_or_remove_breakpoint(pc);
            } else {
                return Err("Can't parse the address.".to_string());
            }
        }
        Ok(())
    }

    pub fn print(dbg: &mut Debugger, args: Vec<&str>) -> Result<(), String> {
        expect_n_args!(1, args);

        let arg = args[0];
        if let Some(capt) = REGISTER_DIRECT_REGEX.captures(arg) {
            let reg_str = &capt[1];
            let reg_id = reg_str.parse().unwrap();
            println!("${} = {:#x}", reg_str, dbg.cpu.get_register(reg_id));
        } else if let Some(capt) = REGISTER_ALIAS_REGEX.captures(arg) {
            let alias = &capt[1];
            if let Some(&reg) = REGISTER_ALIASES.get(alias) {
                println!("${} = {:#x}", alias, dbg.cpu.get_register(reg));
            } else if alias == "pc" {
                println!("$pc = {:#x}", dbg.cpu.pc);
            } else if alias == "hi" {
                println!("$hi = {:#x}", dbg.cpu.hi);
            } else if alias == "lo" {
                println!("$lo = {:#x}", dbg.cpu.lo);
            } else {
                return Err("This alias is not defined.".to_string());
            }
        } else if let Some(capt) = MEMORY_REGEX.captures(arg) {
            let mem_str = &capt[1];
            let mem_id = u32::from_str_radix(mem_str, 16).unwrap();
            println!("memory[{:#x}] = {:#x}", mem_id, dbg.cpu.memory.get_byte(mem_id));
        } else {
            return Err(format!("Can't read {}.", arg));
        }
        Ok(())
    }

    pub fn log(dbg: &mut Debugger, args: Vec<&str>) -> Result<(), String> {
        expect_n_args!(1, args);

        dbg.log = match args[0] {
            "on" => true,
            "off" => false,
            _ => return Err("Unrecognized argument.".to_string())
        };

        Ok(())
    }
}
