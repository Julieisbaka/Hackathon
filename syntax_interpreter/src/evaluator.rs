use std::collections::HashMap;
use num_complex::Complex64;

use crate::ast::{AstNode, UnaryOpKind, BinaryOpKind};

#[derive(Debug, Clone)]
pub enum Value {
    Number(f64),
    Complex(Complex64),
    Str(String),
    Array(Vec<Value>),
    Function(Function),
    Unit,
}

#[derive(Debug, Clone)]
pub struct Function {
    pub params: Vec<String>,
    pub body: AstNode,
}

#[derive(Default)]
pub struct Env {
    vars: HashMap<String, Value>,
    funcs: HashMap<String, Function>,
}

impl Env {
    pub fn new() -> Self { Self { vars: HashMap::new(), funcs: HashMap::new() } }
    pub fn with_builtins() -> Self {
        let mut env = Self::new();
        // constants
        env.vars.insert("e".to_string(), Value::Number(std::f64::consts::E));
        env.vars.insert("pi".to_string(), Value::Number(std::f64::consts::PI));
        env.vars.insert("i".to_string(), Value::Complex(Complex64::new(0.0, 1.0)));
        env.vars.insert("j".to_string(), Value::Complex(Complex64::new(0.0, 1.0)));
        env.vars.insert("k".to_string(), Value::Complex(Complex64::new(0.0, 1.0)));
        env
    }
}

