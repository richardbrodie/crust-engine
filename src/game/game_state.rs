use std::{
    fs::File,
    io::Read,
    path::Path,
    time::{Duration, Instant},
};

use winit::event::{ElementState, MouseButton};

use crate::{
    buffer::Buffer,
    game::{Actor, Graph, Object, Scenery, ShortestPath, Updatable, WalkBox},
    geometry::{point, LineType, Point, Polygon},
    text::GlyphWriter,
};

use super::{astar, Character, MouseState};

pub const TICK: Duration = Duration::from_millis(1000 / 60);

#[derive(Debug)]
pub struct GameState {
    pub exit_requested: bool,
    previous_time: Instant,
    mouse_state: MouseState,
    character: Character,
    walkbox: WalkBox,
    text_writer: GlyphWriter,
    room: Room,

    pub path: Option<ShortestPath>,
}
impl GameState {
    pub fn new() -> Self {
        // let ball_image = "resources/ball.png";
        // let objects = vec![Object::new(ball_image, point(350.0, 350.0))];
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

        Self {
            exit_requested: false,
            previous_time: Instant::now(),
            character: Character::new(),
            mouse_state: MouseState::default(),
            text_writer: GlyphWriter::new(),
            walkbox,
            room: Room::new(),

            path: None,
        }
    }
    pub fn mouse_over(&mut self, loc: Point) {
        self.mouse_state.position = loc;
        self.room.mouse_state(&self.mouse_state);
        self.character.mouse_state(&self.mouse_state);
    }
    pub fn mouse_click(&mut self, button: MouseButton, state: ElementState) {
        let pressed = state == ElementState::Pressed;
        match button {
            MouseButton::Left => self.mouse_state.left_button = pressed,
            MouseButton::Right => self.mouse_state.right_button = pressed,
            _ => {}
        }
        self.room.mouse_state(&self.mouse_state);
        self.character.mouse_state(&self.mouse_state);
    }
    fn mouse_pos(&self) -> Point {
        self.mouse_state.position
    }
    fn mouse_clicked(&self) -> bool {
        self.mouse_state.left_button
    }
    pub fn tick(&mut self) -> bool {
        let delta = self.previous_time.elapsed();
        if delta >= TICK {
            self.previous_time = Instant::now();

            let dest_point = self.walkbox.clamp_destination(self.mouse_pos());
            self.walkbox
                .add_temporary_edges(self.character.position, dest_point);
            let path = astar(&self.walkbox, self.character.position, dest_point);
            if cfg!(debug_assertions) {
                self.path = path.clone();
            }

            if self.mouse_clicked() {
                self.mouse_state.left_button = false;
                self.character.set_path(path);
            }

            self.room.tick(delta);
            self.character.tick(delta);
        }
        delta >= TICK
    }

    pub fn draw(&self, buffer: &mut Buffer) {
        self.room.draw(buffer);

        if cfg!(debug_assertions) {
            for l in self.walkbox.edges.iter() {
                buffer.draw_line(&l, crate::geometry::LineType::Box);
            }
            for l in self.walkbox.walkable_edges() {
                buffer.draw_line(l, LineType::Graph);
            }
        }
        if let Some(path) = &self.character.path {
            for l in path.lines() {
                buffer.draw_line(&l, LineType::Path);
            }
        }
        self.character.draw(buffer);

        if cfg!(debug_assertions) {
            let l = self.mouse_state.position;
            let to = self
                .text_writer
                .make_string(&format!("{}, {}", l.x, l.y))
                .to_bmp();
            let p = point(l.x - 20.0, l.y + 10.0);
            buffer.draw_bmp(&to, p);
        }
    }
}

#[derive(Debug)]
struct Room {
    scenery: Scenery,
    actors: Vec<Actor>,
    objects: Vec<Object>,
}
impl Room {
    fn new() -> Self {
        let scenery = Scenery::new();
        Self {
            scenery,
            actors: vec![],
            objects: vec![],
        }
    }
    fn update_positions(&self, character: Point, mouse: Point) {
        //
    }
    fn load(room_id: &str) {
        let p = Path::new("resources/rooms");
        p.join("room_id");
        let mut f = File::open(p).unwrap();
        let mut contents = String::new();
        f.read_to_string(&mut contents).expect("couldn't read file");
        // let scene: Scene = serde_yaml::from_str(&contents).unwrap();
        // scene
    }
}
impl Updatable for Room {
    fn tick(&mut self, dt: Duration) {
        self.objects.iter_mut().for_each(|s| {
            s.tick(dt);
        });

        self.actors.iter_mut().for_each(|s| {
            s.tick(dt);
        });
    }
    fn draw(&self, buffer: &mut Buffer) {
        self.scenery.draw(buffer);
        self.objects.iter().for_each(|s| {
            s.draw(buffer);
        });
        self.actors.iter().for_each(|s| {
            s.draw(buffer);
        });
    }
    fn mouse_state(&mut self, m: &MouseState) {
        self.objects.iter_mut().for_each(|s| {
            s.mouse_state(m);
        });

        self.actors.iter_mut().for_each(|s| {
            s.mouse_state(m);
        });
    }
}
