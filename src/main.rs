use clap::Parser;

mod bitmap;
mod renderer;

const COUNT: u32 = 256;

/// Convert TTF font files into ASCII indexed sprite sheets
#[derive(Parser)]
#[command(version)]
struct Args {
    /// Input font path
    path: String,
    /// Em size in px
    size: u32,
    /// Sprite sheet columns
    #[arg(short, default_value_t = 16)]
    columns: u32,
    /// Output path [default: <input_path>.png]
    #[arg(short)]
    output: Option<String>,
    /// Color [default: 255 255 255]
    #[arg(long, value_delimiter = ' ', num_args = 3)]
    color: Option<Vec<u8>>,
    /// Vertical gap
    #[arg(long, default_value_t = 0 )]
    v_gap: u32,
    /// Horizontal gap
    #[arg(long, default_value_t = 0 )]
    h_gap: u32
}

fn main() {
    let args = Args::parse();
    let data = std::fs::read(&args.path)
        .expect(&format!("Can't read {}", args.path));

    let r = renderer::Renderer::new(&data, args.size as f32);

    let char_w = args.size;
    let char_h = r.height();
    let grid_w = args.h_gap + char_w;
    let grid_h = args.v_gap + char_h;

    let color = if let Some(c) = args.color {
        if c.len() != 3 { panic!("Invalid color!") }
        [c[0], c[1], c[2]]
    } else {
        [255, 255, 255]
    };

    let w = grid_w * args.columns;
    let rows = (COUNT as f32 / args.columns as f32).ceil() as u32;
    let h = grid_h * rows;

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
        let Some(glyph) = r.glyph(c.into()) else { continue };
        let ox = grid_w * (c as u32 % args.columns);
        let oy = grid_h * (c as u32 / args.columns);
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
    println!("Char width: {}/{}", char_w, grid_w);
    println!("Char: height: {}/{}", char_h, grid_h);
}
