# Changelog

## 2024-04-27
- Change license to be GPL 3.0
- Now print Timendus logo for testing instead of IBM logo
- Implement 00EE - Return from subroutine
- Implement 2NNN - CALL addr
- Implement FX65 - LD Vx, [I]
- Implement FX33 - BCD
- Fixes issues for printing IBM logo
- Framebuffer is now part of the chip8

## 2024-04-26
- Use env logger
- Remove the use of framebuffer for trying to solve the issue with IBM logo

## 2024-01-28
- Implement DXYN, 1NNN, 7XNN
- Implement ANNN, 6XNN

## 2024-01-26
- Implement clear screen instruction
- Implement functions to get information from an opcode
- Add the switch case for emulation based on the upper 4 bits of the opcode
- Add fonts
- Display a space invader in the top left corner
    - use minifb
- Add function to dump all memory
- Load the rom into chip memory
- Read a rom as parameter and print opcode
- Run cargo init
