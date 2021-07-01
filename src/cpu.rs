use crate::instruction::*;
use crate::types::*;

pub const REGISTER_NAMES: [&str; 32] = [
    "zero", "ra", "sp", "gp", "tp", "t0", "t1", "t2", "s0", "s1", "a0", "a1", "a2", "a3", "a4",
    "a5", "a6", "a7", "s2", "s3", "s4", "s5", "s6", "s7", "s8", "s9", "s10", "s11", "t3", "t4",
    "t5", "t6",
];

pub struct CPU {
    regfile: [u64; 32],
    pc: u64,
    dram: Vec<u8>,
}

impl CPU {
    pub fn new(code: Vec<u8>) -> CPU {
        let mut regfile = [0; 32];
        let memory_size = 1024 * 1024 * 128;
        regfile[2] = memory_size;

        CPU {
            regfile,
            pc: 0,
            dram: code,
        }
    }

    pub fn step(&mut self) {
        while self.pc < self.dram.len() as u64 {
            let inst = self.fetch();
            self.pc += 4;
            self.execute(inst);
        }
    }

    fn fetch(&self) -> u32 {
        let index = self.pc as usize;
        return (self.dram[index] as u32)
            | ((self.dram[index + 1] as u32) << 8)
            | ((self.dram[index + 2] as u32) << 16)
            | ((self.dram[index + 3] as u32) << 24);
    }

    fn decode_load(&self, inst: u32) -> Instruction {
        //funct3
        match get_bits(inst, 14, 12) {
            0b000 => Instruction::LB(IType(inst)),
            0b001 => Instruction::LH(IType(inst)),
            0b010 => Instruction::LW(IType(inst)),
            0b100 => Instruction::LBU(IType(inst)),
            0b101 => Instruction::LWU(IType(inst)),
            0b101 => Instruction::LD(IType(inst)),
            _ => panic!(
                "Instruction read as a LOAD but no matching funct3 found. {:#b}",
                get_bits(inst, 14, 12)
            ),
        }
    }

    fn decode_op_imm(&self, inst: u32) -> Instruction {
        match get_bits(inst, 14, 12) {
            0b000 => Instruction::ADDI(IType(inst)),
            _ => panic!("Instruction was a OP IMM but no matching funct3 found."),
        }
    }

    fn decode_op(&self, inst: u32) -> Instruction {
        match get_bits(inst, 14, 12) {
            0b000 => Instruction::ADD(RType(inst)),
            _ => panic!("Instruction was a OP IMM but no matching funct3 found."),
        }
    }

    fn decode_store(&self, inst: u32) -> Instruction {
        match get_bits(inst, 14, 12) {
            0b000 => Instruction::SB(SType(inst)),
            0b001 => Instruction::SH(SType(inst)),
            0b010 => Instruction::SW(SType(inst)),
            0b011 => Instruction::SD(SType(inst)),
            _ => panic!(
                "Instruction was a STORE but no matching funct3 found. {:?}",
                inst
            ),
        }
    }

    fn decode_branch(&self, inst: u32) -> Instruction {
        match get_bits(inst, 14, 12) {
            0b000 => Instruction::BEQ(BType(inst)),
            0b001 => Instruction::BNE(BType(inst)),
            0b100 => Instruction::BLT(BType(inst)),
            0b101 => Instruction::BGE(BType(inst)),
            0b110 => Instruction::BLTU(BType(inst)),
            0b111 => Instruction::BGEU(BType(inst)),
            _ => panic!("Instruction was a BRANCH but no matching funct3 found."),
        }
    }

    pub fn decode(&self, inst: u32) -> Instruction {
        //Shifting right 2 since all opcodes 2 LSBs are 1
        match inst & 0b11 {
            0b11 => match inst >> 2 & 0b11111 {
                0b00000 => self.decode_load(inst),
                0b00100 => self.decode_op_imm(inst),
                0b01100 => self.decode_op(inst),
                0b01000 => self.decode_store(inst),
                0b01101 => Instruction::LUI(UType(inst)),
                0b00101 => Instruction::AUIPC(UType(inst)),
                0b11011 => Instruction::JAL(JType(inst)),
                0b11000 => self.decode_branch(inst),
                _ => panic!(
                    "Decode not yet implemented for opcode: {:#b}",
                    get_bits(inst, 7, 0)
                ),
            },
            _ => panic!(
                "Instruction not supported! Opcode: {:#07b}",
                get_bits(inst, 7, 0)
            ),
        }
    }

    //page 130 for instruction format
    fn execute(&mut self, inst: u32) {
        self.regfile[0] = 0;
        let decoded = self.decode(inst);
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
            Instruction::JAL(decoded) => {
                self.regfile[decoded.rd() as usize] = self.pc;
                //add the jump immediate and subtrat 4 (since the PC has already advanced)
                self.pc = self.pc.wrapping_add(decoded.imm() as u64).wrapping_sub(4);
            }
            Instruction::BEQ(decoded) => {
                if self.regfile[decoded.rs1() as usize] == self.regfile[decoded.rs2() as usize] {
                    self.pc = self.pc.wrapping_add(decoded.imm() as u64).wrapping_sub(4);
                }
            }
            Instruction::BNE(decoded) => {
                if self.regfile[decoded.rs1() as usize] != self.regfile[decoded.rs2() as usize] {
                    self.pc = self.pc.wrapping_add(decoded.imm() as u64).wrapping_sub(4);
                }
            }
            Instruction::BLT(decoded) => {
                if (self.regfile[decoded.rs1() as usize] as i64)
                    < (self.regfile[decoded.rs2() as usize] as i64)
                {
                    self.pc = self.pc.wrapping_add(decoded.imm() as u64).wrapping_sub(4);
                }
            }
            Instruction::BGE(decoded) => {
                if (self.regfile[decoded.rs1() as usize] as i64)
                    >= (self.regfile[decoded.rs2() as usize] as i64)
                {
                    self.pc = self.pc.wrapping_add(decoded.imm() as u64).wrapping_sub(4);
                }
            }
            Instruction::BLTU(decoded) => {
                if self.regfile[decoded.rs1() as usize] < self.regfile[decoded.rs2() as usize] {
                    self.pc = self.pc.wrapping_add(decoded.imm() as u64).wrapping_sub(4);
                }
            }
            Instruction::BGEU(decoded) => {
                if self.regfile[decoded.rs1() as usize] >= self.regfile[decoded.rs2() as usize] {
                    self.pc = self.pc.wrapping_add(decoded.imm() as u64).wrapping_sub(4);
                }
            }
            _ => {
                eprintln!("Execute not yet implemented for: {:?}", decoded);
            }
        }
        self.dump_registers();
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
