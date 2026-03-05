## Xis – An Xperimental Instruction Set

Xis is an experimental instruction set architecture (for a potential 16 bit computer) inspired by a [video](https://www.youtube.com/watch?v=hAZEXqWLTmY) from [Mattbatwings](https://github.com/mattbatwings).

Xis can be imagined as the instruction set for a simple, Game Boy–like system — compact, focused, and intentionally made for games.

### Overview

**This project consists of:**

- A custom instruction set
- A compiler for translating source code into c16 bytecode (c16 stands for compiled xis16)
- A debugger that helps you find errors
- A virtual machine capable of executing c16 programs

The VM uses the [minifb](https://github.com/emoon/rust_minifb) library for a simple way of displaying pixels in real time.

### Goals

**This project is intended as:**

- A learning project
- A base for building small games


### Status

Xis is experimental and subject to change. The instruction set, compiler, and VM may evolve as the design is refined.