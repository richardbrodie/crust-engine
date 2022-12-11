use std::time::{Duration, Instant};

use tracing::info;
use winit::event::ElementState;

use crate::{
    buffer::Buffer,
    components::{Actor, Object, Scenery, Updatable},
    geometry::{Point, Vec2},
};

pub const TICK: Duration = Duration::from_millis(1000 / 60);

pub struct GameState {
    pub exit_requested: bool,
    previous_time: Instant,
    mouse_location: Point,
    mouse_click: bool,
    character: Actor,
    actors: Vec<Actor>,
    objects: Vec<Object>,
    scenery: Scenery,
}
impl GameState {
    pub fn new() -> Self {
        let image = "resources/fox.png";
        // let image = Image::load("resources/ball.png");
        let character = Actor::new(image, Some(0.1));
        let scenery = Scenery::new();

        Self {
            exit_requested: false,
            previous_time: Instant::now(),
            character,
            actors: vec![],
            objects: vec![],
            scenery,
            mouse_location: Vec2(0.0, 0.0),
            mouse_click: false,
        }
    }
    pub fn mouse_over(&mut self, loc: Point) {
        // if let Ok(pos) =
        //     self.click_pos = match self.buffer.window_pos_to_pixel(pos.into()) {
        self.mouse_location = loc;
    }
    pub fn mouse_click(&mut self, state: ElementState) {
        info!("mouse click");
        self.mouse_click = state == ElementState::Pressed;
    }
    pub fn tick(&mut self, buffer: &mut Buffer) -> bool {
        let delta = self.previous_time.elapsed();
        if delta >= TICK {
            self.previous_time = Instant::now();

            self.scenery.draw(buffer);

            self.character.mouse_over(self.mouse_location);
            if self.mouse_click {
                self.mouse_click = false;
                self.character.mouse_click(self.mouse_location);
            }
            self.character.tick(delta);
            self.character.draw(buffer);

            // self.objects.iter_mut().for_each(|s| {
            //     s.mouse_over(self.mouse_location);
            //     s.tick(delta);
            //     s.draw(buffer);
            // });
            //
            // self.actors.iter_mut().for_each(|s| {
            //     s.mouse_over(self.mouse_location);
            //     s.tick(delta);
            //     s.draw(buffer);
            // });
        }
        delta >= TICK
    }
}
