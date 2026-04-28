import sys

opcodes = {
    "NOP": 0b0000_0000,
    "DUMPMEM": 0b0000_0001,
    "DUMPROM": 0b0000_0010,
    "DBGREG": 0b0000_0011,

    "ADD":  0b0001_0000,
    "ADDI": 0b0001_0001,
    "SUB":  0b0001_0010,
    "SUBI": 0b0001_0011,
    "MUL":  0b0001_0100,
    "MULI": 0b0001_0101,
    "DIV":  0b0001_0110,
    "DIVI": 0b0001_0111,
    "MOD":  0b0001_1000,
    "MODI": 0b0001_1001,

    "AND":  0b0010_0000,
    "ANDI": 0b0010_0001,
    "OR":   0b0010_0010,
    "ORI":  0b0010_0011,
    "XOR":  0b0010_0100,
    "XORI": 0b0010_0101,
    "NOT":  0b0010_0110,
    "RSH":  0b0010_0111,
    "RSHI": 0b0010_1000,
    "LSH":  0b0010_1001,
    "LSHI": 0b0010_1010,

    "STOW": 0b0011_0000,
    "STOH": 0b0011_0001,
    "STOB": 0b0011_0010,
    # placeholder
    "LODW": 0b0011_0100,
    "LODH": 0b0011_0101,
    "LODB": 0b0011_0110,
    "LODI": 0b0011_0111,

    "JMP":  0b0100_0000,
    "BREQ": 0b0100_0001,
    "BRNEQ":0b0100_0010,
    "BRGT": 0b0100_0011,
    "BRGTE":0b0100_0100,
    "BRLT": 0b0100_0101,
    "BRLTE":0b0100_0110,
    "BREZ": 0b0100_0111,
    "BRNEZ":0b0100_1000,
    "CALL": 0b0100_1001,
    "RET":  0b0100_1010,
    "HALT": 0b0100_1011,

}

branch_ops = [
    "jump",
    "==",
    "!=",
    ">",
    ">=",
    "<",
    "<="
]

machine_code = []


with open('programs/'+sys.argv[1]+'.asm', 'r') as file:
    for line in file:
        if line.startswith(";") or len(line.strip())<1:
            continue
        print(line.strip())
        for i in range(len(branch_ops)):
            line = line.replace(branch_ops[i],str(i))
        words = line.replace("_"," ").split()
        opcode = opcodes[words[0]]
        args = []
        words.pop(0)
        for word in words:
            word = word.replace("r","",1)
            if word.startswith("0x"):
                args.append(int(word[2:],16))
            elif word.startswith("b"):
                args.append(int(word[1:],2))
            else:
                args.append(int(word))
        machine_code.append(opcode)
        for arg in args:
            machine_code.append(arg)

compressed = []
for i in range(0,len(machine_code),2):
    compressed.append(machine_code[i+1]+(machine_code[i]<<4))

byte_data = bytes(compressed)

with open('rom.rom', 'wb') as f:
    f.write(byte_data)

print(machine_code)
print(len(machine_code))

