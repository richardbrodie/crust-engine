use std::{path::Path, time::Duration};

use tracing::info;

use crate::{geometry::Point, image::Image, Buffer};

pub trait Updatable {
    fn mouse_over(&mut self, p: Point);
    fn mouse_click(&mut self, p: Point);
    fn tick(&mut self, dt: Duration);
}

#[derive(Debug, PartialEq)]
pub struct Object {
    image: Image,
    location: Point,
}
impl Object {
    pub fn draw(&self, buf: &mut Buffer) {
        self.image.draw(buf, self.location);
    }
}
#[derive(Debug, PartialEq)]
pub struct Actor {
    image: Image,
    pub location: Point,
    pub destination: Option<Point>,
    movement_speed: Option<f64>,
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
        self.image.draw(buf, self.location);
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
        components::{Actor, Updatable},
        geometry::Point,
        image::Image,
    };

    #[test]
    fn test_sprite_destination() {
        let image = "resources/fox.png";
        let mut sprite = Actor::new(image, Point::new(0.0, 0.0), None);
        let dest = Point::new(10.0, 10.0);

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
            location: Point::new(0.0, 0.0),
            destination: Some(Point::new(10.0, 10.0)),
            movement_speed: Some(0.05),
        };
        sprite.tick(dt);

        assert_eq!(
            sprite.location,
            Point::new(3.5355339059327373, 3.5355339059327373)
        );
    }
}
