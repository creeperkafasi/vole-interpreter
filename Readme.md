# VOLE interpreter written in Rust

Vole is a fictional machine language used for teaching Computer Science students.

[See here for the instruction set PDF](https://wyrd.hood.edu/~wcrum/it510/documents/VoleMachineLanguage.pdf)

Try it out by running:

`cargo run -- ./examples/fibonacci.vole`

or

`cargo run -- ./examples/toplama.vole` (Explained in Turkish)

## Deviations from the original VOLE instruction set

This interpreter adds 2 extra instructions for printing values into stdout:

- 0xDM0R prints the value of register R into stdout using the mode M
- 0xEMXY prints the value of memory cell XY into stdout using the mode M

Printing modes:
- 0xB -> **B**inary
- 0xC -> **C**haracter
- 0xE -> h**E**xadecimal
- 0xF -> **F**loating point
- Anything else -> Decimal
