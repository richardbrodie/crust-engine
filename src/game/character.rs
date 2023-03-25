use std::time::Duration;

use crate::buffer::Buffer;
use crate::geometry::{point, Point};
use crate::image::Image;

use super::{update_location, MouseState, ShortestPath, Updatable};

#[derive(Debug)]
pub struct Character {
    pub image: Image,
    pub position: Point,
    pub path: Option<ShortestPath>,
    pub movement_speed: f64,
    scale: f32,
}
impl Character {
    pub fn new() -> Self {
        let character_image = "resources/fox.png";
        let image = Image::load(character_image);
        Self {
            image,
            position: point(150.0, 150.0),
            movement_speed: 0.15,
            path: None,
            scale: 1.0,
        }
    }
    pub fn set_position(&mut self, pos: Point) {
        self.position = pos;
    }
    pub fn set_path(&mut self, path: Option<ShortestPath>) {
        self.path = path;
    }
}
impl Updatable for Character {
    fn mouse_state(&mut self, _m: &MouseState) {
        // self.set_path(p);
    }
    fn tick(&mut self, dt: Duration) {
        if let Image::Animated(img) = &mut self.image {
            img.update(dt);
        }
        if let Some(path) = &mut self.path {
            let mn = path.points().next();
            if let Some(n) = mn {
                self.position = update_location(self.position, *n, self.movement_speed, dt);
                if &self.position == n {
                    path.points.remove(0);
                }
                if self.position == path.end {
                    self.path = None;
                }
            }
        }
    }
    fn draw(&self, buf: &mut Buffer) {
        let s = self.image.size();
        let offset_location = self.position - point(s.w as f64 / 2.0, s.h as f64);
        self.image.draw(buf, offset_location.into(), self.scale);
    }
}
