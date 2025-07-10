use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
enum TokenType {
    // literals
    Number,
    String,
    Identifier,
    
    // operators
    Plus,
    Minus,
    Multiply,
    Divide,
    Modulo,
    Assign,
    
    // delimiters
    Semicolon,
    Comma,
    Dot,
    
    // parentheses and brackets
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    
    // keywords
    Let,
    Print,
    If,
    Else,
    While,
    For,
    Function,
    Return,
    
    // special
    EOF,
}

#[derive(Debug, Clone)]
struct Token {
    token_type: TokenType,
    value: String,
    line: usize,
    column: usize,
}

#[derive(Debug)]
struct Lexer {
    input: Vec<char>,
    position: usize,
    line: usize,
    column: usize,
    keywords: HashMap<String, TokenType>,
}

impl Lexer {
    fn new(input: &str) -> Self {
        let mut keywords = HashMap::new();
        keywords.insert("let".to_string(), TokenType::Let);
        keywords.insert("print".to_string(), TokenType::Print);
        keywords.insert("if".to_string(), TokenType::If);
        keywords.insert("else".to_string(), TokenType::Else);
        keywords.insert("while".to_string(), TokenType::While);
        keywords.insert("for".to_string(), TokenType::For);
        keywords.insert("function".to_string(), TokenType::Function);
        keywords.insert("return".to_string(), TokenType::Return);
        
        Lexer {
            input: input.chars().collect(),
            position: 0,
            line: 1,
            column: 1,
            keywords,
        }
    }
    
    fn current_char(&self) -> Option<char> {
        self.input.get(self.position).copied()
    }
    
    fn peek_char(&self) -> Option<char> {
        self.input.get(self.position + 1).copied()
    }
    
    fn advance(&mut self) {
        if let Some(ch) = self.current_char() {
            if ch == '\n' {
                self.line += 1;
                self.column = 1;
            } else {
                self.column += 1;
            }
        }
        self.position += 1;
    }
    
    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.current_char() {
            if ch.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }
    
    fn read_number(&mut self) -> Token {
        let start_line = self.line;
        let start_column = self.column;
        let mut number = String::new();
        
        while let Some(ch) = self.current_char() {
            if ch.is_ascii_digit() || ch == '.' {
                number.push(ch);
                self.advance();
            } else {
                break;
            }
        }
        
        Token {
            token_type: TokenType::Number,
            value: number,
            line: start_line,
            column: start_column,
        }
    }
    
    fn read_string(&mut self) -> Result<Token, String> {
        let start_line = self.line;
        let start_column = self.column;
        let mut string = String::new();
        
        // Skip opening quote
        self.advance();
        
        while let Some(ch) = self.current_char() {
            if ch == '"' {
                self.advance(); // Skip closing quote
                return Ok(Token {
                    token_type: TokenType::String,
                    value: string,
                    line: start_line,
                    column: start_column,
                });
            } else if ch == '\\' {
                // Handle escape sequences
                self.advance();
                if let Some(escaped) = self.current_char() {
                    match escaped {
                        'n' => string.push('\n'),
                        't' => string.push('\t'),
                        'r' => string.push('\r'),
                        '\\' => string.push('\\'),
                        '"' => string.push('"'),
                        _ => return Err(format!("Invalid escape sequence: \\{}", escaped)),
                    }
                    self.advance();
                } else {
                    return Err("Unexpected end of input in escape sequence".to_string());
                }
            } else {
                string.push(ch);
                self.advance();
            }
        }
        
        Err("Unterminated string literal".to_string())
    }
    
    fn read_identifier(&mut self) -> Token {
        let start_line = self.line;
        let start_column = self.column;
        let mut identifier = String::new();
        
        while let Some(ch) = self.current_char() {
            if ch.is_alphanumeric() || ch == '_' {
                identifier.push(ch);
                self.advance();
            } else {
                break;
            }
        }
        
        // Check if it's a keyword
        let token_type = self.keywords.get(&identifier)
            .cloned()
            .unwrap_or(TokenType::Identifier);
        
        Token {
            token_type,
            value: identifier,
            line: start_line,
            column: start_column,
        }
    }
    
    fn read_comment(&mut self) {
        // Skip // and everything until end of line
        while let Some(ch) = self.current_char() {
            if ch == '\n' {
                break;
            }
            self.advance();
        }
    }
    
