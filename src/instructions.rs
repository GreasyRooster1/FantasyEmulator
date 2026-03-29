use crate::cpu::Emulator;
use crate::get_nibble_from_byte;

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
}

pub trait Instruction {
    fn execute(&self, emulator: &mut Emulator, args: InstructionArgs);

    fn bytes_len(&self) -> i32;

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

pub struct ADD;
pub struct SUB;
pub struct MUL;
pub struct DIV;

pub struct NOT;
pub struct REM;
pub struct AND;
pub struct OR;
pub struct XOR;
pub struct PEEK;
pub struct POKE;
pub struct LODI;
pub struct BRANCH;

impl Instruction for ADD {
    fn execute(&self, emulator: &mut Emulator, args: InstructionArgs) {
        math_instruction_execute(emulator, args, |a,b| a.wrapping_add(b));
    }
    fn bytes_len(&self) -> i32 {
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
    fn bytes_len(&self) -> i32 {
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
    fn bytes_len(&self) -> i32 {
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
    fn bytes_len(&self) -> i32 {
        2
    }

    fn opcode(&self) -> u8 {
        0b0100
    }
}

impl Instruction for NOT {
    fn execute(&self, emulator: &mut Emulator, args: InstructionArgs) {
        let a = emulator.registers[args.get_nibble(1) as usize];
        emulator.registers[args.get_nibble(1) as usize] = !a;
    }
    fn bytes_len(&self) -> i32 {
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
    fn bytes_len(&self) -> i32 {
        2
    }

    fn opcode(&self) -> u8 {
        0b0111
    }
}
