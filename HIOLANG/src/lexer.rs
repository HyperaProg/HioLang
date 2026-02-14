#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Keywords
    Space,
    End,
    Make,
    Inspace,
    Call,
    Text,
    Pub,
    Subpub,
    Function,
    Return,
    If,
    Else,
    While,
    For,
    Break,
    Continue,
    Let,
    
    // Identifiers and literals
    Identifier(String),
    String(String),
    Number(i64),
    Float(f64),
    
    // Operators
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Equal,
    EqualEqual,
    NotEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    And,
    Or,
    Not,
    
    // Punctuation
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Semicolon,
    Colon,
    Comma,
    Dot,
    Arrow,
    DashArrow,
    
    // Special
    Eof,
}

#[derive(Debug, Clone)]
pub struct Lexer {
    input: Vec<char>,
    position: usize,
    current_char: Option<char>,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        let chars: Vec<char> = input.chars().collect();
        let current_char = if chars.is_empty() { None } else { Some(chars[0]) };
        
        Lexer {
            input: chars,
            position: 0,
            current_char,
        }
    }
    
    fn advance(&mut self) {
        self.position += 1;
        if self.position >= self.input.len() {
            self.current_char = None;
        } else {
            self.current_char = Some(self.input[self.position]);
        }
    }
    
    fn peek(&self, offset: usize) -> Option<char> {
        let pos = self.position + offset;
        if pos < self.input.len() {
            Some(self.input[pos])
        } else {
            None
        }
    }
    
    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.current_char {
            if ch.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }
    
    fn skip_comment(&mut self) {
        if self.current_char == Some('\'') && self.peek(1) == Some('\'') {
            while self.current_char.is_some() && self.current_char != Some('\n') {
                self.advance();
            }
        }
    }
    
    fn read_identifier(&mut self) -> String {
        let mut result = String::new();
        while let Some(ch) = self.current_char {
            if ch.is_alphanumeric() || ch == '_' {
                result.push(ch);
                self.advance();
            } else {
                break;
            }
        }
        result
    }
    
    fn read_string(&mut self, quote: char) -> String {
        let mut result = String::new();
        self.advance(); // skip opening quote
        
        while let Some(ch) = self.current_char {
            if ch == quote {
                self.advance(); // skip closing quote
                break;
            } else if ch == '\\' {
                self.advance();
                match self.current_char {
                    Some('n') => result.push('\n'),
                    Some('t') => result.push('\t'),
                    Some('r') => result.push('\r'),
                    Some('\\') => result.push('\\'),
                    Some('"') => result.push('"'),
                    Some('\'') => result.push('\''),
                    Some(ch) => result.push(ch),
                    None => break,
                }
                self.advance();
            } else {
                result.push(ch);
                self.advance();
            }
        }
        result
    }
    
    fn read_number(&mut self) -> Token {
        let mut result = String::new();
        while let Some(ch) = self.current_char {
            if ch.is_numeric() {
                result.push(ch);
                self.advance();
            } else if ch == '.' && self.peek(1).map_or(false, |c| c.is_numeric()) {
                result.push(ch);
                self.advance();
            } else {
                break;
            }
        }
        
        if result.contains('.') {
            Token::Float(result.parse().unwrap_or(0.0))
        } else {
            Token::Number(result.parse().unwrap_or(0))
        }
    }
    
    pub fn next_token(&mut self) -> Token {
        loop {
            self.skip_whitespace();
            
            match self.current_char {
                None => return Token::Eof,
                Some('\'') if self.peek(1) == Some('\'') => {
                    self.skip_comment();
                }
                Some('+') => {
                    self.advance();
                    return Token::Plus;
                }
                Some('-') => {
                    self.advance();
                    if self.current_char == Some('>') {
                        self.advance();
                        return Token::Arrow;
                    }
                    return Token::Minus;
                }
                Some('—') => { // em-dash
                    self.advance();
                    return Token::DashArrow;
                }
                Some('*') => {
                    self.advance();
                    return Token::Star;
                }
                Some('/') => {
                    self.advance();
                    return Token::Slash;
                }
                Some('%') => {
                    self.advance();
                    return Token::Percent;
                }
                Some('=') => {
                    self.advance();
                    if self.current_char == Some('=') {
                        self.advance();
                        return Token::EqualEqual;
                    }
                    return Token::Equal;
                }
                Some('!') => {
                    self.advance();
                    if self.current_char == Some('=') {
                        self.advance();
                        return Token::NotEqual;
                    }
                    return Token::Not;
                }
                Some('<') => {
                    self.advance();
                    if self.current_char == Some('=') {
                        self.advance();
                        return Token::LessEqual;
                    }
                    return Token::Less;
                }
                Some('>') => {
                    self.advance();
                    if self.current_char == Some('=') {
                        self.advance();
                        return Token::GreaterEqual;
                    }
                    return Token::Greater;
                }
                Some('&') => {
                    self.advance();
                    if self.current_char == Some('&') {
                        self.advance();
                        return Token::And;
                    }
                    self.position -= 1;
                    self.current_char = Some('&');
                    self.advance();
                    return Token::And;
                }
                Some('|') => {
                    self.advance();
                    if self.current_char == Some('|') {
                        self.advance();
                        return Token::Or;
                    }
                    self.position -= 1;
                    self.current_char = Some('|');
                    self.advance();
                    return Token::Or;
                }
                Some('(') => {
                    self.advance();
                    return Token::LeftParen;
                }
                Some(')') => {
                    self.advance();
                    return Token::RightParen;
                }
                Some('{') => {
                    self.advance();
                    return Token::LeftBrace;
                }
                Some('}') => {
                    self.advance();
                    return Token::RightBrace;
                }
                Some('[') => {
                    self.advance();
                    return Token::LeftBracket;
                }
                Some(']') => {
                    self.advance();
                    return Token::RightBracket;
                }
                Some(';') => {
                    self.advance();
                    return Token::Semicolon;
                }
                Some(':') => {
                    self.advance();
                    return Token::Colon;
                }
                Some(',') => {
                    self.advance();
                    return Token::Comma;
                }
                Some('.') => {
                    self.advance();
                    return Token::Dot;
                }
                Some('"') | Some('\'') => {
                    let quote = self.current_char.unwrap();
                    let result = self.read_string(quote);
                    return Token::String(result);
                }
                Some(ch) if ch.is_numeric() => {
                    return self.read_number();
                }
                Some(ch) if ch.is_alphabetic() || ch == '_' => {
                    let identifier = self.read_identifier();
                    return match identifier.as_str() {
                        "space" => Token::Space,
                        "end" => Token::End,
                        "make" => Token::Make,
                        "inspace" => Token::Inspace,
                        "call" => Token::Call,
                        "text" => Token::Text,
                        "pub" => Token::Pub,
                        "subpub" => Token::Subpub,
                        "function" => Token::Function,
                        "return" => Token::Return,
                        "if" => Token::If,
                        "else" => Token::Else,
                        "while" => Token::While,
                        "for" => Token::For,
                        "break" => Token::Break,
                        "continue" => Token::Continue,
                        "let" => Token::Let,
                        _ => Token::Identifier(identifier),
                    };
                }
                Some(ch) => {
                    self.advance();
                }
            }
        }
    }
    
    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        loop {
            let token = self.next_token();
            if token == Token::Eof {
                tokens.push(token);
                break;
            }
            tokens.push(token);
        }
        tokens
    }
}
