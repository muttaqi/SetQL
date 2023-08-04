pub mod parser;

mod numerical;
mod object;
mod set;
mod string;
mod value;
mod utils;

use pest::iterators::*;
use value::*;
use set::*;
use parser::*;
use utils::*;
use trace::trace;

trace::init_depth_var!();

#[derive(Debug)]
pub enum SetQLClause {
    SetDefinition(SetDefinition),
    FunctionDeclaration {
        symbol: Box<String>,
        domain: Box<SetExpr>,
        range: Box<SetExpr>,
    },
    FunctionDefinition {
        function: Box<String>,
        object: Box<String>,
        value: ValueExpr,
    },
}

#[derive(Debug)]
pub enum SetQLExpr {
    Query(SetExpr),
    Clauses(Vec<SetQLClause>),
}

fn parse_func_declaration(pairs: Pairs<Rule>) -> SetQLClause {
    let pairs_vec: Vec<Pair<Rule>> = pairs.collect();
    SetQLClause::FunctionDeclaration {
        symbol: to_string_box(&pairs_vec[0]),
        domain: Box::new(parse_set_expr(pairs_vec[1].to_owned().into_inner())),
        range: Box::new(parse_set_expr(pairs_vec[3].to_owned().into_inner())),
    }
}

fn parse_func_definition(pairs: Pairs<Rule>) -> SetQLClause {
    let pairs_vec: Vec<Pair<Rule>> = pairs.collect();
    SetQLClause::FunctionDefinition {
        function: to_string_box(&pairs_vec[0]),
        object: to_string_box(&pairs_vec[1]),
        value: parse_val_expr(pairs_vec[2].to_owned().into_inner()),
    }
}

fn parse_setql_clause(mut pairs: Pairs<Rule>) -> SetQLClause {
    let pair = pairs.next().unwrap();
    println!("{:?}", pair);
    match pair.as_rule() {
        Rule::set_definition => SetQLClause::SetDefinition(parse_set_definition(pair.into_inner())),
        Rule::func_declaration => parse_func_declaration(pair.into_inner()),
        Rule::func_definition => parse_func_definition(pair.into_inner()),
        rule =>
            unreachable!(
                "Expected set definition, function declaration or function definition, but found {:?}",
                rule
            ),
    }
}

fn parse_setql_clauses(pairs: Pairs<Rule>) -> Vec<SetQLClause> {
    pairs
        .map(|pair| {
            parse_setql_clause(pair.into_inner())
        })
        .collect()
}

fn parse_primary(primary: Pair<Rule>) -> SetQLExpr {
    match primary.as_rule() {
        Rule::set_expr => SetQLExpr::Query(parse_set_expr(primary.into_inner())),
        Rule::setql_clauses => SetQLExpr::Clauses(parse_setql_clauses(primary.into_inner())),
        rule => unreachable!("Expected set expression or clauses, but found {:?}", rule),
    }
}

#[trace]
pub fn parse_setql_expr(mut pairs: Pairs<Rule>) -> SetQLExpr {
    parse_primary(pairs.next().unwrap().to_owned())
}

#[cfg(test)]
mod tests {
    use pest::Parser;
    use crate::ast::*;

    fn parse(line: &str) {
        match SetQLParser::parse(Rule::setql_expr, line) {
            Ok(pairs) => {
                println!(
                    "Parsed: {:#?}",
                    // inner of expr
                    parse_setql_expr(pairs)
                );
            }
            Err(e) => {
                eprintln!("Parse failed: {:?}", e);
            }
        }
    }

    #[test]
    fn test_new_set() {
        // CREATE TABLE T(f: FLOAT);
        parse("T = {}, f: T -> /R")
    }

    #[test]
    fn test_function_delete() {
        // ALTER TABLE T DROP COLUMN f;
        parse("f: {} -> {}")
    }

    #[test]
    fn test_new_function() {
        // ALTER TABLE T ADD COLUMN f INT;
        parse("f: T -> /Z")
    }
    
    #[test]
    fn test_set_delete() {
        // DELETE FROM T WHERE f = 2;
        parse("T' = T \\ {x /e T | f(x) = 2}")
    }
    
    #[test]
    fn test_set_insert() {
        // INSERT INTO T VALUES (2);
        parse("T' = T /u {x | f(x) = 2}")
    }
    
    #[test]
    fn test_set_update() {
        // UPDATE T SET f = 3 WHERE g = 2;
        parse("S = {x /e T | g(x) = 2}, T' = T \\ S /u /U(S, f, 3)")
    }
    
    #[test]
    fn test_ordered_query() {
        // SELECT * FROM T ORDER BY f;
        parse("/I(T, f)")
    }
    
    #[test]
    fn test_specific_query() {
        // SELECT f FROM T;
        parse("{/v(x, f) | x /e T}")
    }
}
