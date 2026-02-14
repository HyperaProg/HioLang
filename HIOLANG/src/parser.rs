use crate::lexer::{Lexer, Token};
use crate::ast::*;

pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

impl Parser {
    pub fn new(input: &str) -> Self {
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize();
        Parser {
            tokens,
            position: 0,
        }
    }
    
    fn current_token(&self) -> &Token {
        self.tokens.get(self.position).unwrap_or(&Token::Eof)
    }
    
    fn peek_token(&self) -> &Token {
        self.tokens.get(self.position + 1).unwrap_or(&Token::Eof)
    }
    
    fn advance(&mut self) {
        if self.position < self.tokens.len() {
            self.position += 1;
        }
    }
    
    fn expect(&mut self, expected: Token) -> Result<(), String> {
        if std::mem::discriminant(self.current_token()) == std::mem::discriminant(&expected) {
            self.advance();
            Ok(())
        } else {
            Err(format!("Expected {:?}, got {:?}", expected, self.current_token()))
        }
    }
    
    pub fn parse(&mut self) -> Result<Program, String> {
        let mut statements = Vec::new();
        
        while self.current_token() != &Token::Eof {
            statements.push(self.parse_statement()?);
        }
        
        Ok(Program::new(statements))
    }
    
    fn parse_statement(&mut self) -> Result<Stmt, String> {
        match self.current_token() {
            Token::Space => self.parse_space(),
            Token::Pub => self.parse_pub(),
            Token::Subpub => self.parse_subpub(),
            Token::Let => self.parse_let(),
            Token::If => self.parse_if(),
            Token::While => self.parse_while(),
            Token::For => self.parse_for(),
            Token::Return => self.parse_return(),
            Token::Break => {
                self.advance();
                self.expect(Token::Semicolon)?;
                Ok(Stmt::Break)
            }
            Token::Continue => {
                self.advance();
                self.expect(Token::Semicolon)?;
                Ok(Stmt::Continue)
            }
            Token::LeftBrace => self.parse_block(),
            Token::Function => self.parse_function_def(),
            _ => {
                let expr = self.parse_expression()?;
                self.expect(Token::Semicolon)?;
                Ok(Stmt::Expression(expr))
            }
        }
    }
    
    fn parse_space(&mut self) -> Result<Stmt, String> {
        self.expect(Token::Space)?;
        let name = match self.current_token() {
            Token::Identifier(n) => {
                let name = n.clone();
                self.advance();
                name
            }
            _ => return Err("Expected identifier after 'space'".to_string()),
        };
        
        self.expect(Token::Identifier("name".to_string()))?; // Skip "name" keyword if present
        self.expect(Token::LeftBrace)?;
        
        let mut body = Vec::new();
        while self.current_token() != &Token::RightBrace && self.current_token() != &Token::Eof {
            body.push(self.parse_statement()?);
        }
        
        self.expect(Token::RightBrace)?;
        self.expect(Token::End)?;
        self.expect(Token::Make)?;
        self.expect(Token::Semicolon)?;
        
        Ok(Stmt::Space { name, body })
    }
    
    fn parse_pub(&mut self) -> Result<Stmt, String> {
        self.expect(Token::Pub)?;
        self.expect(Token::Semicolon)?;
        self.expect(Token::LeftBrace)?;
        
        let kind = match self.current_token() {
            Token::Semicolon => {
                self.advance();
                "interpretation".to_string()
            }
            _ => "interpretation".to_string(),
        };
        
        self.expect(Token::Identifier("com".to_string()))?;
        let name = if let Token::String(s) = self.current_token().clone() {
            self.advance();
            s
        } else {
            return Err("Expected string token".into());
        };
        
        let mut body = Vec::new();
        self.expect(Token::LeftBrace)?;
        while self.current_token() != &Token::RightBrace && self.current_token() != &Token::Eof {
            body.push(self.parse_statement()?);
        }
        self.expect(Token::RightBrace)?;
        self.expect(Token::DashArrow)?;
        
        Ok(Stmt::Pub { name, kind, body })
    }
    
