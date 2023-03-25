use std::{path::Path, time::Duration};

use crate::{
    buffer::Buffer,
    geometry::{point, Point},
    image::Image,
};

use super::{update_location, MouseState, Updatable};

#[derive(Debug, PartialEq)]
pub struct Actor {
    pub image: Image,
    pub location: Point,
    pub path: Vec<Point>,
    pub movement_speed: Option<f64>,
    scale: f32,
}
impl Actor {
    pub fn new<T: AsRef<Path>>(path: T, loc: Point, ms: Option<f64>) -> Self {
        let image = Image::load(path);
        Self {
            image,
            location: loc,
            path: vec![],
            movement_speed: ms,
            scale: 1.0,
        }
    }
}
impl Updatable for Actor {
    fn mouse_state(&mut self, _m: &MouseState) {
        // self.set_path(p);
    }
    fn tick(&mut self, dt: Duration) {
        if let Image::Animated(img) = &mut self.image {
            img.update(dt);
        }
        if self.movement_speed.is_none() {
            return;
        }
        if let Some(next) = self.path.first() {
            self.location = update_location(self.location, *next, self.movement_speed.unwrap(), dt);
            if next == &self.location {
                self.path.remove(0);
            }
        }
    }
    fn draw(&self, buf: &mut Buffer) {
        let s = self.image.size();
        let offset_location = self.location - point(s.w as f64 / 2.0, s.h as f64);
        self.image.draw(buf, offset_location.into(), 1.0);
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use crate::{
        game::{Actor, Character, Updatable},
        geometry::point,
        image::Image,
    };

    #[test]
    fn test_sprite_destination() {
        let image = "resources/fox.png";
        let mut sprite = Character::new();
        // let mut sprite = Character::new(image, point(0.0, 0.0), None);
        let dest = vec![point(10.0, 10.0)];

        assert_eq!(sprite.path, vec![]);
        sprite.set_path(dest.clone().into_iter());
        assert_eq!(sprite.path, dest);
    }

    #[test]
    fn test_sprite_movement() {
        let image = Image::load("resources/fox.png");
        let dt = Duration::from_secs_f64(0.1);
        let mut sprite = Actor {
            image,
            location: point(0.0, 0.0),
            path: vec![point(10.0, 10.0)],
            movement_speed: Some(0.05),
        };
        sprite.tick(dt);

        assert_eq!(
            sprite.location,
            point(3.5355339059327373, 3.5355339059327373)
        );
    }
}
