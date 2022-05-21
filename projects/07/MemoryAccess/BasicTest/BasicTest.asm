@32767
// push constant 10
@10
D=A
@SP
A=M
M=D
@SP
M=M+1

@32767
// pop local 0
@LCL
D=M
@0
D=A+D
@R13
M=D
@SP
M=M-1
A=M
D=M
@R13
A=M
M=D

@32767
// push constant 21
@21
D=A
@SP
A=M
M=D
@SP
M=M+1

@32767
// push constant 22
@22
D=A
@SP
A=M
M=D
@SP
M=M+1

@32767
// pop argument 2
@ARG
D=M
@2
D=A+D
@R13
M=D
@SP
M=M-1
A=M
D=M
@R13
A=M
M=D

@32767
// pop argument 1
@ARG
D=M
@1
D=A+D
@R13
M=D
@SP
M=M-1
A=M
D=M
@R13
A=M
M=D

@32767
// push constant 36
@36
D=A
@SP
A=M
M=D
@SP
M=M+1

@32767
// pop this 6
@THIS
D=M
@6
D=A+D
@R13
M=D
@SP
M=M-1
A=M
D=M
@R13
A=M
M=D

@32767
// push constant 42
@42
D=A
@SP
A=M
M=D
@SP
M=M+1

@32767
// push constant 45
@45
D=A
@SP
A=M
M=D
@SP
M=M+1

@32767
// pop that 5
@THAT
D=M
@5
D=A+D
@R13
M=D
@SP
M=M-1
A=M
D=M
@R13
A=M
M=D

@32767
// pop that 2
@THAT
D=M
@2
D=A+D
@R13
M=D
@SP
M=M-1
A=M
D=M
@R13
A=M
M=D

@32767
// push constant 510
@510
D=A
@SP
A=M
M=D
@SP
M=M+1

@32767
// pop temp 6
@R11
D=A
@R13
M=D
@SP
M=M-1
A=M
D=M
@R13
A=M
M=D

@32767
// push local 0
@LCL
D=M
@0
D=A+D
@R13
M=D
@R13
A=M
D=M
@SP
A=M
M=D
@SP
M=M+1

@32767
// push that 5
@THAT
D=M
@5
D=A+D
@R13
M=D
@R13
A=M
D=M
@SP
A=M
M=D
@SP
M=M+1

@32767
// add
@SP
M=M-1
A=M
D=M
@SP
M=M-1
A=M
M=M+D
@SP
M=M+1

@32767
// push argument 1
@ARG
D=M
@1
D=A+D
@R13
M=D
@R13
A=M
D=M
@SP
A=M
M=D
@SP
M=M+1

@32767
// sub
@SP
M=M-1
A=M
D=M
@SP
M=M-1
A=M
M=M-D
@SP
M=M+1

@32767
// push this 6
@THIS
D=M
@6
D=A+D
@R13
M=D
@R13
A=M
D=M
@SP
A=M
M=D
@SP
M=M+1

@32767
// push this 6
@THIS
D=M
@6
D=A+D
@R13
M=D
@R13
A=M
D=M
@SP
A=M
M=D
@SP
M=M+1

@32767
// add
@SP
M=M-1
A=M
D=M
@SP
M=M-1
A=M
M=M+D
@SP
M=M+1

@32767
// sub
@SP
M=M-1
A=M
D=M
@SP
M=M-1
A=M
M=M-D
@SP
M=M+1

@32767
// push temp 6
@R11
D=M
@SP
A=M
M=D
@SP
M=M+1

@32767
// add
@SP
M=M-1
A=M
D=M
@SP
M=M-1
A=M
M=M+D
@SP
M=M+1

// infinite loop
@INFINITY
(INFINITY)
0;JMP
