use super::ast::*;
use half::f16;

#[derive(Debug)]
pub struct Parser {
    tokens: Vec<String>,
    current: usize,
    expected_data_type: Option<DataType>,
}
impl Parser {
    pub fn new(tokens: Vec<String>) -> Self {
        Parser {
            tokens,
            current: 0,
            expected_data_type: None,
        }
    }

    fn peek(&self) -> Option<&String> {
        // Some(self.tokens[self.current].clone())
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
            None => Err(format!("Expected '{}', but got EOF", expected)),
        }
    }

    pub fn parse(&mut self) -> Result<Vec<TopLevel>, String> {
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
            Some("bool") => Ok(DataType::Bool),
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
        let peeked = self.peek().cloned();
        match peeked.as_deref() {
            Some("bool") => self.parse_declaration(),
            Some("u8") => self.parse_declaration(),
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
                parser_copy.consume();
                match parser_copy.peek().as_ref().map(|s| s.as_str()) {
                    Some("=") => self.parse_assignment(ident.to_string().clone()),
                    Some("++") => {
                        self.consume();
                        self.consume();
                        Ok(Statement::Increment {
                            variable_name: ident.to_string().clone(),
                        })
                    }
                    Some("--") => {
                        self.consume();
                        self.consume();
                        Ok(Statement::Decrement {
                            variable_name: ident.to_string().clone(),
                        })
                    }
                    Some("(") => self.parse_function_call(ident.to_string().clone()),
                    _ => Err(format!(
                        "Unexpected token after identifier: {:?}",
                        parser_copy.peek()
                    )),
                }
            }
            None => Err("Expected statement, but got EOF".to_string()),
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
        Ok(self.parse_logical_or()?)
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
                ops.push(op.clone());
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

        loop {
            if let Some(op) = self.peek().as_deref() {
                if op == "+" || op == "-" {
                    // The immutable borrow is gone, so we can now borrow mutably.
                    let consumed_op = self.consume().unwrap(); // We know it's Some
                    let right = self.parse_multiplication()?;
                    left = Expression::BinaryOp {
                        op: consumed_op,
                        left: Box::new(left),
                        right: Box::new(right),
                    };
                } else {
                    break;
                }
            } else {
                break;
            }
        }

        Ok(left)
    }

    fn parse_multiplication(&mut self) -> Result<Expression, String> {
        let mut left = self.parse_unary()?;

        let mut ops = Vec::new();
        while let Some(op) = self.peek() {
            if op == "*" || op == "/" {
                ops.push(op.clone());
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
        // If next token is a '!' or '-', consume it, recurse, return a UnaryOp.
        if let Some(op) = self.peek().map(String::as_str) {
            if op == "!" || op == "-" {
                let consumed_op = self.consume().unwrap();
                let expr = self.parse_unary()?;
                return Ok(Expression::UnaryOp {
                    op: consumed_op,
                    expression: Box::new(expr),
                });
            }
        }
        // Otherwise, it's not a unary op, so parse a primary.
        self.parse_primary()
    }

    fn parse_primary(&mut self) -> Result<Expression, String> {
        let peeked = self.peek().cloned();
        match peeked.as_deref() {
            Some("true") => {
                self.consume();
                Ok(Expression::LiteralBool(true))
            }
            Some("false") => {
                self.consume();
                Ok(Expression::LiteralBool(false))
            }
            Some(literal)
                if literal.chars().all(|c| c.is_digit(10))
                    || (literal.contains('.')
                        && literal.chars().filter(|c| *c == '.').count() == 1) =>
            {
                if let Some(expected_type) = self.expected_data_type {
                    self.consume();
                    let result = match expected_type {
                        DataType::U8 => literal
                            .parse::<u8>()
                            .map(Expression::LiteralU8)
                            .map_err(|e| e.to_string()),
                        DataType::I8 => literal
                            .parse::<i8>()
                            .map(Expression::LiteralI8)
                            .map_err(|e| e.to_string()),
                        DataType::U16 => literal
                            .parse::<u16>()
                            .map(Expression::LiteralU16)
                            .map_err(|e| e.to_string()),
                        DataType::I16 => literal
                            .parse::<i16>()
                            .map(Expression::LiteralI16)
                            .map_err(|e| e.to_string()),
                        DataType::F16 => literal
                            .parse::<f16>()
                            .map(Expression::LiteralF16)
                            .map_err(|e| e.to_string()),
                        DataType::U32 => literal
                            .parse::<u32>()
                            .map(Expression::LiteralU32)
                            .map_err(|e| e.to_string()),
                        DataType::I32 => literal
                            .parse::<i32>()
                            .map(Expression::LiteralI32)
                            .map_err(|e| e.to_string()),
                        DataType::F32 => literal
                            .parse::<f32>()
                            .map(Expression::LiteralF32)
                            .map_err(|e| e.to_string()),
                        DataType::Bool => {
                            // This arm was already correct
                            if literal == "true" {
                                Ok(Expression::LiteralBool(true))
                            } else if literal == "false" {
                                Ok(Expression::LiteralBool(false))
                            } else {
                                Err(format!("'{}' is not a valid boolean value", literal))
                            }
                        }
                    };
                    result.map_err(|e| {
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
            Some(ident) => {
                let mut chars = ident.chars();
                if let Some(first) = chars.next() {
                    if (first.is_alphabetic() || first == '_')
                        && chars.all(|c| c.is_alphanumeric() || c == '_')
                    {
                        self.consume();
                        return Ok(Expression::Variable(ident.to_string()));
                    }
                }
                Err(format!("Unexpected token: {}", ident))
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
