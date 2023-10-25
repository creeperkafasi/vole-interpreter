# VOLE interpreter written in Rust

Vole is a fictional machine language used for teaching Computer Science students.

[See here for the instruction set PDF](https://wyrd.hood.edu/~wcrum/it510/documents/VoleMachineLanguage.pdf)

Try it out by running:

`cargo run -- ./examples/fibonacci.vole`

or

`cargo run -- ./examples/toplama.vole` (Explained in Turkish)

## Deviations from the original VOLE instruction set

This interpreter adds 2 extra instructions for printing values into stdout:

- 0xD00R prints the value of register R into stdout
- 0xE0XY prints the value of memory cell XY into stdout
