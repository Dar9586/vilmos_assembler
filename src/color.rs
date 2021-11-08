use rand::random;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
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
    pub fn random(max_value: i32) -> Self {
        let mut max_value = max_value + 1;
        let r = random::<u8>() as i32 % max_value;
        max_value -= r;
        let g = if max_value == 0 { 0 } else { random::<u8>() as i32 % max_value };
        max_value -= g;
        let b = if max_value - 1 <= u8::MAX as i32 { max_value - 1 } else { random::<u8>() as i32 % max_value };
        Color { r: r as u8, g: g as u8, b: b as u8 }
    }
    pub fn sum(&self) -> i32 {
        self.r as i32 + self.g as i32 + self.b as i32
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