pub fn eval(ast: &AstNode, env: &mut Env) -> Value {
    match ast {
        AstNode::Empty => Value::Unit,
        AstNode::Number(n) => Value::Number(*n),
    AstNode::Str(s) => Value::Str(s.clone()),
        AstNode::Constant(name) => env.vars.get(name).cloned().unwrap_or(Value::Unit),
        AstNode::Variable(name) => env.vars.get(name).cloned().unwrap_or(Value::Unit),
        AstNode::Assignment { name, expr } => {
            let val = eval(expr, env);
            env.vars.insert(name.clone(), val.clone());
            val
        }
        AstNode::UnaryOp { op, expr } => {
            let v = eval(expr, env);
            match op {
                UnaryOpKind::Negate => num_neg(v),
                UnaryOpKind::Not => bool_not(v),
                UnaryOpKind::Abs => num_abs(v),
                UnaryOpKind::Factorial => num_factorial(v),
            }
        }
        AstNode::BinaryOp { op, left, right } => {
            let l = eval(left, env);
            let r = eval(right, env);
            match op {
                BinaryOpKind::Add => lift_bin(l, r, |x,y| x+y),
                BinaryOpKind::Sub => lift_bin(l, r, |x,y| x-y),
                BinaryOpKind::Mul => lift_mul(l, r),
                BinaryOpKind::Div => lift_bin(l, r, |x,y| x/y),
                BinaryOpKind::Pow => lift_bin(l, r, |x,y| x.powf(y)),
                BinaryOpKind::Eq | BinaryOpKind::NotEq | BinaryOpKind::Gt | BinaryOpKind::Lt | BinaryOpKind::Gte | BinaryOpKind::Lte =>
                    compare(op.clone(), l, r),
            }
        }
        AstNode::Array(items) => Value::Array(items.iter().map(|e| eval(e, env)).collect()),
        AstNode::FunctionDef { name, params, body } => {
            let f = Function { params: params.clone(), body: (*body.clone()) };
            env.funcs.insert(name.clone(), f.clone());
            Value::Function(f)
        }
        AstNode::FunctionCall { name, args } => {
            let argv: Vec<Value> = args.iter().map(|a| eval(a, env)).collect();
            call_function(name, &argv, env)
        }
        AstNode::Import(path) => {
            // simple import: read file as lines and evaluate each line in current env
            if let Ok(content) = std::fs::read_to_string(path) {
                for line in content.lines() {
                    let tokens = crate::lexer::tokenize(line);
                    let ast = crate::parser::parse(&tokens);
                    let _ = eval(&ast, env);
                }
            }
            Value::Unit
        }
        AstNode::Print(args) => {
            let vals: Vec<Value> = args.iter().map(|a| eval(a, env)).collect();
            let out = vals.iter().map(|v| display_value(v)).collect::<Vec<_>>().join(" ");
            println!("{}", out);
            Value::Unit
        }
        AstNode::Log(args) => {
            let vals: Vec<Value> = args.iter().map(|a| eval(a, env)).collect();
            let out = vals.iter().map(|v| display_value(v)).collect::<Vec<_>>().join(" ");
            eprintln!("{}", out);
            Value::Unit
        }
        AstNode::DerivativeCall { name, args, var: _, order } => {
            // numeric derivative of single-arg function for now
            let n = *order;
            if n == 0 { return eval(&AstNode::FunctionCall { name: name.clone(), args: args.clone() }, env); }
            if args.len() != 1 { return Value::Unit; }
            let x = match eval(&args[0], env) { Value::Number(v) => v, _ => return Value::Unit };
            let h = 1e-5;
            let func_name = name.clone();
            let mut df = |xx: f64| -> f64 {
                let v = call_function(&func_name, &[Value::Number(xx)], env);
                match v { Value::Number(n) => n, _ => f64::NAN }
            };
            let mut f1 = |xx: f64| (df(xx + h) - df(xx - h)) / (2.0 * h);
            if n == 1 { return Value::Number(f1(x)); }
            // second derivative
            let mut f2 = |xx: f64| (df(xx + h) - 2.0*df(xx) + df(xx - h)) / (h*h);
            if n == 2 { return Value::Number(f2(x)); }
            // higher order not implemented
            Value::Unit
        }
        AstNode::Conditional { condition, body } => {
            let c = eval(condition, env);
            if is_true(&c) { eval(body, env) } else { Value::Unit }
        }
        AstNode::DerivativeExpr { var, order, expr } => {
            // numeric derivative of expression w.r.t variable name
            let var_name = var.clone();
            let n = *order;
            if n == 0 { return eval(expr, env); }
            let x0 = match env.vars.get(&var_name).cloned().unwrap_or(Value::Number(0.0)) { Value::Number(v) => v, _ => 0.0 };
            let h = 1e-5;
            let f = |xx: f64, env: &mut Env| -> f64 {
                env.vars.insert(var_name.clone(), Value::Number(xx));
                match eval(expr, env) { Value::Number(v) => v, _ => f64::NAN }
            };
            if n == 1 { return Value::Number((f(x0 + h, env) - f(x0 - h, env)) / (2.0*h)); }
            if n == 2 { return Value::Number((f(x0 + h, env) - 2.0*f(x0, env) + f(x0 - h, env)) / (h*h)); }
            // higher order: repeat first derivative n times
            let mut res = (f(x0 + h, env) - f(x0 - h, env)) / (2.0*h);
            for _ in 2..=n {
                let g = |xx: f64, env: &mut Env| (f(xx + h, env) - f(xx - h, env)) / (2.0*h);
                res = g(x0, env);
            }
            Value::Number(res)
        }
    }
}

fn is_true(v: &Value) -> bool {
    match v {
        Value::Number(n) => *n != 0.0,
        Value::Complex(c) => c.norm() != 0.0,
        Value::Array(a) => !a.is_empty(),
        Value::Str(s) => !s.is_empty(),
        Value::Function(_) => true,
        Value::Unit => false,
    }
}

fn call_function(name: &str, args: &[Value], env: &mut Env) -> Value {
    // built-ins
    if let Some(b) = call_builtin(name, args) { return b; }
    // user-defined
    if let Some(f) = env.funcs.get(name).cloned() {
        let mut local = Env::with_builtins();
        for (i, p) in f.params.iter().enumerate() {
            if let Some(v) = args.get(i) { local.vars.insert(p.clone(), v.clone()); }
        }
        return eval(&f.body, &mut local);
    }
    Value::Unit
}

