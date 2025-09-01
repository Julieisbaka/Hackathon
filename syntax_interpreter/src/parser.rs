// Parser module for building AST from tokens


use crate::lexer::{Token, TokenKind};
use crate::ast::{AstNode, UnaryOpKind, BinaryOpKind};

pub fn parse(tokens: &[Token]) -> AstNode {
    let mut parser: Parser<'_> = Parser::new(tokens);
    let mut stmts: Vec<AstNode> = Vec::new();
    while parser.pos < parser.tokens.len() {
        // skip EOF
        if matches!(parser.peek().map(|t| &t.kind), Some(TokenKind::EOF)) { break; }
        // skip empty/unknown tokens (e.g., newlines)
        while matches!(parser.peek().map(|t| &t.kind), Some(TokenKind::Unknown)) {
            parser.pos += 1;
        }
        if matches!(parser.peek().map(|t| &t.kind), Some(TokenKind::EOF)) { break; }
        if let Some(stmt) = (&mut parser).parse_statement() {
            (&mut stmts).push(stmt);
        }
        // After a statement, if the next token is a semicolon, consume it (explicit statement separator)
        if matches!(parser.peek().map(|t| &t.kind), Some(TokenKind::Semicolon)) {
            parser.pos += 1;
        }
        // If not a semicolon or EOF, do not advance; let the next loop handle the next statement
    }
    AstNode::Program(stmts)
}

struct Parser<'a> {
    tokens: &'a [Token],
    pos: usize,
}

impl<'a> Parser<'a> {
    fn lookahead_kind(&self, n: usize) -> Option<&TokenKind> {
        (*self).tokens.get((*self).pos + n).map(|t: &Token| -> &TokenKind { &(*t).kind })
    }

    fn parse_statement(&mut self) -> Option<AstNode> {
        // function definition: f(x, y) = expr
        // assignment: x = expr
        // import: import "file"
        if let Some(Token { kind: TokenKind::Identifier, lexeme }) = self.peek() {
            if lexeme == "import" {
                self.next();
                match self.peek() {
                    Some(Token { kind: TokenKind::String, lexeme: path }) => {
                        let p: String = path.clone();
                        self.next();
                        return Some(AstNode::Import(p));
                    }
                    _ => return None,
                }
            }
            if lexeme == "print" {
                self.next();
                self.expect(TokenKind::LParen)?;
                let args: Vec<AstNode> = self.parse_arg_list()?;
                return Some(AstNode::Print(args));
            }
            if lexeme == "log" {
                self.next();
                self.expect(TokenKind::LParen)?;
                let args: Vec<AstNode> = self.parse_arg_list()?;
                return Some(AstNode::Log(args));
            }
        }
        if let Some(Token { kind: TokenKind::Identifier, lexeme }) = self.peek() {
            let name: String = lexeme.clone();
            // function def pattern (only allowed as a statement, not as an expression)
            if matches!(self.lookahead_kind(1), Some(TokenKind::LParen)) {
                self.next(); // name
                self.next(); // (
                let params = self.parse_params();
                if params.is_none() {
                    return Some(AstNode::Error("invalid syntax".to_string()));
                }
                let params = params.unwrap();
                if !self.match_kind(TokenKind::Assign) {
                    return Some(AstNode::Error("invalid syntax".to_string()));
                }
                // Parse only a single expression for the function body
                let expr = self.parse_expression(0).unwrap_or(AstNode::Empty);
                let mut expr = expr;
                // Check for trailing {condition} after function body
                if self.match_kind(TokenKind::LBrace) {
                    let cond = self.parse_condition_expression(0);
                    if cond.is_none() || !self.match_kind(TokenKind::RBrace) {
                        return Some(AstNode::Error("invalid syntax".to_string()));
                    }
                    expr = AstNode::Conditional { condition: Box::new(cond.unwrap()), body: Box::new(expr) };
                }
                return Some(AstNode::FunctionDef { name, params, body: Box::new(expr) });
            }
            // assignment pattern
            if matches!(self.lookahead_kind(1), Some(TokenKind::Assign)) {
                self.next(); // name
                self.next(); // =
                let expr: AstNode = self.parse_expression(0)?;
                return Some(AstNode::Assignment { name, expr: Box::new(expr) });
            }
        }
        // fallback to expression
        self.parse_expression(0)
    }

