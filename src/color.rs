use rand::{random, thread_rng};
use rand::seq::SliceRandom;

const RANDOM_STEP_INIT: i32 = 25;
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
    pub fn random(value: i32) -> Self {
        let mut components = [0u8; COLOR_COMPONENTS];
        let base: u8 = (value / (RANDOM_STEP_INIT * COLOR_COMPONENTS as i32) * RANDOM_STEP_INIT) as u8;
        components.fill(base as u8);
        let mut max_value = value - (base as i32 * COLOR_COMPONENTS as i32);
        while max_value != 0 {
            let i = random::<u8>() % COLOR_COMPONENTS as u8;
            let val = (1.max(random::<u8>() as i32 % max_value)) as u8;
            match components[i as usize].checked_add(val) {
                None => {}
                Some(v) => {
                    components[i as usize] = v;
                    max_value -= val as i32;
                }
            }
        }
        components.shuffle(&mut thread_rng());
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