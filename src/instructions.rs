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

const INSTANT_NUMBER: u32 = 750;
const EASY_NUMBER: i32 = 380;
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
    #[strum(props(Params = "1"))]
    RawString(String),
    #[strum(props(Params = "1"))]
    RawInt(i32),
    #[strum(props(Params = "3"))]
    RawColor(u8, u8, u8),
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
    let mut val: u32 = val_original as u32;
    let mut first = true;
    let mut i: u32 = 0;
    if val <= INSTANT_NUMBER {
        return generate_exact_color(val as i32, conf);
    }
    while val != 0 {
        if val & 1 == 1 {
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
        if tokens.is_none() {
            return Err("Invalid instruction");
        }
        let tokens = tokens.unwrap();
        if tokens.len() == 0 {
            return Ok(None);
        }
        let name = tokens[0].as_str();
        let instruction = Instruction::find_name(name);
        if instruction.is_none() {
            return Err("Instruction not found");
        }
        let instruction = instruction.unwrap();
        if tokens.len() - 1 != instruction.get_param_count() as usize {
            return Err("Wrong number of arguments");
        }
        return match instruction {
            Instruction::RawString(_) => {
                Ok(Some(RawString(tokens[1].clone())))
            },
            Instruction::RawInt(_) => {
                match tokens[1].parse::<i32>() {
                    Ok(val) => {
                        Ok(Some(Instruction::RawInt(val)))
                    }
                    Err(_) => {
                        Err("The argument is not a valid integer")
                    }
                }
            },
            Instruction::RawColor(_, _, _) => {
                let r = tokens[1].parse::<u8>();
                let g = tokens[2].parse::<u8>();
                let b = tokens[3].parse::<u8>();
                if r.is_err() || g.is_err() || b.is_err() {
                    return Err("The argument is not a valid integer");
                }
                Ok(Some(Instruction::RawColor(r.unwrap(), g.unwrap(), b.unwrap())))
            },
            _ => Ok(Some(instruction))
        }
    }
}

impl Instruction {
    pub fn get_default_colors(&self, conf: &Params) -> Vec<Color> {
        match self {
            Instruction::RawString(str) => string_to_colors(&str, conf),
            Instruction::RawInt(val) => int_to_colors(*val, conf),
            Instruction::RawColor(r, g, b) => vec![Color::new(*r, *g, *b)],
            _ => {
                let val = u32::from_str_radix(self.get_str("Color").unwrap(), 16);
                vec![Color::from(val.unwrap())]
            }
        }
    }
    pub fn get_param_count(&self) -> u8 {
        let x: Option<&str> = self.get_str("Params");
        match x {
            None => 0,
            Some(val) => val.parse::<u8>().unwrap()
        }
    }
}

#[cfg(test)]
mod instruction_tests {
    use super::*;

    #[test]
    fn wrong_instruction() {
        assert!(Instruction::from_command("ciao").is_err())
    }

    #[test]
    fn correct_normal_instruction() {
        //Correct normal instructions
        assert_eq!(Instruction::And, Instruction::from_command("AND").unwrap().unwrap());
        assert_eq!(Instruction::Cycle, Instruction::from_command("CYCLE").unwrap().unwrap());
        assert_eq!(Instruction::Div, Instruction::from_command("DIV").unwrap().unwrap());
        assert_eq!(Instruction::Dup, Instruction::from_command("DUP").unwrap().unwrap());
        assert_eq!(Instruction::FileClose, Instruction::from_command("FILE_CLOSE").unwrap().unwrap());
        assert_eq!(Instruction::FileOpen, Instruction::from_command("FILE_OPEN").unwrap().unwrap());
        assert_eq!(Instruction::InputAscii, Instruction::from_command("INPUT_ASCII").unwrap().unwrap());
        assert_eq!(Instruction::InputInt, Instruction::from_command("INPUT_INT").unwrap().unwrap());
        assert_eq!(Instruction::Mod, Instruction::from_command("MOD").unwrap().unwrap());
        assert_eq!(Instruction::Mul, Instruction::from_command("MUL").unwrap().unwrap());
        assert_eq!(Instruction::Nand, Instruction::from_command("NAND").unwrap().unwrap());
        assert_eq!(Instruction::Not, Instruction::from_command("NOT").unwrap().unwrap());
        assert_eq!(Instruction::Or, Instruction::from_command("OR").unwrap().unwrap());
        assert_eq!(Instruction::Output, Instruction::from_command("OUTPUT").unwrap().unwrap());
        assert_eq!(Instruction::OutputAscii, Instruction::from_command("OUTPUT_ASCII").unwrap().unwrap());
        assert_eq!(Instruction::OutputInt, Instruction::from_command("OUTPUT_INT").unwrap().unwrap());
        assert_eq!(Instruction::Pop, Instruction::from_command("POP").unwrap().unwrap());
        assert_eq!(Instruction::Quit, Instruction::from_command("QUIT").unwrap().unwrap());
        assert_eq!(Instruction::Rcycle, Instruction::from_command("RCYCLE").unwrap().unwrap());
        assert_eq!(Instruction::Reverse, Instruction::from_command("REVERSE").unwrap().unwrap());
        assert_eq!(Instruction::Rnd, Instruction::from_command("RND").unwrap().unwrap());
        assert_eq!(Instruction::Sub, Instruction::from_command("SUB").unwrap().unwrap());
        assert_eq!(Instruction::Sum, Instruction::from_command("SUM").unwrap().unwrap());
        assert_eq!(Instruction::Swap, Instruction::from_command("SWAP").unwrap().unwrap());
        assert_eq!(Instruction::While, Instruction::from_command("WHILE").unwrap().unwrap());
        assert_eq!(Instruction::WhileEnd, Instruction::from_command("WHILE_END").unwrap().unwrap());
        assert_eq!(Instruction::Xor, Instruction::from_command("XOR").unwrap().unwrap());
    }

