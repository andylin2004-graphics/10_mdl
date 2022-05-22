use crate::color::Color;
use crate::image::Image;
use crate::matrix::CurveType;
use crate::matrix::Matrix;
use crate::pest::{iterators::Pair, Parser};
use std::fs;
use std::fs::File;
use std::io::{self, BufReader, BufRead, Read};

#[derive(Parser)]
#[grammar = "mdl.pest"]
struct MDLParser;

struct Constants{
    pub r: [f32; 3],
    pub g: [f32; 3],
    pub b: [f32; 3],
    pub red: f32,
    pub green: f32,
    pub blue: f32
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
    for pair in commands {
        for command in pair {
            match command.as_rule(){
                // Rule::
            }
        }
    }
    Ok(())
}
