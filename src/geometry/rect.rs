use bdf::BoundingBox;
use winit::dpi::PhysicalSize;

#[derive(Default, Debug, PartialEq, Clone, Copy)]
pub struct Rect {
    pub w: usize,
    pub h: usize,
}
impl Rect {
    pub fn wh(&self) -> (usize, usize) {
        (self.w as usize, self.h as usize)
    }
}

pub fn rect(w: usize, h: usize) -> Rect {
    Rect { w, h }
}

impl From<PhysicalSize<u32>> for Rect {
    fn from(p: PhysicalSize<u32>) -> Self {
        Self {
            w: p.width as usize,
            h: p.height as usize,
        }
    }
}
impl From<(usize, usize)> for Rect {
    fn from(p: (usize, usize)) -> Self {
        Self {
            w: p.0 as usize,
            h: p.1 as usize,
        }
    }
}
impl From<BoundingBox> for Rect {
    fn from(b: BoundingBox) -> Self {
        Self {
            w: b.width as usize,
            h: b.height as usize,
        }
    }
}
