use super::lines::LineString;

#[derive(Default, Debug, PartialEq, Clone)]
pub struct Polygon {
    pub exterior: LineString,
    interior: Vec<LineString>,
}
impl Polygon {
    pub fn new(mut ext: LineString) -> Self {
        ext.close();
        Self {
            exterior: ext,
            interior: vec![],
        }
    }
    pub fn add_interior(&mut self, mut interior: LineString) {
        interior.close();
        self.interior.push(interior);
    }
}
