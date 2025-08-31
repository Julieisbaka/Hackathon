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
    Negate,
    Factorial,
    Abs,
    Not,
}

#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOpKind {
    Add,
    Sub,
    Mul,
    Div,
    Pow,
    Eq,
    NotEq,
    Gt,
    Lt,
    Gte,
    Lte,
}
