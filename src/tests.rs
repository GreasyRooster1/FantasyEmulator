#[cfg(test)]
#[allow(clippy::module_inception)]
mod tests {

    use crate::cpu::*;
    use crate::{get_nibble_from_byte, PC_REGISTER};
    use crate::instructions::*;

    fn test_math_instruct(instruct: &impl Instruction, a:u8, b:u8, expected:u8) {
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
    fn nibble() {
        let data = 0b1001_0011_1100_1111_0000_0000_0000_0000;
        assert_eq!(get_nibble_from_byte(data, 0), 0b0000_1001);
        assert_eq!(get_nibble_from_byte(data, 1), 0b0000_0011);
        assert_eq!(get_nibble_from_byte(data, 2), 0b0000_1100);
        assert_eq!(get_nibble_from_byte(data, 3), 0b0000_1111);
    }

    /* Instruction Tests */
    #[test]
    fn test_instruct_add() {
        test_math_instruct(&ADD, 4, 5, 9);
    }
    #[test]
    fn test_instruct_add_overflow() {
        test_math_instruct(&ADD, 200, 57, 1);
    }
    #[test]
    fn test_instruct_sub() {
        test_math_instruct(&SUB, 9, 5, 4);
    }
    #[test]
    fn test_instruct_sub_overflow() {
        test_math_instruct(&SUB, 10, 11, 255);
    }
    #[test]
    fn test_instruct_mul() {
        test_math_instruct(&MUL, 10, 5, 50);
    }
    #[test]
    fn test_instruct_mul_overflow() {
        test_math_instruct(&MUL, 128, 2, 0);
    }
    #[test]
    fn test_instruct_div() {
        test_math_instruct(&DIV, 100, 2, 50);
    }
    #[test]
    fn test_instruct_div_round() {
        test_math_instruct(&DIV, 100, 3, 33);
    }
    #[test]
    fn test_instruct_rem() {
        test_math_instruct(&REM, 4, 3, 1);
    }
    #[test]
    fn test_instruct_not() {
        let mut emulator = Emulator::hardware_setup();
        let data = vec![0b1111_0001];
        let args = InstructionArgs::from_bytes(data);
        emulator.registers[1] = 0b11110100;

        NOT.execute(&mut emulator, args);
        dbg!(&emulator.registers);
        assert_eq!(emulator.registers[1], 0b00001011);
    }
    #[test]
    fn test_instruct_and() {
        test_math_instruct(&AND, 0b11001100, 0b10111011, 0b10001000);
    }
    #[test]
    fn test_instruct_or() {
        test_math_instruct(&OR, 0b01001000, 0b10110110, 0b11111110);
    }
    #[test]
    fn test_instruct_xor() {
        test_math_instruct(&XOR, 0b10011100, 0b11001100, 0b1010000);
    }
    #[test]
    fn test_peek(){
        let mut emulator = Emulator::hardware_setup();
        let data = vec![0b1011_0110, 0b1001_0001];
        let args = InstructionArgs::from_bytes(data);
        emulator.physical_memory[0b0110_1001] = 0xFA;

        PEEK.execute(&mut emulator, args);
        assert_eq!(emulator.registers[1], 0xFA);
    }
    #[test]
    fn test_poke(){
        let mut emulator = Emulator::hardware_setup();
        let data = vec![0b1011_0110, 0b1001_0001];
        let args = InstructionArgs::from_bytes(data);
        emulator.registers[1] = 0xFA;

        POKE.execute(&mut emulator, args);
        assert_eq!(emulator.physical_memory[0b0110_1001], 0xFA);
    }
    #[test]
    fn test_lodi(){
        let mut emulator = Emulator::hardware_setup();
        let data = vec![0b1101_0001, 0b1001_0001];
        let args = InstructionArgs::from_bytes(data);
        LODI.execute(&mut emulator, args);
        assert_eq!(emulator.registers[1], 0b1001_0001);
    }

    #[test]
    fn test_branch(){
        let mut emulator = Emulator::hardware_setup();
        let data = vec![0b1110_0001, 0b0011_0010, 0b0110_1010];
        let args = InstructionArgs::from_bytes(data);
        emulator.registers[1] = 0xFF;
        emulator.registers[2] = 0xFB;
        BRANCH.execute(&mut emulator, args);
        assert_eq!(emulator.registers[PC_REGISTER], 0b0110_1010);
    }

}
