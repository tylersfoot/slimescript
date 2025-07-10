use core::str;

use regex::Regex;

fn is_opening_parenthesis(character: char) -> bool {
    character == '('
}
fn is_closing_parenthesis(character: char) -> bool {
    character == ')'
}
fn is_parenthesis(character: char) -> bool {
    is_opening_parenthesis(character) || is_closing_parenthesis(character)
}
fn is_letter(character: char) -> bool {
    let letter: Regex = Regex::new(r"[a-zA-Z]").unwrap();
    letter.is_match(&character.to_string())
}
fn is_whitespace(character: char) -> bool {
    let whitespace: Regex = Regex::new(r"\s+").unwrap();
    whitespace.is_match(&character.to_string())
}
fn is_number(character: char) -> bool {
    let number: Regex = Regex::new(r"^[0-9]+$").unwrap();
    number.is_match(&character.to_string())
}
fn is_quote(character: char) -> bool {
    character == '"'
}
fn is_operator(character: char) -> bool {
    let operators: Vec<char> = vec!['+', '-', '*', '/', '%'];
    return operators.contains(&character)
}

struct Token {
    token_type: String,
    value: String,
}

fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let length = input.len();
    let mut cursor = 0;
    while cursor < length {
        let character = input.chars().nth(cursor).unwrap();

        if is_parenthesis(character) {
            tokens.push(Token {
                token_type: "parenthesis".into(),
                value: character.to_string(),
            });
            cursor += 1;
            continue;
        }

        if is_whitespace(character) {
            // skip whitespace
            cursor += 1;
            continue;
        }

        if is_number(character) {
            let mut number = character.to_string();

            // account for multi-digit numbers
            cursor += 1;
            while is_number(input.chars().nth(cursor).unwrap()) {
                number.push(input.chars().nth(cursor).unwrap());
                cursor += 1;
            }

            tokens.push(Token {
                token_type: "number".into(),
                value: number,
            });
            continue;
        }

        if is_letter(character) {
            let mut identifier = character.to_string();

            // account for multi-character identifiers
            cursor += 1;
            while cursor < length && is_letter(input.chars().nth(cursor).unwrap()) {
                identifier.push(input.chars().nth(cursor).unwrap());
                cursor += 1;
            }

            tokens.push(Token {
                token_type: "identifier".into(),
                value: identifier,
            });
            continue;
        }

        if is_quote(character) {
            let mut string = String::new();

            cursor += 1;
            while !is_quote(input.chars().nth(cursor).unwrap()) {
                string.push(input.chars().nth(cursor).unwrap());
                cursor += 1;
            }

            tokens.push(Token {
                token_type: "string".into(),
                value: string,
            });
            cursor += 1; // skip the closing quote
            continue;
        }

        println!("Unexpected character: {character}");
        cursor += 1;
        continue;
    }

    tokens
}


fn main() {
    let input = r#"
    // this is a comment
    let numx = 3;
    let numy = 5;
    let numz = numx + numy;
    print(hey);
    let message = "Hello, World!";
    print(message);
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
