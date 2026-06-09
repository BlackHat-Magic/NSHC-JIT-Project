<div align="center">

# Piru

(Formerly Silverquill)

</div>

A compiler for a simple, strongly-typed, (potentially) C-like language with (probably?) manual memory management targetting a simple... *something*-based virtual machine.

> Note: Work on Piru is currently (mostly) paused while I figure out what I actually want from this language. See work in [this compiler project](https://github.com/BlackHat-Magic/Various-Compilers) (mostly research now which may or may not end up in the issue tracker/README).
>
> Additionally, either the runtime or the entire toolchain might end up being rewritten in Zig. Although I prefer Rust's type system, its semantics, usage patterns, and best practices--namely the borrow checker--don't align themselves well with writing a language runtime or virtual machine, where expressing behavior that might result in the exact type of error that the compiler bends over backwards to avoid might well be intended behavior.

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
