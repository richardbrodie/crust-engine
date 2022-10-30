use std::time::Duration;

use crate::{
    geometry::{Point, Vec2},
    image::Image,
    Buffer,
};

#[derive(Debug, PartialEq)]
pub struct Sprite {
    image: Image,
    location: Point,
    destination: Option<Point>,
    movement_speed: Option<f64>,
}
impl Sprite {
    pub fn new(image: Image) -> Self {
        Self {
            image,
            location: Vec2(10.0, 10.0),
            destination: None,
            movement_speed: None,
        }
    }
    pub fn update(&mut self, dt: Duration) {
        if self.destination.is_some() && self.movement_speed.is_some() {
            self.update_movement(dt);
        }
        if let Image::Animated(img) = &mut self.image {
            img.update(dt);
        }
    }
    pub fn update_movement(&mut self, dt: Duration) {
        let dest = self.destination.unwrap();
        let speed = self.movement_speed.unwrap();

        let diff = dest - self.location;
        let hyp = (diff.0 * diff.0 + diff.1 * diff.1).sqrt();

        let dist_moved = speed * dt.as_millis() as f64;
        if dist_moved >= hyp {
            self.location = dest;
            self.destination = None;
        } else {
            let p_hyp = dist_moved / hyp;
            let p = diff * p_hyp;
            self.location += p;
        }
    }
    pub fn draw(&self, buf: &mut Buffer) {
        self.image.draw(buf, self.location);
    }
    pub fn set_destination(&mut self, dest: Point) {
        self.destination = Some(dest);
    }
}

#[derive(Debug, PartialEq)]
struct Scenery {
    image: Image,
}
impl Scenery {
    pub fn new() -> Self {
        let image = Image::load("resources/Pixel_Art_Background.png");
        Self { image }
    }
    pub fn draw(&self, buffer: &mut Buffer) {
        if let Image::Static(b) = &self.image {
            buffer.draw_to(b.data(), Vec2(0.0, 0.0));
        }
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use crate::{game::Sprite, geometry::Vec2, image::Image};

    #[test]
    fn test_sprite_destination() {
        let image = Image::load("resources/fox.png");
        let mut sprite = Sprite::new(image);
        let dest = Vec2(10.0, 10.0);

        assert_eq!(sprite.destination, None);
        sprite.set_destination(dest);
        assert_eq!(sprite.destination, Some(dest));
    }

    #[test]
    fn test_sprite_movement() {
        let image = Image::load("resources/fox.png");
        let dt = Duration::from_secs_f64(0.1);
        let mut sprite = Sprite {
            image,
            location: Vec2(0.0, 0.0),
            destination: Some(Vec2(10.0, 10.0)),
            movement_speed: Some(0.05),
        };
        sprite.update(dt);

        assert_eq!(
            sprite.location,
            Vec2(3.5355339059327373, 3.5355339059327373)
        );
    }
}
