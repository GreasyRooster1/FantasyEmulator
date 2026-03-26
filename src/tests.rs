#[cfg(test)]
#[allow(clippy::module_inception)]
mod tests {

    use crate::cpu::*;
    use crate::instructions::*;

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
    fn test_boot() {
        let mut emulator = Emulator::hardware_setup();
        emulator.boot("./data/test_rom.rom".to_string());
        emulator.cpu_cycle();
    }

    #[test]
    fn test_instruct_add() {
        let mut emulator = Emulator::hardware_setup();
        let data = vec![0b0001_0001, 0b0010_0011];
        let args = InstructionArgs::from_bytes(data);
        emulator.registers[1] = 2;
        emulator.registers[2] = 3;

        ADD.execute(&mut emulator, args);
        assert_eq!(emulator.registers[3], 5);
    }
}
