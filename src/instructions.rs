struct InstructionArgs {
    data: u32,
}

impl InstructionArgs {

    fn from_bytes(bytes:Vec<u8>){
        let data = 0;
        for i in 0..bytes.len(){
            data += bytes[i]<<(i*8-24)``
        }
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
    fn execute(args: InstructionArgs) {}
}

