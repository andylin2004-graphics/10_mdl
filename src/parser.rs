use crate::color::Color;
use crate::image::Image;
use crate::matrix::CurveType;
use crate::matrix::Matrix;
use crate::pest::{iterators::Pair, Parser};
use crate::ReflectionValue;
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Read};

#[derive(Parser)]
#[grammar = "mdl.pest"]
struct MDLParser;

#[derive(Debug)]
struct Constants {
    pub ambient_reflect: ReflectionValue,
    pub diffuse_reflect: ReflectionValue,
    pub specular_reflect: ReflectionValue,
    pub red: f32,
    pub green: f32,
    pub blue: f32,
}

impl Constants {
    fn new(
        ambient_red: f32,
        diffuse_red: f32,
        specular_red: f32,
        ambient_green: f32,
        diffuse_green: f32,
        specular_green: f32,
        ambient_blue: f32,
        diffuse_blue: f32,
        specular_blue: f32,
        red: f32,
        green: f32,
        blue: f32,
    ) -> Constants {
        Constants {
            ambient_reflect: ReflectionValue::new_values(ambient_red, ambient_green, ambient_blue),
            diffuse_reflect: ReflectionValue::new_values(diffuse_red, diffuse_green, diffuse_blue),
            specular_reflect: ReflectionValue::new_values(
                specular_red,
                specular_green,
                specular_blue,
            ),
            red,
            green,
            blue
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
    cstack.push(Matrix::identity());
    for pair in commands {
        for command in pair {
            // println!("{:?}", command.as_rule());
            match command.as_rule() {
                Rule::CONSTANTS_SDDDDDDDDD => {
                    let mut command_contents = command.into_inner();
                    let name = command_contents.next().unwrap().as_str();
                    let constant = Constants::new(command_contents.next().unwrap().as_str().parse().unwrap(), command_contents.next().unwrap().as_str().parse().unwrap(), command_contents.next().unwrap().as_str().parse().unwrap(), command_contents.next().unwrap().as_str().parse().unwrap(), command_contents.next().unwrap().as_str().parse().unwrap(), command_contents.next().unwrap().as_str().parse().unwrap(), command_contents.next().unwrap().as_str().parse().unwrap(), command_contents.next().unwrap().as_str().parse().unwrap(), command_contents.next().unwrap().as_str().parse().unwrap(), 0.0, 0.0, 0.0);
                    constants_store.insert(name, constant);
                }
                Rule::CONSTANTS_SDDDDDDDDDDDD => {
                    let mut command_contents = command.into_inner();
                    let name = command_contents.next().unwrap().as_str();
                    let constant = Constants::new(command_contents.next().unwrap().as_str().parse().unwrap(), command_contents.next().unwrap().as_str().parse().unwrap(), command_contents.next().unwrap().as_str().parse().unwrap(), command_contents.next().unwrap().as_str().parse().unwrap(), command_contents.next().unwrap().as_str().parse().unwrap(), command_contents.next().unwrap().as_str().parse().unwrap(), command_contents.next().unwrap().as_str().parse().unwrap(), command_contents.next().unwrap().as_str().parse().unwrap(), command_contents.next().unwrap().as_str().parse().unwrap(), command_contents.next().unwrap().as_str().parse().unwrap(), command_contents.next().unwrap().as_str().parse().unwrap(), command_contents.next().unwrap().as_str().parse().unwrap());
                    constants_store.insert(name, constant);
                }
                Rule::PPUSH => {
                    cstack.push(cstack.last().unwrap().clone());
                }
                Rule::PPOP => {
                    cstack.pop();
                }
                Rule::MOVE_DDD => {
                    let mut command_contents = command.into_inner();
                    let mut rot = Matrix::make_translate(
                        command_contents.next().unwrap().as_str().parse().unwrap(),
                        command_contents.next().unwrap().as_str().parse().unwrap(),
                        command_contents.next().unwrap().as_str().parse().unwrap(),
                    );
                    rot.multiply_matrixes(&cstack.pop().unwrap());
                    cstack.push(rot);
                }
                Rule::ROTATE_SD => {
                    let mut command_contents = command.into_inner();
                    let rot_axis = command_contents.next().unwrap().as_str();
                    match rot_axis {
                        "x" => {
                            let mut rot = Matrix::make_rot_x(
                                command_contents.next().unwrap().as_str().parse().unwrap(),
                            );
                            rot.multiply_matrixes(&cstack.pop().unwrap());
                            cstack.push(rot);
                        }
                        "y" => {
                            let mut rot = Matrix::make_rot_y(
                                command_contents.next().unwrap().as_str().parse().unwrap(),
                            );
                            rot.multiply_matrixes(&cstack.pop().unwrap());
                            cstack.push(rot);
                        }
                        "z" => {
                            let mut rot = Matrix::make_rot_z(
                                command_contents.next().unwrap().as_str().parse().unwrap(),
                            );
                            rot.multiply_matrixes(&cstack.pop().unwrap());
                            cstack.push(rot);
                        }
                        _ => {
                            panic!(
                                "Invalid input {} at 0 for rotation: please use x, y, or z.",
                                rot_axis
                            );
                        }
                    }
                }
                Rule::SCALE_DDD => {
                    let mut command_contents = command.into_inner();
                    let mut scale = Matrix::make_scale(
                        command_contents.next().unwrap().as_str().parse().unwrap(),
                        command_contents.next().unwrap().as_str().parse().unwrap(),
                        command_contents.next().unwrap().as_str().parse().unwrap(),
                    );
                    scale.multiply_matrixes(&cstack.pop().unwrap());
                    cstack.push(scale);
                }
                _ => {
                    println!("{:?}", command.as_rule());
                }
            }
        }
    }
    println!("{:?}", constants_store);
    Ok(())
}
