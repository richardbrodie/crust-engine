use std::time::{Duration, Instant};

use tracing::info;
use winit::event::ElementState;

use crate::{
    buffer::Buffer,
    components::{Actor, Object, Scenery, Updatable},
    geometry::{line, point, Line, Point},
};

pub const TICK: Duration = Duration::from_millis(1000 / 90);

pub struct GameState {
    pub exit_requested: bool,
    previous_time: Instant,
    mouse_location: Point,
    mouse_click: bool,
    character: Actor,
    actors: Vec<Actor>,
    objects: Vec<Object>,
    scenery: Scenery,
    walkline: Option<Line>,
}
impl GameState {
    pub fn new() -> Self {
        let character_image = "resources/fox.png";
        let ball_image = "resources/ball.png";
        let character = Actor::new(character_image, point(50.0, 50.0), Some(0.07));
        let actors = vec![Actor::new(ball_image, point(150.0, 150.0), None)];
        let scenery = Scenery::new();

        Self {
            exit_requested: false,
            previous_time: Instant::now(),
            character,
            actors,
            objects: vec![],
            scenery,
            mouse_location: point(0.0, 0.0),
            mouse_click: false,
            walkline: None,
        }
    }
    pub fn mouse_over(&mut self, loc: Point) {
        self.mouse_location = loc;
    }
    pub fn mouse_click(&mut self, state: ElementState) {
        if state == ElementState::Pressed {
            self.walkline = Some(line(self.character.location, self.mouse_location));
            self.mouse_click = true;
        }
    }
    pub fn tick(&mut self, buffer: &mut Buffer) -> bool {
        let delta = self.previous_time.elapsed();
        if delta >= TICK {
            self.previous_time = Instant::now();

            self.scenery.draw(buffer);

            {
                let l = if self.walkline.is_none() {
                    line(self.character.location, self.mouse_location)
                } else {
                    self.walkline.unwrap()
                };
                buffer.draw_line(&l);
                buffer.draw_point(l.start);
                buffer.draw_point(l.end);
            }
            self.character.mouse_over(self.mouse_location);
            if self.mouse_click {
                self.mouse_click = false;
                self.character.mouse_click(self.mouse_location);
            }
            self.character.tick(delta);
            self.character.draw(buffer);

            self.objects.iter_mut().for_each(|s| {
                s.mouse_over(self.mouse_location);
                s.tick(delta);
                s.draw(buffer);
            });

            self.actors.iter_mut().for_each(|s| {
                s.mouse_over(self.mouse_location);
                s.tick(delta);
                s.draw(buffer);
            });
            if self.character.destination.is_none() {
                self.walkline = None;
            }
        }
        delta >= TICK
    }
}
