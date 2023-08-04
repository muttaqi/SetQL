use pest::iterators::{Pair, Pairs};
use super::parser::*;

#[derive(Debug)]
pub enum NumericalBinaryOp {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
}

#[derive(Debug)]
pub enum NumericalExpr {
    Integer(i32),
    Real(f64),
    UnaryMinus(Box<NumericalExpr>),
    Op {
        lhs: Box<NumericalExpr>,
        op: NumericalBinaryOp,
        rhs: Box<NumericalExpr>,
    },
}

fn parse_num(pair: Pair<Rule>) -> NumericalExpr {
    let num_str: String = pair.as_str().to_string();
    if num_str.contains(".") {
        NumericalExpr::Real(num_str.parse::<f64>().unwrap())
    } else {
        NumericalExpr::Integer(num_str.parse::<i32>().unwrap())
    }
}

fn parse_primary(primary: Pair<Rule>) -> NumericalExpr {
    match primary.as_rule() {
        Rule::num => parse_num(primary),
        Rule::num_expr => parse_num_expr(primary.into_inner()),
        rule => unreachable!("Expr::parse expected atom, found {:?}", rule),
    }
}

fn parse_infix(lhs: NumericalExpr, op: Pair<Rule>, rhs: NumericalExpr) -> NumericalExpr {
    let op = match op.as_rule() {
        Rule::add => NumericalBinaryOp::Add,
        Rule::subtract => NumericalBinaryOp::Subtract,
        Rule::multiply => NumericalBinaryOp::Multiply,
        Rule::divide => NumericalBinaryOp::Divide,
        Rule::modulo => NumericalBinaryOp::Modulo,
        rule => unreachable!("Expected infix numerical operation, but found {:?}", rule),
    };
    NumericalExpr::Op {
        lhs: Box::new(lhs),
        op,
        rhs: Box::new(rhs),
    }
}

fn parse_prefix(op: Pair<Rule>, rhs: NumericalExpr) -> NumericalExpr {
    match op.as_rule() {
        Rule::unary_minus => NumericalExpr::UnaryMinus(Box::new(rhs)),
        rule => unreachable!("Expected unary minus, but found {:?}", rule),
    }
}

pub fn parse_num_expr(pairs: Pairs<Rule>) -> NumericalExpr {
    PRATT_PARSER.map_primary(parse_primary)
        .map_infix(parse_infix)
        .map_prefix(parse_prefix)
        .parse(pairs)
}
