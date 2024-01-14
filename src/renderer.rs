use anyhow::{anyhow, Result};
use fontdue::{Font, FontSettings};

pub struct GlyphData {
    pub buf: Vec<u8>,
    pub x: i32,
    pub y: i32,
    pub w: u32,
    pub h: u32
}

#[derive(Debug)]
pub struct Renderer {
    font: Font,
    size: f32,
    ascent: f32,
    descent: f32,
    height: u32
}
impl Renderer {
    pub fn new(data: &[u8], size: f32) -> Self {
        let settings = FontSettings {
            scale: size,
            ..Default::default()
        };
        let font = Font::from_bytes(data, settings)
            .expect("Can't create font renderer!");
        let h_metrics = font.horizontal_line_metrics(size)
            .expect("Cant't read horizontal metrics!");

        let height = (h_metrics.ascent.ceil() - h_metrics.descent.ceil()) as u32;

        Self {
            font,
            size,
            ascent: h_metrics.ascent,
            descent: h_metrics.descent,
            height
        }
    }
    pub fn height(&self) -> u32 {
        self.height
    }
    pub fn glyph(&self, c: char) -> Option<GlyphData> {
        if !self.font.has_glyph(c) { return None }
        let (m, b) = self.font.rasterize(c, self.size);

        let y = m.height as i32 - self.ascent as i32 + m.ymin;
        Some(GlyphData {
            buf: b,
            x: m.xmin,
            y,
            w: m.width as u32,
            h: m.height as u32
        })
    }
}
