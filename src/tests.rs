#[cfg(test)]
#[allow(clippy::module_inception)]
mod tests {

    use crate::instructions::*;

    #[test]
    fn test_args_opcode() {
        let data = vec![0b01010000, 0b11110000];
        let args = InstructionArgs::from_bytes(data);
        println!("{0}", args.opcode());
        assert_eq!(args.opcode(), 0b00000101);
    }
}