    fn parse_params(&mut self) -> Option<Vec<String>> {
        let mut params: Vec<String> = Vec::new();
        if self.match_kind(TokenKind::RParen) { return Some(params); }
        loop {
            match self.peek() {
                Some(Token { kind: TokenKind::Identifier, lexeme }) => {
                    let p: String = lexeme.clone();
                    self.next();
                    (&mut params).push(p);
                }
                _ => return None,
            }
            if self.match_kind(TokenKind::Comma) { continue; }
            if !self.match_kind(TokenKind::RParen) {
                return None;
            }
            break;
        }
        Some(params)
    }
    fn new(tokens: &'a [Token]) -> Self {
        Parser { tokens, pos: 0 }
    }

    fn peek(&self) -> Option<&Token> {
        (*self).tokens.get((*self).pos)
    }

    fn next(&mut self) -> Option<&Token> {
        let tok: Option<&Token> = (*self).tokens.get((*self).pos);
        if (&tok).is_some() {
            (*self).pos += 1;
        }
        tok
    }

    fn parse_expression(&mut self, min_bp: u8) -> Option<AstNode> {
        let mut lhs: AstNode = self.parse_prefix()?;
        lhs = self.parse_postfix(lhs)?;

        loop {
            // Stop implicit multiplication and expression parsing at statement boundaries
            match self.peek() {
                Some(Token { kind: TokenKind::Semicolon, .. }) |
                Some(Token { kind: TokenKind::EOF, .. }) => break,
                _ => {}
            }

            // Check for implicit multiplication
            let implicit_mul = match self.peek() {
                Some(Token { kind: TokenKind::Identifier, .. })
                | Some(Token { kind: TokenKind::Number, .. })
                | Some(Token { kind: TokenKind::LParen, .. })
                | Some(Token { kind: TokenKind::LBracket, .. }) => {
                    match &lhs {
                        AstNode::Number(_) | AstNode::Variable(_) | AstNode::Str(_) | AstNode::FunctionCall { .. } | AstNode::Array(_) | AstNode::UnaryOp { .. } | AstNode::BinaryOp { .. } => true,
                        _ => false,
                    }
                }
                _ => false,
            };
            if implicit_mul {
                // Precedence for implicit multiplication is the same as explicit '*'
                let rbp = 6;
                if 5 < min_bp { break; }
                let rhs = self.parse_expression(rbp)?;
                lhs = AstNode::BinaryOp { op: BinaryOpKind::Mul, left: Box::new(lhs), right: Box::new(rhs) };
                continue;
            }

            let op: TokenKind = match self.peek() {
                Some(t) => (&(*t).kind).clone(),
                None => break,
            };

            let (lbp, rbp, bop) = match op {
                TokenKind::Caret => (7, 6, Some(BinaryOpKind::Pow)),
                TokenKind::Star => (5, 6, Some(BinaryOpKind::Mul)),
                TokenKind::Slash => (5, 6, Some(BinaryOpKind::Div)),
                TokenKind::Plus => (3, 4, Some(BinaryOpKind::Add)),
                TokenKind::Minus => (3, 4, Some(BinaryOpKind::Sub)),
                TokenKind::GreaterEq => (2, 3, Some(BinaryOpKind::Gte)),
                TokenKind::LessEq => (2, 3, Some(BinaryOpKind::Lte)),
                TokenKind::Greater => (2, 3, Some(BinaryOpKind::Gt)),
                TokenKind::Less => (2, 3, Some(BinaryOpKind::Lt)),
                TokenKind::Equal => (2, 3, Some(BinaryOpKind::Eq)),
                TokenKind::NotEqual => (2, 3, Some(BinaryOpKind::NotEq)),
                _ => break,
            };

            if lbp < min_bp { break; }
            self.next();
            let rhs: AstNode = self.parse_expression(rbp)?;
            lhs = AstNode::BinaryOp { op: bop.unwrap(), left: Box::new(lhs), right: Box::new(rhs) };
        }

        Some(lhs)
    }

