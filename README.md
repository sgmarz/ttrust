# Teeny Tiny Compiler

Rust version of [https://github.com/AZHenley/teenytinycompiler](https://github.com/AZHenley/teenytinycompiler)

# Usage

Example usage:

```
cargo run ./examples/average.teeny
```

This will create a file called out.c, which then can be compiled:

```
gcc -o average out.c
./average
```

