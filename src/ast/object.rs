use pest::iterators::*;
use super::{ parser::*, utils::to_string_box };

#[derive(Debug)]
pub enum ObjectOperation {
    View {
        functions: Vec<Box<String>>,
    },
}

fn parse_view(pairs: Pairs<Rule>) -> ObjectOperation {
    let pairs_vec: Vec<Pair<Rule>> = pairs.collect();
    ObjectOperation::View {
        functions: pairs_vec.iter().map(to_string_box).collect(),
    }
}

fn parse_obj_operation(mut pairs: Pairs<Rule>) -> ObjectOperation {
    let pair = pairs.next().unwrap();
    match pair.as_rule() {
        Rule::view => parse_view(pair.into_inner()),
        rule => unreachable!("Expected object operation, but found {:?}", rule),
    }
}

#[derive(Debug)]
pub enum ObjectExpression {
    Identity(Box<String>),
    Operation(ObjectOperation),
}

pub fn parse_obj_expr(mut pairs: Pairs<Rule>) -> ObjectExpression {
    let pair = pairs.next().unwrap();
    match pair.as_rule() {
        Rule::symbol => ObjectExpression::Identity(to_string_box(&pair)),
        Rule::obj_operation => ObjectExpression::Operation(parse_obj_operation(pair.into_inner())),
        rule => unreachable!("Expected object primary, but found {:?}", rule),
    }
}