    #[test]
    fn correct_raw_int_instruction() {
        //Correct normal instructions
        assert_eq!(Instruction::RawInt(0), Instruction::from_command("RAW_INT 0").unwrap().unwrap());
        assert_eq!(Instruction::RawInt(10), Instruction::from_command("RAW_INT \"10\"").unwrap().unwrap());
        assert_eq!(Instruction::RawInt(-1), Instruction::from_command("RAW_INT -1").unwrap().unwrap());
        assert_eq!(Instruction::RawInt(i32::MAX), Instruction::from_command("RAW_INT 2147483647").unwrap().unwrap());
        assert_eq!(Instruction::RawInt(i32::MIN), Instruction::from_command("RAW_INT -2147483648").unwrap().unwrap());
    }

    #[test]
    fn empty_string() {
        //Empty string after trim should return None
        assert!(Instruction::from_command("    ").unwrap().is_none());
        assert!(Instruction::from_command("").unwrap().is_none());
    }

    #[test]
    fn raw_instruction_with_no_param() {
        //RAW Parameters without a param should return err
        assert!(Instruction::from_command("RAW_STRING").is_err());
        assert!(Instruction::from_command("RAW_INT").is_err());
    }

    #[test]
    fn normal_instruction_with_param() {
        //RAW Parameters without a param should return err
        assert!(Instruction::from_command("SUM 0").is_err());
    }

    #[test]
    fn instruction_with_param() {
        assert!(Instruction::from_command("RAW_STRING").is_err());
        assert!(Instruction::from_command("RAW_INT").is_err());
    }

    #[test]
    fn instruction_raw_int_wrong_integer() {
        assert!(Instruction::from_command("RAW_INT x").is_err());
    }

    #[test]
    fn instruction_two_params() {
        assert!(Instruction::from_command("RAW_STRING 0 1").is_err());
        assert!(Instruction::from_command("OUTPUT 0 1").is_err());
    }

    #[test]
    fn instruction_escape_string() {
        match Instruction::from_command("RAW_STRING X\\n\\r\\t\\\"\\\\\0X").unwrap().unwrap() {
            Instruction::RawString(str) => {
                assert_eq!("X\n\r\t\"\\\0X", str);
            }
            _ => { panic!() }
        }
    }

    #[test]
    fn instruction_quote_space_check() {
        match Instruction::from_command("RAW_STRING \"Hello world! \"").unwrap().unwrap() {
            Instruction::RawString(str) => {
                assert_eq!("Hello world! ", str);
            }
            _ => { panic!() }
        }
    }

    #[test]
    fn invalid_unescape() {
        assert!(Instruction::from_command("RAW_STRING \\K").is_err());
        assert!(Instruction::from_command("RAW_STRING \\").is_err());
    }

    #[test]
    fn not_ended_quote() {
        assert!(Instruction::from_command("RAW_STRING \"").is_err());
    }

    #[test]
    fn instruction_comment_check() {
        assert_eq!(Instruction::Sub, Instruction::from_command("SUB #Comment").unwrap().unwrap());
        assert!(Instruction::from_command("SUB#Comment").is_err());
        assert_eq!(Instruction::RawString("X".parse().unwrap()), Instruction::from_command("RAW_STRING X #Comment").unwrap().unwrap())
    }

    fn get_default_map() -> Params {
        Params {
            custom_colors: Default::default(),
            pixel_size: 1,
            input_path: "".to_string(),
            output_path: "".to_string(),
            ini_path: None,
            max_width: 30,
            is_random: false,
        }
    }

    #[test]
    fn simple_parse() {
        let params = get_default_map();
        for instruction in Instruction::iter() {
            match instruction {
                Instruction::RawInt(_) | Instruction::RawString(_) | Instruction::RawColor(_, _, _) => continue,
                _ => {}
            }
            let inst = instruction.get_default_colors(&params);
            let color = u32::from_str_radix(instruction.get_str("Color").unwrap(), 16).unwrap();
            assert_eq!(vec![Color::from(color)], inst);
        }
    }

    #[test]
    fn parse_instant_int() {
        let params = get_default_map();
        let instr = Instruction::RawInt(0).get_default_colors(&params);
        assert_eq!(vec![Color::from(0)], instr);
        let instr = Instruction::RawInt(1).get_default_colors(&params);
        assert_eq!(vec![Color::from(0x010000)], instr);
        let instr = Instruction::RawInt(750).get_default_colors(&params);
        assert_eq!(vec![Color::from(0xfffff0)], instr);
    }

    #[test]
    fn parse_ascii_string() {
        let params = get_default_map();
        let s = "TEEST".to_string();
        let output = vec![
            Color::not_random('\0' as i32),
            Color::not_random('T' as i32),
            Color::not_random('E' as i32),
            Instruction::Dup.get_default_colors(&params)[0],
            Color::not_random('S' as i32),
            Color::not_random('T' as i32),
        ];
        assert_eq!(output, Instruction::RawString(s).get_default_colors(&params));
    }
}
