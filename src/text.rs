const FONT_INNER: [u8; 4] = [255, 255, 255, 255];

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
    pub fn make_codepoint(&self, codepoint: char) -> Vec<u8> {
        let fb = self.font.bounds();
        let glyph = self.font.glyphs().get(&codepoint).unwrap();
        let gb = glyph.bounds();
        let mut buf: Vec<u8> = vec![0; self.width * self.height * 4];
        for i in (3..buf.len()).step_by(4) {
            buf[i] = 128;
        }

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
}
