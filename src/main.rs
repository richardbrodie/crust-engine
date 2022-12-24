use buffer::Buffer;
use game_state::GameState;

use tracing::{error, metadata::LevelFilter};
use tracing_subscriber::{
    fmt, prelude::__tracing_subscriber_SubscriberExt, util::SubscriberInitExt, EnvFilter, Layer,
};
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
mod lines;

fn main() {
    // let default_filter = EnvFilter::from_default_env().add_directive(LevelFilter::INFO.into());
    // // let wgpu_filter = fmt::layer().with_filter(EnvFilter::new("wgpu_core::device=error"));
    // let self_filter = fmt::layer().with_filter(EnvFilter::new("adventure_game_engine=info"));
    // tracing_subscriber::registry()
    //     // .with(wgpu_filter)
    //     .with(default_filter)
    //     .with(self_filter)
    //     .init();
    tracing_subscriber::fmt().init();

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
        control_flow.set_poll();
        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::Resized(size) => {
                    if let Err(err) = buffer.resize(size) {
                        error!("pixels.resize_surface() failed: {err}");
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
            _ => {
                if game_state.exit_requested {
                    *control_flow = ControlFlow::Exit;
                    return;
                }
                game_state.tick(&mut buffer);
                window.request_redraw();
            }
        }
    })
}
