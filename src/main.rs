use std::io;
use std::io::prelude::*;
pub const MEMORY_SIZE: u64 = 1024 * 1024 * 128;

pub const REGISTER_NAMES: [&str; 32] = [
    "zero", "ra", "sp", "gp", "tp", "t0", "t1", "t2", "s0", "s1", "a0", "a1", "a2", "a3", "a4",
    "a5", "a6", "a7", "s2", "s3", "s4", "s5", "s6", "s7", "s8", "s9", "s10", "s11", "t3", "t4",
    "t5", "t6",
];

pub enum OpCode {
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
    SRAW
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
    SRAW
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

struct Instruction(Opcode,)


struct CPU {
    regfile: [u64; 32],
    pc: u64,
    dram: Vec<u8>
}

pub fn get_bits(inst: u32, start: u32, end: u32) -> u32 {
    (inst >> end) & ((1 << (start - end + 1)) - 1)
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

    fn fetch(&self) -> u32 {
        
    }

    //page 130 for instruction format
    fn execute(&self, inst: u32) {
        let opcode = get_bits(inst, 0, 6);
        let rd = get_bits(inst, 7, 12) as usize;
        let rs1 = get_bits(inst, 15, 20) as usize;
        let rs2 = get_bits(inst, 20,25) as usize;

        self.regs[0] = 0;

        println!("OPCODE: {:?}", opcode);
        println!("RD: {:?}", rd);
        println!("RS1: {:?}", rs1);
        println!("RS2: {:?}", rs2);

        match opcode {
            OpCode::ADDI => {
                let imm = get_bits(inst, 20, 31) >> 20;
                self.regs[rs1].wrapping_add(imm); 
            }
            OpCode::ADD => {self.regs[rd] = self.regs[rs1].wrapping_add(self.regs[rs2]);}
        }

    }

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
    let cpu = CPU::new();
    cpu.dump_registers();

    Ok(())
}
