use std::{
    cmp,
    time::{Duration, Instant},
};

use game::Sprite;
use geometry::{Point, Rect, Vec2};
use image::{Bitmap, Image};
use pixels::{Pixels, SurfaceTexture};
use winit::{
    dpi::PhysicalSize,
    event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Fullscreen, Window, WindowBuilder},
};

mod game;
mod geometry;
mod image;

const TICK: Duration = Duration::from_millis(1000 / 60);

#[derive(Debug)]
pub struct Buffer {
    data: Pixels,
    size: Rect,
}
impl Buffer {
    fn new(window: &Window) -> Self {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        let pixels = Pixels::new(
            window_size.width as u32,
            window_size.height as u32,
            surface_texture,
        )
        .unwrap();
        Self {
            data: pixels,
            size: Vec2(window_size.width as usize, window_size.height as usize),
        }
    }
    fn draw_to(&mut self, bmp: &Bitmap, pos: Point) {
        let buffer = self.data.get_frame_mut();
        // buffer
        //     .chunks_exact_mut(4)
        //     .for_each(|c| c.copy_from_slice(&[0, 0, 0, 255]));

        // clipping
        let rows = cmp::min(bmp.rows(), self.size.1 - pos.1 as usize);
        let cols = cmp::min(bmp.cols(), self.size.0 - pos.0 as usize);

        // draw
        for rownum in 0..rows {
            let lstart = (pos.1 as usize + rownum) * (self.size.0 * 4) + (pos.0 as usize * 4);
            for (i, pixel) in bmp.row_partial(rownum, cols).chunks_exact(4).enumerate() {
                let idx = lstart + (i * 4);
                let base = &mut buffer[idx..idx + 4];
                composit_pixel(base, pixel);
            }
        }
    }
    pub fn resize(&mut self, size: PhysicalSize<u32>) {
        self.data.resize_surface(size.width, size.height);
    }
    pub fn render(&self) {
        self.data.render().unwrap();
    }
}
#[inline(always)]
fn composit_pixel(dest: &mut [u8], src: &[u8]) {
    let sa = src[3] as f64 / 255.0;
    for c in 0..3 {
        let x = (src[c] as f64 / 255.0) * sa + (dest[c] as f64 / 255.0) * (1.0 - sa);
        dest[c] = (x * 255.0) as u8
    }
    let ra = sa + (dest[3] as f64 / 255.0) * (1.0 - sa);
    dest[3] = (ra * 255.0) as u8;
}

fn main() {
    // tracing_subscriber::fmt().init();

    let event_loop = EventLoop::new();
    let monitor = event_loop
        .available_monitors()
        .next()
        .expect("no monitor found!");
    let window = {
        WindowBuilder::new()
            .with_fullscreen(Some(Fullscreen::Borderless(Some(monitor.clone()))))
            .build(&event_loop)
            .unwrap()
    };
    let mut buffer = Buffer::new(&window);
    let image = Image::load("resources/fox.png");
    // let image = Image::load("resources/ball.png");
    let mut sprite = Sprite::new(image);

    let mut exit_requested = false;
    let mut previous_time = Instant::now();

    event_loop.run(move |event, _, control_flow| {
        control_flow.set_poll();
        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::Resized(size) => buffer.resize(size),
                WindowEvent::KeyboardInput {
                    input:
                        KeyboardInput {
                            virtual_keycode: Some(VirtualKeyCode::Escape),
                            state: ElementState::Pressed,
                            ..
                        },
                    ..
                } => exit_requested = true,

                _ => {}
            },
            Event::RedrawRequested(_) => {
                buffer.render();
            }
            _ => {
                if exit_requested {
                    *control_flow = ControlFlow::Exit;
                    return;
                }
                // let delta = std::cmp::min(previous_time.elapsed(), TICK);
                let delta = previous_time.elapsed();
                if delta >= TICK {
                    previous_time = Instant::now();
                    sprite.update(delta);
                    sprite.draw(&mut buffer);
                    window.request_redraw();
                } else {
                    *control_flow = ControlFlow::WaitUntil(Instant::now() + TICK - delta);
                }
            }
        }
    })
}
