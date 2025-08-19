# Kiroku 

Kiroku is an application that takes a boolean expression like `A + B * C ^ D` and creates a circuit, truth table and Karnaugh Map. 


## Symbols 

`!` = NOT

`&` = AND

`|` = OR 

`^` = XOR 

`!&` = NAND 

`!|` = NOR 

`!^` = XNOR


Enter an expression like this 

```bash 
A ^ B !| C & (!D !^ E)
```

and press `Enter`


## Installation 

### From crates.io

```bash
cargo install kiroku
```

### From source 

Clone this repo and `cd` into it and run 

```bash
cargo run --release
```

## Features 

- [x] Truth Table generator (From expression)
- [x] Beautify the Table
- [x] Add support for XOR, XNOR , NOR and NAND operations
- [x] User input for expression
- [ ] K-Map generation
- [ ] Circuit Generation

## Outputs

![output_truth_table](./tui_tt.gif)
