# VM JIT

simple virtual machine that runs a JIT compiler for a custom programming language.

Focus on difference between JIT and interpretter; talk about memory and compile time.

## TLang (placeholder name)

T Lang *will* have the following reserved keywords:

- TRUE, FALSE, NULL
- [Data Types]
    - u8, i8
    - u16, i16, f16
    - u32, i32, f32
    - u64, i64, f64
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
    - assert

T Lang *might* have the following reserved keywords:

- typedef
- class

## Basic Shell (`./basic_shell`)

I followed a [tutorial on GitHub](https://github.com/spencertipping/shell-tutorial) to create a basic shell; the later JIT compiler tutorial recommended following it first.

## LC3 Virtual Machine (`./LC3VM`)

I followed an [online tutorial](https://www.jmeiners.com/lc3-vm/) detailing how to write an LC3 virtual machine as an initial proof of concept.