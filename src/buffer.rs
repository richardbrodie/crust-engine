use std::cmp;

use crate::geometry::{Point, Rect, Vec2};
use crate::image::Bitmap;
use pixels::{Pixels, SurfaceTexture};
use winit::{dpi::PhysicalSize, window::Window};

#[derive(Debug)]
pub struct Buffer {
    data: Pixels,
    size: Rect,
}
impl Buffer {
    pub fn new(window: &Window) -> Self {
        // let size: Rect = window.inner_size().into();
        let size: Rect = Vec2(640, 465);
        let surface_texture = SurfaceTexture::new(size.0 as u32, size.1 as u32, &window);
        let pixels = Pixels::new(
            // window_size.width as u32,
            // window_size.height as u32,
            // 640,
            // 465,
            size.0 as u32,
            size.1 as u32,
            surface_texture,
        )
        .unwrap();
        Self { data: pixels, size }
    }
    pub fn draw_slice(&mut self, data: &[u8]) {
        let buffer = self.data.get_frame_mut();
        buffer.copy_from_slice(data)
    }
    pub fn draw_to(&mut self, bmp: &Bitmap, pos: Point) {
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
        self.data.resize_surface(size.width, size.height).unwrap();
    }
    pub fn render(&self) {
        self.data.render().unwrap();
    }
    pub fn convert_pos<T: Into<(f32, f32)>>(&self, pos: T) -> Option<Point> {
        self.data
            .window_pos_to_pixel(pos.into())
            .ok()
            .map(|p| p.into())
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

#[cfg(test)]
mod tests {
    use crate::buffer::composit_pixel;

    #[test]
    fn test_composit() {
        let mut bg = [128, 64, 32, 255];
        let obj = [255, 128, 64, 128];
        let res = [191, 96, 48, 255];
        composit_pixel(&mut bg, &obj);
        assert_eq!(bg, res);
    }
}
