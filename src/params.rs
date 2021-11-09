use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

use strum::IntoEnumIterator;

use crate::color::Color;
use crate::instructions::Instruction;

pub struct Params {
    pub(crate) custom_colors: HashMap<Instruction, Color>,
    pub(crate) pixel_size: u16,
    pub(crate) input_path: String,
    pub(crate) output_path: String,
    pub(crate) ini_path: Option<String>,
    pub(crate) max_width: i16,
}

impl Params {
    pub fn get_color(&self, instruction: Instruction) -> Vec<Color> {
        match self.custom_colors.get(&instruction) {
            None => instruction.get_default_colors(self),
            Some(k) => vec![k.clone()]
        }
    }
    pub fn read_colors(&mut self) {
        for i in Instruction::iter() {
            match i {
                Instruction::RawInt(_) => {}
                Instruction::RawString(_) => {}
                _ => { self.custom_colors.insert(i.clone(), i.get_default_colors(self)[0]); }
            }
        }
        if self.ini_path.is_none() || self.ini_path.as_ref().unwrap().trim().is_empty() {
            return;
        }
        let path = Path::new(self.ini_path.as_ref().unwrap());
        let file = File::open(path).expect("Unable to open ini file");
        let mut reader = BufReader::new(file);
        let mut line = String::new();
        while reader.read_line(&mut line).unwrap() > 0 {
            let x = line.trim();
            if x.len() == 0 {
                continue;
            }
            let mut iter = x.splitn(2, "=");
            let name = iter.next().expect("Invalid INI file").trim();
            let mut value = iter.next().expect("Invalid INI file").trim();
            if value.starts_with("#") {
                value = &value[1..];
            }
            let instruction = Instruction::from_command(name).expect("Invalid INI file").expect("Invalid INI file");
            self.custom_colors.insert(instruction, Color::from(u32::from_str_radix(value, 16).expect("Invalid INI file")));
            line.clear();
        }
    }
}