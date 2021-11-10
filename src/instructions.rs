use std::collections::hash_map::Values;

use strum::EnumProperty;
use strum::IntoEnumIterator;
use strum::VariantNames;
use strum_macros::EnumIter;
use strum_macros::EnumProperty;
use strum_macros::EnumString;
use strum_macros::EnumVariantNames;

use crate::color::Color;
use crate::instructions::Instruction::RawString;
use crate::params::Params;
use crate::parser;

const EASY_NUMBER: i32 = 400;
const RETRY_RANDOM: u32 = 100;
const BIT_PER_COLOR: u32 = 9;
const BIT_MASK: u32 = (1 << BIT_PER_COLOR) - 1;

#[derive(Clone, Debug, EnumIter, EnumString, Hash, Eq, PartialEq, EnumVariantNames, EnumProperty)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum Instruction {
    #[strum(props(Color = "2d6a7d"))]
    Lshift,
    #[strum(props(Color = "439dba"))]
    RShift,
    #[strum(props(Color = "ffffff"))]
    InputInt,
    #[strum(props(Color = "000001"))]
    OutputInt,
    #[strum(props(Color = "00ced1"))]
    Sum,
    #[strum(props(Color = "ffa500"))]
    Sub,
    #[strum(props(Color = "8a2be2"))]
    Div,
    #[strum(props(Color = "8b0000"))]
    Mul,
    #[strum(props(Color = "ffdab9"))]
    Mod,
    #[strum(props(Color = "008000"))]
    Rnd,
    #[strum(props(Color = "ecf3dc"))]
    And,
    #[strum(props(Color = "b7c6e6"))]
    Or,
    #[strum(props(Color = "f5e3d7"))]
    Xor,
    #[strum(props(Color = "e1d3ef"))]
    Nand,
    #[strum(props(Color = "ff9aa2"))]
    Not,
    #[strum(props(Color = "e3e3e3"))]
    InputAscii,
    #[strum(props(Color = "4b4b4b"))]
    OutputAscii,
    #[strum(props(Color = "cc9e06"))]
    Pop,
    #[strum(props(Color = "ffbd4a"))]
    Swap,
    #[strum(props(Color = "e37f9d"))]
    Cycle,
    #[strum(props(Color = "e994ae"))]
    Rcycle,
    #[strum(props(Color = "006994"))]
    Dup,
    #[strum(props(Color = "a5a58d"))]
    Reverse,
    #[strum(props(Color = "b7e4c7"))]
    Quit,
    #[strum(props(Color = "9B2242"))]
    Output,
    #[strum(props(Color = "2e1a47"))]
    While,
    #[strum(props(Color = "68478d"))]
    WhileEnd,
    #[strum(props(Color = "91f68b"))]
    FileOpen,
    #[strum(props(Color = "2fed23"))]
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
    let mut last_char = '\0';
    for c in str.chars() {
        if c == last_char {
            colors.extend(conf.get_color(Instruction::Dup));
        } else {
            colors.extend(char_to_colors(c, conf));
            last_char = c;
        }
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



pub fn generate_exact_color(val: i32, conf: &Params) -> Vec<Color> {
    let cc = conf.custom_colors.values();
    if !conf.is_random {
        let k=Color::not_random(val);
        let contains = color_contains(k, cc.clone());
        if !contains {
            return vec![k];
        }
    }
    for _ in 0..RETRY_RANDOM {
        let k = Color::random(val);
        let contains = color_contains(k, cc.clone());
        if !contains {
            return vec![k];
        }
    };
    let mut colors:Vec<Color>=Vec::new();
    colors.extend(generate_exact_color(EASY_NUMBER+val,conf));
    colors.extend(generate_exact_color(EASY_NUMBER,conf));
    colors.extend(conf.get_color(Instruction::Sub));

    colors
}


fn int_to_colors(val_original: i32, conf: &Params) -> Vec<Color> {
    let mut colors: Vec<Color> = Vec::new();
    let mut val:u32 = val_original as u32;
    let mut first = true;
    let mut i:u32=0;
    while val!=0 {
        if val&1==1 {
            let bits = val & BIT_MASK;
            if bits != 0 {
                colors.extend(generate_exact_color(bits as i32, conf));
                if i != 0 {
                    colors.extend(generate_exact_color(i as i32, conf));
                    colors.extend(conf.get_color(Instruction::Lshift));
                }
                if first {
                    first = false;
                } else {
                    colors.extend(conf.get_color(Instruction::Sum));
                }
            }
            val >>= BIT_PER_COLOR;
            i+=BIT_PER_COLOR;
        }else{
            val>>=1;
            i+=1;
        }
    }
    if val_original == 0{
        colors.extend(generate_exact_color(0, conf));
    }
    colors
}

impl Instruction {
    pub fn find_name(name: &str) -> Option<Instruction> {
        let index = Instruction::VARIANTS.iter().position(|&r| r == name);
        if index.is_none() {
            return None;
        }
        let index = index.unwrap();
        let instruction: Instruction = Instruction::iter().nth(index).unwrap();
        Some(instruction)
    }
    pub fn from_command(command: &str) -> Result<Option<Instruction>, &'static str> {
        let tokens = parser::parse(command);
        if tokens.len() == 0 {
            return Ok(None);
        }
        if tokens.len() > 2 {
            return Err("Too many operands");
        }
        let name = tokens[0].as_str();
        let instruction = Instruction::find_name(name);
        if instruction.is_none() {
            return Err("Instruction not found");
        }
        let instruction = instruction.unwrap();
        if tokens.len() == 1 {
            return match instruction {
                Instruction::RawString(_) => Err("RAW_STRING needs an argument"),
                Instruction::RawInt(_) => Err("RAW_INT needs an argument"),
                _ => Ok(Some(instruction))
            }
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
            Instruction::RawString(str) => string_to_colors(&str, conf),
            Instruction::RawInt(val) => int_to_colors(*val, conf),
            _ => {
                let val = u32::from_str_radix(self.get_str("Color").unwrap(), 16);
                vec![Color::from(val.unwrap())]
            }
        }
    }
}
