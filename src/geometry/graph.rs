use super::{
    line::{self, lineseg},
    LineSegment, Point,
};

struct Graph {
    edges: Vec<LineSegment>,
}
impl Graph {
    fn add_edge(&mut self, start: Point, end: Point) {
        let l = lineseg(start, end);
        self.edges.push(l);
    }
}
