use std::cmp;

use crate::error::Error;
use crate::geometry::{Point, Rect};
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
        let size = Rect::new(640, 465);
        let surface_texture = SurfaceTexture::new(size.width as u32, size.height as u32, &window);
        let pixels = Pixels::new(size.width as u32, size.height as u32, surface_texture).unwrap();
        Self { data: pixels, size }
    }
    pub fn draw_slice(&mut self, data: &[u8]) {
        let buffer = self.data.get_frame_mut();
        buffer.copy_from_slice(data)
    }
    pub fn draw_to(&mut self, bmp: &Bitmap, pos: Point) {
        let buffer = self.data.get_frame_mut();

        // clipping
        let rows = cmp::min(bmp.rows(), self.size.height - pos.y as usize);
        let cols = cmp::min(bmp.cols(), self.size.width - pos.x as usize);

        // draw
        for rownum in 0..rows {
            let lstart = (pos.y as usize + rownum) * (self.size.width * 4) + (pos.x as usize * 4);
            for (i, pixel) in bmp.row_partial(rownum, cols).chunks_exact(4).enumerate() {
                let idx = lstart + (i * 4);
                let base = &mut buffer[idx..idx + 4];
                composit_pixel(base, pixel);
            }
        }
    }
    pub fn resize(&mut self, size: PhysicalSize<u32>) -> Result<(), Error> {
        self.data
            .resize_surface(size.width, size.height)
            .map_err(|e| e.into())
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
