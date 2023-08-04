mod ast;

use std::io::{ self, BufRead };

use ast::parser::*;
use ast::parse_setql_expr;
use trace::trace;

trace::init_depth_var!();

use pest::Parser;

#[trace]
fn main() -> io::Result<()> {
    for line in io::stdin().lock().lines() {
        match SetQLParser::parse(Rule::setql_expr, &line?) {
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
    Ok(())
}
