use half::f16;

#[derive(Clone, Copy, Debug, PartialEq)]
enum DataType {
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

#[derive(Debug)]
enum Expression {
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
        op: String,
        left: Box<Expression>,
        right: Box<Expression>,
    },
    UnaryOp {
        op: String, // `-` or `!`
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
        op: String, // `==`, `!=`, `<`, `>`, `<=`, `>=`
        left: Box<Expression>,
        right: Box<Expression>,
    },
    LogicalOp {
        op: String, // `&&`, `||`
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
enum Statement {
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
struct FunctionDeclaration {
    name: String,
    parameters: Vec<(DataType, String)>,
    return_type: Option<DataType>, // optional
    body: Vec<Statement>,
}

#[derive(Debug)]
enum TopLevel {
    Function(FunctionDeclaration),
    // include structs in the future
}

#[derive(Debug)]
struct Parser {
    tokens: Vec<String>,
    current: usize,
    expected_data_type: Option<DataType>,
}
impl Parser {
    fn new(tokens: Vec<String>) -> Self {
        Parser {
            tokens,
            current: 0,
            expected_data_type: None,
        }
    }

    fn peek(&self) -> Option<&String> {
        self.tokens.get(self.current)
    }

    fn consume(&mut self) -> Option<String> {
        if self.current < self.tokens.len() {
            let token = self.tokens[self.current].clone();
            self.current += 1;
            Some(token)
        } else {
            None
        }
    }

    fn expect(&mut self, expected: &str) -> Result<(), String> {
        match self.consume() {
            Some(token) if token == expected => Ok(()),
            Some(token) => Err(format!("Expected '{}', but got '{}'", expected, token)),
            None => Err(format!("Expected '{}', but got EOF".to_string(), expected)),
        }
    }

    fn parse(&mut self) -> Result<Vec<TopLevel>, String> {
        let mut top_levels = Vec::new();
        while self.peek().is_some() {
            if self.peek() == Some(&"fn".to_string()) {
                top_levels.push(TopLevel::Function(self.parse_function_declaration()?));
            } else {
                // TODO: Improve error
                return Err("Expected 'fn' or struct definition".to_string());
            }
        }
        Ok(top_levels)
    }

    fn parse_function_declaration(&mut self) -> Result<FunctionDeclaration, String> {
        self.expect("fn")?;

        let name = self.consume().ok_or("Expected function name")?;
        self.expect("(")?;
        let mut parameters = Vec::new();
        if self.peek() != Some(&")".to_string()) {
            loop {
                let data_type = self.parse_data_type()?;
                let variable_name = self.consume().ok_or("Expected parameter name")?;
                parameters.push((data_type, variable_name));
                if self.peek() == Some(&",".to_string()) {
                    self.consume();
                } else {
                    break;
                }
            }
        }
        self.expect(")")?;
        let return_type = if self.peek() == Some(&"->".to_string()) {
            self.consume();
            Some(self.parse_data_type()?)
        } else {
            None
        };

        self.expect("{")?;
        let body = self.parse_block()?;
        self.expect("}")?;

        Ok(FunctionDeclaration {
            name,
            parameters,
            return_type,
            body,
        })
    }

    fn parse_data_type(&mut self) -> Result<DataType, String> {
        match self.consume().as_deref() {
            Some("Bool") => Ok(DataType::Bool),
            Some("u8") => Ok(DataType::U8),
            Some("i8") => Ok(DataType::I8),
            Some("u16") => Ok(DataType::U16),
            Some("i16") => Ok(DataType::I16),
            Some("f16") => Ok(DataType::F16),
            Some("u32") => Ok(DataType::U32),
            Some("i32") => Ok(DataType::I32),
            Some("f32") => Ok(DataType::F32),
            Some(other) => Err(format!("Invalid data type: {}", other)),
            None => Err("Expected data type, but got EOF".to_string()),
        }
    }

    fn parse_block(&mut self) -> Result<Vec<Statement>, String> {
        let mut statements = Vec::new();
        while self.peek() != Some(&"}".to_string()) && self.peek().is_some() {
            statements.push(self.parse_statement()?);
        }
        Ok(statements)
    }

    fn parse_statement(&mut self) -> Result<Statement, String> {
        match self.peek().as_deref() {
            Some("bool") => self.parse_declaration(),
            Some("i8") => self.parse_declaration(),
            Some("u16") => self.parse_declaration(),
            Some("i16") => self.parse_declaration(),
            Some("f16") => self.parse_declaration(),
            Some("u32") => self.parse_declaration(),
            Some("i32") => self.parse_declaration(),
            Some("f32") => self.parse_declaration(),
            Some("if") => self.parse_if_statement(),
            Some("for") => self.parse_for_statement(),
            Some("while") => self.parse_while_statement(),
            Some("return") => self.parse_return_statement(),
            Some(ident) => {
                // could be an assignment, increment, decrement, or function call
                // we need to look ahead to differentiate
                let mut parser_copy = Parser {
                    tokens: self.tokens.clone(),
                    current: self.current,
                    expected_data_type: self.expected_data_type,
                };
                parser_copy.consume(); // consume identifier
                match parser_copy.peek().as_deref() {
                    Some("=") => self.parse_assignment(ident.clone()),
                    Some("++") => {
                        self.consume();
                        self.consume();
                        Ok(Statement::Increment {
                            variable_name: ident.clone(),
                        })
                    }
                    Some("--") => {
                        self.consume();
                        self.consume();
                        Ok(Statement::Decrement {
                            variable_name: ident.clone(),
                        })
                    }
                    Some("(") => self.parse_function_call(ident.clone()),
                    _ => Err(format!(
                        "Unexpected token after identifier: {:?}",
                        parser_copy.peek()
                    )),
                }
            }
            None => Err("Expected statement, but got EOF".to_string()),
            _ => Err(format!("Unexpected token {:?}", self.peek())),
        }
    }

    fn parse_declaration(&mut self) -> Result<Statement, String> {
        let data_type = self.parse_data_type()?;
        self.expected_data_type = Some(data_type);
        let variable_name = self.consume().ok_or("Expected variable name")?;
        self.expect("=")?;
        let expression = self.parse_expression()?;
        self.expected_data_type = None;
        // self.expect(";")?; //removed since there's no semicolon...?
        Ok(Statement::Declaration {
            data_type,
            variable_name,
            expression,
        })
    }

    fn parse_assignment(&mut self, variable_name: String) -> Result<Statement, String> {
        // TODO: associativity
        self.expect("=")?;
        let expression = self.parse_expression()?;
        // self.expect(";")?;
        Ok(Statement::Assignment {
            variable_name,
            expression,
        })
    }

    fn parse_if_statement(&mut self) -> Result<Statement, String> {
        self.expect("if")?;
        self.expect("(")?;
        let condition = self.parse_expression()?;
        self.expect(")")?;
        self.expect("{")?;
        let then_branch = self.parse_block()?;
        self.expect("}")?;

        let mut elif_branches = Vec::new();
        while self.peek() == Some(&"elif".to_string()) {
            self.consume();
            self.expect("(")?;
            let condition = self.parse_expression()?;
            self.expect(")")?;
            self.expect("{")?;
            let elif_branch = self.parse_block()?;
            self.expect("}")?;
            elif_branches.push((condition, elif_branch));
        }

        let else_branch = if self.peek() == Some(&"else".to_string()) {
            self.consume();
            self.expect("{")?;
            let else_branch = self.parse_block()?;
            self.expect("}")?;
            Some(else_branch)
        } else {
            None
        };

        Ok(Statement::If {
            condition,
            then_branch,
            elif_branches,
            else_branch,
        })
    }

    fn parse_while_statement(&mut self) -> Result<Statement, String> {
        self.expect("while")?;
        self.expect("(")?;
        let condition = self.parse_expression()?;
        self.expect(")")?;
        self.expect("{")?;
        let body = self.parse_block()?;
        self.expect("}")?;

        Ok(Statement::While { condition, body })
    }

    fn parse_for_statement(&mut self) -> Result<Statement, String> {
        self.expect("for")?;
        self.expect("(")?;
        let initialization = self.parse_statement()?;
        // self.expect(";");
        let condition = self.parse_expression()?;
        // self.expect(";");
        let increment = match self.parse_statement() {
            Ok(stmt) => Box::new(stmt),
            Err(e) => return Err(e),
        };
        self.expect(")")?;
        self.expect("{")?;
        let body = self.parse_block()?;
        self.expect("}")?;

        Ok(Statement::For {
            initialization: Box::new(initialization),
            condition,
            increment,
            body,
        })
    }

    fn parse_return_statement(&mut self) -> Result<Statement, String> {
        self.expect("return")?;
        self.expect("(")?;
        let expression = self.parse_expression()?;
        self.expect(")")?;
        // self.expect(";")
        Ok(Statement::Return { expression })
    }

    fn parse_function_call(&mut self, name: String) -> Result<Statement, String> {
        self.expect("(")?;
        let mut arguments = Vec::new();
        if self.peek() != Some(&")".to_string()) {
            loop {
                let expression = self.parse_expression()?;
                arguments.push(expression);
                if self.peek() == Some(&",".to_string()) {
                    self.consume();
                } else {
                    break;
                }
            }
        }
        self.expect(")")?;
        // self.expect(";")
        Ok(Statement::FunctionCall { name, arguments })
    }

    fn parse_expression(&mut self) -> Result<Expression, String> {
        // Start with the lowest precedence: logical operators
        let mut left = self.parse_logical_or()?;
        Ok(left)
    }

    fn parse_logical_or(&mut self) -> Result<Expression, String> {
        let mut left = self.parse_logical_and()?;

        let mut ops = Vec::new();
        while let Some(op) = self.peek() {
            if op == "||" {
                ops.push(op.clone());
                self.consume();
            } else {
                break;
            }
        }
        for op in ops {
            let right = self.parse_logical_and()?;
            left = Expression::LogicalOp {
                op: op.clone(),
                left: Box::new(left),
                right: Box::new(right),
            }
        }
        Ok(left)
    }

    fn parse_logical_and(&mut self) -> Result<Expression, String> {
        let mut left = self.parse_comparison()?;

        let mut ops = Vec::new();
        while let Some(op) = self.peek() {
            if op == "&&" {
                ops.push(op.clone());
                self.consume();
            } else {
                break;
            }
        }
        for op in ops {
            let right = self.parse_comparison()?;
            left = Expression::LogicalOp {
                op: op.clone(),
                left: Box::new(left),
                right: Box::new(right),
            };
        }
        Ok(left)
    }

    fn parse_comparison(&mut self) -> Result<Expression, String> {
        let mut left = self.parse_addition()?;

        let mut ops = Vec::new();
        while let Some(op) = self.peek() {
            if op == "==" || op == "!=" || op == "<" || op == ">" || op == "<=" || op == ">=" {
                ops.push(op);
                self.consume();
            } else {
                break;
            }
        }
        for op in ops {
            let right = self.parse_addition()?;
            left = Expression::Comparison {
                op: op.clone(),
                left: Box::new(left),
                right: Box::new(right),
            };
        }
        Ok(left)
    }

    fn parse_addition(&mut self) -> Result<Expression, String> {
        let mut left = self.parse_multiplication()?;

        let mut ops = Vec::new();
        while let Some(op) = self.peek() {
            if op == "+" || op == "-" {
                ops.push(op);
                self.consume();
            } else {
                break;
            }
        }
        for op in ops {
            let right = self.parse_multiplication()?;
            left = Expression::BinaryOp {
                op: op.clone(),
                left: Box::new(left),
                right: Box::new(right),
            };
        }
        Ok(left)
    }

    fn parse_multiplication(&mut self) -> Result<Expression, String> {
        let mut left = self.parse_unary()?;

        let mut ops = Vec::new();
        while let Some(op) = self.peek() {
            if op == "*" || op == "/" {
                ops.push(op);
                self.consume();
            } else {
                break;
            }
        }
        for op in ops {
            let right = self.parse_unary()?;
            left = Expression::BinaryOp {
                op: op.clone(),
                left: Box::new(left),
                right: Box::new(right),
            };
        }
        Ok(left)
    }

    fn parse_unary(&mut self) -> Result<Expression, String> {
        if let Some(op) = self.peek() {
            if op == "!" || op == "-" {
                self.consume();
                let expression = self.parse_unary()?; // Recursive call for chained unary operators
                let unary_op = Expression::UnaryOp {
                    op: op.clone(),
                    expression: Box::new(expression),
                };
                Ok(unary_op)
            } else {
                //If it's not a unary operator, fall through to primary
                let primary = self.parse_primary()?;
                Ok(primary)
            }
        } else {
            Err("Expected unary operator or primary expression, but got EOF".to_string())
        }
    }

    fn parse_primary(&mut self) -> Result<Expression, String> {
        match self.peek().as_deref() {
            Some("true") => {
                self.consume();
                Ok(Expression::LiteralBool(true))
            }
            Some("false") => {
                self.consume();
                Ok(Expression::LiteralBool(false))
            }
            Some(literal)
                if literal.chars.all()(|c| c.is_digit(10))
                    || (literal.contains('.')
                        && literal.chars().filter(|c| *c == '.').count() == 1) =>
            {
                if let Some(expected_type) = self.expected_data_type {
                    self.consume();
                    match expected_type {
                        DataType::U8 => {
                            literal
                                .parse::<u8>()
                                .map(Expression::LiteralU8)
                                .map_err(|e| e.to_string());
                        }
                        DataType::I8 => {
                            literal
                                .parse::<i8>()
                                .map(Expression::LiteralI8)
                                .map_err(|e| e.to_string());
                        }
                        DataType::U16 => {
                            literal
                                .parse::<u16>()
                                .map(Expression::LiteralU16)
                                .map_err(|e| e.to_string());
                        }
                        DataType::I16 => {
                            literal
                                .parse::<i16>()
                                .map(Expression::LiteralI16)
                                .map_err(|e| e.to_string());
                        }
                        DataType::F16 => {
                            literal
                                .parse::<f16>()
                                .map(Expression::LiteralF16)
                                .map_err(|e| e.to_string());
                        }
                        DataType::U32 => {
                            literal
                                .parse::<u32>()
                                .map(Expression::LiteralU32)
                                .map_err(|e| e.to_string());
                        }
                        DataType::I32 => {
                            literal
                                .parse::<i32>()
                                .map(Expression::LiteralI32)
                                .map_err(|e| e.to_string());
                        }
                        DataType::F32 => {
                            literal
                                .parse::<f32>()
                                .map(Expression::LiteralF32)
                                .map_err(|e| e.to_string());
                        }
                        DataType::Bool => {
                            if let Ok(val) = literal.parse::<bool>() {
                                Ok(Expression::LiteralBool(val))
                            } else {
                                Err(format!("Unexpected type, expected boolean value"))
                            }
                        }
                    }
                    .map_err(|e| {
                        format!(
                            "Invalid literal '{}' for type '{:?}': {}",
                            literal, expected_type, e
                        )
                    })
                } else {
                    if let Ok(val) = literal.parse::<f32>() {
                        self.consume();
                        Ok(Expression::LiteralF32(val))
                    } else if let Ok(val) = literal.parse::<i32>() {
                        self.consume();
                        Ok(Expression::LiteralI32(val))
                    } else {
                        Err(format!(
                            "Numeric literal '{}' could not be automatically typed",
                            literal
                        ))
                    }
                }
            }
            Some(ident) if ident.chars().all(|c| c.is_alphabetic() || c == '_') => {
                self.consume();
                Ok(Expression::Variable(ident.clone()))
            }
            Some("(") => {
                self.consume();
                let expression = self.parse_expression()?;
                self.expect(")")?;
                Ok(expression)
            }
            Some(token) => Err(format!("Unexpected token: {}", token)),
            None => Err("Expected expression, but got EOF".to_string()),
        }
    }
}

fn main() {
    let tokens = vec![
        "fn".to_string(),
        "main".to_string(),
        "(".to_string(),
        ")".to_string(),
        "{".to_string(),
        "u8".to_string(),
        "my_u8".to_string(),
        "=".to_string(),
        "123".to_string(),
        "i32".to_string(),
        "my_i32".to_string(),
        "=".to_string(),
        "456".to_string(),
        "f32".to_string(),
        "my_f32".to_string(),
        "=".to_string(),
        "3.14".to_string(),
        "return".to_string(),
        "(".to_string(),
        "my_i32".to_string(),
        ")".to_string(),
        "}".to_string(),
    ];
    let mut parser = Parser::new(tokens);
    let program = parser.parse()?;
    println!("{:?}", program);
}
