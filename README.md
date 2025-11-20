<div align="center">

# Piru

(Formerly Silverquill)

</div>

A compiler for a simple, strongly-typed, C-like language with manual memory management targetting a simple RISC-V based virtual machine.

## Planned Reserved Keywords

- TRUE, FALSE, NULL
- [Data Types]
    - u8, i8
    - u16, i16, f16
    - u32, i32, f32
    - u64, i64, f64
    - char, string, bool
- fn, lambda
- struct
    - Maybe has functions and v-tables?
    - no inheritance 🤮
- control flow
    - if, else
    - for, while
    - return
- error handling
    - try, catch
        - Maybe errors as values instead
    - assert

*might* have the following reserved keywords:

- typedef
- class
- vector
- enum
