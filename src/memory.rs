const CAPACITY : u64 = 1024 * 1024 * 128;

pub struct Memory {
    //mem is a vector of bytes
    pub mem: Vec<u8>
}

impl Memory {
    pub fn new() -> Self {
        Memory {
            mem: vec![0; CAPACITY]
        } 
    }

    pub fn load_byte(&self, address: u64){
        
    }

    pub fn load_halfword(&self, address: u64){

    }

    pub fn load_word(&self, address: u64){

    }

    pub fn load_dword(&self, address: u64){

    }

    pub fn store_byte(&self, address: u64, value: u8){

    }

    pub fn store_halfword(&self, address: u64, value: u16){

    }

    pub fn store_word(&self, address: u64, value: u32){

    }

    pub fn store_dword(&self, address: u64, value: u64){

    }
}
