use crate::game::WalkBox;

use super::{line::line_segment, LineSegment, Point};

#[derive(Default, Debug, PartialEq, Clone)]
pub struct Graph {
    walkbox: WalkBox,
    pub walkable_edges: Vec<LineSegment>,
    pub temp_edges: Vec<LineSegment>,
}
impl Graph {
    pub fn new(wb: WalkBox) -> Self {
        let mut edges: Vec<LineSegment> = vec![];
        wb.concave_vertexes().for_each(|i| {
            wb.concave_vertexes().for_each(|j| {
                let ls = line_segment(i, j);
                if ls.length() >= f64::EPSILON && !wb.intersects(&ls) {
                    if !edges.iter().any(|e| {
                        e.start == ls.start && e.end == ls.end
                            || e.start == ls.end && e.end == ls.start
                    }) {
                        edges.push(ls);
                    }
                }
            })
        });
        Self {
            walkbox: wb,
            walkable_edges: edges,
            temp_edges: vec![],
        }
    }
    pub fn walkable_edges(&self) -> impl Iterator<Item = &LineSegment> + '_ {
        self.walkable_edges.iter().chain(self.temp_edges.iter())
    }
    pub fn add_temporary_edges(&mut self, location: Point, pointer: Point) {
        self.temp_edges.clear();
        if let Some(l) = self.add_edge(location, pointer) {
            self.temp_edges.push(l);
        }
        for end in self.walkbox.concave_vertexes() {
            if let Some(l) = self.add_edge(location, end) {
                self.temp_edges.push(l);
            }
            if let Some(l) = self.add_edge(pointer, end) {
                self.temp_edges.push(l);
            }
        }
    }
    fn add_edge(&self, start: Point, end: Point) -> Option<LineSegment> {
        if !self.walkbox.exterior.contains(start) {
            return None;
        }
        let l = line_segment(start, end);
        if l.length() < f64::EPSILON {
            return None;
        }
        if !self.walkbox.intersects(&l) {
            if !self.temp_edges.iter().any(|e| {
                e.start == l.start && e.end == l.end || e.start == l.end && e.end == l.start
            }) {
                return Some(l);
            }
        }
        return None;
    }
    fn get_neighbours(&self, point: Point) -> impl Iterator<Item = &LineSegment> {
        self.walkable_edges
            .iter()
            .filter(move |l| l.start == point || l.end == point)
            .chain(
                self.temp_edges
                    .iter()
                    .filter(move |l| l.start == point || l.end == point),
            )
    }
}

// #[cfg(test)]
// mod tests {
//
//     #[test]
//     fn test_graph() {}
// }
