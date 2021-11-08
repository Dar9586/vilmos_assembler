use std::str::FromStr;

use strum_macros::EnumIter;
use strum_macros::EnumString;

use crate::color::Color;
use crate::instructions::Instruction::RawString;
use crate::params::Params;
use crate::parser;

const SMALL_NUMBER: i32 = 350;

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

fn int_to_colors(val: i32, conf: &Params) -> Vec<Color> {
    let mut val = val;
    let mut colors: Vec<Color> = Vec::new();
    if val > SMALL_NUMBER || val == 0 {
        colors.push(Color::from('\0'));
    }
    let cc = conf.custom_colors.values();
    while val != 0 {
        let c = loop {
            let mut contains = false;
            let k = Color::random(val);
            for v in cc.clone() {
                if *v == k {
                    contains = true;
                    break;
                }
            }
            if contains || (val <= SMALL_NUMBER && val != k.sum()) {
                continue;
            }
            break k;
        };
        assert_eq!(c.b, 0);
        colors.push(c);
        colors.push(conf.get_color(Instruction::Sum)[0]);
        val -= c.sum();
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
