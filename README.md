# Unnamed c64 emulator project

So, this is a hobby project with some general goals, and some competing thoughts on what to include or not. It's driven with a whole lot of love for the old 8-bit platforms as this was what I started to develop on myself back in the 80's. 🥰

There is an companion repo to this with some great documentation about the C64 internals that I have used as reference when building this. It's in it's own repo as those files are more static than normal code, and I didn't want to add all that to this repo.

[C64-Docs](https://github.com/MunchyYDL/c64-docs)

## General goals

First and foremost, I want to write a c64 emulator that is kind of simple and small without support for everything, so I might just skip to add support for BASIC, Cartridges and Tapes, as my main focus is demos.

I'd also like to be able to run it natively on my local computer, as well as on the web, so I will add support for WASM as a target.

## First steps

Key concepts:
- 6502/6510 CPU Emulation

###  Simple c64 code emulator

Emulation is a new journey for me personally, but this part should be fairly straight forward, as the instruction set and the MOS6510 is a fairly simple processor.

This step won't be to concerned with cycle perfect emulation and will instead focus on the functionally correct implementation of the CPU instructions and addressing modes.

This work will be done by the means of unit-tests and static verification of operands in isolation.

## Next

Maybe these sections will be broken down into smaller steps when I have some more understanding of them, but the general goals are in the headings.

### Simple execution

Key concepts:
- Loading code
- Running programs

### Basic output

Key concepts:
- VIC-II Emulation
- SID Emulation
- Desktop execution

### WASM

Key concepts:
- WASM Packaging
- Running on the web

### 1541 emulation

Key concepts:
- I/O Emulation
- CIA Emulation
- 1541 Emulation

Basic support of .d64 images, to support easier loading of code for testing purposes and to drive the development of better support for everything. 

> ⚠️ I might need to add this earlier to get the bare minimum working, or possibly some other means of loading existing code into the emulator.

### C64 cycle perfect emulation

Key concepts:
- PAL/NTSC
- VIC-II / CPU cycle considerations
- Bad lines

At this point, I presume that to be able to run demos in a good way, cycle perfect emulation needs to be looked at. While we can probably achieve a decent result for many demos without this, some demos need this in order to be rendered correctly, as they depend on the timings of everything to be consistent.

### C64 tracker enabled emulator

Key concepts:
- Advanced I/O, CIA & 1541 emulation

I need to understand a bit more and do some experiments to get a better understanding of what to include to get this working.

Better/more advanced 1541 emulation is needed to be able to load disk images (.d64) directly and support tracker style demos as well.
