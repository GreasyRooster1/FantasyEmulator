import sys

opcodes = {
    "NOP": 0b00000000,

    "ADD": 1,
    "SUB": 2,
    "MUL": 3,
    "DIV": 4,
    "INC": 5,
    "REM": 6,
    "NOT": 7,
    "AND": 8,
    "OR": 9,
    "XOR": 10,
    "PEEK": 11,
    "POKE": 12,
    "LODI": 13,
    "BRANCH": 14,
    "HALT": 15,
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

