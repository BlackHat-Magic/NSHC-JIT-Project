#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    // Keywords
    Fn,     // fn
    If,     // if
    Else,   // else
    Elif,   // elif
    For,    // for
    While,  // while
    Return, // return
    True,   // true
    False,  // false

    // Data Types
    Bool,   // bool
    String, // string
    Char,   // char
    U8,     // u8
    I8,     // i8
    U16,    // u16
    I16,    // i16
    F16,    // f16
    U32,    // u32
    I32,    // i32
    F32,    // f32
    U64,    // u64
    I64,    // i64
    F64,    // f64

    // Identifiers
    Identifier(String), // Variable names, function names

    // Literals
    LiteralBool(bool),     // Boolean literal
    LiteralChar(char),     // Char literal
    LiteralString(String), // String literal
    LiteralU8(u8),         // 8-bit unsigned integer literal
    LiteralI8(i8),         // 8-bit signed integer literal
    LiteralU16(u16),       // 16-bit unsigned integer literal
    LiteralI16(i16),       // 16-bit signed integer literal
    LiteralF16(half::f16), // 16-bit float literal
    LiteralU32(u32),       // 32-bit unsigned integer literal
    LiteralI32(i32),       // 32-bit signed integer literal
    LiteralF32(f32),       // 32-bit float literal

    // Operators
    Assign,   // =
    Plus,     // +
    Minus,    // -
    Multiply, // *
    Divide,   // /
    Modulo,   // %
    Power,    // **
    Bang,     // !

    // Comparison Operators
    Equals,            // ==
    NotEquals,         // !=
    LessThan,          // <
    GreaterThan,       // >
    LessThanEquals,    // <=
    GreaterThanEquals, // >=

    // Logical Operators
    And, // &&
    Or,  // ||

    // Increment/Decrement
    Increment,    // ++
    PlusEquals,   // +=
    Decrement,    // --
    MinusEquals,  // -=
    TimesEquals,  // *=
    DivideEquals, // /=

    // Delimiters
    OpenParen,      // (
    CloseParen,     // )
    OpenBrace,      // {
    CloseBrace,     // }
    OpenBracket,    // [
    CloseBracket,   // ]
    Comma,          // ,

    // Other
    Arrow, // -> (used for function return types)
    EOF,   // End of File
}
