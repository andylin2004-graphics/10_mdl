use crate::color::Color;
use crate::image::Image;
use crate::matrix::CurveType;
use crate::matrix::Matrix;
use crate::pest::{iterators::Pair, Parser};
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Read};

#[derive(Parser)]
#[grammar = "mdl.pest"]
struct MDLParser;

#[derive(Debug)]
struct Constants {
    pub r: [f32; 3],
    pub g: [f32; 3],
    pub b: [f32; 3],
    pub red: f32,
    pub green: f32,
    pub blue: f32,
}

impl Constants {
    fn new() -> Constants {
        Constants {
            r: [0.0, 0.0, 0.0],
            g: [0.0, 0.0, 0.0],
            b: [0.0, 0.0, 0.0],
            red: 0.0,
            green: 0.0,
            blue: 0.0,
        }
    }
}

pub fn parse(fname: &str) -> io::Result<()> {
    let file = File::open(&fname)?;
    let mut reader = BufReader::new(file);
    let mut instructions = String::new();
    reader.read_to_string(&mut instructions);
    let commands = MDLParser::parse(Rule::IDENT_LIST, &instructions);
    let mut screen = Image::new(500, 500);
    let color = Color::new_color(0, 255, 0);
    let mut edges = Matrix::new(0, 0);
    let mut polygons = Matrix::new(0, 0);
    let mut cstack = vec![Matrix::new(0, 0); 0];
    let mut constants_store = HashMap::new();
    for pair in commands {
        for command in pair {
            println!("{:?}", command.as_rule());
            match command.as_rule() {
                Rule::CONSTANTS_SDDDDDDDDD => {
                    let mut command_contents = command.into_inner();
                    let name = command_contents.next().unwrap().as_str();
                    println!("{:?}", command_contents);
                    let constant = Constants {
                        r: [
                            command_contents.next().unwrap().as_str().parse().unwrap(),
                            command_contents.next().unwrap().as_str().parse().unwrap(),
                            command_contents.next().unwrap().as_str().parse().unwrap(),
                        ],
                        g: [
                            command_contents.next().unwrap().as_str().parse().unwrap(),
                            command_contents.next().unwrap().as_str().parse().unwrap(),
                            command_contents.next().unwrap().as_str().parse().unwrap(),
                        ],
                        b: [
                            command_contents.next().unwrap().as_str().parse().unwrap(),
                            command_contents.next().unwrap().as_str().parse().unwrap(),
                            command_contents.next().unwrap().as_str().parse().unwrap(),
                        ],
                        red: 0.0,
                        green: 0.0,
                        blue: 0.0,
                    };
                    constants_store.insert(name, constant);
                    // println!("{:?}", constant);
                }
                _ => {}
            }
        }
    }
    Ok(())
}
