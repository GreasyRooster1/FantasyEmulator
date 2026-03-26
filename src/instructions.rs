use crate::get_nibble_from_byte;

pub struct InstructionArgs {
    data: u32,
}

impl InstructionArgs {
    pub fn from_bytes(bytes: Vec<u8>) -> InstructionArgs {
        let mut data: u32 = 0;
        let total_bits = bytes.len() * 8;
        for i in 0..bytes.len() {
            let shift_num = (bytes.len() as i32 - i as i32).abs() as usize - 1;
            println!("{shift_num}");
            data += (bytes[i] as u32) << (total_bits - shift_num * 8);
        }
        data = data << (32 - total_bits);
        InstructionArgs { data }
    }

    pub fn opcode(&self) -> u8 {
        (self.data >> 28) as u8
    }

    pub fn get_nibble(&self, i: u32) -> u8 {
        get_nibble_from_byte(self.data,i)
    }
}

trait Instruction {
    fn execute(args: InstructionArgs);

    fn bytes_len() -> i32;

    fn opcode() -> u8;
}

struct ADD;

impl Instruction for ADD {
    fn execute(args: InstructionArgs) {
        println!("{0}", args.opcode());
    }
    fn bytes_len() -> i32 {
        2
    }

    fn opcode() -> u8 {
        0b0001
    }
}
