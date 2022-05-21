extern crate pest;
#[macro_use]
extern crate pest_derive;

use std::io::{BufRead, Read};
use std::io::{self, BufReader};
use std::fs::File;
use crate::pest::{Parser, iterators::Pair};
use std::fs;

#[derive(Parser)]
#[grammar = "mdl.pest"]
struct MDLParser;

fn main() -> io::Result<()>{
    // let file = File::open(&fname)?;
    let file = File::open("test.mdl")?;
    let mut reader = BufReader::new(file);
    let mut instructions = String::new();
    reader.read_to_string(&mut instructions);
    // println!("{}", instructions);
    // for line in reader.lines(){
    let commands = MDLParser::parse(Rule::IDENT_LIST, &instructions);
    // println!("{:?}", commands);
    for pair in commands{
        for command in pair{
            match command.as_rule(){
                // Rule::
            }
        }
    }
    Ok(())
}