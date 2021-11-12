use std::collections::HashMap;

use strum::IntoEnumIterator;

use crate::color::Color;
use crate::instructions::Instruction;

pub struct Params {
    pub custom_colors: HashMap<Instruction, Color>,
    pub pixel_size: u16,
    pub input_path: String,
    pub output_path: String,
    pub ini_path: Option<String>,
    pub max_width: i16,
    pub is_random: bool
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
                Instruction::RawInt(_) | Instruction::RawString(_) => {}
                _ => { self.custom_colors.insert(i.clone(), i.get_default_colors(self)[0]); }
            }
        }
        if self.ini_path.is_none() {
            return;
        }
        let name = self.ini_path.as_ref().unwrap().trim();
        if name.is_empty() {
            return;
        }
        let map = ini!(name);
        if map.get("colors").is_none() {
            return;
        }
        let color_section = map.get("colors").unwrap();
        for i in color_section {
            if i.1.is_none() || i.1.as_ref().unwrap().is_empty() {
                continue;
            }
            let command = i.0.as_str().to_uppercase();
            if command.starts_with("RAW") {
                panic!("Can't overwrite the RAW_ instruction")
            }
            let command = Instruction::find_name(command.as_str()).expect("Wrong instruction name in config file");

            let mut color_str = i.1.clone().unwrap();
            if color_str.len() == 3 {
                for i in 0..3 {
                    color_str.insert(i * 2, color_str.chars().nth(i * 2).unwrap());
                }
            }
            let hex_code = u32::from_str_radix(color_str.as_str(), 16).expect("Invalid value for config file");
            self.custom_colors.insert(command, Color::from(hex_code));
        }
    }
}