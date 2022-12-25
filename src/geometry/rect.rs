use winit::dpi::PhysicalSize;

#[derive(Default, Debug, PartialEq, Clone, Copy)]
pub struct Rect {
    pub w: f64,
    pub h: f64,
}
impl Rect {
    pub fn wh(&self) -> (usize, usize) {
        (self.w as usize, self.h as usize)
    }
}

pub fn rect(w: f64, h: f64) -> Rect {
    Rect { w, h }
}

impl From<PhysicalSize<u32>> for Rect {
    fn from(p: PhysicalSize<u32>) -> Self {
        Self {
            w: p.width as f64,
            h: p.height as f64,
        }
    }
}
impl From<(usize, usize)> for Rect {
    fn from(p: (usize, usize)) -> Self {
        Self {
            w: p.0 as f64,
            h: p.1 as f64,
        }
    }
}
