use std::time::{Duration, Instant};

use winit::event::ElementState;

use crate::{
    buffer::Buffer,
    game::{Actor, Object, Scenery, Updatable, WalkBox},
    geometry::{point, Graph, LineType, Point, Polygon},
    text::GlyphWriter,
};

pub const TICK: Duration = Duration::from_millis(1000 / 90);

#[derive(Debug)]
pub struct GameState {
    pub exit_requested: bool,
    previous_time: Instant,
    mouse_location: Point,
    mouse_click: bool,
    character: Actor,
    actors: Vec<Actor>,
    objects: Vec<Object>,
    scenery: Scenery,
    walkbox: WalkBox,
    text_writer: GlyphWriter,
    graph: Graph,
}
impl GameState {
    pub fn new() -> Self {
        let character_image = "resources/fox.png";
        let ball_image = "resources/ball.png";
        let character = Actor::new(character_image, point(150.0, 150.0), Some(0.07));
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
        // font.draw_codepoint('c');
        // font.draw_codepoint('d');
        let graph = Graph::new(walkbox.clone());

        Self {
            exit_requested: false,
            previous_time: Instant::now(),
            character,
            actors: vec![],
            objects: vec![],
            scenery,
            mouse_location: point(0.0, 0.0),
            mouse_click: false,
            text_writer,
            walkbox,
            graph,
        }
    }
    pub fn mouse_over(&mut self, loc: Point) {
        self.graph.add_temporary_edges(self.character.location, loc);
        self.mouse_location = loc;
    }
    pub fn mouse_click(&mut self, state: ElementState) {
        if state == ElementState::Pressed {
            self.mouse_click = true;
        }
    }
    pub fn tick(&mut self, buffer: &mut Buffer) -> bool {
        let delta = self.previous_time.elapsed();
        if delta >= TICK {
            self.previous_time = Instant::now();

            self.scenery.draw(buffer);

            for l in self.walkbox.exterior.edges() {
                buffer.draw_line(&l, crate::geometry::LineType::Box);
            }
            for l in self.graph.walkable_edges() {
                buffer.draw_line(l, LineType::Path);
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

            let bmp = self.text_writer.make_codepoint('z');
            let p = point(256.0, 256.0);
            // buffer.draw_bmp(&bmp, p);
        }
        delta >= TICK
    }
    // fn calculate_path(&self) -> Line {}
}
