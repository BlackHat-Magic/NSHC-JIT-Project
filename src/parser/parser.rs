use super::ast::*;
use crate::lexer::token::Token;

#[derive(Debug)]
pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens,
            current: 0,
        }
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

    pub fn parse(&mut self) -> Result<Vec<TopLevel>, String> {
        let mut top_levels = Vec::new();
        while self.peek().is_some() && self.peek() != Some(&Token::EOF) {
            if self.peek() == Some(&Token::Fn) {
                top_levels.push(TopLevel::Function(self.parse_function_declaration()?));
            } else {
                return Err(format!("Expected 'fn' or struct definition, got {:?}", self.peek()));
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
            Some(Token::Bool) | Some(Token::U8) | Some(Token::I8) | Some(Token::U16) | Some(Token::I16) | Some(Token::F16) | Some(Token::U32) | Some(Token::I32) | Some(Token::F32) => self.parse_declaration(),
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
                        Ok(Statement::Increment { variable_name: name })
                    }
                    Some(Token::Decrement) => {
                        let name = match self.consume() {
                            Some(Token::Identifier(name)) => name,
                            _ => unreachable!(),
                        };
                        self.consume(); // consume '--'
                        Ok(Statement::Decrement { variable_name: name })
                    }
                    Some(Token::OpenParen) => self.parse_function_call_statement(),
                    _ => Err(format!("Unexpected token after identifier: {:?}", next_token)),
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
        let expression = self.parse_expression()?;
        self.expect(Token::Semicolon)?;
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
        self.expect(Token::Semicolon)?;
        Ok(Statement::Assignment {
            variable_name,
            expression,
        })
    }

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
        self.expect(Token::Semicolon)?;
        let condition = self.parse_expression()?;
        self.expect(Token::Semicolon)?;
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
        self.expect(Token::Semicolon)?;
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
        self.expect(Token::Semicolon)?;
        Ok(Statement::FunctionCall { name, arguments })
    }

    fn parse_expression(&mut self) -> Result<Expression, String> {
        self.parse_logical_or()
    }

    fn parse_logical_or(&mut self) -> Result<Expression, String> {
        let mut left = self.parse_logical_and()?;
        while self.peek() == Some(&Token::Or) {
            self.consume();
            let right = self.parse_logical_and()?;
            left = Expression::LogicalOp {
                op: LogicalOp::Or,
                left: Box::new(left),
                right: Box::new(right),
            };
        }
        Ok(left)
    }

    fn parse_logical_and(&mut self) -> Result<Expression, String> {
        let mut left = self.parse_comparison()?;
        while self.peek() == Some(&Token::And) {
            self.consume();
            let right = self.parse_comparison()?;
            left = Expression::LogicalOp {
                op: LogicalOp::And,
                left: Box::new(left),
                right: Box::new(right),
            };
        }
        Ok(left)
    }

    fn parse_comparison(&mut self) -> Result<Expression, String> {
        let mut left = self.parse_addition()?;
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
                let right = self.parse_addition()?;
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

    fn parse_addition(&mut self) -> Result<Expression, String> {
        let mut left = self.parse_multiplication()?;
        while let Some(token) = self.peek() {
            let op = match token {
                Token::Plus => Some(BinaryOp::Plus),
                Token::Minus => Some(BinaryOp::Minus),
                _ => None,
            };
            if let Some(op) = op {
                self.consume();
                let right = self.parse_multiplication()?;
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

    fn parse_multiplication(&mut self) -> Result<Expression, String> {
        let mut left = self.parse_unary()?;
        while let Some(token) = self.peek() {
            let op = match token {
                Token::Multiply => Some(BinaryOp::Multiply),
                Token::Divide => Some(BinaryOp::Divide),
                Token::Modulo => Some(BinaryOp::Modulo),
                _ => None,
            };
            if let Some(op) = op {
                self.consume();
                let right = self.parse_unary()?;
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

    fn parse_unary(&mut self) -> Result<Expression, String> {
        if let Some(token) = self.peek() {
            let op = match token {
                Token::Minus => Some(UnaryOp::Negate),
                Token::Bang => Some(UnaryOp::Not),
                _ => None,
            };
            if let Some(op) = op {
                self.consume();
                let expression = self.parse_unary()?;
                return Ok(Expression::UnaryOp {
                    op,
                    expression: Box::new(expression),
                });
            }
        }
        self.parse_primary()
    }

    fn parse_primary(&mut self) -> Result<Expression, String> {
        match self.consume() {
            Some(Token::LiteralBool(val)) => Ok(Expression::LiteralBool(val)),
            Some(Token::LiteralChar(_)) => Err("char literal parsing not implemented".to_string()),
            Some(Token::LiteralString(_)) => Err("string literal parsing not implemented".to_string()),
            Some(Token::LiteralU8(val)) => Ok(Expression::LiteralU8(val)),
            Some(Token::LiteralI8(val)) => Ok(Expression::LiteralI8(val)),
            Some(Token::LiteralU16(val)) => Ok(Expression::LiteralU16(val)),
            Some(Token::LiteralI16(val)) => Ok(Expression::LiteralI16(val)),
            Some(Token::LiteralF16(val)) => Ok(Expression::LiteralF16(val)),
            Some(Token::LiteralU32(val)) => Ok(Expression::LiteralU32(val)),
            Some(Token::LiteralI32(val)) => Ok(Expression::LiteralI32(val)),
            Some(Token::LiteralF32(val)) => Ok(Expression::LiteralF32(val)),
            Some(Token::LiteralU64(_)) => Err("u64 literal parsing not implemented".to_string()),
            Some(Token::LiteralI64(_)) => Err("i64 literal parsing not implemented".to_string()),
            Some(Token::LiteralF64(_)) => Err("f64 literal parsing not implemented".to_string()),
            Some(Token::Identifier(name)) => {
                if self.peek() == Some(&Token::OpenParen) {
                    self.current -= 1; // backtrack to let parse_function_call_expression handle it
                    self.parse_function_call_expression()
                } else {
                    Ok(Expression::Variable(name))
                }
            }
            Some(Token::OpenParen) => {
                let expression = self.parse_expression()?;
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
