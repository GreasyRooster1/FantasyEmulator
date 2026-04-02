
opcodes = {
    "NOP": 0,
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

machine_code = []


with open('example.txt', 'r') as file:
    for line in file:
        print(line.strip())
        words = line.split()
        opcode = opcodes[words[0]]
        args = []
        words.pop(0)
        for word in words
            args.append(int(word)
        machine_code.append(opcode)
        for arg in args
            machine_code.append(arg)

print(machine_code)

