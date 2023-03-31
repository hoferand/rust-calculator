<!-- PROJECT NAME -->
# Calculator
A simple command line calculator built with Rust.



<!-- TABLE OF CONTENTS -->
## Table of Contents
 1. [About The Project](#about-the-project)
 2. [Getting Started](#getting-started)
 3. [Usage](#usage)
 4. [Features](#features)
 5. [License](#license)



<!-- ABOUT THE PROJECT -->
## About The Project

A simple calculator using a lexer and parser for evaluating mathematical expressions.  
After the expression is read from the command line it gets split into single tokens (`Number`, `Operator`, `Bracket`, etc).  
These tokens are now used by the parser to calculate the result.




<!-- GETTING STARTED -->
## Getting Started

For using this calculator you only have to install the Rust language.  
Instruction for installing it can be found here: [Install Rust](https://www.rust-lang.org/tools/install).  
Afterwards you can clone this repository and run it as described in [Usage](#usage).



<!-- USAGE EXAMPLES -->
## Usage

You can use this calculator by running `cargo run` in your shell.
```
$ cargo run
Simple calculator in Rust
> 4 + 5
= 9

> (4 + 5.5) * -9
= -85.5

> (5 + 5(       
Error: Unexpected token found [(]!

>
```

And building it by running `cargo build` in your shell.  
After that you can find the executable in `target/debug/calculator`.



<!-- FEATURES -->
## Features

### Numbers
You can use integer `45` as well as floating point values `45.43`.


### Supported Operators

 - Addition: `+`
 - Subtraction: `-`
 - Multiplication: `*`
 - Division: `/`
 - Modulo: `%`


### Brackets

For changing the evaluation order of the expressions you can use brackets `( ... )`.  
The brackets can be nested arbitrarily `(( ... ) + ( ... ))`.


### Signs

 - Positive number: `+4.5`
 - Negative number: `-4.5`

You can apply signs multiple times like `--4` which evaluates to `4`.


### Operation Order

 1. Brackets
 2. Signs
 3. Multiplicative Operators: `* / %`
 4. Additional Operators: `+ -`

So `5 + -4 * 5` is evaluated as `5 + ((-4) * 5)`.



<!-- LICENSE -->
## License

Distributed under the MIT License. See [LICENSE.txt](LICENSE.txt) for more information.
