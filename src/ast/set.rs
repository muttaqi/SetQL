use super::object::ObjectExpression;
use super::object::parse_obj_expr;
use super::value::*;
use super::parser::*;
use super::utils::*;
use pest::iterators::*;

#[derive(Debug)]
pub enum PredefinedSet {
    Integers,
    Reals,
    Strings,
    Empty,
}

fn parse_predefined_set(mut pairs: Pairs<Rule>) -> PredefinedSet {
    let pair = pairs.next().unwrap();
    match pair.as_rule() {
        Rule::integers => PredefinedSet::Integers,
        Rule::reals => PredefinedSet::Reals,
        Rule::strings => PredefinedSet::Strings,
        Rule::empty => PredefinedSet::Empty,
        rule => unreachable!("Expected a predefined set, but found {:?}", rule),
    }
}

#[derive(Debug)]
pub enum SetOperation {
    Index {
        set: Box<SetExpr>,
        symbol: Box<String>,
    },
    Group {
        set: Box<SetExpr>,
        symbols: Vec<Box<String>>,
    },
    Join {
        left_set: Box<SetExpr>,
        right_set: Box<SetExpr>,
        symbols: Vec<Box<String>>,
    },
    Offset {
        set: Box<SetExpr>,
        value: i32,
    },
    Distinct {
        set: Box<SetExpr>,
    },
    Update {
        set: Box<SetExpr>,
        function: Box<String>,
        value: ValueExpr,
    },
}

fn parse_index(pairs: Pairs<Rule>) -> SetOperation {
    let pairs_vec: Vec<Pair<Rule>> = pairs.collect();
    SetOperation::Index {
        set: Box::new(parse_set_expr(pairs_vec[1].to_owned().into_inner())),
        symbol: to_string_box(&pairs_vec[2]),
    }
}

fn parse_group(pairs: Pairs<Rule>) -> SetOperation {
    let pairs_vec: Vec<Pair<Rule>> = pairs.collect();
    SetOperation::Group {
        set: Box::new(parse_set_expr(pairs_vec[1].to_owned().into_inner())),
        symbols: pairs_vec[2..].iter().map(to_string_box).collect(),
    }
}

fn parse_join(pairs: Pairs<Rule>) -> SetOperation {
    let pairs_vec: Vec<Pair<Rule>> = pairs.collect();
    SetOperation::Join {
        left_set: Box::new(parse_set_expr(pairs_vec[1].to_owned().into_inner())),
        right_set: Box::new(parse_set_expr(pairs_vec[2].to_owned().into_inner())),
        symbols: pairs_vec[3..].iter().map(to_string_box).collect(),
    }
}

fn parse_offset(pairs: Pairs<Rule>) -> SetOperation {
    let pairs_vec: Vec<Pair<Rule>> = pairs.collect();
    SetOperation::Offset {
        set: Box::new(parse_set_expr(pairs_vec[1].to_owned().into_inner())),
        value: pairs_vec[2].as_str().parse::<i32>().unwrap(),
    }
}

fn parse_distinct(pairs: Pairs<Rule>) -> SetOperation {
    let pairs_vec: Vec<Pair<Rule>> = pairs.collect();
    SetOperation::Distinct {
        set: Box::new(parse_set_expr(pairs_vec[1].to_owned().into_inner())),
    }
}

fn parse_update(pairs: Pairs<Rule>) -> SetOperation {
    let pairs_vec: Vec<Pair<Rule>> = pairs.collect();
    SetOperation::Update {
        set: Box::new(parse_set_expr(pairs_vec[1].to_owned().into_inner())),
        function: to_string_box(&pairs_vec[2]),
        value: parse_val_expr(pairs_vec[3].to_owned().into_inner()),
    }
}

fn parse_set_operation(mut pairs: Pairs<Rule>) -> SetExpr {
    let pair = pairs.next().unwrap();
    match pair.as_rule() {
        Rule::index => SetExpr::Operation(parse_index(pair.into_inner())),
        Rule::group => SetExpr::Operation(parse_group(pair.into_inner())),
        Rule::join => SetExpr::Operation(parse_join(pair.into_inner())),
        Rule::offset => SetExpr::Operation(parse_offset(pair.into_inner())),
        Rule::distinct => SetExpr::Operation(parse_distinct(pair.into_inner())),
        Rule::update => SetExpr::Operation(parse_update(pair.into_inner())),
        rule => unreachable!("Expected set operation, but found {:?}", rule),
    }
}

