A5: rum
Marceline Kelly
CSC 411

During the course of this assigment, I shared ideas with Nicolas Leffray, but all code is my own.

To my knowledge, all parts of the UM are correctly implemented except the execution step. On large binaries, the program seems to get stuck in some sort of loop and exits too early.

This UM is composed of the following modules:
- rumload
  - Loads a UM binary into memory as a series of 32-bit words
- rumdis
  - Takes the words from rumload and builds instructions with opcodes, registers, and values
- rummem
  - Manages program memory
  - Maps & unmaps virtual memory segments
  - Reduces need for direct access to memory structure
- rumio
  - Uses stdin/stdout for input/output
- rumrun
  - Executes the program
  - Manages registers (a simple array of eight 32-bit words)

My UM takes less than 10 ms to execute 50 million instructions, based on the timing of "sandmark" and some simple calculations. I do not believe this is correct and believe the program exits early somewhere.

The time I spent on this assignment is as follows:
- Analysis: 3 hours
- Design: 2 hours
- Solution: 5 hours
- TOTAL: 10 hours (and counting as I continue to debug the program)
