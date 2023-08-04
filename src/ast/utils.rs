use pest::iterators::*;
use super::parser::*;

pub fn to_string_box(pair: &Pair<Rule>) -> Box<String> {
    Box::new(pair.as_str().to_owned())
}
