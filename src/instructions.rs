use crate::cpu::Emulator;
use crate::{get_nibble_from_byte, PC_REGISTER, RA_REGISTER};

pub struct InstructionArgs {
    data: u128,
}

impl InstructionArgs {
    pub fn from_bytes(bytes: Vec<u8>) -> InstructionArgs {
        let mut data: u128 = 0;
        let total_bits = bytes.len() * 8;
        for i in 0..bytes.len() {
            let shift_num = (bytes.len() as i32 - i as i32).abs() as usize;
            println!("{shift_num}");
            data += (bytes[i] as u128) << (32 - (i + 1) * 8);
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
        let nib_idx = i + 1;
        ((self.data >> (32 - nib_idx * 8)) & 0xFF) as u8
    }
    pub fn get_u32(&self, i: u32) -> u32 {
        let nib_idx = i + 1;
        ((self.data >> (32 - nib_idx * 8)) & 0xFFFFFFFF) as u32
    }
}

pub trait Instruction {
    fn execute(&self, emulator: &mut Emulator, args: InstructionArgs);

    fn bytes_len(&self) -> u8;

    fn opcode(&self) -> u8;
}

pub fn math_instruction_execute<F>(emulator: &mut Emulator, args: InstructionArgs, op:F)
where
    F: Fn(i32,i32) -> i32,
{
    let a = emulator.registers[args.get_byte(1) as usize];
    let b = emulator.registers[args.get_byte(2) as usize];
    let c = op(a,b);
    emulator.registers[args.get_byte(3) as usize] = c;
}

pub fn imm_math_instruction_execute<F>(emulator: &mut Emulator, args: InstructionArgs, op:F)
where
    F: Fn(i32,i32) -> i32,
{
    let a = emulator.registers[args.get_byte(1) as usize];
    let b = emulator.registers[args.get_u32(2) as usize];
    let c = op(a,b);
    emulator.registers[args.get_byte(1) as usize] = c;
}

pub fn branch_instruction_execute<F>(emulator: &mut Emulator, args: InstructionArgs, op:F)
where
    F: Fn(i32,i32) -> bool,
{
    let a = emulator.registers[args.get_byte(1) as usize];
    let b = emulator.registers[args.get_byte(2) as usize];
    if op(a,b) {
        emulator.registers[PC_REGISTER] = args.get_u32(3) as i32;
    }
}

pub struct NOP;
pub struct ADD;
pub struct ADDI;
pub struct SUB;
pub struct SUBI;
pub struct MUL;
pub struct MULI;
pub struct DIV;
pub struct DIVI;
pub struct MOD;
pub struct MODI;

pub struct AND;
pub struct ANDI;
pub struct OR;
pub struct ORI;
pub struct XOR;
pub struct XORI;
pub struct NOT;
pub struct RSH;
pub struct RSHI;
pub struct LSH;
pub struct LSHI;

pub struct JMP;
pub struct BREQ;
pub struct BRNEQ;
pub struct BRGT;
pub struct BRGTE;
pub struct BRLT;
pub struct BRLTE;
pub struct BREZ;
pub struct BRNEZ;
pub struct CALL;
pub struct RET;




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
        4
    }

    fn opcode(&self) -> u8 {
        0b00010000
    }
}
impl Instruction for ADDI {
    fn execute(&self, emulator: &mut Emulator, args: InstructionArgs) {
        imm_math_instruction_execute(emulator, args, |a,b| a.wrapping_add(b));
    }
    fn bytes_len(&self) -> u8 {
        6
    }

    fn opcode(&self) -> u8 {
        0b00010001
    }
}
impl Instruction for SUB {
    fn execute(&self, emulator: &mut Emulator, args: InstructionArgs) {
        math_instruction_execute(emulator, args, |a,b| a.wrapping_sub(b));
    }
    fn bytes_len(&self) -> u8 {
        4
    }

    fn opcode(&self) -> u8 {
        0b00010010
    }
}
impl Instruction for SUBI {
    fn execute(&self, emulator: &mut Emulator, args: InstructionArgs) {
        imm_math_instruction_execute(emulator, args, |a,b| a.wrapping_sub(b));
    }
    fn bytes_len(&self) -> u8 {
        6
    }

    fn opcode(&self) -> u8 {
        0b00010011
    }
}
impl Instruction for MUL {
    fn execute(&self, emulator: &mut Emulator, args: InstructionArgs) {
        math_instruction_execute(emulator, args, |a,b| a.wrapping_mul(b));
    }
    fn bytes_len(&self) -> u8 {
        4
    }

