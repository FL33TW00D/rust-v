mod cpu;
mod instruction;
mod types;

use cpu::CPU;
use std::env;
use std::fs::File;
use std::io;
use std::io::prelude::*;

//TODO:
//1. Implement proper DRAM
//2. Implement status registers

pub const MEMORY_SIZE: u64 = 1024 * 1024 * 128;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        panic!("AHHHHH EVERYONE PANIC!");
    }
    let mut file = File::open(&args[1])?;
    let mut code = Vec::new();
    file.read_to_end(&mut code);

    let mut cpu = CPU::new(code);

    cpu.step();
    //    cpu.dump_registers();

    Ok(())
}
