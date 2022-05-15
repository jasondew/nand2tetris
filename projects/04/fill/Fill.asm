// This file is part of www.nand2tetris.org
// and the book "The Elements of Computing Systems"
// by Nisan and Schocken, MIT Press.
// File name: projects/04/Fill.asm

// Runs an infinite loop that listens to the keyboard input.
// When a key is pressed (any key), the program blackens the screen,
// i.e. writes "black" in every pixel;
// the screen should remain fully black as long as the key is pressed. 
// When no key is pressed, the program clears the screen, i.e. writes
// "white" in every pixel;
// the screen should remain fully clear as long as no key is pressed.

(LOOP)
// if RAM[KBD] != 0 GOTO KEYDOWN else GOTO LOOP
@KBD
D=M
@KEYDOWN
D;JNE
@LOOP
0;JMP

(KEYDOWN)
// RAM[0] = KEYDOWN_LOOP
@KEYDOWN_LOOP
D=A
@R0
M=D
// GOTO FILL_BLACK
@FILL_BLACK
0;JMP

(KEYDOWN_LOOP)
// if RAM[KBD] == 0 GOTO KEYUP else GOTO KEYDOWN_LOOP
@KBD
D=M
@KEYUP
D;JEQ
@KEYDOWN_LOOP
0;JMP

(KEYUP)
// RAM[0] = LOOP
@LOOP
D=A
@R0
M=D
// GOTO FILL_WHITE
@FILL_WHITE
0;JMP

(FILL_BLACK)
// D = 8191
@8191
D=A

(FILL_BLACK_LOOP)
// RAM[SCREEN + D] -= 1
@SCREEN
A=A+D
M=M-1
// D -= 1
D=D-1

// if D < 0 GOTO FILL_BLACK_DONE else GOTO FILL_BLACK_LOOP
@FILL_BLACK_DONE
D;JLT
@FILL_BLACK_LOOP
0;JMP

(FILL_BLACK_DONE)
// GOTO RAM[0]
@R0
A=M
0;JMP

(FILL_WHITE)
// D = 8191
@8191
D=A

(FILL_WHITE_LOOP)
// RAM[SCREEN + D] += 1
@SCREEN
A=A+D
M=M+1
// D -= 1
D=D-1

// if D < 0 GOTO FILL_WHITE_DONE else GOTO FILL_WHITE_LOOP
@FILL_WHITE_DONE
D;JLT
@FILL_WHITE_LOOP
0;JMP

(FILL_WHITE_DONE)
// GOTO RAM[0]
@R0
A=M
0;JMP
