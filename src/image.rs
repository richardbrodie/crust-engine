use std::{fs::File, path::Path, time::Duration};

use crate::{
    geometry::{point, rect, Point, Rect, Vector},
    Buffer,
};

// simple image buffer: pixels and size
#[derive(Default, Debug, PartialEq)]
pub struct Bitmap {
    data: Vec<u8>,
    size: Rect,
}
impl Bitmap {
    pub fn new(buf: Vec<u8>, size: Rect) -> Self {
        Self { data: buf, size }
    }
    pub fn cols(&self) -> usize {
        let (w, _) = self.size.wh();
        w
    }
    pub fn rows(&self) -> usize {
        let (_, h) = self.size.wh();
        h
    }

    pub fn row_partial(&self, rownum: usize, len: usize) -> &[u8] {
        let full_row = self.cols() * 4;
        let a = rownum * full_row;
        let b = a + (len * 4);
        &self.data[a..b]
    }
    pub fn data(&self) -> &[u8] {
        &self.data
    }
}

#[derive(Default, Debug, PartialEq)]
struct Frame {
    data: Bitmap,
    offset: Point,
    interval: Duration,
}
impl Frame {
    fn new(reader: &mut png::Reader<File>) -> Self {
        let mut buf = vec![0; reader.output_buffer_size()];
        let frame = reader.next_frame(&mut buf).unwrap();
        buf.truncate(frame.buffer_size());
        let fc = &reader.info().frame_control().unwrap();
        let interval = Duration::from_secs_f64(fc.delay_num as f64 / fc.delay_den as f64);
        let size = rect(fc.width as usize, fc.height as usize);
        Frame {
            data: Bitmap::new(buf, size),
            offset: point(fc.x_offset as f64, fc.y_offset as f64),
            interval,
        }
    }
    pub fn offset(&self, p: Point) -> Vector {
        p + self.offset
    }
    pub fn data(&self) -> &Bitmap {
        &self.data
    }
}

#[derive(Default, Debug, PartialEq)]
pub struct AnimatedImage {
    data: Vec<Frame>,
    size: Rect,
    current_frame_idx: usize,
    last_frame_change: Duration,
}
impl AnimatedImage {
    fn new(frames: Vec<Frame>, size: Rect) -> Self {
        Self {
            data: frames,
            size,
            current_frame_idx: 0,
            last_frame_change: Duration::default(),
        }
    }
    pub fn update(&mut self, dt: Duration) {
        self.last_frame_change += dt;
        let f = self.current_frame().interval;
        if self.last_frame_change >= f {
            self.last_frame_change -= f;
            if self.current_frame_idx == self.data.len() - 1 {
                self.current_frame_idx = 0;
            } else {
                self.current_frame_idx += 1;
            }
        }
    }
    fn current_frame(&self) -> &Frame {
        &self.data[self.current_frame_idx]
    }
}

#[derive(Default, Debug, PartialEq)]
pub struct StaticImage {
    data: Bitmap,
}
impl StaticImage {
    fn new(data: Bitmap) -> Self {
        Self { data }
    }
    pub fn data(&self) -> &Bitmap {
        &self.data
    }
}

#[derive(Debug, PartialEq)]
pub enum Image {
    Animated(AnimatedImage),
    Static(StaticImage),
}
impl Image {
    pub fn load<T: AsRef<Path>>(path: T) -> Self {
        let image_file = File::open(path).unwrap();
        let decoder = png::Decoder::new(image_file);
        let mut reader = decoder.read_info().unwrap();
        let img_info = reader.info();
        let size = rect(img_info.width as usize, img_info.height as usize);
        if reader.info().is_animated() {
            let frames: Vec<_> = (0..img_info.animation_control().unwrap().num_frames)
                .map(|_| Frame::new(&mut reader))
                .collect();
            Self::Animated(AnimatedImage::new(frames, size))
        } else {
            let mut buf = vec![0; reader.output_buffer_size()];
            reader.next_frame(&mut buf).unwrap();
            let bmp = Bitmap::new(buf, size);
            Self::Static(StaticImage::new(bmp))
        }
    }
    pub fn draw(&self, buf: &mut Buffer, p: Point) {
        match self {
            Self::Static(i) => buf.draw_bmp(i.data(), p),
            Self::Animated(a) => {
                let f = a.current_frame();
                let p = f.offset(p);
                buf.draw_bmp(f.data(), p);
            }
        }
    }
    pub fn size(&self) -> Rect {
        match self {
            Self::Static(i) => i.data.size,
            Self::Animated(i) => i.size,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use crate::image::Image;

    #[test]
    fn test_load_static_image() {
        let image = Image::load("resources/fox.png");
        assert!(matches!(image, Image::Static(_)));
    }

    #[test]
    fn test_load_animated_image() {
        let image = Image::load("resources/ball.png");
        assert!(matches!(image, Image::Animated(_)));
    }

    #[test]
    fn test_animated_image_frame() {
        if let Image::Animated(mut image) = Image::load("resources/ball.png") {
            let dt = Duration::from_secs_f64(1.0);
            image.update(dt);

            assert_eq!(image.current_frame_idx, 1);
            assert_eq!(image.last_frame_change, Duration::from_millis(925));
        }
    }
}