    fn parse_subpub(&mut self) -> Result<Stmt, String> {
        self.expect(Token::Subpub)?;
        self.expect(Token::Semicolon)?;
        self.expect(Token::LeftBrace)?;
        
        let compilation_type = "compilation".to_string();
        let name = "subpub_block".to_string();
        
        let mut body = Vec::new();
        while self.current_token() != &Token::RightBrace && self.current_token() != &Token::Eof {
            body.push(self.parse_statement()?);
        }
        self.expect(Token::RightBrace)?;
        
        Ok(Stmt::Subpub { name, compilation_type, body })
    }
    
    fn parse_let(&mut self) -> Result<Stmt, String> {
        self.expect(Token::Let)?;
        
        let name = match self.current_token() {
            Token::Identifier(n) => {
                let name = n.clone();
                self.advance();
                name
            }
            _ => return Err("Expected identifier after 'let'".to_string()),
        };
        
        self.expect(Token::Equal)?;
        let value = self.parse_expression()?;
        self.expect(Token::Semicolon)?;
        
        Ok(Stmt::Let { name, value })
    }
    
    fn parse_if(&mut self) -> Result<Stmt, String> {
        self.expect(Token::If)?;
        self.expect(Token::LeftParen)?;
        let condition = self.parse_expression()?;
        self.expect(Token::RightParen)?;
        self.expect(Token::LeftBrace)?;
        
        let mut then_branch = Vec::new();
        while self.current_token() != &Token::RightBrace && self.current_token() != &Token::Eof {
            then_branch.push(self.parse_statement()?);
        }
        self.expect(Token::RightBrace)?;
        
        let else_branch = if self.current_token() == &Token::Else {
            self.advance();
            self.expect(Token::LeftBrace)?;
            let mut branch = Vec::new();
            while self.current_token() != &Token::RightBrace && self.current_token() != &Token::Eof {
                branch.push(self.parse_statement()?);
            }
            self.expect(Token::RightBrace)?;
            Some(branch)
        } else {
            None
        };
        
        Ok(Stmt::If { condition, then_branch, else_branch })
    }
    
    fn parse_while(&mut self) -> Result<Stmt, String> {
        self.expect(Token::While)?;
        self.expect(Token::LeftParen)?;
        let condition = self.parse_expression()?;
        self.expect(Token::RightParen)?;
        self.expect(Token::LeftBrace)?;
        
        let mut body = Vec::new();
        while self.current_token() != &Token::RightBrace && self.current_token() != &Token::Eof {
            body.push(self.parse_statement()?);
        }
        self.expect(Token::RightBrace)?;
        
        Ok(Stmt::While { condition, body })
    }
    
    fn parse_for(&mut self) -> Result<Stmt, String> {
        self.expect(Token::For)?;
        self.expect(Token::LeftParen)?;
        
        let init = if self.current_token() != &Token::Semicolon {
            Some(Box::new(self.parse_statement()?))
        } else {
            self.advance();
            None
        };
        
        let condition = if self.current_token() != &Token::Semicolon {
            Some(self.parse_expression()?)
        } else {
            None
        };
        self.expect(Token::Semicolon)?;
        
        let increment = if self.current_token() != &Token::RightParen {
            Some(Box::new(self.parse_expression()?))
        } else {
            None
        };
        self.expect(Token::RightParen)?;
        self.expect(Token::LeftBrace)?;
        
        let mut body = Vec::new();
        while self.current_token() != &Token::RightBrace && self.current_token() != &Token::Eof {
            body.push(self.parse_statement()?);
        }
        self.expect(Token::RightBrace)?;
        
        Ok(Stmt::For { init, condition, increment, body })
    }
    
    fn parse_return(&mut self) -> Result<Stmt, String> {
        self.expect(Token::Return)?;
        
        let value = if self.current_token() != &Token::Semicolon {
            Some(self.parse_expression()?)
        } else {
            None
        };
        self.expect(Token::Semicolon)?;
        
        Ok(Stmt::Return(value))
    }
    
