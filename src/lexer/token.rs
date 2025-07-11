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
    LiteralNumber(String),

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
    OpenParen,    // (
    CloseParen,   // )
    OpenBrace,    // {
    CloseBrace,   // }
    OpenBracket,  // [
    CloseBracket, // ]
    Comma,        // ,

    // Other
    Arrow, // -> (used for function return types)
    EOF,   // End of File
}
