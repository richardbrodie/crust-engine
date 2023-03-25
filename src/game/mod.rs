mod actor;
mod character;
mod game_state;
mod object;
mod pathfinding;
mod scenery;
mod walkbox;

pub use actor::Actor;
pub use character::Character;
pub use game_state::GameState;
pub use object::Object;
pub use pathfinding::{astar, Graph, ShortestPath};
pub use scenery::Scenery;
pub use walkbox::WalkBox;

use std::time::Duration;

use crate::{
    buffer::Buffer,
    geometry::{point, vector, Point},
};

pub trait Updatable {
    fn mouse_state(&mut self, m: &MouseState);
    fn tick(&mut self, dt: Duration);
    fn draw(&self, buffer: &mut Buffer);
}

#[derive(Default, Debug, PartialEq, Clone, Copy)]
pub struct MouseState {
    position: Point,
    left_button: bool,
    right_button: bool,
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
