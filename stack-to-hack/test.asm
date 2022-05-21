// push constant 8
@8
D=A
@SP
A=M
M=D
@SP
M=M+1

// push constant 3
@3
D=A
@SP
A=M
M=D
@SP
M=M+1

// and
@SP
M=M-1
A=M
D=M
@SP
M=M-1
A=M
M=D&M
@SP
M=M+1

// not
@SP
M=M-1
@SP
A=M
M=!M
@SP
M=M+1

// neg
@SP
M=M-1
@SP
A=M
M=-M
@SP
M=M+1

// infinite loop
@INFINITY
(INFINITY)
0;JMP
