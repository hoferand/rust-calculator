<!-- PROJECT NAME -->
# Calculator
A simple Rust library for evaluating mathematical expressions.



<!-- TABLE OF CONTENTS -->
## Table of Contents
 1. [About The Project](#about-the-project)
 2. [Getting Started](#getting-started)
 3. [Usage](#usage)
 4. [Features](#features)
 5. [License](#license)



<!-- ABOUT THE PROJECT -->
## About The Project

A simple calculator library using a lexer and parser for evaluating mathematical expressions.  
First the input gets split into single tokens e.g.: `4.5 + 5` => [`4.5`, `+`, `5`]  
These tokens are now used by the parser to calculate the result e.g.: [`4.5`, `+`, `5`] => `9.5`



<!-- GETTING STARTED -->
## Getting Started

For using this calculator you only have to install the Rust language.  
Instruction for installing it can be found here: [Install Rust](https://www.rust-lang.org/tools/install).  
Afterwards you can clone this repository and run it as described in [Usage](#usage).



<!-- USAGE EXAMPLES -->
## Usage

You can try this calculator by running the binary in the `examples/cli-calculator` directory.
```
$ cargo run
Simple calculator in Rust
> 4 + 5
= 9

> (4 + 5.5) * -9
= -85.5

> (5 + 5(       
Error: Unexpected token `(` found!
 | (5 + 5(
 |       ^

>
```

Or using it in your own bianry:
```rust
use calculator::*;

fn main() {
  let expr = "3 * -(4 + 5)";
  let mut calculator = Calculator::new();
	
	let val = calculator.calculate(expr).unwrap();
  println!("{}", val); // prints `-27`
}
```



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
 - Power: `**`
 - Root: `//`


### Brackets

For changing the evaluation order of the expressions you can use brackets `( ... )`.  
The brackets can be nested arbitrarily `(( ... ) + ( ... ))`.


### Signs

 - Positive number: `+4.5`
 - Negative number: `-4.5`

You can apply signs multiple times like `--4` which evaluates to `4`.


### Variables

It is also possible to use variables to store results and reuse it in other calculations.  
The variable names may only consist of letters `a-zA-Z`, numbers `0-9` and `_`, but cannot start with a number.

Predefined variables:
 - Pi: `pi`
 - Euler number: `e`
 - Last result: `$` (only defined after the first evaluation)

 Example: `a = 4 * 5` and after `a + 4` evaluates to `24`


### Functions

Functions can not be defined by yourself but there are some predefined:
 - Sinus: `sin`
 - Arcus-Sinus: `asin`
 - Cosinus: `cos`
 - Arcus-Cosinus: `acos`
 - Tangens: `tan`
 - Arcus-Tangens: `atan`
 - Radiants to Degrees: `r2d`
 - Degrees to Radiants: `d2r`

**All trigonometric functions uses radiants.**

Example: `r2d pi` evaluates to `180`


### Operation Order

 1. Numbers / Variables: `3`, `4.5`, `var`, etc
 2. Brackets: `( ... )`
 3. Signs: `+`, `-`
 4. Function calls: `r2d`, `sin`, etc
 5. Exponential Operators: `**`, `//`
 6. Multiplicative Operators: `*`, `/`, `%`
 7. Additions Operators: `+`, `-`

So `5 + -4 * 5 + r2d pi + 12` is evaluated as `5 + ((-4) * 5) + (r2d pi) + 12`.



<!-- LICENSE -->
## License

Distributed under the MIT License. See [LICENSE.txt](LICENSE.txt) for more information.
