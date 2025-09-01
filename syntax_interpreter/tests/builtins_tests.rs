// Basic tests for new built-in functions
use syntax_interpreter::evaluator::{Env, eval};
use syntax_interpreter::ast::AstNode;

fn eval_expr(expr: &str) -> f64 {
    let mut env = Env::with_builtins();
    let tokens = syntax_interpreter::lexer::tokenize(expr);
    let ast = syntax_interpreter::parser::parse(&tokens);
    match eval(&ast, &mut env) {
        syntax_interpreter::evaluator::Value::Number(n) => n,
        _ => panic!("Expected number result"),
    }
}

#[test]

#[test]
fn test_lim() {
    // lim {x -> 0} x^2 = 0
    assert!((eval_expr("lim {x -> 0} x^2") - 0.0).abs() < 1e-8);
    // lim {x -> 0} sin(x)/x = 1
    assert!((eval_expr("lim {x -> 0} sin(x)/x") - 1.0).abs() < 1e-4);
}

#[test]
fn test_round() {
    assert_eq!(eval_expr("round(2.7)"), 3.0);
    assert_eq!(eval_expr("round(-2.3)"), -2.0);
}

#[test]
fn test_trunc() {
    assert_eq!(eval_expr("trunc(2.7)"), 2.0);
    assert_eq!(eval_expr("trunc(-2.7)"), -2.0);
}

#[test]
fn test_floor() {
    assert_eq!(eval_expr("floor(2.7)"), 2.0);
    assert_eq!(eval_expr("floor(-2.7)"), -3.0);
}

#[test]
fn test_ceil() {
    assert_eq!(eval_expr("ceil(2.1)"), 3.0);
    assert_eq!(eval_expr("ceil(-2.1)"), -2.0);
}

#[test]
fn test_mod() {
    assert_eq!(eval_expr("10 mod 3"), 1.0);
    assert_eq!(eval_expr("-10 mod 3"), -1.0);
}

#[test]
fn test_clamp_alias() {
    assert_eq!(eval_expr("clamp(5, 1, 10)"), 5.0);
    assert_eq!(eval_expr("clamp(-1, 0, 10)"), 0.0);
    assert_eq!(eval_expr("clamp(15, 0, 10)"), 10.0);
}

#[test]
fn test_rand_range() {
    let v = eval_expr("rand(0, 1)");
    assert!(v >= 0.0 && v < 1.0);
}

#[test]
fn test_rand_unit() {
    let v = eval_expr("rand()");
    assert!(v >= 0.0 && v < 1.0);
}

#[test]
fn test_int() {
    // int of f(x)=1 from 0 to 1 should be ~1
    let code = "int((x) => 1, 0, 1)";
    let v = eval_expr(code);
    assert!((v - 1.0).abs() < 0.01);
}