@32767
// function SimpleFunction.test 2
(SimpleFunction.SimpleFunction.test)
@SP
A=M
M=0
@SP
M=M+1
@SP
A=M
M=0
@SP
M=M+1

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
// push local 1
@LCL
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
// not
@SP
M=M-1
@SP
A=M
M=!M
@SP
M=M+1

@32767
// push argument 0
@ARG
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
// return
@LCL
D=M
@R14
M=D
@SP
M=M-1
A=M
D=M
@ARG
A=M
M=D
@ARG
D=M
@SP
M=D+1
@R14
M=M-1
A=M
D=M
@THAT
M=D
@R14
M=M-1
A=M
D=M
@THIS
M=D
@R14
M=M-1
A=M
D=M
@ARG
M=D
@R14
M=M-1
A=M
D=M
@LCL
M=D
@R14
M=M-1
A=M
0;JMP

// infinite loop
@INFINITY
(INFINITY)
0;JMP
