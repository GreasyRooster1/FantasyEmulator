#[cfg(test)]
#[allow(clippy::module_inception)]
mod tests {

    use crate::cpu::*;
    use crate::{get_byte_from_data, get_nibble_from_byte};
    use crate::instructions::*;


    #[test]
    fn test_args_opcode() {
        let data = vec![0b01010000, 0b11110000];
        let args = InstructionArgs::from_bytes(data);
        println!("{0}", args.opcode());
        assert_eq!(args.opcode(), 0b01010000);
    }

    #[test]
    fn test_args_byte() {
        let data = vec![0b1111_0010, 0b1111_0000];
        let args = InstructionArgs::from_bytes(data);
        println!("{0}", args.opcode());
        assert_eq!(args.get_byte(1), 0b1111_0000);
    }

    #[test]
    fn test_args_u32() {
        let data = vec![0b10010011,0b11001111,0b00000000,0b00000000,0b00000000,0b00000000,0b00000000,0b00000000,0b10010011,0b11001111,0b00000000,0b00000000,0b00000000,0b00000000,0b00000000,0b00000000];
        let args = InstructionArgs::from_bytes(data);
        println!("{0}", args.opcode());
        assert_eq!(args.get_u32(0), 0b10010011_11001111_00000000_00000000);
    }

    #[test]
    fn test_boot() {
        let mut emulator = Emulator::hardware_setup();
        emulator.boot("./data/test_rom.rom".to_string());
        emulator.cpu_cycle();
    }

    #[test]
    fn nibble() {
        let data = 0b10010011_11001111_00000000_00000000_00000000_00000000_00000000_00000000__10010011_11001111_00000000_00000000_00000000_00000000_00000000_00000000;
        assert_eq!(get_nibble_from_byte(data, 0), 0b0000_1001);
        assert_eq!(get_nibble_from_byte(data, 1), 0b0000_0011);
        assert_eq!(get_nibble_from_byte(data, 2), 0b0000_1100);
        assert_eq!(get_nibble_from_byte(data, 3), 0b0000_1111);
    }

    #[test]
    fn byte() {
        let data = 0b10010011_11001111_00000000_00000000_00000000_00000000_00000000_00000000__10010011_11001111_00000000_00000000_00000000_00000000_00000000_00000000;
        assert_eq!(get_byte_from_data(data, 0), 0b10010011);
        assert_eq!(get_byte_from_data(data, 1), 0b11001111);
        assert_eq!(get_byte_from_data(data, 2), 0b00000000);
    }


}