    fn next_token(&mut self) -> Result<Token, String> {
        self.skip_whitespace();
        
        let current_char = match self.current_char() {
            Some(ch) => ch,
            None => {
                return Ok(Token {
                    token_type: TokenType::EOF,
                    value: "".to_string(),
                    line: self.line,
                    column: self.column,
                });
            }
        };
        
        let start_line = self.line;
        let start_column = self.column;
        
        match current_char {
            // Numbers
            '0'..='9' => Ok(self.read_number()),
            
            // Strings
            '"' => self.read_string(),
            
            // Identifiers and keywords
            'a'..='z' | 'A'..='Z' | '_' => Ok(self.read_identifier()),
            
            // Operators
            '+' => {
                self.advance();
                Ok(Token {
                    token_type: TokenType::Plus,
                    value: "+".to_string(),
                    line: start_line,
                    column: start_column,
                })
            }
            '-' => {
                self.advance();
                Ok(Token {
                    token_type: TokenType::Minus,
                    value: "-".to_string(),
                    line: start_line,
                    column: start_column,
                })
            }
            '*' => {
                self.advance();
                Ok(Token {
                    token_type: TokenType::Multiply,
                    value: "*".to_string(),
                    line: start_line,
                    column: start_column,
                })
            }
            '/' => {
                self.advance();
                if let Some('/') = self.peek_char() {
                    self.read_comment();
                    self.next_token() // Recursively get next token after comment
                } else {
                    self.advance();
                    Ok(Token {
                        token_type: TokenType::Divide,
                        value: "/".to_string(),
                        line: start_line,
                        column: start_column,
                    })
                }
            }
            '%' => {
                self.advance();
                Ok(Token {
                    token_type: TokenType::Modulo,
                    value: "%".to_string(),
                    line: start_line,
                    column: start_column,
                })
            }
            '=' => {
                self.advance();
                Ok(Token {
                    token_type: TokenType::Assign,
                    value: "=".to_string(),
                    line: start_line,
                    column: start_column,
                })
            }
            
            // Delimiters
            ';' => {
                self.advance();
                Ok(Token {
                    token_type: TokenType::Semicolon,
                    value: ";".to_string(),
                    line: start_line,
                    column: start_column,
                })
            }
            ',' => {
                self.advance();
                Ok(Token {
                    token_type: TokenType::Comma,
                    value: ",".to_string(),
                    line: start_line,
                    column: start_column,
                })
            }
            '.' => {
                self.advance();
                Ok(Token {
                    token_type: TokenType::Dot,
                    value: ".".to_string(),
                    line: start_line,
                    column: start_column,
                })
            }
            
            // Parentheses and brackets
            '(' => {
                self.advance();
                Ok(Token {
                    token_type: TokenType::LeftParen,
                    value: "(".to_string(),
                    line: start_line,
                    column: start_column,
                })
            }
            ')' => {
                self.advance();
                Ok(Token {
                    token_type: TokenType::RightParen,
                    value: ")".to_string(),
                    line: start_line,
                    column: start_column,
                })
            }
            '{' => {
                self.advance();
                Ok(Token {
                    token_type: TokenType::LeftBrace,
                    value: "{".to_string(),
                    line: start_line,
                    column: start_column,
                })
            }
            '}' => {
                self.advance();
                Ok(Token {
                    token_type: TokenType::RightBrace,
                    value: "}".to_string(),
                    line: start_line,
                    column: start_column,
                })
            }
            '[' => {
                self.advance();
                Ok(Token {
                    token_type: TokenType::LeftBracket,
                    value: "[".to_string(),
                    line: start_line,
                    column: start_column,
                })
            }
            ']' => {
                self.advance();
                Ok(Token {
                    token_type: TokenType::RightBracket,
                    value: "]".to_string(),
                    line: start_line,
                    column: start_column,
                })
            }
            
            // Invalid character
            _ => Err(format!("Unexpected character '{}' at line {}, column {}", 
                           current_char, start_line, start_column)),
        }
    }
    
    fn tokenize(&mut self) -> Result<Vec<Token>, String> {
        let mut tokens = Vec::new();
        
        loop {
            let token = self.next_token()?;
            let is_eof = matches!(token.token_type, TokenType::EOF);
            
            tokens.push(token);
            
            if is_eof {
                break;
            }
        }
        
        Ok(tokens)
    }
}

fn main() {
    let input = r#"
    // This is a comment
    let hello = 3;
    let hi = 5;
    let hey = hello + hi;
    print(hey);
    let message = "Hello, World!";
    "#;

    let mut lexer = Lexer::new(input);
    
    match lexer.tokenize() {
        Ok(tokens) => {
            println!("Tokens:");
            for token in tokens {
                println!("  {:?} '{}' at line {}, column {}", 
                        token.token_type, token.value, token.line, token.column);
            }
        }
        Err(error) => {
            eprintln!("Lexer error: {}", error);
        }
    }
}
