
struct InstructionArgs{
    data: u32,
}

impl InstructionArgs{
    fn opcode(){
    
    }
}

trait Instruction{
    fn execute(InstructionArgs args);

    fn bytes_len() -> i32;

    fn opcode() -> u8;
}

struct ADD{
    fn execute(InstructionArgs args)
}

