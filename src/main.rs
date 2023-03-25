use buffer::Buffer;
use game::GameState;

use winit::{
    event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Fullscreen, WindowBuilder},
};

mod buffer;
mod error;
mod game;
mod geometry;
mod image;
mod text;

fn main() {
    let event_loop = EventLoop::new();
    let monitor = event_loop
        .available_monitors()
        .next()
        .expect("no monitor found!");
    let window = {
        WindowBuilder::new()
            .with_fullscreen(Some(Fullscreen::Borderless(Some(monitor))))
            .build(&event_loop)
            .unwrap()
    };
    let mut buffer = Buffer::new(&window);
    let mut game_state = GameState::new();

    event_loop.run(move |event, _, control_flow| {
        control_flow.set_wait();
        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::Resized(size) => {
                    if let Err(_) = buffer.resize(size) {
                        *control_flow = ControlFlow::Exit;
                        return;
                    }
                }
                WindowEvent::KeyboardInput {
                    input:
                        KeyboardInput {
                            virtual_keycode: Some(VirtualKeyCode::Escape),
                            state: ElementState::Pressed,
                            ..
                        },
                    ..
                } => game_state.exit_requested = true,
                WindowEvent::CursorMoved { position, .. } => {
                    if let Some(pos) = buffer.convert_pos(position) {
                        game_state.mouse_over(pos)
                    }
                }
                WindowEvent::MouseInput { button, state, .. } => {
                    game_state.mouse_click(button, state)
                }
                _ => {}
            },
            Event::RedrawRequested(_) => {
                game_state.draw(&mut buffer);
                buffer.render()
            }
            Event::MainEventsCleared => {
                if game_state.exit_requested {
                    *control_flow = ControlFlow::Exit;
                    return;
                }
                game_state.tick();
                window.request_redraw();
            }
            _ => {}
        }
    })
}
