use std::cmp;

use crate::geometry::{LineSegment, LineType, Point, Rect};
use crate::image::Bitmap;
use crate::{error::Error, geometry::rect};
use pixels::{Pixels, SurfaceTexture};
use winit::{dpi::PhysicalSize, window::Window};

const POINT_COLOUR: [u8; 4] = [155, 255, 055, 255];

#[derive(Debug)]
pub struct Buffer {
    data: Pixels,
    size: Rect,
}
impl Buffer {
    pub fn new(window: &Window) -> Self {
        let (w, h) = (640, 465);
        let surface_texture = SurfaceTexture::new(w, h, &window);
        let pixels = Pixels::new(w, h, surface_texture).unwrap();
        Self {
            data: pixels,
            size: rect(w as usize, h as usize),
        }
    }
    pub fn draw_raw_slice(&mut self, data: &[u8]) {
        let buffer = self.data.frame_mut();
        buffer.copy_from_slice(data)
    }
    pub fn draw_bmp<T: Into<Point>>(&mut self, bmp: &Bitmap, pos: T) {
        let pos = pos.into();
        let buffer = self.data.frame_mut();

        // clipping
        let (size_w, size_h) = self.size.wh();
        let (pos_x, pos_y) = (pos.x as usize, pos.y as usize);
        let rows = cmp::min(bmp.rows(), size_h - pos_y);
        let cols = cmp::min(bmp.cols(), size_w - pos_x);

        // draw
        for rownum in 0..rows {
            let lstart = (pos_y + rownum) * (size_w * 4) + (pos_x * 4);
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
    pub fn draw_line(&mut self, l: &LineSegment, t: LineType) {
        let buffer = self.data.frame_mut();
        let points = l.points();
        for p in points {
            let (px, py) = p.xy();
            let pidx = py * (self.size.wh().0 * 4) + (px * 4);
            buffer[pidx..pidx + 4].copy_from_slice(t.colour());
        }
    }
    pub fn draw_point(&mut self, p: Point) {
        let buffer = self.data.frame_mut();
        let (px, py) = p.xy();
        for x in cmp::max(px, 2) - 2..px + 2 {
            for y in cmp::max(py, 2) - 2..py + 1 {
                let pidx = y * (self.size.wh().0 * 4) + x * 4;
                buffer[pidx..pidx + 4].copy_from_slice(&POINT_COLOUR);
            }
        }
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
