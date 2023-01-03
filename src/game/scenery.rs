use crate::{buffer::Buffer, image::Image};

#[derive(Debug, PartialEq)]
pub struct Scenery {
    image: Image,
}
impl Scenery {
    pub fn new() -> Self {
        let image = Image::load("resources/Pixel_Art_Background.png");
        Self { image }
    }
    pub fn draw(&self, buffer: &mut Buffer) {
        if let Image::Static(i) = &self.image {
            let b = i.data();
            buffer.draw_raw_slice(b.data());
        }
    }
}
