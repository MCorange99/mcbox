# spec

32 bit cpu

256 MB ram

## Registers

There are 4 registers: A, B, C and D.
But they can be refernced with diffrent sizes (Byte, Word, Double), kinda like x86

```txt
A -> BA, WA, DA
B -> BB, WB, DB
C -> BC, WC, DC
D -> BD, WD, DD
```

## ops (short)

| name    | hex  | dec | explanation                                              |
|---------|------|-----|----------------------------------------------------------|
| NOP     | 0x01 | 01  | Skips an instruction                                     |
| MOVIA   | 0x02 | 02  | Moves immediate into A                                   |
| MOVIB   | 0x03 | 03  | Moves immediate into B                                   |
| MOVIC   | 0x04 | 04  | Moves immediate into C                                   |
| MOVID   | 0x05 | 05  | Moves immediate into D                                   |
| MOVAM   | 0x06 | 06  | Moves A into memory                                      |
| MOVBM   | 0x07 | 07  | Moves B into memory                                      |
| MOVCM   | 0x08 | 08  | Moves C into memory                                      |
| MOVDM   | 0x09 | 09  | Moves D into memory                                      |
| MOVIM   | 0x0A | 10  | Moves immediate into                                     |
| PRNT    | 0x0B | 11  | Prints char in A to 7 seg                                |
| ADD     | 0x0C | 12  | Adds the value to register A                             |
| SUB     | 0x0D | 13  | Subtracts the value from reg A                           |
| INC     | 0x0E | 14  | Increments value                                         |
| DEC     | 0x0F | 15  | Decrements value                                         |
| LDI     | 0x10 | 16  | Loads immediate value                                    |
| HLT     | 0x11 | 17  | Stops the clock                                          |
| CALL    | 0x12 | 18  | Jumps to a label and pushes the return addr to stack     |
| RET     | 0x13 | 19  | Jumps to the address in the stack                        |
| AND     | 0x14 | 20  | Binary AND                                               |
| OR      | 0x15 | 21  | Binary OR                                                |
| XOR     | 0x16 | 22  | Binary XOR                                               |
| JMP     | 0x17 | 23  | Unconditional jump                                       |
| JNE     | 0x18 | 24  | Jumps if EQ flag is not set                              |
| JE      | 0x19 | 25  | Jumps if EQ flag is set                                  |
| JZ      | 0x1A | 26  | Jumps if ZERO flag is set                                |
| JNZ     | 0x1B | 27  | Jumps if ZERO flag is not set                            |
| JGT     | 0x1C | 28  | Jumps if GT flag is set                                  |
| JGE     | 0x1D | 29  | Jumps if GT or EQ flag is set                            |
| JLT     | 0x1E | 30  | Jumps if LT flag is set                                  |
| JLE     | 0x1F | 31  | Jumps if LT or EQ flag is set                            |
| JO      | 0x20 | 32  | Jumps if overflow                                        |
| CMP     | 0x21 | 33  | Compares the num in reg A with num in argument           |
| POPA    | 0x22 | 34  | Pops the last element from the stack into A              |
| POPB    | 0x23 | 35  | Pops the last element from the stack into B              |
| POPC    | 0x24 | 36  | Pops the last element from the stack into C              |
| POPD    | 0x25 | 37  | Pops the last element from the stack into D              |
| POPM    | 0x26 | 38  | Pops the last element from the stack into an address     |
| PUSHA   | 0x27 | 39  | Push the value A to the back of the stack                |
| PUSHB   | 0x28 | 40  | Push the value B to the back of the stack                |
| PUSHC   | 0x29 | 41  | Push the value C to the back of the stack                |
| PUSHD   | 0x2A | 42  | Push the value D to the back of the stack                |
| PUSHM   | 0x2B | 43  | Push the value of an address to the back of the stack    |
| PUSHI   | 0x2C | 43  | Push an immediate value to the back of the stack         |

## ops (long)

### sizes

| index | size                 |
|-------|----------------------|
| 0b00  | 8b  (byte)           |
| 0b01  | 16b (word)           |
| 0b10  | 32b (dword)          |
| 0b11  | Argument not needed  |

Ops are stored like this:

```txt
00000000 00000000 00000000 00000000 00000000 00000000
|------| |------| |-------|--------|--------|-------|
|               |                                   |
|               |                                   | number or address (from 0 to 32 bits, depending on size)
|               |
|               | Op Size (2 bits), Arg size (2 bits) reserved space (4 bits)
|         
| op code (8 bits)
```
