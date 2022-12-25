mod actor;
mod object;

pub use actor::Actor;
pub use object::Object;

use std::time::Duration;

use crate::{geometry::Point, image::Image, Buffer};

pub trait Updatable {
    fn mouse_over(&mut self, p: Point);
    fn mouse_click(&mut self, p: Point);
    fn tick(&mut self, dt: Duration);
}

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

pub fn update_location(loc: Point, dest: Point, speed: f64, dt: Duration) -> Point {
    let diff = dest - loc;
    let hyp = (diff.x * diff.x + diff.y * diff.y).sqrt();

    let dist_moved = speed * dt.as_millis() as f64;
    if dist_moved >= hyp {
        dest
    } else {
        let p_hyp = dist_moved / hyp;
        let p = diff * p_hyp;
        loc + p
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use crate::{
        components::{actor::Actor, Updatable},
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
