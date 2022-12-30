use buffer::Buffer;
use game_state::GameState;

use tracing::error;
use winit::{
    event::{ElementState, Event, KeyboardInput, MouseButton, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Fullscreen, WindowBuilder},
};

mod buffer;
mod components;
mod error;
mod game_state;
mod geometry;
mod image;
mod text;

fn main() {
    // tracing_subscriber::fmt().init();

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
                    if let Err(err) = buffer.resize(size) {
                        error!("pixels.resize_surface() failed: {:?}", err);
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
                WindowEvent::MouseInput {
                    button: MouseButton::Left,
                    state,
                    ..
                } => game_state.mouse_click(state),
                _ => {}
            },
            Event::RedrawRequested(_) => buffer.render(),
            Event::MainEventsCleared => {
                if game_state.exit_requested {
                    *control_flow = ControlFlow::Exit;
                    return;
                }
                game_state.tick(&mut buffer);
                window.request_redraw();
            }
            _ => {}
        }
    })
}
