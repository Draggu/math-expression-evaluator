# Math expression evaluator

## Table of content

- [Build](#Build)
- [Features](#Features)
- [Usage](#Usage)
- [Technologies](#Technologies)
- [Purpose](#Purpose)

## Build

run

```sh
cargo build --release
```

## Features

- operator precedense
- operator associativity
- nesting
- functions
- function from operator
- pipes
- variables

## Usage

for convenience `target/release/math-expression-evaluator` will be called `calc`

`all whitespace characters are ignored!`

to evaluate expression run following

```sh
calc "expression"
```

to use variables specifie them in following format

```sh
calc --var <var_name>=<value> "expression"
```

example

```sh
calc --var a=1 --var b=2 "a+b-3"
```

parentheses are respected

```sh
calc "4*(2+3)"
```

outputs `20` as expected

functions can be used in different ways

classic call (trailing comma is optional)

```sh
calc "add(2,5,)"
```

converting operator to function

```sh
calc "(+)(2,5)"
```

every function is automatically curried

```sh
calc "(+)(2)(5)"
calc "add(2)(5)"
```

pipes syntax is

```sh
calc "<value> | <one_arg_function>"
```

example

```sh
calc "2|(+)(4)|(^)(3)"
```

and it's equal to

```sh
calc "3^(2+(4))"
```

## Technologies

- [Rust](https://www.rust-lang.org/)

## Purpose

this project was created to learn rust and parsers
