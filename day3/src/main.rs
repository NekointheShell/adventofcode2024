use std::env::args;
use readfile::readfile;

mod filter;
use crate::filter::filter;

mod parser;
use crate::parser::parse;

mod processor;


fn main() {
    let arguments: Vec<String> = args().collect();
    let content = readfile(&arguments[1]);

    let mut results: Vec<String> = vec![];
    for line in content.lines() {
        results.push(filter(line, false));
    }

    println!("Part 1 Result: {}", parse(&results.join(", ")));

    results = vec![];
    for line in content.lines() {
        results.push(filter(line, true));
    }

    println!("Part 2 Result: {}", parse(&results.join(", ")));
}
