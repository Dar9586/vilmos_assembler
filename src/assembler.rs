use std::cmp::min;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::Path;

use crate::color;
use crate::color::Color;
use crate::instructions::Instruction;
use crate::params::Params;

const MAX_IMAGE_WIDTH: u32 = 1_000_000u32;

pub fn parse(conf: &Params) -> Vec<Color> {
    let path = &conf.input_path;
    let mut reader = BufReader::new(File::open(path).unwrap());
    let mut line = String::new();
    let mut colors: Vec<Color> = Vec::new();
    while reader.read_line(&mut line).unwrap() > 0 {
        line.pop();
        let y = Instruction::from_command(&line).expect("Invalid input file ");
        match y {
            None => {}
            Some(val) => { colors.append(&mut conf.get_color(val)); }
        }
        line.clear();
    }
    colors
}

fn fill_row(start_index: u32, count: u32, conf: &Params, colors: &Vec<Color>, buffer: &mut Vec<u8>) {
    for i in start_index..start_index + count {
        let color = match colors.get(i as usize) {
            None => conf.get_color(Instruction::Quit)[0],
            Some(c) => c.clone()
        };
        for _ in 0..conf.pixel_size {
            color.write_data(buffer);
        }
    }
}


pub fn write_image(conf: &Params, colors: &Vec<Color>) {
    let size = colors.len() as u32;
    let pixel_size = conf.pixel_size as u32;
    let pixel_per_row = if conf.max_width == -1 { MAX_IMAGE_WIDTH / pixel_size } else { conf.max_width as u32 };
    let pixel_per_row = min(pixel_per_row, size);
    let height = size / pixel_per_row + u32::from(size % pixel_per_row != 0);
    let path = Path::new(&conf.output_path);
    let file = File::create(path).expect("Unable to create output file");
    let ref mut w = BufWriter::new(file);
    let mut encoder = png::Encoder::new(w, pixel_per_row * pixel_size, height * pixel_size);
    encoder.set_color(png::ColorType::Rgb);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header().expect("Unable to write file");
    let mut stream = writer.stream_writer().expect("Unable to write file");
    let mut buffer: Vec<u8> = Vec::with_capacity((pixel_size * pixel_per_row * color::COLOR_COMPONENTS as u32) as usize);
    for i in 0..height {
        fill_row(i * pixel_per_row, pixel_per_row, conf, colors, &mut buffer);
        for _ in 0..pixel_size {
            stream.write(&buffer).expect("Error while writing the output file");
        }
        buffer.clear();
    }
}
