struct InstructionArgs {
    data: u32,
}

impl InstructionArgs {
    fn from_bytes(bytes: Vec<u8>) -> InstructionArgs {
        let mut data: u32 = 0;
        let total_bits = bytes.len() * 8;
        for i in 0..bytes.len() {
            data += (bytes[i] as u32) << (i * 8 - total_bits);
        }
        InstructionArgs { data }
    }

    fn opcode(&self) -> u8 {
        (self.data >> 28) as u8
    }
}

trait Instruction {
    fn execute(args: InstructionArgs);

    fn bytes_len() -> i32;

    fn opcode() -> u8;
}

struct ADD;

impl Instruction for ADD {
    fn bytes_len() -> i32 {
        2
    }
    fn opcode() -> u8 {
        0b0001
    }

    fn execute(args: InstructionArgs) {
        println!("{0}", args.opcode());
    }
}
