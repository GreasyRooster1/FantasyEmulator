use crate::cpu::Emulator;
use crate::{get_nibble_from_byte, PC_REGISTER};

pub struct InstructionArgs {
    data: u32,
}

impl InstructionArgs {
    pub fn from_bytes(bytes: Vec<u8>) -> InstructionArgs {
        let mut data: u32 = 0;
        let total_bits = bytes.len() * 8;
        for i in 0..bytes.len() {
            let shift_num = (bytes.len() as i32 - i as i32).abs() as usize;
            println!("{shift_num}");
            data += (bytes[i] as u32) << (32 - (i + 1) * 8);
        }
        //data = data << (32 - total_bits);
        InstructionArgs { data }
    }

    pub fn opcode(&self) -> u8 {
        (self.data >> 28) as u8
    }

    pub fn get_nibble(&self, i: u32) -> u8 {
        dbg!(format!("{:#b}", self.data));
        get_nibble_from_byte(self.data, i)
    }
    pub fn get_byte(&self, i: u32) -> u8 {
        (self.get_nibble(i)<<4)+self.get_nibble(i+1)
    }
}

pub trait Instruction {
    fn execute(&self, emulator: &mut Emulator, args: InstructionArgs);

    fn bytes_len(&self) -> u8;

    fn opcode(&self) -> u8;
}

pub fn math_instruction_execute<F>(emulator: &mut Emulator, args: InstructionArgs, op:F)
where
    F: Fn(u8,u8) -> u8,
{
    let a = emulator.registers[args.get_nibble(1) as usize];
    let b = emulator.registers[args.get_nibble(2) as usize];
    let c = op(a,b);
    emulator.registers[args.get_nibble(3) as usize] = c;
}

pub struct NOP;
pub struct ADD;
pub struct SUB;
pub struct MUL;
pub struct DIV;
pub struct REM;
pub struct INC;
pub struct NOT;
pub struct AND;
pub struct OR;
pub struct XOR;
pub struct PEEK;
pub struct POKE;
pub struct LODI;
pub struct BRANCH;
pub struct HALT;

impl Instruction for NOP {
    fn execute(&self, emulator: &mut Emulator, args: InstructionArgs) {
        match args.get_nibble(1) {
            0b0001 => {dbg!(emulator.registers);},
            0b0010 => {emulator.debug_memory_report()},
            0b0011 => {emulator.debug_rom_report()},
            _ => {},
        };
    }
    fn bytes_len(&self) -> u8 {
        1
    }

    fn opcode(&self) -> u8 {
        0b0000
    }
}


impl Instruction for ADD {
    fn execute(&self, emulator: &mut Emulator, args: InstructionArgs) {
        math_instruction_execute(emulator, args, |a,b| a.wrapping_add(b));
    }
    fn bytes_len(&self) -> u8 {
        2
    }

    fn opcode(&self) -> u8 {
        0b0001
    }
}

impl Instruction for SUB {
    fn execute(&self, emulator: &mut Emulator, args: InstructionArgs) {
        math_instruction_execute(emulator, args, |a,b| a.wrapping_sub(b));
    }
    fn bytes_len(&self) -> u8 {
        2
    }

    fn opcode(&self) -> u8 {
        0b0010
    }
}

impl Instruction for MUL {
    fn execute(&self, emulator: &mut Emulator, args: InstructionArgs) {
        math_instruction_execute(emulator, args, |a,b| a.wrapping_mul(b));
    }
    fn bytes_len(&self) -> u8 {
        2
    }

    fn opcode(&self) -> u8 {
        0b0011
    }
}


impl Instruction for DIV {
    fn execute(&self, emulator: &mut Emulator, args: InstructionArgs) {
        math_instruction_execute(emulator, args, |a,b| a.wrapping_div(b));
    }
    fn bytes_len(&self) -> u8 {
        2
    }

    fn opcode(&self) -> u8 {
        0b0100
    }
}

impl Instruction for INC {
    fn execute(&self, emulator: &mut Emulator, args: InstructionArgs) {
        let a = emulator.registers[args.get_nibble(1) as usize].wrapping_add(1);
        emulator.registers[args.get_nibble(1) as usize] = a;
    }
    fn bytes_len(&self) -> u8 {
        1
    }

    fn opcode(&self) -> u8 {
        0b0101
    }
}

