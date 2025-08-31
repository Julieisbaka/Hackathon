// AST module for representing parsed expressions/statements

#[derive(Debug, Clone, PartialEq)]
pub enum AstNode {
    Empty,
    Number(f64),
    Str(String),
    Constant(String),
    Variable(String),
    UnaryOp {
        op: UnaryOpKind,
        expr: Box<AstNode>,
    },
    BinaryOp {
        op: BinaryOpKind,
        left: Box<AstNode>,
        right: Box<AstNode>,
    },
    Assignment {
        name: String,
        expr: Box<AstNode>,
    },
    FunctionDef {
        name: String,
        params: Vec<String>,
        body: Box<AstNode>,
    },
    FunctionCall {
        name: String,
        args: Vec<AstNode>,
    },
    Array(Vec<AstNode>),
    Conditional {
        condition: Box<AstNode>,
        body: Box<AstNode>,
    },
    DerivativeExpr {
        var: String,
        order: usize,
        expr: Box<AstNode>,
    },
    DerivativeCall {
        name: String,
        args: Vec<AstNode>,
        var: Option<String>,
        order: usize,
    },
    Import(String),
    Print(Vec<AstNode>),
    Log(Vec<AstNode>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnaryOpKind {
    Negate = 0,
    Factorial = 1,
    Abs = 2,
    Not = 3,
}

#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOpKind {
    Add = 0,
    Sub = 1,
    Mul = 2,
    Div = 3,
    Pow = 4,
    Eq = 5,
    NotEq = 6,
    Gt = 7,
    Lt = 8,
    Gte = 9,
    Lte = 10,
}