#[derive(Debug)]
struct InSet {
    symbol: Box<String>,
    set: Box<SetExpr>,
}

fn parse_in_set(pairs: Pairs<Rule>) -> InSet {
    let pairs_vec: Vec<Pair<Rule>> = pairs.collect();
    InSet {
        symbol: to_string_box(&pairs_vec[0]),
        set: Box::new(parse_set_expr(pairs_vec[2].to_owned().into_inner())),
    }
}

#[derive(Debug)]
pub enum ObjectDefinition {
    InSet(InSet),
    Expression(ObjectExpression),
}

fn parse_object_definition(mut pairs: Pairs<Rule>) -> ObjectDefinition {
    let pair = pairs.next().unwrap();
    match pair.as_rule() {
        Rule::in_set => ObjectDefinition::InSet(parse_in_set(pair.into_inner())),
        Rule::obj_expr => ObjectDefinition::Expression(parse_obj_expr(pair.into_inner())),
        rule => unreachable!("Expected object definition, but found {:?}", rule),
    }
}

#[derive(Debug)]
enum ObjectClause {
    InSet(InSet),
    ValueClause(ValueClause),
}

fn parse_obj_clause(mut pairs: Pairs<Rule>) -> ObjectClause {
    let pair = pairs.next().unwrap();
    match pair.as_rule() {
        Rule::in_set => ObjectClause::InSet(parse_in_set(pair.into_inner())),
        Rule::val_clause => ObjectClause::ValueClause(parse_value_clause(pair.into_inner())),
        rule => unreachable!("Expected object clause, but found {:?}", rule),
    }
}

#[derive(Debug)]
pub enum SetBinaryOperator {
    Union,
    Minus,
}

#[derive(Debug)]
pub enum SetExpr {
    Symbol(Box<String>),
    Predefined(PredefinedSet),
    SetBuilder {
        object: ObjectDefinition,
        clauses: Vec<ObjectClause>,
    },
    Operation(SetOperation),
    Op {
        lhs: Box<SetExpr>,
        op: SetBinaryOperator,
        rhs: Box<SetExpr>,
    },
}

fn parse_set_builder(pairs: Pairs<Rule>) -> SetExpr {
    let pairs_vec: Vec<Pair<Rule>> = pairs.collect();
    SetExpr::SetBuilder {
        object: parse_object_definition(pairs_vec[0].to_owned().into_inner()),
        clauses: pairs_vec[1..]
            .iter()
            .map(|pair| { parse_obj_clause(pair.to_owned().into_inner()) })
            .collect(),
    }
}

fn parse_primary(primary: Pair<Rule>) -> SetExpr {
    match primary.as_rule() {
        Rule::symbol => SetExpr::Symbol(to_string_box(&primary)),
        Rule::predefined_set => SetExpr::Predefined(parse_predefined_set(primary.into_inner())),
        Rule::set_builder => parse_set_builder(primary.into_inner()),
        Rule::set_operation => parse_set_operation(primary.into_inner()),
        rule => unreachable!("Expected set primary, but found {:?}", rule),
    }
}

fn parse_infix(lhs: SetExpr, op: Pair<Rule>, rhs: SetExpr) -> SetExpr {
    let op = match op.as_rule() {
        Rule::union => SetBinaryOperator::Union,
        Rule::set_minus => SetBinaryOperator::Minus,
        rule => unreachable!("Expected union or set minus, but found {:?}", rule),
    };
    SetExpr::Op {
        lhs: Box::new(lhs),
        op,
        rhs: Box::new(rhs),
    }
}

pub fn parse_set_expr(pairs: Pairs<Rule>) -> SetExpr {
    PRATT_PARSER.map_primary(parse_primary).map_infix(parse_infix).parse(pairs)
}

#[derive(Debug)]
pub struct SetDefinition {
    symbol: Box<String>,
    expr: SetExpr,
}

pub fn parse_set_definition(pairs: Pairs<Rule>) -> SetDefinition {
    let pairs_vec: Vec<Pair<Rule>> = pairs.collect();
    SetDefinition {
        symbol: to_string_box(&pairs_vec[0]),
        expr: parse_set_expr(pairs_vec[1].to_owned().into_inner()),
    }
}
