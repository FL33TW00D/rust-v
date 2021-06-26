use std::env;
use std::fs::File;
use std::io;
use std::io::prelude::*;

pub const MEMORY_SIZE: u64 = 1024 * 1024 * 128;

pub const REGISTER_NAMES: [&str; 32] = [
    "zero", "ra", "sp", "gp", "tp", "t0", "t1", "t2", "s0", "s1", "a0", "a1", "a2", "a3", "a4",
    "a5", "a6", "a7", "s2", "s3", "s4", "s5", "s6", "s7", "s8", "s9", "s10", "s11", "t3", "t4",
    "t5", "t6",
];

pub enum Instruction {
    //RV32I
    LUI,
    AUIPC,
    JAL,
    JALR,
    BEQ,
    BNE,
    BLT,
    BGE,
    BLTU,
    BGEU,
    LB,
    LH,
    LW,
    LBU,
    LHU,
    SB,
    SH,
    SW,
    ADDI{
        imm,
        rs1,
        rd,
        opcode
    },
    SLTI,
    SLTIU,
    XORI,
    ORI,
    ADNI,
    SLLI,
    SRLI,
    SRAI,
    ADD{
        rs2,
        rs1,
        rd,
        opcode
    },
    SUB,
    SLL,
    SLT,
    SLTU,
    XOR,
    SRL,
    SRA,
    OR,
    AND,
    FENCE,
    ECALL,
    EBREAK,

    //RV64I
    LWU,
    LD,
    SD,
    ADDIW,
    SLLIW,
    SRLIW,
    SRAIW,
    ADDW,
    SUBW,
    SLLW,
    SRLW,
    SRAW,
}

pub enum Funct3 {
    JAL,
    JALR,
    BEQ,
    BNE,
    BLT,
    BGE,
    BLTU,
    BGEU,
    LB,
    LH,
    LW,
    LBU,
    LHU,
    SB,
    SH,
    SW,
    ADDI,
    SLTI,
    SLTIU,
    XORI,
    ORI,
    ADNI,
    SLLI,
    SRLI,
    SRAI,
    ADD,
    SUB,
    SLL,
    SLT,
    SLTU,
    XOR,
    SRL,
    SRA,
    OR,
    AND,
    FENCE,
    ECALL,
    EBREAK,

    //RV64I
    LWU,
    LD,
    SD,
    ADDIW,
    SLLIW,
    SRLIW,
    SRAIW,
    ADDW,
    SUBW,
    SLLW,
    SRLW,
    SRAW,
}

pub enum Funct7 {
    SLLI,
    SRLI,
    SRAI,
    SLLIW,
    SRLIW,
    SRAIW,
    ADDW,
    SUBW,
    SLLW,
    SRLW,
    SRAW,
}


struct CPU {
    regfile: [u64; 32],
    pc: u64,
    dram: Vec<u8>,
}

pub fn get_bits(inst: u32, start: u32, end: u32) -> u32 {
    (inst >> end) & ((1 << (start - end + 1)) - 1)
}

fn step(cpu: &mut CPU) {
    while cpu.pc < cpu.dram.len() as u64 {
        let inst = cpu.fetch();
        cpu.pc += 4;
        cpu.execute(inst);
    }
}

impl CPU {
    fn new(code: Vec<u8>) -> CPU {
        let mut regfile = [0; 32];
        let memory_size = 1024 * 1024 * 128;
        regfile[2] = memory_size;

        CPU {
            regfile,
            pc: 0,
            dram: code,
        }
    }

    fn fetch(&self) -> u32 {
        let index = self.pc as usize;
        return (self.dram[index] as u32)
            | ((self.dram[index + 1] as u32) << 8)
            | ((self.dram[index + 2] as u32) << 16)
            | ((self.dram[index + 3] as u32) << 24);
    }

    pub fn decode(&mut self, inst: u32) -> Instruction {
        //called decode for now but is actually executing, crazy right
        let opcode = get_bits(inst, 6, 0);
        let rd = get_bits(inst, 11, 7);
        let rs1 = get_bits(inst, 19, 15) as usize;
        let rs2 = get_bits(inst, 24, 20) as usize;

        match opcode {
            0x13 => { 
                let imm = ((inst & 0xfff00000) as i32 as i64 >> 20);
                Instruction::ADDI(imm, rs1, rd, opcode) 
            },
            0x33 => { 
                Instruction::ADD(rs2, rs1,)
            },
        }
    }

    //page 130 for instruction format
    fn execute(&mut self, inst: u32) {
        self.regfile[0] = 0;
        let decoded: Instruction = self.decode(inst);
        match decoded {
            Instruction::ADDI => {
                self.regfile[rd] = self.regfile[rs1].wrapping_add(imm as u64);
            }
            Instruction::ADD => {
                self.regfile[rd] = self.regfile[rs1].wrapping_add(self.regfile[rs2]);
            }
            _ => {
                eprintln!("Not yet implemented: {:#x}", opcode);
            }
        }
    }

    pub fn dump_registers(&self) {
        for (regidx, reg) in self.regfile.iter().enumerate() {
            print!("{:^4}\t {:#18x} \t", REGISTER_NAMES[regidx], reg);
            if (regidx + 1) % 4 == 0 {
                println!("")
            }
        }
    }
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        panic!("AHHHHH EVERYONE PANIC!");
    }
    let mut file = File::open(&args[1])?;
    let mut code = Vec::new();
    file.read_to_end(&mut code);

    let mut cpu = CPU::new(code);

    step(&mut cpu);
    cpu.dump_registers();

    Ok(())
}
