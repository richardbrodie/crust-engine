mod actor;
mod object;
mod scenery;

pub use actor::Actor;
pub use object::Object;
pub use scenery::Scenery;

use std::time::Duration;

use crate::{
    buffer::Buffer,
    geometry::{point, vector, Point},
};

pub trait Updatable {
    fn mouse_over(&mut self, p: Point);
    fn mouse_click(&mut self, p: Point);
    fn tick(&mut self, dt: Duration);
    fn draw(&self, buf: &mut Buffer);
}

fn update_location(loc: Point, dest: Point, speed: f64, dt: Duration) -> Point {
    let diff = vector(dest.x as f64 - loc.x, dest.y as f64 - loc.y);
    let hyp = (diff.x * diff.x + diff.y * diff.y).sqrt();

    let dist_moved = speed * dt.as_millis() as f64;
    if dist_moved >= hyp {
        point(dest.x as f64, dest.y as f64)
    } else {
        let p_hyp = dist_moved / hyp;
        let p = diff * p_hyp;
        loc + p
    }
}
