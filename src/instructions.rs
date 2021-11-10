use std::collections::hash_map::Values;
use std::str::FromStr;

use strum_macros::EnumIter;
use strum_macros::EnumString;

use crate::color::Color;
use crate::instructions::Instruction::RawString;
use crate::params::Params;
use crate::parser;

const SMALL_NUMBER: i32 = 0xfe + 0xff + 0xff;

#[derive(Clone, Debug, EnumIter, EnumString, Hash, Eq, PartialEq)]
pub enum Instruction {
    InputInt,
    OutputInt,
    Sum,
    Sub,
    Div,
    Mul,
    Mod,
    Rnd,
    And,
    Or,
    Xor,
    Nand,
    Not,
    InputAscii,
    OutputAscii,
    Pop,
    Swap,
    Cycle,
    Rcycle,
    Dup,
    Reverse,
    Quit,
    Output,
    While,
    WhileEnd,
    FileOpen,
    FileClose,
    Lshift,
    RShift,
    RawString(String),
    RawInt(i32),
}

fn char_to_colors(ch: char, conf: &Params) -> Vec<Color> {
    int_to_colors(ch as i32, conf)
}

fn string_to_colors(str: &str, conf: &Params) -> Vec<Color> {
    let mut colors: Vec<Color> = Vec::new();
    colors.extend(char_to_colors('\0', conf));
    for c in str.chars() {
        colors.extend(char_to_colors(c, conf));
    }
    colors
}

fn color_contains(k: Color, values: Values<Instruction, Color>) -> bool {
    for v in values {
        if k.eq(v) {
            return true;
        }
    }
    return false;
}

pub fn generate_exact_color(val: i32, conf: &Params) -> Color {
    if !conf.is_random {
        return Color::not_random(val);
    }
    let cc = conf.custom_colors.values();
    loop {
        let k = Color::random(val);
        let contains = color_contains(k, cc.clone());
        if !contains {
            return k;
        }
    };
}


fn int_to_colors(val_original: i32, conf: &Params) -> Vec<Color> {
    let mut colors: Vec<Color> = Vec::new();
    if val_original == i32::MIN {
        colors.extend(int_to_colors(-i32::MAX, conf));
        colors.push(generate_exact_color(1, conf));
        colors.push(conf.get_color(Instruction::Sub)[0]);
        return colors;
    }
    if val_original < 0 {
        colors.push(generate_exact_color(0, conf));
        colors.extend(int_to_colors(-val_original, conf));
        colors.push(conf.get_color(Instruction::Sub)[0]);
        return colors;
    }
    if val_original <= SMALL_NUMBER {
        return vec![generate_exact_color(val_original, conf)];
    }
    let mut val = val_original;

    while val > SMALL_NUMBER {
        let sqrt = num_integer::sqrt(val);
        colors.extend(int_to_colors(sqrt, conf));
        colors.push(conf.get_color(Instruction::Dup)[0]);
        colors.push(conf.get_color(Instruction::Mul)[0]);
        if val != val_original {
            colors.push(conf.get_color(Instruction::Sum)[0]);
        }
        val -= sqrt * sqrt;
    }
    if val != 0 {
        colors.push(generate_exact_color(val, conf));
        colors.push(conf.get_color(Instruction::Sum)[0]);
    }
    colors
}

pub fn get_instruction_name(val: &str) -> String {
    let mut instr_name = String::new();
    for i in val.splitn(2, "_") {
        let mut chars = i.chars();
        instr_name.push(chars.next().unwrap().to_ascii_uppercase());
        for c in chars {
            instr_name.push(c.to_ascii_lowercase());
        }
    }
    instr_name
}

impl Instruction {
    pub fn from_command(command: &str) -> Result<Option<Instruction>, &'static str> {
        let tokens = parser::parse(command);
        if tokens.len() == 0 {
            return Ok(None);
        }
        if tokens.len() > 2 {
            return Err("Too many operands");
        }
        let instruction = Instruction::from_str(&get_instruction_name(&tokens[0]));
        if instruction.is_err() {
            return Err("Invalid instruction name");
        }
        let instruction: Instruction = instruction.unwrap();
        if tokens.len() == 1 {
            return Ok(Some(instruction));
        }
        return match instruction {
            Instruction::RawInt(_) => {
                match tokens[1].parse::<i32>() {
                    Ok(val) => {
                        Ok(Some(Instruction::RawInt(val)))
                    }
                    Err(_) => {
                        Err("The argument is not a valid integer")
                    }
                }
            }
            Instruction::RawString(_) => {
                Ok(Some(RawString(tokens[1].clone())))
            }
            _ => {
                Err("Too many operands")
            }
        };
    }
}

impl Instruction {
    pub fn get_default_colors(&self, conf: &Params) -> Vec<Color> {
        match self {
            Instruction::Lshift => vec![Color::from(0x2d6a7d)],
            Instruction::RShift => vec![Color::from(0x439dba)],
            Instruction::InputInt => vec![Color::from(0xffffff)],
            Instruction::OutputInt => vec![Color::from(0x000001)],
            Instruction::Sum => vec![Color::from(0x00ced1)],
            Instruction::Sub => vec![Color::from(0xffa500)],
            Instruction::Div => vec![Color::from(0x8a2be2)],
            Instruction::Mul => vec![Color::from(0x8b0000)],
            Instruction::Mod => vec![Color::from(0xffdab9)],
            Instruction::Rnd => vec![Color::from(0x008000)],
            Instruction::And => vec![Color::from(0xecf3dc)],
            Instruction::Or => vec![Color::from(0xb7c6e6)],
            Instruction::Xor => vec![Color::from(0xf5e3d7)],
            Instruction::Nand => vec![Color::from(0xe1d3ef)],
            Instruction::Not => vec![Color::from(0xff9aa2)],
            Instruction::InputAscii => vec![Color::from(0xe3e3e3)],
            Instruction::OutputAscii => vec![Color::from(0x4b4b4b)],
            Instruction::Pop => vec![Color::from(0xcc9e06)],
            Instruction::Swap => vec![Color::from(0xffbd4a)],
            Instruction::Cycle => vec![Color::from(0xe37f9d)],
            Instruction::Rcycle => vec![Color::from(0xe994ae)],
            Instruction::Dup => vec![Color::from(0x006994)],
            Instruction::Reverse => vec![Color::from(0xa5a58d)],
            Instruction::Quit => vec![Color::from(0xb7e4c7)],
            Instruction::Output => vec![Color::from(0x9B2242)],
            Instruction::While => vec![Color::from(0x2e1a47)],
            Instruction::WhileEnd => vec![Color::from(0x68478d)],
            Instruction::FileOpen => vec![Color::from(0x91f68b)],
            Instruction::FileClose => vec![Color::from(0x2fed23)],
            Instruction::RawString(str) => string_to_colors(&str, conf),
            Instruction::RawInt(val) => int_to_colors(*val, conf),
        }
    }
}
