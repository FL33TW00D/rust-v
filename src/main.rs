use std::io;
use std::io::prelude::*;
pub const MEMORY_SIZE: u64 = 1024 * 1024 * 128;

pub const REGISTER_NAMES: [&str; 32] = [
    "zero", "ra", "sp", "gp", "tp", "t0", "t1", "t2", "s0", "s1", "a0", "a1", "a2", "a3", "a4",
    "a5", "a6", "a7", "s2", "s3", "s4", "s5", "s6", "s7", "s8", "s9", "s10", "s11", "t3", "t4",
    "t5", "t6",
];

#[cfg_eval(rustfmt, rustfmt_skip)]
pub enum OpCode {
    LUI = 0x37,
    AUIPC = 0x17,
    JAL = 0x6F,
    JALR = 0x67,
    BRANCH = 0x63,
    LOAD = 0x03,
    STORE = 0x23,
    IMM = 0x13,
    OP = 0x33,
    FENCE = 0x0F,
    EX = 0x73,
}

#[cfg_eval(rustfmt, rustfmt_skip)]
pub enum Funct3 {
    ADD = 0x0,
    SUB = 0x0,
    ADDI = 0x0,
    SLLI = 0x1,
    SLT = 0x2,
    SLTI = 0x2,
    SLTU = 0x3,
    SLTUI = 0x3,
    
    XOR = 0x4,
    XORI = 0x4,
    
    SRL = 0x5,
    SRLI = 0x5,
    SRA = 0x5,
    SRAI = 0x5,
    
    OR = 0x6,
    ORI = 0x6,
    AND = 0x7,
    ANDI = 0x7,
    
    BEQ = 0x0,
    BNE = 0x1,
    BLT = 0x4,
    BGE = 0x5,
    BLTU = 0x6,
    BGEU = 0x7,

    LB = 0x0,
    SB = 0x0,
    LH = 0x1,
    SH = 0x1,
    LW = 0x2,
    SW = 0x2,
    LBU = 0x4,
    LHU = 0x5,

}

#[cfg_eval(rustfmt, rustfmt_skip)]
pub enum Funct7 {
    SLLI = 0x00,
    SRLI = 0x00,
    SRAI = 0x10,
    SLLIW = 0x00,
    SRLIW = 0x00,
    SRAIW = 0x10,
    ADDW = 0x00,
    SUBW = 0x20,
    SLLW = 0x00,
    SRLW = 0x00,
    SRAW = 0x20,
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

    fn fetch(&self) -> Result<u64> {}

    fn execute(&self, inst: u64) {}

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
