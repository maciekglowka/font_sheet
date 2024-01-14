use clap::Parser;

mod bitmap;
mod renderer;

const COUNT: u32 = 256;

#[derive(Parser)]
struct Args {
    path: String,
    size: u32,
    #[arg(short, default_value_t = 16)]
    columns: u32,
    #[arg(short)]
    output: Option<String>,
    #[arg(long, value_delimiter = ' ', num_args = 3)]
    color: Option<Vec<u8>>,
}

fn main() {
    let args = Args::parse();
    let data = std::fs::read(&args.path)
        .expect(&format!("Can't read {}", args.path));

    let r = renderer::Renderer::new(&data, args.size as f32);

    let char_w = args.size;
    let char_h = r.height();

    let color = if let Some(c) = args.color {
        if c.len() != 3 { panic!("Invalid color!") }
        [c[0], c[1], c[2]]
    } else {
        [255, 255, 255]
    };

    let w = char_w * args.columns;
    let rows = (COUNT as f32 / args.columns as f32).ceil() as u32;
    let h = char_h * rows;

    let output_path = if let Some(output) = args.output {
        output
    } else {
        let stem = std::path::Path::new(&args.path)
            .file_stem()
            .expect("Wrong output path!")
            .to_str()
            .expect("Wrong output path!");
        format!("{}.png", stem)
    };

    let mut bmp = bitmap::Bitmap::new(w, h, color);
    for c in 0..255 {
        let glyph = r.glyph(c.into());
        let ox = char_w * (c as u32 % args.columns);
        let oy = char_h * (c as u32 / args.columns);
        bmp.draw_character(
            &glyph.buf,
            (ox as i32 - glyph.x).max(0) as u32,
            (oy as i32 - glyph.y).max(0) as u32,
            glyph.w, 
            glyph.h
        );
    }
    let _ = bmp.save(&output_path);
    println!("Crated bitmap font at: {}", output_path);
    println!("Char width: {}", char_w);
    println!("Char: height: {}", char_h);
}
