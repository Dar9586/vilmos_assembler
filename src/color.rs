use rand::{random, thread_rng};
use rand::seq::SliceRandom;

pub const COLOR_COMPONENTS: usize = 3;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
}

impl From<u32> for Color {
    fn from(val: u32) -> Self {
        Color {
            r: (val >> 16) as u8,
            g: (val >> 8) as u8,
            b: (val >> 0) as u8,
        }
    }
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Color { r, g, b }
    }

    pub fn not_random(mut value: i32) -> Self {
        let r = (u8::MAX as i32).min(value);
        value -= r;
        let g = (u8::MAX as i32).min(value);
        value -= g;
        let b = (u8::MAX as i32).min(value);
        Color { r: r as u8, g: g as u8, b: b as u8 }
    }


    pub fn random(value: i32) -> Self {
        let mut components = [0u8; COLOR_COMPONENTS];
        let mut value = value;
        while value != 0 {
            let min = *components.iter().min().unwrap() as i32;
            let i = 1 + (random::<u8>() as i32 % value.min(255 - min)) as u8;
            for item in 0..3 {
                if let Some(new_val) = components[item].checked_add(i) {
                    components[item] = new_val;
                    break;
                }
            }
            components.shuffle(&mut thread_rng());
            value -= i as i32;
        }

        Color { r: components[0], g: components[1], b: components[2] }
    }
}

impl From<i32> for Color {
    fn from(val: i32) -> Self {
        Color::from(val as u32)
    }
}

impl From<&i32> for Color {
    fn from(val: &i32) -> Self {
        Color::from(*val as u32)
    }
}

impl From<char> for Color {
    fn from(val: char) -> Self {
        Color::from(val as u32)
    }
}

impl Color {
    pub fn write_data(&self, data: &mut Vec<u8>) {
        data.push(self.r);
        data.push(self.g);
        data.push(self.b);
    }
}