    fn parse_prefix(&mut self) -> Option<AstNode> {
        // derivative operator: d^n/dx^n expr
        if let Some(Token { kind: TokenKind::Identifier, lexeme }) = self.peek() {
            if lexeme == "d" {
                // consume 'd'
                self.next();
                let mut order = 1usize;
                if self.match_kind(TokenKind::Caret) {
                    if let Some(Token { kind: TokenKind::Number, lexeme }) = self.peek() {
                        if let Ok(n) = (&**lexeme).parse::<usize>() { order = n; }
                        self.next();
                    } else { return None; }
                }
                // expect '/' 'd' var [^n]
                self.expect(TokenKind::Slash)?;
                if let Some(Token { kind: TokenKind::Identifier, lexeme }) = self.peek() {
                    if lexeme == "d" {
                        self.next(); // 'd'
                        let var: String = if let Some(Token { kind: TokenKind::Identifier, lexeme }) = self.peek() {
                            let v: String = lexeme.clone();
                            self.next();
                            v
                        } else { return None };
                        // optional ^n on denominator (ignored if differs)
                        if self.match_kind(TokenKind::Caret) {
                            if let Some(Token { kind: TokenKind::Number, .. }) = self.peek() { self.next(); }
                        }
                        // target expression, parentheses optional
                        let expr: AstNode = if self.match_kind(TokenKind::LParen) {
                            let e: AstNode = self.parse_expression(0)?;
                            self.expect(TokenKind::RParen)?;
                            e
                        } else {
                            self.parse_expression(6)?
                        };
                        return Some(AstNode::DerivativeExpr { var, order, expr: Box::new(expr) });
                    }
                }
                return None;
            }
        }
        match (*self.peek()?).kind {
            TokenKind::Number => {
                let tok: &Token = self.next()?;
                let v: f64 = (&*(*tok).lexeme).parse::<f64>().ok()?;
                Some(AstNode::Number(v))
            }
            TokenKind::String => {
                let tok: &Token = self.next()?;
                Some(AstNode::Str((&(*tok).lexeme).clone()))
            }
            TokenKind::Identifier => {
                // function call or variable
                let mut name: String = (&(*self.next()?).lexeme).clone();
                // inverse marker name^-1(...)
                let mut inverse: bool = false;
                if self.match_inverse_marker() { inverse = true; }
                // prime markers name' ' '
                let mut prime_order = 0usize;
                while self.match_kind(TokenKind::Prime) { prime_order += 1; }
                // bracket derivative order name[5]'
                let mut bracket_order: Option<usize> = None;
                if self.match_kind(TokenKind::LBracket) {
                    if let Some(Token { kind: TokenKind::Number, lexeme }) = self.peek() {
                        let k = (&**lexeme).parse::<usize>().ok()?;
                        self.next();
                        self.expect(TokenKind::RBracket)?;
                        if self.match_kind(TokenKind::Prime) { bracket_order = Some(k); }
                    } else {
                        return None;
                    }
                }
                if self.match_kind(TokenKind::LParen) {
                    let args: Vec<AstNode> = self.parse_arg_list()?;
                    if inverse { name = inverse_name(&**&name); }
                    let total_order = bracket_order.unwrap_or(0) + prime_order;
                    if total_order > 0 {
                        Some(AstNode::DerivativeCall { name, args, var: None, order: total_order })
                    } else {
                        Some(AstNode::FunctionCall { name, args })
                    }
                } else {
                    if inverse { name = inverse_name(&**&name); }
                    Some(AstNode::Variable(name))
                }
            }
            TokenKind::Bang => {
                // logical negation
                self.next();
                let expr = self.parse_expression(6)?;
                Some(AstNode::UnaryOp { op: UnaryOpKind::Not, expr: Box::new(expr) })
            }
            TokenKind::LParen => {
                self.next();
                let expr: AstNode = self.parse_expression(0)?;
                self.expect(TokenKind::RParen)?;
                Some(expr)
            }
            TokenKind::Pipe => {
                // |expr|
                self.next();
                let inner: AstNode = self.parse_expression(0)?;
                self.expect(TokenKind::Pipe)?;
                Some(AstNode::UnaryOp { op: UnaryOpKind::Abs, expr: Box::new(inner) })
            }
            TokenKind::LBracket => {
                // [a, b, c]
                self.next();
                let mut items: Vec<AstNode> = Vec::new();
                if self.match_kind(TokenKind::RBracket) {
                    return Some(AstNode::Array(items));
                }
                loop {
                    let expr: AstNode = self.parse_expression(0)?;
                    (&mut items).push(expr);
                    if self.match_kind(TokenKind::Comma) { continue; }
                    self.expect(TokenKind::RBracket)?;
                    break;
                }
                Some(AstNode::Array(items))
            }
            TokenKind::Minus => {
                self.next();
                let expr: AstNode = self.parse_expression(6)?;
                Some(AstNode::UnaryOp { op: UnaryOpKind::Negate, expr: Box::new(expr) })
            }
            _ => None,
        }
    }