    fn parse_block(&mut self) -> Result<Stmt, String> {
        self.expect(Token::LeftBrace)?;
        let mut statements = Vec::new();
        
        while self.current_token() != &Token::RightBrace && self.current_token() != &Token::Eof {
            statements.push(self.parse_statement()?);
        }
        
        self.expect(Token::RightBrace)?;
        
        Ok(Stmt::Block(statements))
    }
    
    fn parse_function_def(&mut self) -> Result<Stmt, String> {
        self.expect(Token::Function)?;
        
        let name = match self.current_token() {
            Token::Identifier(n) => {
                let name = n.clone();
                self.advance();
                name
            }
            _ => return Err("Expected function name".to_string()),
        };
        
        self.expect(Token::LeftParen)?;
        let mut params = Vec::new();
        
        while self.current_token() != &Token::RightParen && self.current_token() != &Token::Eof {
            match self.current_token() {
                Token::Identifier(p) => {
                    params.push(p.clone());
                    self.advance();
                    if self.current_token() == &Token::Comma {
                        self.advance();
                    }
                }
                _ => return Err("Expected parameter name".to_string()),
            }
        }
        
        self.expect(Token::RightParen)?;
        self.expect(Token::LeftBrace)?;
        
        let mut body = Vec::new();
        while self.current_token() != &Token::RightBrace && self.current_token() != &Token::Eof {
            body.push(self.parse_statement()?);
        }
        
        self.expect(Token::RightBrace)?;
        
        Ok(Stmt::FunctionDef { name, params, body })
    }
    
    fn parse_expression(&mut self) -> Result<Expr, String> {
        self.parse_logical_or()
    }
    
    fn parse_logical_or(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_logical_and()?;
        
        while self.current_token() == &Token::Or {
            self.advance();
            let right = self.parse_logical_and()?;
            left = Expr::Binary {
                left: Box::new(left),
                op: BinaryOp::Or,
                right: Box::new(right),
            };
        }
        
        Ok(left)
    }
    
    fn parse_logical_and(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_equality()?;
        
        while self.current_token() == &Token::And {
            self.advance();
            let right = self.parse_equality()?;
            left = Expr::Binary {
                left: Box::new(left),
                op: BinaryOp::And,
                right: Box::new(right),
            };
        }
        
        Ok(left)
    }
    
    fn parse_equality(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_comparison()?;
        
        while let Some(op) = match self.current_token() {
            Token::EqualEqual => Some(BinaryOp::Equal),
            Token::NotEqual => Some(BinaryOp::NotEqual),
            _ => None,
        } {
            self.advance();
            let right = self.parse_comparison()?;
            left = Expr::Binary {
                left: Box::new(left),
                op,
                right: Box::new(right),
            };
        }
        
        Ok(left)
    }
    
    fn parse_comparison(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_additive()?;
        
        while let Some(op) = match self.current_token() {
            Token::Less => Some(BinaryOp::Less),
            Token::LessEqual => Some(BinaryOp::LessEqual),
            Token::Greater => Some(BinaryOp::Greater),
            Token::GreaterEqual => Some(BinaryOp::GreaterEqual),
            _ => None,
        } {
            self.advance();
            let right = self.parse_additive()?;
            left = Expr::Binary {
                left: Box::new(left),
                op,
                right: Box::new(right),
            };
        }
        
        Ok(left)
    }
    
    fn parse_additive(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_multiplicative()?;
        
        while let Some(op) = match self.current_token() {
            Token::Plus => Some(BinaryOp::Add),
            Token::Minus => Some(BinaryOp::Subtract),
            _ => None,
        } {
            self.advance();
            let right = self.parse_multiplicative()?;
            left = Expr::Binary {
                left: Box::new(left),
                op,
                right: Box::new(right),
            };
        }
        
        Ok(left)
    }
    