impl Instruction for NOT {
    fn execute(&self, emulator: &mut Emulator, args: InstructionArgs) {
        let a = emulator.registers[args.get_nibble(1) as usize];
        emulator.registers[args.get_nibble(1) as usize] = !a;
    }
    fn bytes_len(&self) -> u8 {
        1
    }

    fn opcode(&self) -> u8 {
        0b0110
    }
}

impl Instruction for REM {
    fn execute(&self, emulator: &mut Emulator, args: InstructionArgs) {
        math_instruction_execute(emulator, args, |a,b| a.wrapping_rem(b));
    }
    fn bytes_len(&self) -> u8 {
        2
    }

    fn opcode(&self) -> u8 {
        0b0111
    }
}

impl Instruction for AND {
    fn execute(&self, emulator: &mut Emulator, args: InstructionArgs) {
        math_instruction_execute(emulator, args, |a,b| a&b);
    }
    fn bytes_len(&self) -> u8 {
        2
    }

    fn opcode(&self) -> u8 {
        0b1000
    }
}

impl Instruction for OR {
    fn execute(&self, emulator: &mut Emulator, args: InstructionArgs) {
        math_instruction_execute(emulator, args, |a,b| a|b);
    }
    fn bytes_len(&self) -> u8 {
        2
    }

    fn opcode(&self) -> u8 {
        0b1001
    }
}

impl Instruction for XOR {
    fn execute(&self, emulator: &mut Emulator, args: InstructionArgs) {
        math_instruction_execute(emulator, args, |a,b| a^b);
    }
    fn bytes_len(&self) -> u8 {
        2
    }

    fn opcode(&self) -> u8 {
        0b1010
    }
}

impl Instruction for PEEK {
    fn execute(&self, emulator: &mut Emulator, args: InstructionArgs) {
        let mem_loc = args.get_byte(1) as usize;
        let reg = emulator.registers[args.get_nibble(3) as usize];
        emulator.registers[args.get_nibble(3) as usize] = emulator.physical_memory[mem_loc];
    }
    fn bytes_len(&self) -> u8 {
        2
    }

    fn opcode(&self) -> u8 {
        0b1011
    }
}

impl Instruction for POKE {
    fn execute(&self, emulator: &mut Emulator, args: InstructionArgs) {
        let mem_loc = args.get_byte(1) as usize;
        let reg = emulator.registers[args.get_nibble(3) as usize];
        emulator.physical_memory[mem_loc] = emulator.registers[args.get_nibble(3) as usize];
    }
    fn bytes_len(&self) -> u8 {
        2
    }

    fn opcode(&self) -> u8 {
        0b1100
    }
}

impl Instruction for LODI {
    fn execute(&self, emulator: &mut Emulator, args: InstructionArgs) {
        let reg =args.get_nibble(1) as usize;
        let imm = args.get_byte(2);
        dbg!(imm,reg);
        emulator.registers[reg as usize] = imm;
    }
    fn bytes_len(&self) -> u8 {
        2
    }

    fn opcode(&self) -> u8 {
        0b1101
    }
}

impl Instruction for BRANCH {
    fn execute(&self, emulator: &mut Emulator, args: InstructionArgs) {
        let reg_a = emulator.registers[args.get_nibble(1) as usize];
        let op = args.get_nibble(2);
        let reg_b = emulator.registers[args.get_nibble(3) as usize];
        let mem_loc = args.get_byte(4);
        dbg!(reg_a, op, reg_b, mem_loc);
        let result = match op {
            0b0000 => true,
            
            0b0001 => reg_a == reg_b,
            0b0010 => reg_a != reg_b,
            0b0011 => reg_a > reg_b,
            0b0100 => reg_a >= reg_b,
            0b0101 => reg_a < reg_b,
            0b0110 => reg_a <= reg_b,
            _ => false,
        };
        if result {
            emulator.registers[PC_REGISTER] = mem_loc;
        }
    }
    fn bytes_len(&self) -> u8 {
        3
    }

    fn opcode(&self) -> u8 {
        0b1110
    }
}


impl Instruction for HALT {
    fn execute(&self, emulator: &mut Emulator, args: InstructionArgs) {
        emulator.halt();
    }
    fn bytes_len(&self) -> u8 {
        3
    }

    fn opcode(&self) -> u8 {
        0b1110
    }
}
