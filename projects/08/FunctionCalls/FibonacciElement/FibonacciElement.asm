// SP = 256
@256
D=A
@SP
M=D
// call Sys.init
@Sys.init$return.0
D=A
@SP
A=M
M=D
@SP
M=M+1
@LCL
D=M
@SP
A=M
M=D
@SP
M=M+1
@ARG
D=M
@SP
A=M
M=D
@SP
M=M+1
@THIS
D=M
@SP
A=M
M=D
@SP
M=M+1
@THAT
D=M
@SP
A=M
M=D
@SP
M=M+1
@SP
D=M
@5
D=D-A
@ARG
M=D
@SP
D=M
@LCL
M=D
@Sys.init
0;JMP
(Sys.init$return.0)

// This file is part of www.nand2tetris.org
// and the book "The Elements of Computing Systems"
// by Nisan and Schocken, MIT Press.
// File name: projects/08/FunctionCalls/FibonacciElement/Main.vm

// Computes the n'th element of the Fibonacci series, recursively.
// n is given in argument[0].  Called by the Sys.init function
// (part of the Sys.vm file), which also pushes the argument[0]
// parameter before this code starts running.

// function Main.fibonacci 0
(Main.fibonacci)

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

// push constant 2
@2
D=A
@SP
A=M
M=D
@SP
M=M+1

// lt                     // checks if n<2
@SP
M=M-1
A=M
D=M
@SP
M=M-1
A=M
D=M-D
@Main.fibonacci$LT
D;JLT
D=0
@Main.fibonacci$DONE
0;JMP
(Main.fibonacci$LT)
D=-1
(Main.fibonacci$DONE)
@SP
A=M
M=D
@SP
M=M+1

// if-goto IF_TRUE
@SP
M=M-1
A=M
D=M
@Main.fibonacci$IF_TRUE
D;JNE

// goto IF_FALSE
@Main.fibonacci$IF_FALSE
0;JMP

// label IF_TRUE          // if n<2, return n
(Main.fibonacci$IF_TRUE)

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

// return
@LCL
D=M
@R14
M=D
@5
A=D-A
D=M
@R15
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
@R15
A=M
0;JMP

// label IF_FALSE         // if n>=2, returns fib(n-2)+fib(n-1)
(Main.fibonacci$IF_FALSE)

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

// push constant 2
@2
D=A
@SP
A=M
M=D
@SP
M=M+1

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

// call Main.fibonacci 1  // computes fib(n-2)
@Main.fibonacci$return.1
D=A
@SP
A=M
M=D
@SP
M=M+1
@LCL
D=M
@SP
A=M
M=D
@SP
M=M+1
@ARG
D=M
@SP
A=M
M=D
@SP
M=M+1
@THIS
D=M
@SP
A=M
M=D
@SP
M=M+1
@THAT
D=M
@SP
A=M
M=D
@SP
M=M+1
@SP
D=M
@6
D=D-A
@ARG
M=D
@SP
D=M
@LCL
M=D
@Main.fibonacci
0;JMP
(Main.fibonacci$return.1)

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

// push constant 1
@1
D=A
@SP
A=M
M=D
@SP
M=M+1

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

// call Main.fibonacci 1  // computes fib(n-1)
@Main.fibonacci$return.2
D=A
@SP
A=M
M=D
@SP
M=M+1
@LCL
D=M
@SP
A=M
M=D
@SP
M=M+1
@ARG
D=M
@SP
A=M
M=D
@SP
M=M+1
@THIS
D=M
@SP
A=M
M=D
@SP
M=M+1
@THAT
D=M
@SP
A=M
M=D
@SP
M=M+1
@SP
D=M
@6
D=D-A
@ARG
M=D
@SP
D=M
@LCL
M=D
@Main.fibonacci
0;JMP
(Main.fibonacci$return.2)

// add                    // returns fib(n-1) + fib(n-2)
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

// return
@LCL
D=M
@R14
M=D
@5
A=D-A
D=M
@R15
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
@R15
A=M
0;JMP

// This file is part of www.nand2tetris.org
// and the book "The Elements of Computing Systems"
// by Nisan and Schocken, MIT Press.
// File name: projects/08/FunctionCalls/FibonacciElement/Sys.vm

// Pushes a constant, say n, onto the stack, and calls the Main.fibonacii
// function, which computes the n'th element of the Fibonacci series.
// Note that by convention, the Sys.init function is called "automatically"
// by the bootstrap code.

// function Sys.init 0
(Sys.init)

// push constant 4
@4
D=A
@SP
A=M
M=D
@SP
M=M+1

// call Main.fibonacci 1   // computes the 4'th fibonacci element
@Main.fibonacci$return.3
D=A
@SP
A=M
M=D
@SP
M=M+1
@LCL
D=M
@SP
A=M
M=D
@SP
M=M+1
@ARG
D=M
@SP
A=M
M=D
@SP
M=M+1
@THIS
D=M
@SP
A=M
M=D
@SP
M=M+1
@THAT
D=M
@SP
A=M
M=D
@SP
M=M+1
@SP
D=M
@6
D=D-A
@ARG
M=D
@SP
D=M
@LCL
M=D
@Main.fibonacci
0;JMP
(Main.fibonacci$return.3)

// label WHILE
(Sys.init$WHILE)

// goto WHILE              // loops infinitely
@Sys.init$WHILE
0;JMP

