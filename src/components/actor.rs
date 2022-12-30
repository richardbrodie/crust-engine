use std::{path::Path, time::Duration};

use tracing::info;

use crate::{
    buffer::Buffer,
    geometry::{point, Point},
    image::Image,
};

use super::{update_location, Updatable};

#[derive(Debug, PartialEq)]
pub struct Actor {
    pub image: Image,
    pub location: Point,
    pub destination: Option<Point>,
    pub movement_speed: Option<f64>,
}
impl Actor {
    pub fn new<T: AsRef<Path>>(path: T, loc: Point, ms: Option<f64>) -> Self {
        let image = Image::load(path);
        Self {
            image,
            location: loc,
            destination: None,
            movement_speed: ms,
        }
    }
    pub fn draw(&self, buf: &mut Buffer) {
        let s = self.image.size();
        let offset_location = self.location - point(s.w as f64 / 2.0, s.h as f64);
        self.image.draw(buf, offset_location);
    }
    pub fn set_destination(&mut self, dest: Point) {
        self.destination = Some(dest);
    }
}
impl Updatable for Actor {
    fn mouse_over(&mut self, _p: Point) {
        //
    }
    fn mouse_click(&mut self, p: Point) {
        info!("setting destination");
        self.set_destination(p);
    }
    fn tick(&mut self, dt: Duration) {
        if let Image::Animated(img) = &mut self.image {
            img.update(dt);
        }
        if let (Some(dest), Some(speed)) = (self.destination, self.movement_speed) {
            self.location = update_location(self.location, dest, speed, dt);
            if dest == self.location {
                self.destination = None
            }
        }
    }
}
