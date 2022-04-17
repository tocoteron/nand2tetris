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
    @color
    M=0
    @KBD
    D=M
    @DRAW
    D;JEQ
(BLACK)
    @color
    D=M
    D=!D
    M=D
(DRAW)
    @SCREEN
    D=A
    @cur
    M=D
(DRAW_LOOP)
    // if cur >= 16384 + 8192 then break
    @cur
    D=M
    @24576
    D=D-A
    @LOOP
    D;JGE
    // Draw color
    @color
    D=M
    @cur
    A=M
    M=D
    // Move cursor
    @cur
    M=M+1
    // Loop
    @DRAW_LOOP
    0;JMP
