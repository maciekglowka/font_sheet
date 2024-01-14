use image::{ImageBuffer, Rgba};

pub struct Bitmap {
    img: ImageBuffer<Rgba<u8>, Vec<u8>>,
    color: [u8; 3],
    width: u32,
    height: u32
}
impl Bitmap {
    pub fn new(width: u32, height: u32, color: [u8; 3]) -> Self {
        Self {
            img: ImageBuffer::<Rgba<u8>, Vec<u8>>::new(width, height),
            color,
            width,
            height
        }
    }
    pub fn draw_character(
        &mut self,
        data: &[u8],
        x: u32,
        y: u32,
        w: u32,
        h: u32
    ) {
        for px in 0..w {
            for py in 0..h {
                self.img.put_pixel(
                    x + px.min(self.width-1),
                    (y + py).min(self.height-1),
                    Rgba([
                        self.color[0],
                        self.color[1],
                        self.color[2],
                        data[(py * w + px) as usize]
                    ])
                );
            }
        }
    }
    pub fn save(&self, path: &str) {
        self.img.save(path)
            .expect("File save failed!");
    }
}

