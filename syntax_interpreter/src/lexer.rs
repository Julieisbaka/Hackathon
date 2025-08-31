// Lexer module for tokenizing input

pub struct Token {
    pub kind: TokenKind,
    pub lexeme: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    Identifier,
    Number,
    String,
    DocString,
    Assign,         // =
    Plus,           // +
    Minus,          // -
    Star,           // *
    Slash,          // /
    Caret,          // ^
    Bang,           // ! (prefix not or postfix factorial)
    LParen,         // (
    RParen,         // )
    LBrace,         // {
    RBrace,         // }
    LBracket,       // [
    RBracket,       // ]
    Comma,          // ,
    Pipe,           // |
    Prime,          // '
    Colon,          // :
    Greater,        // >
    Less,           // <
    GreaterEq,      // >=
    LessEq,         // <=
    Equal,          // ==
    NotEqual,       // !=
    EOF,
    Unknown,
}

pub fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();
    while let Some(&c) = chars.peek() {
        if c.is_whitespace() {
            chars.next();
        } else if c == '#' {
            // comment to end of line
            while let Some(ch) = chars.next() { if ch == '\n' { break; } }
        } else if c.is_ascii_digit() || (c == '.' && chars.clone().nth(1).map_or(false, |n| n.is_ascii_digit())) {
            // Number (integer or float)
            let mut num = String::new();
            let mut dot_seen = false;
            while let Some(&d) = chars.peek() {
                if d.is_ascii_digit() {
                    num.push(d);
                    chars.next();
                } else if d == '.' && !dot_seen {
                    dot_seen = true;
                    num.push(d);
                    chars.next();
                } else {
                    break;
                }
            }
            tokens.push(Token { kind: TokenKind::Number, lexeme: num });
        } else if c == '"' {
            // String or DocString
            // Check for triple quotes
            let mut clone = chars.clone();
            clone.next();
            let second = clone.peek().copied();
            let third = if second.is_some() { clone.clone().nth(1) } else { None };
            if second == Some('"') && third == Some('"') {
                // DocString
                chars.next(); chars.next(); chars.next(); // consume """
                let mut buf = String::new();
                loop {
                    if let Some(&ch) = chars.peek() {
                        if ch == '"' {
                            let mut look = chars.clone();
                            look.next();
                            if look.peek().copied() == Some('"') && look.clone().nth(1) == Some('"') {
                                // end
                                chars.next(); chars.next(); chars.next();
                                break;
                            }
                        }
                        buf.push(ch);
                        chars.next();
                    } else { break; }
                }
                tokens.push(Token { kind: TokenKind::DocString, lexeme: buf });
            } else {
                // normal string
                chars.next(); // consume opening
                let mut buf = String::new();
                while let Some(ch) = chars.next() {
                    if ch == '"' { break; }
                    if ch == '\\' {
                        if let Some(esc) = chars.next() { buf.push(esc); } else { break; }
                    } else {
                        buf.push(ch);
                    }
                }
                tokens.push(Token { kind: TokenKind::String, lexeme: buf });
            }
        } else if c.is_ascii_alphabetic() || c == '_' {
            // Identifier or keyword
            let mut ident = String::new();
            while let Some(&d) = chars.peek() {
                if d.is_ascii_alphanumeric() || d == '_' {
                    ident.push(d);
                    chars.next();
                } else {
                    break;
                }
            }
            // TODO: Check for keywords, functions, constants
            tokens.push(Token { kind: TokenKind::Identifier, lexeme: ident });
        } else {
            // Single- and multi-character tokens and operators
            match c {
                '=' => {
                    chars.next();
                    if let Some('=') = chars.peek().copied() {
                        chars.next();
                        tokens.push(Token { kind: TokenKind::Equal, lexeme: "==".to_string() });
                    } else {
                        tokens.push(Token { kind: TokenKind::Assign, lexeme: "=".to_string() });
                    }
                }
                '!' => {
                    chars.next();
                    if let Some('=') = chars.peek().copied() {
                        chars.next();
                        tokens.push(Token { kind: TokenKind::NotEqual, lexeme: "!=".to_string() });
                    } else {
                        tokens.push(Token { kind: TokenKind::Bang, lexeme: "!".to_string() });
                    }
                }
                '>' => {
                    chars.next();
                    if let Some('=') = chars.peek().copied() {
                        chars.next();
                        tokens.push(Token { kind: TokenKind::GreaterEq, lexeme: ">=".to_string() });
                    } else {
                        tokens.push(Token { kind: TokenKind::Greater, lexeme: ">".to_string() });
                    }
                }
                '<' => {
                    chars.next();
                    if let Some('=') = chars.peek().copied() {
                        chars.next();
                        tokens.push(Token { kind: TokenKind::LessEq, lexeme: "<=".to_string() });
                    } else {
                        tokens.push(Token { kind: TokenKind::Less, lexeme: "<".to_string() });
                    }
                }
                '+' => { chars.next(); tokens.push(Token { kind: TokenKind::Plus, lexeme: "+".to_string() }); }
                '-' => { chars.next(); tokens.push(Token { kind: TokenKind::Minus, lexeme: "-".to_string() }); }
                '*' => { chars.next(); tokens.push(Token { kind: TokenKind::Star, lexeme: "*".to_string() }); }
                '/' => { chars.next(); tokens.push(Token { kind: TokenKind::Slash, lexeme: "/".to_string() }); }
                '^' => { chars.next(); tokens.push(Token { kind: TokenKind::Caret, lexeme: "^".to_string() }); }
                '(' => { chars.next(); tokens.push(Token { kind: TokenKind::LParen, lexeme: "(".to_string() }); }
                ')' => { chars.next(); tokens.push(Token { kind: TokenKind::RParen, lexeme: ")".to_string() }); }
                '{' => { chars.next(); tokens.push(Token { kind: TokenKind::LBrace, lexeme: "{".to_string() }); }
                '}' => { chars.next(); tokens.push(Token { kind: TokenKind::RBrace, lexeme: "}".to_string() }); }
                '[' => { chars.next(); tokens.push(Token { kind: TokenKind::LBracket, lexeme: "[".to_string() }); }
                ']' => { chars.next(); tokens.push(Token { kind: TokenKind::RBracket, lexeme: "]".to_string() }); }
                ',' => { chars.next(); tokens.push(Token { kind: TokenKind::Comma, lexeme: ",".to_string() }); }
                '|' => { chars.next(); tokens.push(Token { kind: TokenKind::Pipe, lexeme: "|".to_string() }); }
                '\'' => { chars.next(); tokens.push(Token { kind: TokenKind::Prime, lexeme: "'".to_string() }); }
                ':' => { chars.next(); tokens.push(Token { kind: TokenKind::Colon, lexeme: ":".to_string() }); }
                _ => {
                    tokens.push(Token { kind: TokenKind::Unknown, lexeme: c.to_string() });
                    chars.next();
                }
            }
        }
    }
    tokens.push(Token { kind: TokenKind::EOF, lexeme: String::new() });
    tokens
}
