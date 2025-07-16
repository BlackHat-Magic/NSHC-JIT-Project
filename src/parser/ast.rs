use half::f16;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DataType {
    U8,
    I8,
    U16,
    I16,
    F16,
    U32,
    I32,
    F32,
    Bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOp {
    Plus,
    Minus,
    Multiply,
    Divide,
    Modulo,
    Power,
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnaryOp {
    Negate,
    Not,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ComparisonOp {
    Equals,
    NotEquals,
    LessThan,
    GreaterThan,
    LessThanEquals,
    GreaterThanEquals,
}

#[derive(Debug, Clone, PartialEq)]
pub enum LogicalOp {
    And,
    Or,
}

#[derive(Debug)]
pub enum Expression {
    LiteralU8(u8),
    LiteralI8(i8),
    LiteralU16(u16),
    LiteralI16(i16),
    LiteralF16(f16),
    LiteralU32(u32),
    LiteralI32(i32),
    LiteralF32(f32),
    LiteralBool(bool),
    Variable(String),
    BinaryOp {
        op: BinaryOp,
        left: Box<Expression>,
        right: Box<Expression>,
    },
    UnaryOp {
        op: UnaryOp,
        expression: Box<Expression>,
    },
    FunctionCall {
        name: String,
        arguments: Vec<Expression>,
    },
    FieldAccess {
        // TODO: implement
        expression: Box<Expression>,
        field_name: String,
    },
    ArrayAccess {
        // TODO: implement
        array: Box<Expression>,
        index: Box<Expression>,
    },
    Comparison {
        op: ComparisonOp,
        left: Box<Expression>,
        right: Box<Expression>,
    },
    LogicalOp {
        op: LogicalOp,
        left: Box<Expression>,
        right: Box<Expression>,
    },
    // cast might not be necessary; can be parsed as a function?
    Cast {
        // TODO: implement
        data_type: DataType,
        expression: Box<Expression>,
    },
}

#[derive(Debug)]
pub enum Statement {
    Declaration {
        data_type: DataType,
        variable_name: String,
        expression: Expression,
    },
    Assignment {
        variable_name: String,
        expression: Expression,
    },
    Increment {
        variable_name: String,
    },
    Decrement {
        variable_name: String,
    },
    If {
        condition: Expression,
        then_branch: Vec<Statement>,
        elif_branches: Vec<(Expression, Vec<Statement>)>,
        else_branch: Option<Vec<Statement>>,
    },
    For {
        initialization: Box<Statement>,
        condition: Expression,
        increment: Box<Statement>,
        body: Vec<Statement>,
    },
    While {
        condition: Expression,
        body: Vec<Statement>,
    },
    FunctionCall {
        name: String,
        arguments: Vec<Expression>,
    },
    Return {
        expression: Expression,
    },
}

#[derive(Debug)]
pub struct FunctionDeclaration {
    pub name: String,
    pub parameters: Vec<(DataType, String)>,
    pub return_type: Option<DataType>, // optional
    pub body: Vec<Statement>,
}

#[derive(Debug)]
pub enum TopLevel {
    Function(FunctionDeclaration),
    // include structs in the future
}
