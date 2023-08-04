use pest::pratt_parser::PrattParser;

#[derive(pest_derive::Parser)]
#[grammar = "setql.pest"]
pub struct SetQLParser;

lazy_static::lazy_static! {
    pub static ref PRATT_PARSER: PrattParser<Rule> = {
        use pest::pratt_parser::{ Assoc::*, Op };
        use Rule::*;

        // Precedence is defined lowest to highest
        PrattParser::new()
            // Addition and subtract have equal precedence
            .op(Op::infix(add, Left) | Op::infix(subtract, Left))
            .op(Op::infix(multiply, Left) | Op::infix(divide, Left) | Op::infix(modulo, Left))
            .op(Op::prefix(unary_minus))
            .op(Op::infix(concat, Left))
            .op(Op::infix(union, Left) | Op::infix(set_minus, Left))
    };
}
