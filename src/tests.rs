#[cfg(test)]
#[allow(clippy::module_inception)]
mod tests {

    use crate::instructions::*;
    use crate::cpu::*;

    #[test]
    fn test_args_opcode() {
        let data = vec![0b01010000, 0b11110000];
        let args = InstructionArgs::from_bytes(data);
        println!("{0}", args.opcode());
        assert_eq!(args.opcode(), 0b00000101);
    }

    #[test]
    fn test_args_nibble() {
        let data = vec![0b1111_0010, 0b1111_0000];
        let args = InstructionArgs::from_bytes(data);
        println!("{0}", args.opcode());
        assert_eq!(args.get_nibble(1), 0b0000_0010);
    }

    #[test]
    fn test_boot(){
        let mut emulator = Emulator::hardware_setup();
        emulator.cpu_cycle();
    }
}
