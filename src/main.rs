use argparse::{ArgumentParser, Store};

mod reader;
mod instructions;
mod color;
mod params;
mod parser;
mod assembler;

fn main() {
    let mut ini_path = String::new();
    let mut in_path: String = String::new();
    let mut out_path: String = String::new();
    let mut pixel_size: u16 = 1;
    let mut max_width: i16 = -1;

    {  // this block limits scope of borrows by ap.refer() method
        let mut ap = ArgumentParser::new();
        ap.set_description("Vilmos assembler");
        ap.refer(&mut in_path)
            .add_option(&["--input", "-i"], Store,
                        "Input VASM files").required();
        ap.refer(&mut out_path)
            .add_option(&["--output", "-o"], Store,
                        "Output PNG files").required();
        ap.refer(&mut ini_path)
            .add_option(&["--config"], Store,
                        "Config file for custom colors");
        ap.refer(&mut pixel_size)
            .add_option(&["--pixel-size"], Store,
                        "Size of each pixel");
        ap.refer(&mut max_width)
            .add_option(&["--max-width"], Store,
                        "Max pixels per row [-1 for unlimited]");

        ap.parse_args_or_exit();
    }

    let mut conf = params::Params {
        custom_colors: Default::default(),
        max_width,
        pixel_size,
        input_path: in_path,
        output_path: out_path,
        ini_path: Option::from(ini_path.clone()),
    };
    conf.read_colors();
    let colors = assembler::parse(&conf);
    assembler::write_image(&conf, &colors);
}
