use crate::{geometry::Rect, image::Bitmap};

const FONT_INNER: [u8; 4] = [255, 255, 255, 255];

pub struct TextObject {
    pub buffer: Vec<u8>,
    width: usize,
    height: usize,
    glyphs: usize,
}
impl TextObject {
    fn new(w: usize, h: usize) -> Self {
        Self {
            buffer: vec![],
            width: w,
            height: h,
            glyphs: 0,
        }
    }
    fn add_glyph(&mut self, buf: &[u8]) {
        let row_size = self.width * 4;
        self.buffer.reserve(self.width * self.height * 4);
        for y in 0..self.height {
            let src_start = y * row_size;
            let src_end = src_start + row_size;
            let t = (self.glyphs * y * row_size) + (row_size * (self.glyphs + y));
            self.buffer
                .splice(t..t, buf[src_start..src_end].iter().cloned());
        }
        self.glyphs += 1;
    }
    pub fn to_bmp(self) -> Bitmap {
        Bitmap::new(
            self.buffer,
            Rect {
                w: self.width * self.glyphs,
                h: self.height,
            },
        )
    }
}

#[derive(Debug)]
pub struct GlyphWriter {
    font: bdf::Font,
    width: usize,
    height: usize,
}
impl GlyphWriter {
    pub fn new() -> Self {
        let font = bdf::open("resources/font.bdf").unwrap();
        let height = font.bounds().height as usize;
        let width = font.bounds().width as usize;
        Self {
            font,
            width,
            height,
        }
    }
    fn make_codepoint(&self, codepoint: char) -> Vec<u8> {
        let mut buf: Vec<u8> = vec![0; self.width * self.height * 4];
        for i in (3..buf.len()).step_by(4) {
            buf[i] = 128;
        }

        let fb = self.font.bounds();
        let glyph = self.font.glyphs().get(&codepoint).unwrap();
        let gb = glyph.bounds();

        let y_off = (self.height - gb.height as usize) as i32 - gb.y + fb.y;
        for y in 0..glyph.height() {
            let ay = (y + y_off as u32) * (fb.width * 4) + (gb.x * 4) as u32;
            for x in 0..glyph.width() {
                let ax = (x * 4) as usize + ay as usize;
                if glyph.get(x, y) {
                    buf[ax..ax + 4].copy_from_slice(&FONT_INNER);
                }
            }
        }
        buf
    }

    pub fn make_string(&self, string: &str) -> TextObject {
        let mut output = TextObject::new(self.width, self.height);
        for c in string.chars() {
            let buf = self.make_codepoint(c);
            output.add_glyph(&buf);
        }

        output
    }
}