    fn parse_postfix(&mut self, mut lhs: AstNode) -> Option<AstNode> {
        loop {
            match self.peek().map(|t: &Token| -> TokenKind { (&(*t).kind).clone() }) {
                Some(TokenKind::Bang) => {
                    self.next();
                    lhs = AstNode::UnaryOp { op: UnaryOpKind::Factorial, expr: Box::new(lhs) };
                }
                Some(TokenKind::Prime) => {
                    self.next();
                    // Represent prime as derivative order on variable-less function call later; for now, ignore or wrap as Unary Not supported
                    // If lhs is a function call f(x), transform into DerivativeCall with order increment.
                    lhs = match lhs {
                        AstNode::FunctionCall { name, args } => AstNode::DerivativeCall { name, args, var: None, order: 1 },
                        other => other,
                    };
                }
                Some(TokenKind::LBrace) => {
                    self.next();
                    let cond: AstNode = self.parse_condition_expression(0)?;
                    self.expect(TokenKind::RBrace)?;
                    lhs = AstNode::Conditional { condition: Box::new(cond), body: Box::new(lhs) };
                }
                _ => break,
            }
        }
        Some(lhs)
    }

    fn parse_condition_expression(&mut self, min_bp: u8) -> Option<AstNode> {
        // same as parse_expression, but treat '=' as equality
        let mut lhs: AstNode = self.parse_prefix()?;
        loop {
            let op: TokenKind = match self.peek() { Some(t) => (&(*t).kind).clone(), None => break };
            let (lbp, rbp, bop) = match op {
                TokenKind::Caret => (7, 6, Some(BinaryOpKind::Pow)),
                TokenKind::Star => (5, 6, Some(BinaryOpKind::Mul)),
                TokenKind::Slash => (5, 6, Some(BinaryOpKind::Div)),
                TokenKind::Plus => (3, 4, Some(BinaryOpKind::Add)),
                TokenKind::Minus => (3, 4, Some(BinaryOpKind::Sub)),
                TokenKind::GreaterEq => (2, 3, Some(BinaryOpKind::Gte)),
                TokenKind::LessEq => (2, 3, Some(BinaryOpKind::Lte)),
                TokenKind::Greater => (2, 3, Some(BinaryOpKind::Gt)),
                TokenKind::Less => (2, 3, Some(BinaryOpKind::Lt)),
                TokenKind::Equal => (2, 3, Some(BinaryOpKind::Eq)),
                TokenKind::NotEqual => (2, 3, Some(BinaryOpKind::NotEq)),
                TokenKind::Assign => (2, 3, Some(BinaryOpKind::Eq)), // reinterpret '=' as equality in conditions
                _ => break,
            };
            if lbp < min_bp { break; }
            self.next();
            let rhs: AstNode = self.parse_condition_expression(rbp)?;
            lhs = AstNode::BinaryOp { op: bop.unwrap(), left: Box::new(lhs), right: Box::new(rhs) };
        }
        Some(lhs)
    }

    fn parse_arg_list(&mut self) -> Option<Vec<AstNode>> {
        let mut args: Vec<AstNode> = Vec::new();
        if self.match_kind(TokenKind::RParen) {
            return Some(args);
        }
        loop {
            let expr: AstNode = self.parse_expression(0)?;
            (&mut args).push(expr);
            if self.match_kind(TokenKind::Comma) { continue; }
            self.expect(TokenKind::RParen)?;
            break;
        }
        Some(args)
    }

    fn match_kind(&mut self, kind: TokenKind) -> bool {
        if let Some(tok) = self.peek() {
            if (*tok).kind == kind { self.next(); return true; }
        }
        false
    }

    fn expect(&mut self, kind: TokenKind) -> Option<()> {
        if self.match_kind(kind) { Some(()) } else { None }
    }

    fn match_inverse_marker(&mut self) -> bool {
        // matches ^ - 1 sequence
        if matches!(self.lookahead_kind(0), Some(TokenKind::Caret))
            && matches!(self.lookahead_kind(1), Some(TokenKind::Minus))
            && matches!(self.lookahead_kind(2), Some(TokenKind::Number))
        {
            // ensure the number is 1
            // consume tokens and verify lexeme
            self.next(); // ^
            self.next(); // -
            // number token must be "1"
            if let Some(tok) = self.next() {
                if (*tok).lexeme == "1" { return true; }
            }
            return false;
        }
        false
    }
}

fn inverse_name(name: &str) -> String {
    match name {
        "sin" => "asin".to_string(),
        "cos" => "acos".to_string(),
        "tan" => "atan".to_string(),
        "sinh" => "asinh".to_string(),
        "cosh" => "acosh".to_string(),
        "tanh" => "atanh".to_string(),
        _ => format!("{}^-1", name),
    }
}
