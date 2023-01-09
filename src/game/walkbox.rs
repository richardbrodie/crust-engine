use crate::geometry::{LineSegment, Point, Polygon};

#[derive(Default, Debug, PartialEq, Clone)]
pub struct WalkBox {
    pub exterior: Polygon,
    interior: Vec<Polygon>,
}
impl WalkBox {
    pub fn new(exterior: Polygon, interior: Vec<Polygon>) -> Self {
        Self { exterior, interior }
    }
    pub fn concave_vertexes(&self) -> impl Iterator<Item = Point> + '_ {
        let internal_vertices = self.exterior.concave_vertices();
        internal_vertices.chain(self.interior.iter().flat_map(|elem| elem.convex_vertices()))
    }
    pub fn edges(&self) -> impl Iterator<Item = LineSegment> + '_ {
        let internal_edges = self.exterior.edges();
        internal_edges.chain(self.interior.iter().flat_map(|elem| elem.edges()))
    }
    pub fn intersects(&self, ls: &LineSegment) -> bool {
        self.edges().any(|e| ls.crosses(&e))
    }
    pub fn inside(&self, p: Point) -> bool {
        // if (polygon.Count < 3) return false;
        false
    }
}