    fn parse_multiplicative(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_unary()?;
        
        while let Some(op) = match self.current_token() {
            Token::Star => Some(BinaryOp::Multiply),
            Token::Slash => Some(BinaryOp::Divide),
            Token::Percent => Some(BinaryOp::Modulo),
            _ => None,
        } {
            self.advance();
            let right = self.parse_unary()?;
            left = Expr::Binary {
                left: Box::new(left),
                op,
                right: Box::new(right),
            };
        }
        
        Ok(left)
    }
    
    fn parse_unary(&mut self) -> Result<Expr, String> {
        match self.current_token() {
            Token::Not => {
                self.advance();
                let expr = self.parse_unary()?;
                Ok(Expr::Unary {
                    op: UnaryOp::Not,
                    expr: Box::new(expr),
                })
            }
            Token::Minus => {
                self.advance();
                let expr = self.parse_unary()?;
                Ok(Expr::Unary {
                    op: UnaryOp::Negate,
                    expr: Box::new(expr),
                })
            }
            _ => self.parse_postfix(),
        }
    }
    
    fn parse_postfix(&mut self) -> Result<Expr, String> {
        let mut expr = self.parse_primary()?;
        
        loop {
            match self.current_token() {
                Token::LeftParen => {
                    self.advance();
                    let mut args = Vec::new();
                    
                    while self.current_token() != &Token::RightParen && self.current_token() != &Token::Eof {
                        args.push(self.parse_expression()?);
                        if self.current_token() == &Token::Comma {
                            self.advance();
                        }
                    }
                    
                    self.expect(Token::RightParen)?;
                    expr = Expr::Call {
                        func: Box::new(expr),
                        args,
                    };
                }
                Token::LeftBracket => {
                    self.advance();
                    let index = self.parse_expression()?;
                    self.expect(Token::RightBracket)?;
                    expr = Expr::Index {
                        object: Box::new(expr),
                        index: Box::new(index),
                    };
                }
                Token::Dot => {
                    self.advance();
                    match self.current_token() {
                        Token::Identifier(member) => {
                            let member = member.clone();
                            self.advance();
                            expr = Expr::Member {
                                object: Box::new(expr),
                                member,
                            };
                        }
                        _ => return Err("Expected member name after '.'".to_string()),
                    }
                }
                _ => break,
            }
        }
        
        Ok(expr)
    }
    
    fn parse_primary(&mut self) -> Result<Expr, String> {
        match self.current_token().clone() {
            Token::Number(n) => {
                let val = n;
                self.advance();
                Ok(Expr::Number(val))
            }
            Token::Float(f) => {
                let val = f;
                self.advance();
                Ok(Expr::Float(val))
            }
            Token::String(s) => {
                let val = s;
                self.advance();
                Ok(Expr::String(val))
            }
            Token::Identifier(id) => {
                self.advance();
                Ok(Expr::Identifier(id))
            }
            Token::LeftParen => {
                self.advance();
                let expr = self.parse_expression()?;
                self.expect(Token::RightParen)?;
                Ok(expr)
            }
            Token::LeftBracket => {
                self.advance();
                let mut elements = Vec::new();
                
                while self.current_token() != &Token::RightBracket && self.current_token() != &Token::Eof {
                    elements.push(self.parse_expression()?);
                    if self.current_token() == &Token::Comma {
                        self.advance();
                    }
                }
                
                self.expect(Token::RightBracket)?;
                Ok(Expr::Array(elements))
            }
            Token::Call => {
                self.advance();
                self.expect(Token::Dot)?;
                match self.current_token() {
                    Token::Identifier(func_name) => {
                        let name = func_name.clone();
                        self.advance();
                        self.expect(Token::LeftParen)?;
                        let mut args = Vec::new();
                        while self.current_token() != &Token::RightParen {
                            args.push(self.parse_expression()?);
                            if self.current_token() == &Token::Comma {
                                self.advance();
                            }
                        }
                        self.expect(Token::RightParen)?;
                        Ok(Expr::Call {
                            func: Box::new(Expr::Identifier(name)),
                            args,
                        })
                    }
                    _ => Err("Expected function name after 'call.'".to_string()),
                }
            }
            _ => Err(format!("Unexpected token: {:?}", self.current_token())),
        }
    }
}
