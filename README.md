## Xis – An *X*perimental *I*nstruction *S*et

Xis is an experimental instruction set architecture (ISA) inspired by a [video](https://www.youtube.com/watch?v=hAZEXqWLTmY) from [Mattbatwings](https://github.com/mattbatwings).

Xis can be imagined as the instruction set for a simple, Game Boy–like system — compact, focused, and intentionally made for games.

### Overview

**This project consists of:**

- A custom instruction set

- A compiler for translating source code into c16 bytecode (c16 stands for compiled xis16)

- A virtual machine capable of executing xis16 or c16 programs


The VM uses the [minifb](https://github.com/emoon/rust_minifb) library for a simple way of displaying pixels in real time.

**Goals**

This project is intended as:

- A learning project *and* tool

- A base for building small funny games


### Status

**Xis is experimental and subject to change. The instruction set, compiler, and VM may evolve as the design is refined.**