// src/ir/ir.rs

#[derive(Debug, Clone, PartialEq)]
pub enum IRType {
    U8,
    I8,
    U16,
    I16,
    F16,
    U32,
    I32,
    F32,
    Bool,
    Void, // For functions that don't return a value
}

#[derive(Debug, Clone, PartialEq)]
pub enum IROpcode {
    // Arithmetic Operations
    Add,
    Sub,
    Mul,
    Div,
    Rem, // Remainder

    // Comparison Operations
    Eq, // Equal
    Ne, // Not Equal
    Lt, // Less Than
    Gt, // Greater Than
    Le, // Less Than or Equal
    Ge, // Greater Than or Equal

    // Logical Operations
    And,
    Or,
    Not,

    // Memory Operations
    Load,
    Store,

    // Control Flow Operations
    Jump,
    JumpIfFalse,
    Call,
    Return,

    // Other
    ConstU8(u8),
    ConstI8(i8),
    ConstU16(u16),
    ConstI16(i16),
    ConstF16(half::f16),
    ConstU32(u32),
    ConstI32(i32),
    ConstF32(f32),
    ConstBool(bool),

    //Placeholder
    Nop,
}

#[derive(Debug, Clone, PartialEq)]
pub struct IRInstruction {
    pub opcode: IROpcode,
    pub result_type: IRType,
    pub operands: Vec<IRValue>, // Operands can be other instructions or constants
}

#[derive(Debug, Clone, PartialEq)]
pub enum IRValue {
    Instruction(usize), // Index of the instruction in the IR program
    Constant(IRConstant),
    GlobalVariable(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum IRConstant {
    U8(u8),
    I8(i8),
    U16(u16),
    I16(i16),
    F16(half::f16),
    U32(u32),
    I32(i32),
    F32(f32),
    Bool(bool),
}

#[derive(Debug, Clone, PartialEq)]
pub struct IRFunction {
    pub name: String,
    pub parameters: Vec<(IRType, String)>,
    pub return_type: IRType,
    pub body: Vec<IRInstruction>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct IRProgram {
    pub functions: Vec<IRFunction>,
}
