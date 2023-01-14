use std::{path::Path, time::Duration};

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
    pub fn set_destination(&mut self, dest: Point) {
        self.destination = Some(dest);
    }
}
impl Updatable for Actor {
    fn mouse_over(&mut self, _p: Point) {
        //
    }
    fn mouse_click(&mut self, p: Point) {
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
    fn draw(&self, buf: &mut Buffer) {
        let s = self.image.size();
        let offset_location = self.location - point(s.w as f64 / 2.0, s.h as f64);
        self.image.draw(buf, offset_location.into());
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use crate::{
        game::{Actor, Updatable},
        geometry::point,
        image::Image,
    };

    #[test]
    fn test_sprite_destination() {
        let image = "resources/fox.png";
        let mut sprite = Actor::new(image, point(0.0, 0.0), None);
        let dest = point(10.0, 10.0);

        assert_eq!(sprite.destination, None);
        sprite.set_destination(dest);
        assert_eq!(sprite.destination, Some(dest));
    }

    #[test]
    fn test_sprite_movement() {
        let image = Image::load("resources/fox.png");
        let dt = Duration::from_secs_f64(0.1);
        let mut sprite = Actor {
            image,
            location: point(0.0, 0.0),
            destination: Some(point(10.0, 10.0)),
            movement_speed: Some(0.05),
        };
        sprite.tick(dt);

        assert_eq!(
            sprite.location,
            point(3.5355339059327373, 3.5355339059327373)
        );
    }
}
