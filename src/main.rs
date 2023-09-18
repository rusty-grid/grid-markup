use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "gm.pest"]
struct GridParser;

fn main() {
    let unparsed_file =
        std::fs::read_to_string(std::env::args().nth(1).unwrap()).expect("cannot read file");
    dbg!(GridParser::parse(Rule::document, &unparsed_file));
}
