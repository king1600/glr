#ifndef _GLR_OPCODES_H
#define _GLR_OPCODES_H

// opcode:[type:3, op:5]:8
// if opcode == CVT:
//   type = [ftoi, ftou, itou, itof, utoi, utof]
// if opcode == CALL:
//   type = [normal call, tail call, virtual call]
// if opcode == BRACH (JUMP, JE, etc.):
//   type = [u8, u16, u32, i8, i16, i32]
// if opcode == Object/Array:
//   type = [new, get, set, size]
// else:
//   type = [u8, u16, u32, u64, i32, i64, f32, f64]

#define GLR_OP_RET      0       // returns register
#define GLR_OP_LDC      1       // loads constant pool entry into register
#define GLR_OP_ADD      2       // adds 2 registers and places the result in another
#define GLR_OP_SUB      3       // subtracts 2 registers and places the result in another
#define GLR_OP_MUL      4       // multiplies 2 registers and places the result in another
#define GLR_OP_DIV      5       // divides 2 registers and places the result in another
#define GLR_OP_MOV      6       // moves one register into another
#define GLR_OP_NEG      7       // negates one register and places the result in another
#define GLR_OP_CVT      8       // converts one register type to another and places the result in another
#define GLR_OP_MOD      9       // computes the modules of 2 registers and places the result in another
#define GLR_OP_XOR      10      // xor's 2 register and places the result in another
#define GLR_OP_SHR      11      // bitshifts right one register by another and places the result in another
#define GLR_OP_SHL      12      // bitshifts left one register by another and places the result in another
#define GLR_OP_AND      13      // bitwise and's one register by another and places the result in another
#define GLR_OP_OR       14      // bitwise or's one register by another and places the result in another
#define GLR_OP_JMP      15      // jumps to another position in the bytecode
#define GLR_OP_JE       16      // compares 2 register and jumps to another position if theyre equal
#define GLR_OP_JNE      17      // compares 2 register and jumps to another position if theyre not equal
#define GLR_OP_JL       18      // compares 2 register and jumps to another position if one is less than other
#define GLR_OP_JLE      19      // compares 2 register and jumps to another position if one is less than or equal to other
#define GLR_OP_JG       20      // compares 2 register and jumps to another position if one is greater than other
#define GLR_OP_JGE      21      // compares 2 register and jumps to another position if one is greater than or equal to other
#define GLR_OP_JINST    22      // compares a registers object type and jumps to another position if equal
#define GLR_OP_CALL     23      // uses the arguments pushed on stack and performs a call (using the type)
#define GLR_OP_OBJ      24      // using the type, create, getfield, setfield or get size of object in register
#define GLR_OP_ARR      25      // using the type, create, getitem, setitem or get length of array in register
#define GLR_OP_THROW    26      // throw an exception object inside of a register

#endif // _GLR_OPCODES_H