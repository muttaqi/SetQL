use super::{ parser::*, utils::to_string_box };
use pest::iterators::*;

#[derive(Debug)]
pub enum StringBinaryOp {
    Concat,
}

#[derive(Debug)]
pub enum StringExpr {
    String(Box<String>),
    Op {
        lhs: Box<StringExpr>,
        op: StringBinaryOp,
        rhs: Box<StringExpr>,
    },
}

fn parse_primary(primary: Pair<Rule>) -> StringExpr {
    match primary.as_rule() {
        Rule::str => StringExpr::String(to_string_box(&primary)),
        Rule::str_expr => parse_str_expr(primary.into_inner()),
        rule => unreachable!("Expr::parse expected string atom, found {:?}", rule),
    }
}

fn parse_infix(lhs: StringExpr, op: Pair<Rule>, rhs: StringExpr) -> StringExpr {
    let op = match op.as_rule() {
        Rule::concat => StringBinaryOp::Concat,
        rule => unreachable!("Expected concatenation, but found {:?}", rule),
    };
    StringExpr::Op {
        lhs: Box::new(lhs),
        op,
        rhs: Box::new(rhs),
    }
}

pub fn parse_str_expr(pairs: Pairs<Rule>) -> StringExpr {
    PRATT_PARSER.map_primary(parse_primary).map_infix(parse_infix).parse(pairs)
}
