# Chip8
## Emulator
An emulator is a computer program that mimics the internal design and functionality of a computer system (System A)

## opcode
It's essentially a code that instructs a computer's processor to perform a specific operation. Think of it as a command in a machine language that the CPU understands. 

* The graphics system: The chip 8 has one instruction that draws sprite to the screen.
Drawing is done in XOR mode and if a pixel is turned off as a result of drawing, the VF register is set. T
his is used for collision detection.

## Interupts 
An interrupt is a signal sent to a processor by a hardware device or software to request immediate attention.
This mechanism allows a system to respond to events in a timely manner, even when the processor is busy with other tasks.

## Hardware Registers
Hardware registers are small storage locations within a CPU that are used to store data temporarily.
They are used to hold data that is being processed by the CPU, such as instructions, operands, and intermediate results.

* The graphics of the Chip 8 are black and white and the screen has a total of 2048 pixels (64 x 32)
* Interupts and hardware registers. The Chip 8 has none, but there are two timer registers that count at 60 Hz. 
* the Chip 8 has a HEX based keypad (0x0-0xF), you can use an array to store the current state of the key.
* Chip 8 instruction set has opcodes that allow the program to jump to a certain address or call a subroutine.
