use std::{path::Path, time::Duration};

use crate::{buffer::Buffer, geometry::Point, image::Image};

use super::Updatable;

#[derive(Debug, PartialEq)]
pub struct Object {
    image: Image,
    location: Point,
}
impl Object {
    pub fn new<T: AsRef<Path>>(path: T, loc: Point) -> Self {
        let image = Image::load(path);
        Self {
            image,
            location: loc,
        }
    }
}
impl Updatable for Object {
    fn mouse_over(&mut self, _p: Point) {
        //
    }
    fn mouse_click(&mut self, _p: Point) {
        //
    }
    fn tick(&mut self, dt: Duration) {
        if let Image::Animated(img) = &mut self.image {
            img.update(dt);
        }
    }
    fn draw(&self, buf: &mut Buffer) {
        self.image.draw(buf, self.location);
    }
}
