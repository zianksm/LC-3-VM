const MAX_MEMORY: usize = 1 << 16; // 65536 

static mut MEMORY: [u16; MAX_MEMORY] = [0; MAX_MEMORY];

pub enum Register {
    R0 = 0,
    R1,
    R2,
    R3,
    R4,
    R5,
    R6,
    R7,
    PC, // Program Counter
    COND, // Conditional Flag
    COUNT,
}

impl Register {
    pub fn from_u16(value: u16) -> Register {
        match value {
            0 => Register::R0,
            1 => Register::R1,
            2 => Register::R2,
            3 => Register::R3,
            4 => Register::R4,
            5 => Register::R5,
            6 => Register::R6,
            7 => Register::R7,
            8 => Register::PC,
            9 => Register::COND,
            _ => panic!("Invalid register"),
        }
    }
}

pub enum Opcodes{
    BR = 0, // Branch
    ADD, // Add
    LD, // Load
    ST, // Store
    JSR, // Jump Register
    AND, // Bitwise And
    LDR, // Load Register
    STR, // Store Register
    RTI, // Unused
    NOT, // Bitwise Not
    LDI, // Load Indirect
    STI, // Store Indirect
    JMP, // Jump
    RES, // Reserved (Unused)
    LEA, // Load Effective Address
    TRAP, // Execute Trap
}

pub enum Flags {
    POS = 1 << 0, // Positive
    ZRO = 1 << 1, // Zero
    NEG = 1 << 2, // Negative
}