use std::io;
use std::io::prelude::*;
pub const MEMORY_SIZE: u64 = 1024 * 1024 * 128;

pub const REGISTER_NAMES: [&str; 32] = [
    "zero", "ra", "sp", "gp", "tp", "t0", "t1", "t2", "s0", "s1", "a0", "a1", "a2", "a3", "a4",
    "a5", "a6", "a7", "s2", "s3", "s4", "s5", "s6", "s7", "s8", "s9", "s10", "s11", "t3", "t4",
    "t5", "t6",
];

pub enum INST {
    ///U-type
    LUI = 0x37,
    LO

    AUIPC,
    BRANCH,
    JAL,
    JALR,

    IMM,
    OP,

    MISC,
    SYSTEM,
}
struct CPU {
    regfile: [u64; 32],
    pc: u64,
}

fn step(cpu: CPU) {
    while cpu.pc < cpu.dram.len() as u64 {
        let ins = cpu.fetch();
        cpu.pc += 4;
        cpu.execute(inst);
    }
}

impl CPU {
    fn new() -> CPU {
        let mut regfile = [0; 32];
        let memory_size = 1024 * 1024 * 128;
        regfile[2] = memory_size;

        CPU { regfile, pc: 0 }
    }

    fn get_bits(inst: u32) {}

    fn fetch(&self) {}

    fn execute(&self) {}

    pub fn dump_registers(&self) {
        for (regidx, reg) in self.regfile.iter().enumerate() {
            print!("{:^4}\t {:^09} \t", REGISTER_NAMES[regidx], reg);
            if (regidx + 1) % 4 == 0 {
                println!("")
            }
        }
    }
}

fn main() -> io::Result<()> {
    ///Here need to read in all of the instruction tests
    /// perhaps with an elf crate
    /// Need to read in all the tests from the risc-v folder
    /// Need a CPU struct to hold all the registers
    /// How many registers does the CPU have?
    /// How does memory work on the CPU
    /// IF, ID, EX, MEM, WB
    ///
    /// Need to read the files in, might be an ELF crate
    /// dump_registers (pp)
    /// fetch function
    /// execute function
    let cpu = CPU::new();
    cpu.dump_registers();

    Ok(())
}
