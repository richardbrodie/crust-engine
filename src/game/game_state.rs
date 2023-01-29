use std::time::{Duration, Instant};

use winit::event::ElementState;

use crate::{
    buffer::Buffer,
    game::{Actor, Graph, Object, Scenery, ShortestPath, Updatable, WalkBox},
    geometry::{point, LineType, Point, Polygon},
    text::GlyphWriter,
};

use super::astar;

pub const TICK: Duration = Duration::from_millis(1000 / 90);

#[derive(Debug)]
pub struct GameState {
    pub exit_requested: bool,
    previous_time: Instant,
    mouse_location: Point,
    mouse_click: bool,
    character: Actor,
    character_destimation: Option<Point>,
    character_path: Option<ShortestPath>,
    actors: Vec<Actor>,
    objects: Vec<Object>,
    scenery: Scenery,
    walkbox: WalkBox,
    text_writer: GlyphWriter,
}
impl GameState {
    pub fn new() -> Self {
        let character_image = "resources/fox.png";
        // let ball_image = "resources/ball.png";
        let character = Actor::new(character_image, point(150.0, 150.0), Some(0.15));
        // let objects = vec![Object::new(ball_image, point(350.0, 350.0))];
        let scenery = Scenery::new();
        let walkbox = WalkBox::new(
            Polygon::new(vec![
                point(60.0, 60.0),
                point(300.0, 60.0),
                point(300.0, 240.0),
                point(360.0, 240.0),
                point(360.0, 60.0),
                point(610.0, 60.0),
                point(610.0, 260.0),
                point(510.0, 260.0),
                point(510.0, 280.0),
                point(610.0, 280.0),
                point(610.0, 435.0),
                point(60.0, 435.0),
                point(60.0, 60.0),
            ]),
            vec![],
        );
        let text_writer = GlyphWriter::new();

        Self {
            exit_requested: false,
            previous_time: Instant::now(),
            character,
            character_path: None,
            character_destimation: None,
            actors: vec![],
            objects: vec![],
            scenery,
            mouse_location: point(0.0, 0.0),
            mouse_click: false,
            text_writer,
            walkbox,
        }
    }
    pub fn mouse_over(&mut self, loc: Point) {
        self.mouse_location = loc;
    }
    pub fn mouse_click(&mut self, state: ElementState) {
        if state == ElementState::Pressed {
            self.mouse_click = true;
        }
    }
    pub fn tick(&mut self) -> bool {
        let delta = self.previous_time.elapsed();
        if delta >= TICK {
            self.previous_time = Instant::now();

            if cfg!(debug_assertions) {
                let dest_point = self.calculate_destination();
                self.character_destimation = Some(dest_point);
                self.walkbox
                    .add_temporary_edges(self.character.location, dest_point);
                self.character_path = astar(&self.walkbox, self.character.location, dest_point);
            }

            if self.mouse_click {
                self.mouse_click = false;
                if !cfg!(debug_assertions) {
                    let dest_point = self.calculate_destination();
                    self.walkbox
                        .add_temporary_edges(self.character.location, dest_point);
                    self.character_path = astar(&self.walkbox, self.character.location, dest_point);
                }
                if let Some(path) = &self.character_path {
                    self.character.set_path(path.points().map(|e| e.to_owned()));
                }
            }

            self.character.mouse_over(self.mouse_location);
            self.character.tick(delta);

            self.objects.iter_mut().for_each(|s| {
                s.mouse_over(self.mouse_location);
                s.tick(delta);
            });

            self.actors.iter_mut().for_each(|s| {
                s.mouse_over(self.mouse_location);
                s.tick(delta);
            });
        }
        delta >= TICK
    }

    pub fn draw(&self, buffer: &mut Buffer) {
        self.scenery.draw(buffer);

        if cfg!(debug_assertions) {
            for l in self.walkbox.edges.iter() {
                buffer.draw_line(&l, crate::geometry::LineType::Box);
            }
            for l in self.walkbox.walkable_edges() {
                buffer.draw_line(l, LineType::Graph);
            }
        }

        self.character.draw(buffer);
        self.objects.iter().for_each(|s| {
            s.draw(buffer);
        });
        self.actors.iter().for_each(|s| {
            s.draw(buffer);
        });

        if cfg!(debug_assertions) {
            if let (Some(path), Some(dest_point)) =
                (&self.character_path, self.character_destimation)
            {
                for l in path.lines() {
                    buffer.draw_line(&l, LineType::Path);
                }
                buffer.draw_point(dest_point);
            }

            let l = self.mouse_location;
            let to = self
                .text_writer
                .make_string(&format!("{}, {}", l.x, l.y))
                .to_bmp();
            let p = point(l.x - 20.0, l.y + 10.0);
            buffer.draw_bmp(&to, p);
        }
    }

    fn calculate_destination(&self) -> Point {
        if self.walkbox.contains(self.mouse_location) {
            return self.mouse_location;
        }
        let res = self
            .walkbox
            .edges
            .iter()
            .map(|side| side.closest_point(self.mouse_location))
            .fold((f64::MAX, self.mouse_location), |acc, p| {
                let dist = (p - self.mouse_location).length();
                if acc.0 > dist {
                    return (dist, p);
                }
                acc
            });
        res.1
    }
}