fn call_builtin(name: &str, args: &[Value]) -> Option<Value> {
    let n1 = |v: &Value| if let Value::Number(x) = v { Some(*x) } else { None };
    let map1 = |f: fn(f64)->f64| args.get(0).and_then(n1).map(|x| Value::Number(f(x)));
    match name {
        "sin" => return map1(f64::sin),
        "cos" => return map1(f64::cos),
        "tan" => return map1(f64::tan),
        "sec" => return map1(|x| 1.0 / x.cos()),
        "csc" => return map1(|x| 1.0 / x.sin()),
        "cot" => return map1(|x| x.cos() / x.sin()),
        "asin" => return map1(f64::asin),
        "acos" => return map1(f64::acos),
        "atan" => return map1(f64::atan),
        "asec" => return map1(|x| (1.0/x).acos()),
        "acsc" => return map1(|x| (1.0/x).asin()),
        "acot" => return map1(|x| (1.0/x).atan()),
        "sinh" => return map1(f64::sinh),
        "cosh" => return map1(f64::cosh),
        "tanh" => return map1(f64::tanh),
        "sech" => return map1(|x| 1.0 / x.cosh()),
        "csch" => return map1(|x| 1.0 / x.sinh()),
        "coth" => return map1(|x| x.cosh() / x.sinh()),
        "asinh" => return map1(f64::asinh),
        "acosh" => return map1(f64::acosh),
        "atanh" => return map1(f64::atanh),
        "asech" => return map1(|x| (1.0/x).acosh()),
        "acsch" => return map1(|x| (1.0/x).asinh()),
        "acoth" => return map1(|x| (1.0/x).atanh()),
        "ln" => return map1(f64::ln),
        "log" => return map1(|x| x.log10()),
        "erf" => return map1(statrs::function::erf::erf),
        "erfc" => return map1(statrs::function::erf::erfc),
        "print" => {
            // print variadic: convert to strings and return ()
            let parts: Vec<String> = args.iter().map(|v| display_value(v)).collect();
            println!("{}", parts.join(" "));
            return Some(Value::Unit)
        }
        _ => {}
    }
    None
}

fn num_neg(v: Value) -> Value { match v { Value::Number(n) => Value::Number(-n), _ => Value::Unit } }
fn bool_not(v: Value) -> Value { Value::Number(if is_true(&v) {0.0} else {1.0}) }
fn num_abs(v: Value) -> Value { match v { Value::Number(n) => Value::Number(n.abs()), Value::Complex(c) => Value::Number(c.norm()), _ => Value::Unit } }
fn num_factorial(v: Value) -> Value {
    match v {
        Value::Number(n) if n >= 0.0 && n.fract() == 0.0 => {
            let mut acc = 1.0;
            let mut i = 1.0;
            while i <= n { acc *= i; i += 1.0; }
            Value::Number(acc)
        }
        _ => Value::Unit,
    }
}


fn bin_num(a: Value, b: Value, f: fn(f64,f64)->f64) -> Value {
    match (a, b) {
        (Value::Number(x), Value::Number(y)) => Value::Number(f(x,y)),
        _ => Value::Unit,
    }
}

fn compare(op: BinaryOpKind, a: Value, b: Value) -> Value {
    let (l, r) = match (a, b) {
        (Value::Number(x), Value::Number(y)) => (x, y),
        _ => return Value::Unit,
    };
    let res = match op {
    BinaryOpKind::Eq => l == r,
    BinaryOpKind::NotEq => l != r,
    BinaryOpKind::Gt => l > r,
    BinaryOpKind::Lt => l < r,
    BinaryOpKind::Gte => l >= r,
    BinaryOpKind::Lte => l <= r,
        _ => false,
    };
    Value::Number(if res {1.0} else {0.0})
}

fn lift_bin(a: Value, b: Value, f: fn(f64,f64)->f64) -> Value {
    match (a, b) {
        (Value::Number(x), Value::Number(y)) => Value::Number(f(x,y)),
        (Value::Array(ax), Value::Number(y)) => map_array_scalar_right(ax, y, f),
        (Value::Number(x), Value::Array(by)) => map_array_scalar_left(x, by, f),
        (Value::Array(ax), Value::Array(by)) => map_arrays_rec(ax, by, f),
        (x, y) => bin_num(x, y, f),
    }
}

