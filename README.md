# 🔐 PassGen

A simple and secure password generator written in Rust.

## Features

- 🎲 Cryptographically secure random generation
- 🔧 Customizable length
- 🔣 Optional symbols
- 📊 Generate multiple passwords at once
- ✅ Full test coverage

## Installation

```bash
cargo install --path .
```

## Usage

```bash
# Generate a password (default: 16 characters)
passgen

# Custom length
passgen --length 20

# With symbols
passgen --symbols

# Generate multiple passwords
passgen --count 5 --length 24 --symbols
```

## Examples

```bash
$ passgen --length 12
🔐 aB3xZ9mQ2pL7

$ passgen --symbols --count 3
🔐 kT8@wY6^jH5*vC2&
🔐 fD7#sA4$bN3@zX8%
🔐 mP9!gQ1^tR4&nL6$
```

## Options

```
-l, --length <LENGTH>    Length of the password [default: 16]
-s, --symbols            Include symbols (!@#$%^&*)
-c, --count <COUNT>      Number of passwords to generate [default: 1]
-h, --help               Print help
```

## Running Tests

```bash
cargo test
```

## Built with

- 🦀 Rust
- 📦 clap (CLI parsing)
- 🎲 rand (secure random generation)

## Author

[@whispem](https://github.com/whispem)

## MIT License

