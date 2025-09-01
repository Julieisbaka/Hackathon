// Lexer module for tokenizing input

pub struct Token {
    pub kind: TokenKind,
    pub lexeme: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    Identifier, // 0
    Number,     // 1
    String,     // 2
    DocString,  // 3
    Assign,     // =
    Plus,       // +
    Minus,      // -
    Star,       // *
    Slash,      // /
    Caret,      // ^
    Bang,       // ! (prefix not or postfix factorial)
    LParen,     // (
    RParen,     // )
    LBrace,     // {
    RBrace,     // }
    LBracket,   // [
    RBracket,   // ]
    Comma,      // ,
    Pipe,       // |
    Prime,      // '
    Colon,      // :
    Greater,    // >
    Less,       // <
    GreaterEq,  // >=
    LessEq,     // <=
    Equal,      // ==
    NotEqual,   // !=
    Semicolon,  // ;
    Newline,    // \n
    EOF,
    Unknown,
}

pub fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut chars: std::iter::Peekable<std::str::Chars<'_>> = input.chars().peekable();
    while let Some(&c) = (&mut chars).peek() {
        if c == '\n' {
            (&mut chars).next();
            (&mut tokens).push(Token { kind: TokenKind::Newline, lexeme: "\n".to_string() });
        } else if c.is_whitespace() {
            (&mut chars).next();
        } else if c == '#' {
            // comment to end of line
            while let Some(ch) = (&mut chars).next() { if ch == '\n' { break; } }
        } else if (&c).is_ascii_digit() || (c == '.' && (&mut (&chars).clone()).nth(1).map_or(false, |n: char| (&n).is_ascii_digit())) {
            // Number (integer or float)
            let mut num: String = String::new();
            let mut dot_seen: bool = false;
            while let Some(&d) = (&mut chars).peek() {
                if (&d).is_ascii_digit() {
                    (&mut num).push(d);
                    (&mut chars).next();
                } else if d == '.' && !dot_seen {
                    dot_seen = true;
                    (&mut num).push(d);
                    (&mut chars).next();
                } else {
                    break;
                }
            }
            (&mut tokens).push(Token { kind: TokenKind::Number, lexeme: num });
        } else if c == '"' {
            // String or DocString
            // Check for triple quotes
            let mut clone: std::iter::Peekable<std::str::Chars<'_>> = (&chars).clone();
            (&mut clone).next();
            let second: Option<char> = (&mut clone).peek().copied();
            let third: Option<char> = if (&second).is_some() { (&mut (&clone).clone()).nth(1) } else { None };
            if second == Some('"') && third == Some('"') {
                // DocString
                (&mut chars).next(); (&mut chars).next(); (&mut chars).next(); // consume """
                let mut buf: String = String::new();
                loop {
                    if let Some(&ch) = (&mut chars).peek() {
                        if ch == '"' {
                            let mut look: std::iter::Peekable<std::str::Chars<'_>> = (&chars).clone();
                            (&mut look).next();
                            if (&mut look).peek().copied() == Some('"') && (&mut (&look).clone()).nth(1) == Some('"') {
                                // end
                                (&mut chars).next(); (&mut chars).next(); (&mut chars).next();
                                break;
                            }
                        }
                        (&mut buf).push(ch);
                        (&mut chars).next();
                    } else { break; }
                }
                (&mut tokens).push(Token { kind: TokenKind::DocString, lexeme: buf });
            } else {
                // normal string
                (&mut chars).next(); // consume opening
                let mut buf: String = String::new();
                while let Some(ch) = (&mut chars).next() {
                    if ch == '"' { break; }
                    if ch == '\\' {
                        if let Some(esc) = (&mut chars).next() { (&mut buf).push(esc); } else { break; }
                    } else {
                        (&mut buf).push(ch);
                    }
                }
                (&mut tokens).push(Token { kind: TokenKind::String, lexeme: buf });
            }
        } else if c.is_ascii_alphabetic() || c == '_' {
            // Identifier or keyword
            let mut ident: String = String::new();
            while let Some(&d) = (&mut chars).peek() {
                if (&d).is_ascii_alphanumeric() || d == '_' {
                    (&mut ident).push(d);
                    (&mut chars).next();
                } else {
                    break;
                }
            }
            // TODO: Check for keywords, functions, constants
            (&mut tokens).push(Token { kind: TokenKind::Identifier, lexeme: ident });
        } else {
            // Single- and multi-character tokens and operators
            match c {
                ';' => { (&mut chars).next(); (&mut tokens).push(Token { kind: TokenKind::Semicolon, lexeme: ";".to_string() }); }
                '=' => {
                    (&mut chars).next();
                    if let Some('=') = (&mut chars).peek().copied() {
                        (&mut chars).next();
                        (&mut tokens).push(Token { kind: TokenKind::Equal, lexeme: "==".to_string() });
                    } else {
                        (&mut tokens).push(Token { kind: TokenKind::Assign, lexeme: "=".to_string() });
                    }
                }
                '!' => {
                    (&mut chars).next();
                    if let Some('=') = (&mut chars).peek().copied() {
                        (&mut chars).next();
                        (&mut tokens).push(Token { kind: TokenKind::NotEqual, lexeme: "!=".to_string() });
                    } else {
                        (&mut tokens).push(Token { kind: TokenKind::Bang, lexeme: "!".to_string() });
                    }
                }
                '>' => {
                    (&mut chars).next();
                    if let Some('=') = (&mut chars).peek().copied() {
                        (&mut chars).next();
                        (&mut tokens).push(Token { kind: TokenKind::GreaterEq, lexeme: ">=".to_string() });
                    } else {
                        (&mut tokens).push(Token { kind: TokenKind::Greater, lexeme: ">".to_string() });
                    }
                }
                '<' => {
                    (&mut chars).next();
                    if let Some('=') = (&mut chars).peek().copied() {
                        (&mut chars).next();
                        (&mut tokens).push(Token { kind: TokenKind::LessEq, lexeme: "<=".to_string() });
                    } else {
                        (&mut tokens).push(Token { kind: TokenKind::Less, lexeme: "<".to_string() });
                    }
                }
                '+' => { (&mut chars).next(); (&mut tokens).push(Token { kind: TokenKind::Plus, lexeme: "+".to_string() }); }
                '-' => { (&mut chars).next(); (&mut tokens).push(Token { kind: TokenKind::Minus, lexeme: "-".to_string() }); }
                '*' => { (&mut chars).next(); (&mut tokens).push(Token { kind: TokenKind::Star, lexeme: "*".to_string() }); }
                '/' => { (&mut chars).next(); (&mut tokens).push(Token { kind: TokenKind::Slash, lexeme: "/".to_string() }); }
                '^' => { (&mut chars).next(); (&mut tokens).push(Token { kind: TokenKind::Caret, lexeme: "^".to_string() }); }
                '(' => { (&mut chars).next(); (&mut tokens).push(Token { kind: TokenKind::LParen, lexeme: "(".to_string() }); }
                ')' => { (&mut chars).next(); (&mut tokens).push(Token { kind: TokenKind::RParen, lexeme: ")".to_string() }); }
                '{' => { (&mut chars).next(); (&mut tokens).push(Token { kind: TokenKind::LBrace, lexeme: "{".to_string() }); }
                '}' => { (&mut chars).next(); (&mut tokens).push(Token { kind: TokenKind::RBrace, lexeme: "}".to_string() }); }
                '[' => { (&mut chars).next(); (&mut tokens).push(Token { kind: TokenKind::LBracket, lexeme: "[".to_string() }); }
                ']' => { (&mut chars).next(); (&mut tokens).push(Token { kind: TokenKind::RBracket, lexeme: "]".to_string() }); }
                ',' => { (&mut chars).next(); (&mut tokens).push(Token { kind: TokenKind::Comma, lexeme: ",".to_string() }); }
                '|' => { (&mut chars).next(); (&mut tokens).push(Token { kind: TokenKind::Pipe, lexeme: "|".to_string() }); }
                '\'' => { (&mut chars).next(); (&mut tokens).push(Token { kind: TokenKind::Prime, lexeme: "'".to_string() }); }
                ':' => { (&mut chars).next(); (&mut tokens).push(Token { kind: TokenKind::Colon, lexeme: ":".to_string() }); }
                _ => {
                    (&mut tokens).push(Token { kind: TokenKind::Unknown, lexeme: (&c).to_string() });
                    (&mut chars).next();
                }
            }
        }
    }
    (&mut tokens).push(Token { kind: TokenKind::EOF, lexeme: String::new() });
    tokens
}