    fn opcode(&self) -> u8 {
        0b00010100
    }
}
impl Instruction for MULI {
    fn execute(&self, emulator: &mut Emulator, args: InstructionArgs) {
        imm_math_instruction_execute(emulator, args, |a,b| a.wrapping_mul(b));
    }
    fn bytes_len(&self) -> u8 {
        6
    }

    fn opcode(&self) -> u8 {
        0b00010101
    }
}
impl Instruction for DIV {
    fn execute(&self, emulator: &mut Emulator, args: InstructionArgs) {
        math_instruction_execute(emulator, args, |a,b| a.wrapping_div(b));
    }
    fn bytes_len(&self) -> u8 {
        4
    }

    fn opcode(&self) -> u8 {
        0b00010110
    }
}
impl Instruction for DIVI {
    fn execute(&self, emulator: &mut Emulator, args: InstructionArgs) {
        imm_math_instruction_execute(emulator, args, |a,b| a.wrapping_div(b));
    }
    fn bytes_len(&self) -> u8 {
        6
    }

    fn opcode(&self) -> u8 {
        0b00010111
    }
}
impl Instruction for MOD {
    fn execute(&self, emulator: &mut Emulator, args: InstructionArgs) {
        math_instruction_execute(emulator, args, |a,b| a.wrapping_rem(b));
    }
    fn bytes_len(&self) -> u8 {
        4
    }

    fn opcode(&self) -> u8 {
        0b00011000
    }
}
impl Instruction for MODI {
    fn execute(&self, emulator: &mut Emulator, args: InstructionArgs) {
        imm_math_instruction_execute(emulator, args, |a,b| a.wrapping_rem(b));
    }
    fn bytes_len(&self) -> u8 {
        6
    }

    fn opcode(&self) -> u8 {
        0b00011001
    }
}

impl Instruction for AND {
    fn execute(&self, emulator: &mut Emulator, args: InstructionArgs) {
        math_instruction_execute(emulator, args, |a,b| a & b);
    }
    fn bytes_len(&self) -> u8 {
        4
    }

    fn opcode(&self) -> u8 {
        0b0010_0000
    }
}
impl Instruction for ANDI {
    fn execute(&self, emulator: &mut Emulator, args: InstructionArgs) {
        imm_math_instruction_execute(emulator, args, |a,b| a & b);
    }
    fn bytes_len(&self) -> u8 {
        6
    }

    fn opcode(&self) -> u8 {
        0b0010_0001
    }
}
impl Instruction for OR {
    fn execute(&self, emulator: &mut Emulator, args: InstructionArgs) {
        math_instruction_execute(emulator, args, |a,b| a | b);
    }
    fn bytes_len(&self) -> u8 {
        4
    }

    fn opcode(&self) -> u8 {
        0b0010_0010
    }
}
impl Instruction for ORI {
    fn execute(&self, emulator: &mut Emulator, args: InstructionArgs) {
        imm_math_instruction_execute(emulator, args, |a,b| a | b);
    }
    fn bytes_len(&self) -> u8 {
        6
    }

    fn opcode(&self) -> u8 {
        0b0010_0011
    }
}
impl Instruction for XOR {
    fn execute(&self, emulator: &mut Emulator, args: InstructionArgs) {
        math_instruction_execute(emulator, args, |a,b| a ^ b);
    }
    fn bytes_len(&self) -> u8 {
        4
    }

    fn opcode(&self) -> u8 {
        0b0010_0100
    }
}
impl Instruction for XORI {
    fn execute(&self, emulator: &mut Emulator, args: InstructionArgs) {
        imm_math_instruction_execute(emulator, args, |a,b| a ^ b);
    }
    fn bytes_len(&self) -> u8 {
        6
    }

    fn opcode(&self) -> u8 {
        0b0010_0101
    }
}
impl Instruction for NOT {
    fn execute(&self, emulator: &mut Emulator, args: InstructionArgs) {
        let a = emulator.registers[args.get_byte(1) as usize];
        emulator.registers[args.get_byte(2) as usize] = !a;
    }
    fn bytes_len(&self) -> u8 {
        3
    }

    fn opcode(&self) -> u8 {
        0b0010_0110
    }
}
impl Instruction for RSH {
    fn execute(&self, emulator: &mut Emulator, args: InstructionArgs) {
        math_instruction_execute(emulator, args, |a,b| a >> b);
    }
    fn bytes_len(&self) -> u8 {
        4
    }

    fn opcode(&self) -> u8 {
        0b0010_0111
    }
}
impl Instruction for RSHI {
    fn execute(&self, emulator: &mut Emulator, args: InstructionArgs) {
        imm_math_instruction_execute(emulator, args, |a,b| a >> b);
    }
    fn bytes_len(&self) -> u8 {
        6
    }

