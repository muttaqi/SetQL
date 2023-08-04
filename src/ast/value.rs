use super::numerical::*;
use super::string::*;
use super::parser::*;
use super::utils::to_string_box;
use pest::iterators::*;
use trace::trace;

trace::init_depth_var!();

#[derive(Debug)]
pub enum BinaryComparator {
    Equals,
    GreaterThan,
    LessThan,
    GreaterThanOrEqual,
    LessThanOrEqual,
}

fn parse_binary_comparator(mut pairs: Pairs<Rule>) -> BinaryComparator {
    let pair = pairs.next().unwrap();
    match pair.as_rule() {
        Rule::eq => BinaryComparator::Equals,
        Rule::gt => BinaryComparator::GreaterThan,
        Rule::lt => BinaryComparator::LessThan,
        Rule::geq => BinaryComparator::GreaterThanOrEqual,
        Rule::leq => BinaryComparator::LessThanOrEqual,
        rule => unreachable!("Expected binary comparator, but found {:?}", rule),
    }
}

#[derive(Debug)]
pub struct ValueClause {
    lhs: ValueExpr,
    comparator: BinaryComparator,
    rhs: ValueExpr,
}

pub fn parse_value_clause(pairs: Pairs<Rule>) -> ValueClause {
    let pairs_vec: Vec<Pair<Rule>> = pairs.collect();
    ValueClause {
        lhs: parse_val_expr(pairs_vec[0].to_owned().into_inner()),
        comparator: parse_binary_comparator(pairs_vec[1].to_owned().into_inner()),
        rhs: parse_val_expr(pairs_vec[2].to_owned().into_inner()),
    }
}

#[derive(Debug)]
pub enum ValueOperator {
    Identity,
    Function(Box<String>),
}

#[trace]
fn parse_val_operator(mut pairs: Pairs<Rule>) -> ValueOperator {
    let pair = pairs.next().unwrap();
    match pair.as_rule() {
        Rule::iota => ValueOperator::Identity,
        Rule::symbol => ValueOperator::Function(to_string_box(&pair)),
        rule => unreachable!("Expected value operator, but found {:?}", rule),
    }
}

#[derive(Debug)]
pub enum ValueExpr {
    NumericalExpr(NumericalExpr),
    StringExpr(StringExpr),
    ValueOperation {
        op: ValueOperator,
        symbol: Box<String>,
    },
}

#[trace]
fn parse_val_operation(pairs: Pairs<Rule>) -> ValueExpr {
    let pairs_vec: Vec<Pair<Rule>> = pairs.collect();
    ValueExpr::ValueOperation {
        op: parse_val_operator(pairs_vec[0].to_owned().into_inner()),
        symbol: to_string_box(&pairs_vec[1]),
    }
}

#[trace]
pub fn parse_val_expr(mut pairs: Pairs<Rule>) -> ValueExpr {
    let pair = pairs.next().unwrap();
    match pair.as_rule() {
        Rule::num_expr => ValueExpr::NumericalExpr(parse_num_expr(pair.into_inner())),
        Rule::str_expr => ValueExpr::StringExpr(parse_str_expr(pair.into_inner())),
        Rule::val_operation => parse_val_operation(pair.into_inner()),
        rule => unreachable!("Expected value expression, but found {:?}", rule),
    }
}
