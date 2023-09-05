
macro_rules! opcode {
    ($name:ident, $code:expr) => {
        pub const $name: u8 = $code;
    };
}

opcode!(OP_LOAD, 9);
opcode!(OP_HLT, 1);

#[derive(Debug, PartialEq, Clone)]
pub enum Opcode {
    // halt
    HLT,
    // illegal
    IGL,
    // load
    LOAD,
}

impl From<u8> for Opcode {
    fn from(v: u8) -> Self {
        match v {
            OP_LOAD => Self::LOAD,
            OP_HLT => Self::HLT,
            _ => Self::IGL,
        }
    }
}

pub struct Instruction {
    opcode: Opcode,
}

impl Instruction {
    pub fn new(opcode: Opcode) -> Self {
        Self { opcode }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_instruction() {
        let instruction = Instruction::new(Opcode::HLT);

        assert_eq!(instruction.opcode, Opcode::HLT);
    }
}
