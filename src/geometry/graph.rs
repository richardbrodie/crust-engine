use crate::game::{astar, ShortestPath, WalkBox};

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
                    if !edges.iter().any(|e| e.start == ls.start && e.end == ls.end) {
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
            if self.walkbox.contains(pointer) {
                if let Some(l) = self.add_edge(end, pointer) {
                    self.temp_edges.push(l);
                }
            }
        }
    }
    fn add_edge(&self, start: Point, end: Point) -> Option<LineSegment> {
        let l = line_segment(start, end);
        if l.length() < f64::EPSILON {
            return None;
        }
        if !self.walkbox.intersects(&l) {
            if !self
                .temp_edges
                .iter()
                .any(|e| e.start == l.start && e.end == l.end)
            {
                return Some(l);
            }
        }
        return None;
    }
    pub fn neighbours(&self, point: Point) -> impl Iterator<Item = &LineSegment> {
        self.walkable_edges
            .iter()
            .filter(move |l| l.start == point)
            .chain(self.temp_edges.iter().filter(move |l| l.start == point))
    }
    pub fn path_to(&self, start: Point, end: Point) -> Option<ShortestPath> {
        astar(&self, start, end)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        game::WalkBox,
        geometry::{point, Polygon},
    };

    use super::Graph;

    fn make_graph() -> Graph {
        Graph::new(WalkBox::new(
            Polygon::new(vec![
                point(60.0, 60.0),
                point(300.0, 60.0),
                point(300.0, 240.0),
                point(360.0, 240.0),
                point(360.0, 60.0),
                point(610.0, 60.0),
                point(610.0, 260.0),
                point(510.0, 260.0),
                point(510.0, 280.0),
                point(610.0, 280.0),
                point(610.0, 435.0),
                point(60.0, 435.0),
                point(60.0, 60.0),
            ]),
            vec![],
        ))
    }

    #[test]
    fn test_walkable_edges() {
        let graph = make_graph();
        let mut we = graph.walkable_edges();
        assert!(we.next().is_some());
    }

    #[test]
    fn test_temp_edges() {
        let mut graph = make_graph();
        let start = point(75.0, 75.0);
        let end = point(570.0, 80.0);
        graph.add_temporary_edges(start, end);
        assert_eq!(graph.temp_edges.len(), 3);
    }

    #[test]
    fn test_neighbours() {
        let mut graph = make_graph();
        let start = point(75.0, 75.0);
        let end = point(570.0, 80.0);
        graph.add_temporary_edges(start, end);
        let mut n = graph.neighbours(start);
        assert!(n.next().is_some());
        let mut n = graph.neighbours(end);
        assert!(n.next().is_none());
    }
}
