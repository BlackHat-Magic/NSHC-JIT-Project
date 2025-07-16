use super::ast::*;
use crate::lexer::token::Token;

#[derive(Debug)]
pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, current: 0 }
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.current)
    }

    fn consume(&mut self) -> Option<Token> {
        if self.current < self.tokens.len() {
            let token = self.tokens[self.current].clone();
            self.current += 1;
            Some(token)
        } else {
            None
        }
    }

    fn expect(&mut self, expected: Token) -> Result<(), String> {
        match self.consume() {
            Some(token) if token == expected => Ok(()),
            Some(token) => Err(format!("Expected '{:?}', but got '{:?}'", expected, token)),
            None => Err(format!("Expected '{:?}', but got EOF", expected)),
        }
    }

    // Helper method to parse a number string into an appropriate Expression
    fn parse_number_literal(
        &self,
        num_str: &str,
        expected_type: Option<&DataType>,
    ) -> Result<Expression, String> {
        // If we have context about the expected type, use it
        if let Some(data_type) = expected_type {
            return self.parse_number_with_type(num_str, data_type);
        }

        // Otherwise, infer the type from the string format
        if num_str.contains('.') {
            // It's a float
            if let Ok(value) = num_str.parse::<f64>() {
                Ok(Expression::LiteralF64(value))
            } else {
                Err(format!("Invalid float literal: {}", num_str))
            }
        } else {
            // It's an integer - try to fit in the smallest appropriate type
            if let Ok(value) = num_str.parse::<i32>() {
                Ok(Expression::LiteralI32(value))
            } else if let Ok(value) = num_str.parse::<i64>() {
                Ok(Expression::LiteralI64(value))
            } else {
                Err(format!("Invalid integer literal: {}", num_str))
            }
        }
    }

    // Helper method to parse a number string with a specific expected type
    fn parse_number_with_type(
        &self,
        num_str: &str,
        data_type: &DataType,
    ) -> Result<Expression, String> {
        match data_type {
            DataType::Bool => match num_str {
                "0" => Ok(Expression::LiteralBool(false)),
                "1" => Ok(Expression::LiteralBool(true)),
                _ => Err(format!("Invalid boolean literal: {}", num_str)),
            },
            DataType::U8 => {
                if let Ok(value) = num_str.parse::<u8>() {
                    Ok(Expression::LiteralU8(value))
                } else {
                    Err(format!("Invalid u8 literal: {}", num_str))
                }
            }
            DataType::I8 => {
                if let Ok(value) = num_str.parse::<i8>() {
                    Ok(Expression::LiteralI8(value))
                } else {
                    Err(format!("Invalid i8 literal: {}", num_str))
                }
            }
            DataType::U16 => {
                if let Ok(value) = num_str.parse::<u16>() {
                    Ok(Expression::LiteralU16(value))
                } else {
                    Err(format!("Invalid u16 literal: {}", num_str))
                }
            }
            DataType::I16 => {
                if let Ok(value) = num_str.parse::<i16>() {
                    Ok(Expression::LiteralI16(value))
                } else {
                    Err(format!("Invalid i16 literal: {}", num_str))
                }
            }
            DataType::F16 => {
                if let Ok(value) = num_str.parse::<f32>() {
                    Ok(Expression::LiteralF16(half::f16::from_f32(value)))
                } else {
                    Err(format!("Invalid f16 literal: {}", num_str))
                }
            }
            DataType::U32 => {
                if let Ok(value) = num_str.parse::<u32>() {
                    Ok(Expression::LiteralU32(value))
                } else {
                    Err(format!("Invalid u32 literal: {}", num_str))
                }
            }
            DataType::I32 => {
                if let Ok(value) = num_str.parse::<i32>() {
                    Ok(Expression::LiteralI32(value))
                } else {
                    Err(format!("Invalid i32 literal: {}", num_str))
                }
            }
            DataType::F32 => {
                if let Ok(value) = num_str.parse::<f32>() {
                    Ok(Expression::LiteralF32(value))
                } else {
                    Err(format!("Invalid f32 literal: {}", num_str))
                }
            }
            DataType::U64 => {
                if let Ok(value) = num_str.parse::<u64>() {
                    Ok(Expression::LiteralU64(value))
                } else {
                    Err(format!("Invalid u64 literal: {}", num_str))
                }
            }
            DataType::I64 => {
                if let Ok(value) = num_str.parse::<i64>() {
                    Ok(Expression::LiteralI64(value))
                } else {
                    Err(format!("Invalid i64 literal: {}", num_str))
                }
            }
            DataType::F64 => {
                if let Ok(value) = num_str.parse::<f64>() {
                    Ok(Expression::LiteralF64(value))
                } else {
                    Err(format!("Invalid f64 literal: {}", num_str))
                }
            }
            // Add other types as needed
            _ => Err(format!(
                "Unsupported data type for number literal: {:?}",
                data_type
            )),
        }
    }

    pub fn parse(&mut self) -> Result<Vec<TopLevel>, String> {
        let mut top_levels = Vec::new();
        while self.peek().is_some() && self.peek() != Some(&Token::EOF) {
            if self.peek() == Some(&Token::Fn) {
                top_levels.push(TopLevel::Function(self.parse_function_declaration()?));
            } else {
                return Err(format!(
                    "Expected 'fn' or struct definition, got {:?}",
                    self.peek()
                ));
            }
        }
        Ok(top_levels)
    }

    fn parse_function_declaration(&mut self) -> Result<FunctionDeclaration, String> {
        self.expect(Token::Fn)?;

        let name = match self.consume() {
            Some(Token::Identifier(name)) => name,
            other => return Err(format!("Expected function name, got {:?}", other)),
        };

        self.expect(Token::OpenParen)?;
        let mut parameters = Vec::new();
        if self.peek() != Some(&Token::CloseParen) {
            loop {
                let data_type = self.parse_data_type()?;
                let variable_name = match self.consume() {
                    Some(Token::Identifier(name)) => name,
                    other => return Err(format!("Expected parameter name, got {:?}", other)),
                };
                parameters.push((data_type, variable_name));
                if self.peek() == Some(&Token::Comma) {
                    self.consume();
                } else {
                    break;
                }
            }
        }
        self.expect(Token::CloseParen)?;
        let return_type = if self.peek() == Some(&Token::Arrow) {
            self.consume();
            Some(self.parse_data_type()?)
        } else {
            None
        };

        self.expect(Token::OpenBrace)?;
        let body = self.parse_block()?;
        self.expect(Token::CloseBrace)?;

        Ok(FunctionDeclaration {
            name,
            parameters,
            return_type,
            body,
        })
    }

    fn parse_data_type(&mut self) -> Result<DataType, String> {
        match self.consume() {
            Some(Token::Bool) => Ok(DataType::Bool),
            Some(Token::U8) => Ok(DataType::U8),
            Some(Token::I8) => Ok(DataType::I8),
            Some(Token::U16) => Ok(DataType::U16),
            Some(Token::I16) => Ok(DataType::I16),
            Some(Token::F16) => Ok(DataType::F16),
            Some(Token::U32) => Ok(DataType::U32),
            Some(Token::I32) => Ok(DataType::I32),
            Some(Token::F32) => Ok(DataType::F32),
            Some(Token::U64) => Ok(DataType::U64),
            Some(Token::I64) => Ok(DataType::I64),
            Some(Token::F64) => Ok(DataType::F64),
            Some(Token::Char) => Ok(DataType::Char),
            Some(Token::String) => Ok(DataType::String),
            Some(other) => Err(format!("Invalid data type: {:?}", other)),
            None => Err("Expected data type, but got EOF".to_string()),
        }
    }

    fn parse_block(&mut self) -> Result<Vec<Statement>, String> {
        let mut statements = Vec::new();
        while self.peek() != Some(&Token::CloseBrace) && self.peek() != Some(&Token::EOF) {
            statements.push(self.parse_statement()?);
        }
        Ok(statements)
    }

    fn parse_statement(&mut self) -> Result<Statement, String> {
        match self.peek() {
            Some(Token::Bool) | Some(Token::U8) | Some(Token::I8) | Some(Token::U16)
            | Some(Token::I16) | Some(Token::F16) | Some(Token::U32) | Some(Token::I32)
            | Some(Token::F32) | Some(Token::U64) | Some(Token::I64) | Some(Token::F64)
            | Some(Token::Char) | Some(Token::String) => self.parse_declaration(),
            Some(Token::If) => self.parse_if_statement(),
            Some(Token::For) => self.parse_for_statement(),
            Some(Token::While) => self.parse_while_statement(),
            Some(Token::Return) => self.parse_return_statement(),
            Some(Token::Identifier(_)) => {
                let next_token = self.tokens.get(self.current + 1);
                match next_token {
                    Some(Token::Assign) => self.parse_assignment(),
                    Some(Token::Increment) => {
                        let name = match self.consume() {
                            Some(Token::Identifier(name)) => name,
                            _ => unreachable!(),
                        };
                        self.consume(); // consume '++'
                        Ok(Statement::Increment {
                            variable_name: name,
                        })
                    }
                    Some(Token::Decrement) => {
                        let name = match self.consume() {
                            Some(Token::Identifier(name)) => name,
                            _ => unreachable!(),
                        };
                        self.consume(); // consume '--'
                        Ok(Statement::Decrement {
                            variable_name: name,
                        })
                    }
                    Some(Token::OpenParen) => self.parse_function_call_statement(),
                    _ => Err(format!(
                        "Unexpected token after identifier: {:?}",
                        next_token
                    )),
                }
            }
            other => Err(format!("Expected statement, but got {:?}", other)),
        }
    }

    fn parse_declaration(&mut self) -> Result<Statement, String> {
        let data_type = self.parse_data_type()?;
        let variable_name = match self.consume() {
            Some(Token::Identifier(name)) => name,
            other => return Err(format!("Expected variable name, got {:?}", other)),
        };
        self.expect(Token::Assign)?;
        let expression = self.parse_expression_with_context(Some(&data_type))?;
        // Note: Removed semicolon expectation - add it back if your grammar requires it
        Ok(Statement::Declaration {
            data_type,
            variable_name,
            expression,
        })
    }

    fn parse_assignment(&mut self) -> Result<Statement, String> {
        let variable_name = match self.consume() {
            Some(Token::Identifier(name)) => name,
            other => return Err(format!("Expected variable name, got {:?}", other)),
        };
        self.expect(Token::Assign)?;
        let expression = self.parse_expression()?;
        // Note: Removed semicolon expectation - add it back if your grammar requires it
        Ok(Statement::Assignment {
            variable_name,
            expression,
        })
    }

    // ... (other statement parsing methods remain the same) ...

    fn parse_if_statement(&mut self) -> Result<Statement, String> {
        self.expect(Token::If)?;
        self.expect(Token::OpenParen)?;
        let condition = self.parse_expression()?;
        self.expect(Token::CloseParen)?;
        self.expect(Token::OpenBrace)?;
        let then_branch = self.parse_block()?;
        self.expect(Token::CloseBrace)?;

        let mut elif_branches = Vec::new();
        while self.peek() == Some(&Token::Elif) {
            self.consume();
            self.expect(Token::OpenParen)?;
            let condition = self.parse_expression()?;
            self.expect(Token::CloseParen)?;
            self.expect(Token::OpenBrace)?;
            let elif_branch = self.parse_block()?;
            self.expect(Token::CloseBrace)?;
            elif_branches.push((condition, elif_branch));
        }

        let else_branch = if self.peek() == Some(&Token::Else) {
            self.consume();
            self.expect(Token::OpenBrace)?;
            let else_branch = self.parse_block()?;
            self.expect(Token::CloseBrace)?;
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
        self.expect(Token::While)?;
        self.expect(Token::OpenParen)?;
        let condition = self.parse_expression()?;
        self.expect(Token::CloseParen)?;
        self.expect(Token::OpenBrace)?;
        let body = self.parse_block()?;
        self.expect(Token::CloseBrace)?;

        Ok(Statement::While { condition, body })
    }

    fn parse_for_statement(&mut self) -> Result<Statement, String> {
        self.expect(Token::For)?;
        self.expect(Token::OpenParen)?;
        let initialization = self.parse_statement()?;
        let condition = self.parse_expression()?;
        // Note: Removed semicolon expectation here - the increment is a statement, not an expression
        let increment = self.parse_statement()?;
        self.expect(Token::CloseParen)?;
        self.expect(Token::OpenBrace)?;
        let body = self.parse_block()?;
        self.expect(Token::CloseBrace)?;

        Ok(Statement::For {
            initialization: Box::new(initialization),
            condition,
            increment: Box::new(increment),
            body,
        })
    }

    fn parse_return_statement(&mut self) -> Result<Statement, String> {
        self.expect(Token::Return)?;
        self.expect(Token::OpenParen)?;
        let expression = self.parse_expression()?;
        self.expect(Token::CloseParen)?;
        Ok(Statement::Return { expression })
    }

    fn parse_function_call_statement(&mut self) -> Result<Statement, String> {
        let name = match self.consume() {
            Some(Token::Identifier(name)) => name,
            other => return Err(format!("Expected function name, got {:?}", other)),
        };
        self.expect(Token::OpenParen)?;
        let mut arguments = Vec::new();
        if self.peek() != Some(&Token::CloseParen) {
            loop {
                arguments.push(self.parse_expression()?);
                if self.peek() == Some(&Token::Comma) {
                    self.consume();
                } else {
                    break;
                }
            }
        }
        self.expect(Token::CloseParen)?;
        Ok(Statement::FunctionCall { name, arguments })
    }

    fn parse_expression(&mut self) -> Result<Expression, String> {
        self.parse_expression_with_context(None)
    }

    fn parse_expression_with_context(
        &mut self,
        expected_type: Option<&DataType>,
    ) -> Result<Expression, String> {
        self.parse_logical_or_with_context(expected_type)
    }

    fn parse_logical_or_with_context(
        &mut self,
        expected_type: Option<&DataType>,
    ) -> Result<Expression, String> {
        let mut left = self.parse_logical_and_with_context(expected_type)?;
        while self.peek() == Some(&Token::Or) {
            self.consume();
            let right = self.parse_logical_and_with_context(expected_type)?;
            left = Expression::LogicalOp {
                op: LogicalOp::Or,
                left: Box::new(left),
                right: Box::new(right),
            };
        }
        Ok(left)
    }

    fn parse_logical_and_with_context(
        &mut self,
        expected_type: Option<&DataType>,
    ) -> Result<Expression, String> {
        let mut left = self.parse_comparison_with_context(expected_type)?;
        while self.peek() == Some(&Token::And) {
            self.consume();
            let right = self.parse_comparison_with_context(expected_type)?;
            left = Expression::LogicalOp {
                op: LogicalOp::And,
                left: Box::new(left),
                right: Box::new(right),
            };
        }
        Ok(left)
    }

    fn parse_comparison_with_context(
        &mut self,
        expected_type: Option<&DataType>,
    ) -> Result<Expression, String> {
        let mut left = self.parse_addition_with_context(expected_type)?;
        while let Some(token) = self.peek() {
            let op = match token {
                Token::Equals => Some(ComparisonOp::Equals),
                Token::NotEquals => Some(ComparisonOp::NotEquals),
                Token::LessThan => Some(ComparisonOp::LessThan),
                Token::GreaterThan => Some(ComparisonOp::GreaterThan),
                Token::LessThanEquals => Some(ComparisonOp::LessThanEquals),
                Token::GreaterThanEquals => Some(ComparisonOp::GreaterThanEquals),
                _ => None,
            };
            if let Some(op) = op {
                self.consume();
                let right = self.parse_addition_with_context(expected_type)?;
                left = Expression::Comparison {
                    op,
                    left: Box::new(left),
                    right: Box::new(right),
                };
            } else {
                break;
            }
        }
        Ok(left)
    }

    fn parse_addition_with_context(
        &mut self,
        expected_type: Option<&DataType>,
    ) -> Result<Expression, String> {
        let mut left = self.parse_multiplication_with_context(expected_type)?;
        while let Some(token) = self.peek() {
            let op = match token {
                Token::Plus => Some(BinaryOp::Plus),
                Token::Minus => Some(BinaryOp::Minus),
                _ => None,
            };
            if let Some(op) = op {
                self.consume();
                let right = self.parse_multiplication_with_context(expected_type)?;
                left = Expression::BinaryOp {
                    op,
                    left: Box::new(left),
                    right: Box::new(right),
                };
            } else {
                break;
            }
        }
        Ok(left)
    }

    fn parse_multiplication_with_context(
        &mut self,
        expected_type: Option<&DataType>,
    ) -> Result<Expression, String> {
        let mut left = self.parse_unary_with_context(expected_type)?;
        while let Some(token) = self.peek() {
            let op = match token {
                Token::Multiply => Some(BinaryOp::Multiply),
                Token::Divide => Some(BinaryOp::Divide),
                Token::Modulo => Some(BinaryOp::Modulo),
                _ => None,
            };
            if let Some(op) = op {
                self.consume();
                let right = self.parse_unary_with_context(expected_type)?;
                left = Expression::BinaryOp {
                    op,
                    left: Box::new(left),
                    right: Box::new(right),
                };
            } else {
                break;
            }
        }
        Ok(left)
    }

    fn parse_unary_with_context(
        &mut self,
        expected_type: Option<&DataType>,
    ) -> Result<Expression, String> {
        if let Some(token) = self.peek() {
            let op = match token {
                Token::Minus => Some(UnaryOp::Negate),
                Token::Bang => Some(UnaryOp::Not),
                _ => None,
            };
            if let Some(op) = op {
                self.consume();
                let expression = self.parse_unary_with_context(expected_type)?;
                return Ok(Expression::UnaryOp {
                    op,
                    expression: Box::new(expression),
                });
            }
        }
        self.parse_primary_with_context(expected_type)
    }

    fn parse_primary_with_context(
        &mut self,
        expected_type: Option<&DataType>,
    ) -> Result<Expression, String> {
        match self.consume() {
            Some(Token::False) => Ok(Expression::LiteralBool(false)),
            Some(Token::True) => Ok(Expression::LiteralBool(true)),
            Some(Token::LiteralChar(val)) => Ok(Expression::LiteralChar(val)),
            Some(Token::LiteralString(val)) => Ok(Expression::LiteralString(val)),
            Some(Token::LiteralNumber(num_str)) => {
                self.parse_number_literal(&num_str, expected_type)
            }
            Some(Token::Identifier(name)) => {
                if self.peek() == Some(&Token::OpenParen) {
                    self.current -= 1; // backtrack to let parse_function_call_expression handle it
                    self.parse_function_call_expression()
                } else {
                    Ok(Expression::Variable(name))
                }
            }
            Some(Token::OpenParen) => {
                let expression = self.parse_expression_with_context(expected_type)?;
                self.expect(Token::CloseParen)?;
                Ok(expression)
            }
            other => Err(format!("Unexpected token in expression: {:?}", other)),
        }
    }

    fn parse_function_call_expression(&mut self) -> Result<Expression, String> {
        let name = match self.consume() {
            Some(Token::Identifier(name)) => name,
            other => return Err(format!("Expected function name, got {:?}", other)),
        };
        self.expect(Token::OpenParen)?;
        let mut arguments = Vec::new();
        if self.peek() != Some(&Token::CloseParen) {
            loop {
                arguments.push(self.parse_expression()?);
                if self.peek() == Some(&Token::Comma) {
                    self.consume();
                } else {
                    break;
                }
            }
        }
        self.expect(Token::CloseParen)?;
        Ok(Expression::FunctionCall { name, arguments })
    }
}
