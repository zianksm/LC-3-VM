use crate::instruction::Opcode;

const DEFAULT_REGISTER: [i32; 32] = [0; 32];
const DEFAULT_PC: usize = 0;

pub struct VM {
    /// 32 bit registers
    registers: [i32; 32],
    /// program counters, used to track bytes we're currently executing
    pc: usize,
    /// program, store the actual bytes of the program
    program: Vec<u8>,
}

impl VM {
    pub fn new() -> Self {
        Self { registers: DEFAULT_REGISTER, pc: DEFAULT_PC, program: vec![] }
    }

    /// Loops as long as instructions can be executed.
    pub fn run(&mut self) {
        let mut is_done = false;
        while !is_done {
            is_done = self.execute_instruction();
        }
    }

    /// Executes one instruction. Meant to allow for more controlled execution of the VM
    pub fn run_once(&mut self) {
        self.execute_instruction();
    }

    fn execute_instruction(&mut self) -> bool {
        if self.is_pc_exceed_program_len() {
            return false;
        }

        match self.decode_opcode() {
            Opcode::LOAD => {
                let register = self.next_8_bits() as usize;
                let number = self.next_16_bits() as u32;
                self.registers[register] = number as i32;
            }
            Opcode::HLT => {
                println!("HLT encountered");
                false;
            }
            Opcode::IGL => {
                println!("IGL encountered");
                false;
            }
        }
        true
    }

    /// check if the program counter is exceed the program length.
    /// if this returns false then we must stop the vm
    fn is_pc_exceed_program_len(&self) -> bool {
        self.pc >= self.program.len()
    }

    fn decode_opcode(&mut self) -> Opcode {
        let current_bit = self.current_bit();

        // increment the program counter after we read the current bytes
        self.increment_pc_by_8_bit();

        // decode the opcode
        Opcode::from(current_bit)
    }

    fn next_8_bits(&mut self) -> u8 {
        let current_bit = self.current_bit();
        self.increment_pc_by_8_bit();

        current_bit
    }

    fn next_16_bits(&mut self) -> u16 {
        let mut current_bit = self.current_bit() as u16;
        // make room for another 8 bits
        current_bit = current_bit << 8;

        // merge the next 8 bits with the previous 8 bits using bitwise OR
        // this effetively merge the 2 8 bits into 1 16 bits by shifting the previous 8 bits to the left
        // and then merge it with the next 8 bits
        current_bit = current_bit | (self.peek_next_8_bits() as u16);

        // increment the pc by 2 byte ( 2 byte = 16 bits)
        self.increment_pc_by_8_bit();
        self.increment_pc_by_8_bit();

        current_bit
    }

    fn current_bit(&mut self) -> u8 {
        let current_pc = self.pc;
        let current_bit = self.program[current_pc];
        current_bit
    }

    fn peek_next_8_bits(&mut self) -> u8 {
        let next_pc = self.pc + 1;
        let next_bit = self.program[next_pc];
        next_bit
    }

    /// increment the program counter by 1 bytes ( 1 byte = 8 bits )
    fn increment_pc_by_8_bit(&mut self) {
        self.pc += 1;
    }
}

#[cfg(test)]
mod tests {
    use crate::instruction::{ OP_HLT, OP_LOAD };

    use super::*;
    #[test]
    fn test_create_vm() {
        let test_vm = VM::new();
        assert_eq!(test_vm.registers[0], 0)
    }

    #[test]
    fn test_opcode_hlt() {
        let mut test_vm = VM::new();
        let test_bytes = vec![OP_HLT];
        test_vm.program = test_bytes;
        test_vm.run();
        assert_eq!(test_vm.pc, 1);
    }

    #[test]
    fn test_opcode_igl() {
        let mut test_vm = VM::new();
        let test_bytes = vec![200, 0, 0, 0];
        test_vm.program = test_bytes;
        test_vm.run();
        assert_eq!(test_vm.pc, 1);
    }

    #[test]
    fn test_opcode_load() {
        let mut vm = VM::new();
        let register_index = 1;

        let instruction = vec![OP_LOAD, register_index, 1, 244];
        vm.program = instruction; // Remember, this is how we represent 500 using two u8s in little endian format

        vm.run();

        assert_eq!(vm.registers[register_index as usize], 500);
    }
}
