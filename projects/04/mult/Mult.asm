// This file is part of www.nand2tetris.org
// and the book "The Elements of Computing Systems"
// by Nisan and Schocken, MIT Press.
// File name: projects/04/Mult.asm

// Multiplies R0 and R1 and stores the result in R2.
// (R0, R1, R2 refer to RAM[0], RAM[1], and RAM[2], respectively.)
//
// This program only needs to handle arguments that satisfy
// R0 >= 0, R1 >= 0, and R0*R1 < 32768.

// R2 = 0
@R2
M=0

// GOTO END if R0 = 0
@R0
M;JEQ

// GOTO END if R1 = 0
@R1
M;JEQ

// R2 = R0
@R0
D=M
@R2
M=D

// i = R1
@R1
D=M
@i
M=D

// while i > 1
(LOOP)
@i
D=M-1
@END
D;JLE

// R2 += R0
@R0
D=M
@R2
M=M+D

// i -= 1
@i
M=M-1

// GOTO LOOP
@LOOP
0;JMP

(END)
@END
0;JMP
