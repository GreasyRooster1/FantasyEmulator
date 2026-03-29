# ASM Spec

### Instruction Set
```
0000 -> NOP
0001 -> ADD
0010 -> SUB
0011 -> MUL
0100 -> DIV
0101 -> 
0110 -> REM
0111 -> NOT
1000 -> AND
1001 -> OR 
1010 -> XOR
1011 -> PEEK
1100 -> POKE
1101 -> LODI
1110 -> BRANCH
1111 -> 
```

```
 0000    0000 0000 0000 0000 0000
OPCODE |        ARG SPACE         |
```

### Math ops (ADD, SUB, etc)
2 bytes
```
0000      0000   0000      0000
OPCODE | REG A | REG B | REG OUT |
```

### NOT
1 bytes
```
0000     0000
OPCODE | REG 
```

### PEEK
2 bytes
```
 1011    0000  0000   0000
OPCODE | MEMORY LOC | REG
```

### POKE
2 bytes
```
 1100    0000  0000   0000
OPCODE | MEMORY LOC | REG
```

### LODI
2 bytes
```
 1101    0000   0000 0000
OPCODE | REG  | IMMEDIATE 
```

### BRANCH
3 bytes
```
 1110  | 0000  |   0000    | 0000  | 0000 0000 
OPCODE | REG 1 | OPERATION | REG 2 | MEM LOC   |
```

### BRANCH Operations
- 0000 -> Jump
- 0001 -> Equals
- 0010 -> Not Equals
- 0011 -> Greater Than
- 0100 -> Greater Than or Equals
- 0101 -> Less Then
- 0110 -> Less Then or Equals
- 0111 ->

### NOP
1 byte
```
 0000  | 0001 |
OPCODE | DBG  |
```

### Debug Operations
- 0000 -> None
- 0001 -> Register Dump
- 0010 -> Report memory to file
- 0011 -> Report ROM to file
- 0100 -> 