fn lift_mul(a: Value, b: Value) -> Value {
    // If both are proper 2D matrices, try matrix multiplication; otherwise element-wise
    if let (Some(am), Some(bm)) = (as_matrix(&a), as_matrix(&b)) {
        if let Some(prod) = matrix_mul(&am, &bm) {
            return from_matrix(prod);
        }
    }
    lift_bin(a, b, |x,y| x*y)
}

fn map_array_scalar_right(arr: Vec<Value>, y: f64, f: fn(f64,f64)->f64) -> Value {
    Value::Array(arr.into_iter().map(|v| match v {
        Value::Number(x) => Value::Number(f(x, y)),
        Value::Array(a2) => match map_array_scalar_right(a2, y, f) { Value::Array(inner) => Value::Array(inner), other => other },
        other => other,
    }).collect())
}

fn map_array_scalar_left(x: f64, arr: Vec<Value>, f: fn(f64,f64)->f64) -> Value {
    Value::Array(arr.into_iter().map(|v| match v {
        Value::Number(y) => Value::Number(f(x, y)),
        Value::Array(a2) => match map_array_scalar_left(x, a2, f) { Value::Array(inner) => Value::Array(inner), other => other },
        other => other,
    }).collect())
}

fn map_arrays_rec(a: Vec<Value>, b: Vec<Value>, f: fn(f64,f64)->f64) -> Value {
    if a.len() != b.len() { return Value::Unit; }
    let mut out = Vec::with_capacity(a.len());
    for (va, vb) in a.into_iter().zip(b.into_iter()) {
        match (va, vb) {
            (Value::Number(x), Value::Number(y)) => out.push(Value::Number(f(x,y))),
            (Value::Array(ax), Value::Array(by)) => {
                match map_arrays_rec(ax, by, f) { Value::Array(inner) => out.push(Value::Array(inner)), _ => return Value::Unit }
            }
            _ => return Value::Unit,
        }
    }
    Value::Array(out)
}

fn as_matrix(v: &Value) -> Option<Vec<Vec<f64>>> {
    match v {
        Value::Array(rows) => {
            let mut m = Vec::with_capacity(rows.len());
            let mut width: Option<usize> = None;
            for r in rows {
                if let Value::Array(cols) = r {
                    let mut row = Vec::with_capacity(cols.len());
                    for c in cols {
                        if let Value::Number(x) = c { row.push(*x); } else { return None; }
                    }
                    if let Some(w) = width { if w != row.len() { return None; } } else { width = Some(row.len()); }
                    m.push(row);
                } else {
                    return None;
                }
            }
            Some(m)
        }
        _ => None,
    }
}

fn from_matrix(m: Vec<Vec<f64>>) -> Value {
    let rows: Vec<Value> = m.into_iter().map(|r| Value::Array(r.into_iter().map(Value::Number).collect())).collect();
    Value::Array(rows)
}

fn matrix_mul(a: &Vec<Vec<f64>>, b: &Vec<Vec<f64>>) -> Option<Vec<Vec<f64>>> {
    let m = a.len();
    let n = if m > 0 { a[0].len() } else { 0 };
    let n2 = b.len();
    let p = if n2 > 0 { b[0].len() } else { 0 };
    if n != n2 { return None; }
    // verify consistent widths
    if a.iter().any(|row| row.len()!=n) { return None; }
    if b.iter().any(|row| row.len()!=p) { return None; }
    let mut out = vec![vec![0.0; p]; m];
    for i in 0..m {
        for k in 0..n {
            let aik = a[i][k];
            for j in 0..p {
                out[i][j] += aik * b[k][j];
            }
        }
    }
    Some(out)
}

fn display_value(v: &Value) -> String {
    match v {
        Value::Number(n) => n.to_string(),
        Value::Complex(c) => format!("{}+{}i", c.re, c.im),
        Value::Str(s) => s.clone(),
        Value::Array(a) => {
            let parts: Vec<String> = a.iter().map(|x| display_value(x)).collect();
            format!("[{}]", parts.join(", "))
        }
        Value::Function(f) => format!("<function:{} params>", f.params.len()),
        Value::Unit => "()".to_string(),
    }
}

// numerical_derivative_n removed for simplicity
