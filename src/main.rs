mod instruction;
mod types;

use instruction::*;
use std::env;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use types::*;

pub const MEMORY_SIZE: u64 = 1024 * 1024 * 128;

pub const REGISTER_NAMES: [&str; 32] = [
    "zero", "ra", "sp", "gp", "tp", "t0", "t1", "t2", "s0", "s1", "a0", "a1", "a2", "a3", "a4",
    "a5", "a6", "a7", "s2", "s3", "s4", "s5", "s6", "s7", "s8", "s9", "s10", "s11", "t3", "t4",
    "t5", "t6",
];

struct CPU {
    regfile: [u64; 32],
    pc: u64,
    dram: Vec<u8>,
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

    fn decode_load(&mut self, inst: u32) -> Instruction {
        //funct3
        match get_bits(inst, 14, 12) {
            0b000 => Instruction::LB(IType(inst)),
            0b001 => Instruction::LH(IType(inst)),
            0b010 => Instruction::LW(IType(inst)),
            0b100 => Instruction::LBU(IType(inst)),
            0b101 => Instruction::LWU(IType(inst)),
            0b101 => Instruction::LD(IType(inst)),
            _ => panic!("Instruction read as a LOAD but no matching funct3 found."),
        }
    }

    fn decode_op_imm(&mut self, inst: u32) -> Instruction {
        match get_bits(inst, 14, 12) {
            0b000 => Instruction::ADDI(IType(inst)),
            _ => panic!("Instruction was a OP IMM but no matching funct3 found."),
        }
    }

    fn decode_op(&mut self, inst: u32) -> Instruction {
        match get_bits(inst, 14, 12) {
            0b000 => Instruction::ADD(RType(inst)),
            _ => panic!("Instruction was a OP IMM but no matching funct3 found."),
        }
    }
    
    fn decode_store(&mut self, inst: u32) -> Instruction {
        match get_bits(inst, 14, 12) {
            0b000 => Instruction::SB(SType(inst)),
            0b001 => Instruction::SH(SType(inst)),
            0b010 => Instruction::SW(SType(inst)),
            0b011 => Instruction::SD(SType(inst)),
            _ => panic!("Instruction was a STORE but no matching funct3 found."),
        }
    }

    pub fn decode(&mut self, inst: u32) -> Instruction {
        //Shifting right 2 since all opcodes 2 LSBs are 1
        match inst >> 2 & 0b11111 {
            0b00000 => self.decode_load(inst),
            0b00100 => self.decode_op_imm(inst),
            0b01100 => self.decode_op(inst),
            0b01000 => self.decode_store(inst),
            _ => panic!("Decode not yet implemented for: {:#18x}", inst),
        }
    }

    //page 130 for instruction format
    fn execute(&mut self, inst: u32) {
        self.regfile[0] = 0;
        let decoded: Instruction = self.decode(inst);
        println!("EXECUTING: {:?}", decoded);
        match decoded {
            Instruction::ADDI(decoded) => {
                self.regfile[decoded.rd() as usize] =
                    self.regfile[decoded.rs1() as usize].wrapping_add(decoded.imm() as u64);
            }
            Instruction::ADD(decoded) => {
                self.regfile[decoded.rd() as usize] = self.regfile[decoded.rs1() as usize]
                    .wrapping_add(self.regfile[decoded.rs2() as usize]);
            }
            _ => {
                eprintln!("Execute not yet implemented for: {:?}", decoded);
            }
        }
        self.dump_registers();
    }

    pub fn dump_registers(&self) {
        for (regidx, reg) in self.regfile.iter().enumerate() {
            print!("x{:^4}\t {:#18x} \t", regidx, reg);
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
    //    cpu.dump_registers();

    Ok(())
}