    fn opcode(&self) -> u8 {
        0b0010_1000
    }
}
impl Instruction for LSH {
    fn execute(&self, emulator: &mut Emulator, args: InstructionArgs) {
        math_instruction_execute(emulator, args, |a,b| a << b);
    }
    fn bytes_len(&self) -> u8 {
        4
    }

    fn opcode(&self) -> u8 {
        0b0010_1001
    }
}
impl Instruction for LSHI {
    fn execute(&self, emulator: &mut Emulator, args: InstructionArgs) {
        imm_math_instruction_execute(emulator, args, |a, b| a << b);
    }
    fn bytes_len(&self) -> u8 {
        6
    }

    fn opcode(&self) -> u8 {
        0b0010_1010
    }
}

impl Instruction for JMP {
    fn execute(&self, emulator: &mut Emulator, args: InstructionArgs) {
        let mem_loc = args.get_u32(1);
        emulator.registers[PC_REGISTER]=mem_loc as i32;
    }
    fn bytes_len(&self) -> u8 {
        5
    }

    fn opcode(&self) -> u8 {
        0b0100_0000
    }
}
impl Instruction for BREQ {
    fn execute(&self, emulator: &mut Emulator, args: InstructionArgs) {
        branch_instruction_execute(emulator, args, |a,b| a==b);
    }
    fn bytes_len(&self) -> u8 {
        7
    }

    fn opcode(&self) -> u8 {
        0b0100_0001
    }
}
impl Instruction for BRNEQ {
    fn execute(&self, emulator: &mut Emulator, args: InstructionArgs) {
        branch_instruction_execute(emulator, args, |a,b| a!=b);
    }
    fn bytes_len(&self) -> u8 {
        7
    }

    fn opcode(&self) -> u8 {
        0b0100_0010
    }
}
impl Instruction for BRGT {
    fn execute(&self, emulator: &mut Emulator, args: InstructionArgs) {
        branch_instruction_execute(emulator, args, |a,b| a>b);
    }
    fn bytes_len(&self) -> u8 {
        7
    }

    fn opcode(&self) -> u8 {
        0b0100_0011
    }
}
impl Instruction for BRGTE {
    fn execute(&self, emulator: &mut Emulator, args: InstructionArgs) {
        branch_instruction_execute(emulator, args, |a,b| a>=b);
    }
    fn bytes_len(&self) -> u8 {
        7
    }

    fn opcode(&self) -> u8 {
        0b0100_0100
    }
}
impl Instruction for BRLT {
    fn execute(&self, emulator: &mut Emulator, args: InstructionArgs) {
        branch_instruction_execute(emulator, args, |a,b| a<b);
    }
    fn bytes_len(&self) -> u8 {
        7
    }

    fn opcode(&self) -> u8 {
        0b0100_0101
    }
}
impl Instruction for BRLTE {
    fn execute(&self, emulator: &mut Emulator, args: InstructionArgs) {
        branch_instruction_execute(emulator, args, |a,b| a<=b);
    }
    fn bytes_len(&self) -> u8 {
        7
    }

    fn opcode(&self) -> u8 {
        0b0100_0110
    }
}
impl Instruction for BREZ {
    fn execute(&self, emulator: &mut Emulator, args: InstructionArgs) {
        let a = emulator.registers[args.get_byte(1) as usize];
        if a==0 {
            emulator.registers[PC_REGISTER] = args.get_u32(2) as i32;
        }
    }
    fn bytes_len(&self) -> u8 {
        6
    }

    fn opcode(&self) -> u8 {
        0b0100_0111
    }
}
impl Instruction for BRNEZ {
    fn execute(&self, emulator: &mut Emulator, args: InstructionArgs) {
        let a = emulator.registers[args.get_byte(1) as usize];
        if a!=0 {
            emulator.registers[PC_REGISTER] = args.get_u32(2) as i32;
        }
    }
    fn bytes_len(&self) -> u8 {
        6
    }

    fn opcode(&self) -> u8 {
        0b0100_1000
    }
}
impl Instruction for CALL {
    fn execute(&self, emulator: &mut Emulator, args: InstructionArgs) {
        let mem_loc = args.get_u32(1);
        emulator.registers[RA_REGISTER]=emulator.registers[PC_REGISTER];
        emulator.registers[PC_REGISTER]=mem_loc as i32;
    }
    fn bytes_len(&self) -> u8 {
        5
    }

    fn opcode(&self) -> u8 {
        0b0100_1001
    }
}
impl Instruction for CALL {
    fn execute(&self, emulator: &mut Emulator, args: InstructionArgs) {
        emulator.registers[PC_REGISTER]=emulator.registers[RA_REGISTER];
    }
    fn bytes_len(&self) -> u8 {
        1
    }

    fn opcode(&self) -> u8 {
        0b0100_1010
    }
}