use std::fmt;

pub struct RType(pub u32);

impl RType {
    pub fn rs2(&self) -> u32 {
        get_bits(self.0, 24, 20)
    }
    pub fn rs1(&self) -> u32 {
        get_bits(self.0, 19, 15)
    }
    pub fn rd(&self) -> u32 {
        get_bits(self.0, 11, 7)
    }
}

impl fmt::Debug for RType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:#010x}", self.0)
    }
}

pub struct IType(pub u32);

impl IType {
    pub fn imm(&self) -> u32 {
        get_bits(self.0, 31, 20)
    }
    pub fn rs1(&self) -> u32 {
        get_bits(self.0, 19, 15)
    }
    pub fn rd(&self) -> u32 {
        get_bits(self.0, 11, 7)
    }
}

impl fmt::Debug for IType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:#010x}", self.0)
    }
}

#[derive(Debug)]
pub struct SType(pub u32);

impl SType {
    pub fn imm(&self) -> u32 {
        get_bits(self.0, 31, 25)
    }
    pub fn rs2(&self) -> u32 {
        get_bits(self.0, 24, 20)
    }
    pub fn rs1(&self) -> u32 {
        get_bits(self.0, 19, 15)
    }
}

#[derive(Debug)]
pub struct UType(pub u32);

impl UType {
    pub fn imm(&self) -> u32 {
        get_bits(self.0, 31, 12)
    }
    pub fn rd(&self) -> u32 {
        get_bits(self.0, 11, 7)
    }
}

#[derive(Debug)]
pub struct JType(pub u32);

impl JType {
    pub fn rd(&self) -> u32 {
        get_bits(self.0, 11, 7)
    }
    //Not so sure about this IMM here
    pub fn imm(&self) -> u32 {
        get_bits(self.0, 31, 31)
            | get_bits(self.0, 30, 21)
            | get_bits(self.0, 20, 20)
            | get_bits(self.0, 19, 12)
    }
}

#[derive(Debug)]
pub struct BType(pub u32);

impl BType {
    //Not so sure about this IMM here
    pub fn imm(&self) -> u32 {
        (get_bits(self.0, 31, 31) | get_bits(self.0, 30, 25))
            | (get_bits(self.0, 11, 8) | get_bits(self.0, 8, 6))
    }
    pub fn rs2(&self) -> u32 {
        get_bits(self.0, 24, 20)
    }
    pub fn rs1(&self) -> u32 {
        get_bits(self.0, 19, 15)
    }
}

pub fn get_bits(inst: u32, start: u32, end: u32) -> u32 {
    (inst >> end) & ((1 << (start - end + 1)) - 1)
}

//Write some tests for the above to ensure that we are getting the correct bits
