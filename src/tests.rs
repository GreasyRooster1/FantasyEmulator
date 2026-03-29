#[cfg(test)]
#[allow(clippy::module_inception)]
mod tests {

    use crate::cpu::*;
    use crate::get_nibble_from_byte;
    use crate::instructions::*;

    fn test_instruct(instruct: &impl Instruction,a:u8, b:u8, expected:u8) {
        let mut emulator = Emulator::hardware_setup();
        let data = vec![0b1111_0001, 0b0010_0011];
        let args = InstructionArgs::from_bytes(data);
        emulator.registers[1] = a;
        emulator.registers[2] = b;

        instruct.execute(&mut emulator, args);
        dbg!(&emulator.registers);
        assert_eq!(emulator.registers[3], expected);
    }

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
        test_instruct(&ADD, 4, 5, 9);
    }
    #[test]
    fn test_instruct_sub() {
        test_instruct(&SUB, 9, 5, 4);
    }

    #[test]
    fn nibble() {
        let data = 0b1001_0011_1100_1111_0000_0000_0000_0000;
        assert_eq!(get_nibble_from_byte(data, 0), 0b0000_1001);
        assert_eq!(get_nibble_from_byte(data, 1), 0b0000_0011);
        assert_eq!(get_nibble_from_byte(data, 2), 0b0000_1100);
        assert_eq!(get_nibble_from_byte(data, 3), 0b0000_1111);
    }
}
