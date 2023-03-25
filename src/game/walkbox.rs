use crate::geometry::{line_segment, point, LineSegment, Point, Polygon};

use super::pathfinding::{EdgeIterator, Graph};

#[derive(Default, Debug, PartialEq, Clone)]
pub struct WalkBox {
    pub edges: Vec<LineSegment>,
    concave_vertices: Vec<Point>,
    graph_edges: Vec<LineSegment>,
    temp_edges: Vec<LineSegment>,
    xmin: f64,
    xmax: f64,
    ymin: f64,
    ymax: f64,
}
impl WalkBox {
    pub fn new(exterior: Polygon, interior: Vec<Polygon>) -> Self {
        let mut xmin = f64::MAX;
        let mut xmax = 0.0;
        let mut ymin = f64::MAX;
        let mut ymax = 0.0;
        for v in &exterior.vertices {
            if v.x > xmax {
                xmax = v.x
            } else if v.x < xmin {
                xmin = v.x
            }
            if v.y > ymax {
                ymax = v.y
            } else if v.y < ymin {
                ymin = v.y
            }
        }
        let edges: Vec<_> = exterior
            .edges()
            .chain(interior.iter().flat_map(|elem| elem.edges()))
            .collect();
        let concave_vertices: Vec<_> = exterior
            .concave_vertices()
            .chain(interior.iter().flat_map(|elem| elem.convex_vertices()))
            .collect();
        let graph_edges = graph(&edges, &concave_vertices);

        Self {
            edges,
            concave_vertices,
            graph_edges,
            temp_edges: vec![],
            xmin,
            xmax,
            ymin,
            ymax,
        }
    }
    pub fn contains(&self, p: Point) -> bool {
        if p.x <= self.xmin || p.x >= self.xmax || p.y <= self.ymin || p.y >= self.ymax {
            return false;
        }
        let ray_start = point(self.xmin - f64::EPSILON, self.ymin - f64::EPSILON);
        let ls = line_segment(ray_start, p);

        let count = self.edges.iter().fold(0, |mut acc, e| {
            if ls.intersects3(&e) {
                acc += 1;
            }
            acc
        });
        (count % 2) == 0
    }
    pub fn add_temporary_edges(&mut self, location: Point, pointer: Point) {
        self.temp_edges.clear();
        if let Some(l) = self.add_edge(location, pointer) {
            self.temp_edges.push(l);
        }
        for end in self.concave_vertices.iter() {
            if let Some(l) = self.add_edge(location, *end) {
                self.temp_edges.push(l);
            }
            if self.contains(pointer) {
                if let Some(l) = self.add_edge(*end, pointer) {
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
        if !self.edges.iter().any(|e| l.crosses(&e)) {
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
    pub fn clamp_destination(&self, pos: Point) -> Point {
        if self.contains(pos) {
            return pos;
        }
        let res = self.edges.iter().map(|side| side.closest_point(pos)).fold(
            (f64::MAX, pos),
            |acc, p| {
                let dist = (p - pos).length();
                if acc.0 > dist {
                    return (dist, p);
                }
                acc
            },
        );
        res.1
    }
}
impl Graph for WalkBox {
    fn walkable_edges(&self) -> EdgeIterator {
        EdgeIterator::new(
            self.graph_edges
                .iter()
                .chain(self.temp_edges.iter())
                .collect(),
        )
    }
    fn neighbours(&self, point: Point) -> EdgeIterator {
        EdgeIterator::new(
            self.walkable_edges()
                .filter(move |l| l.start == point)
                .chain(self.temp_edges.iter().filter(move |l| l.start == point))
                .collect(),
        )
    }
}

fn graph(edges: &[LineSegment], vertices: &[Point]) -> Vec<LineSegment> {
    let mut new_edges: Vec<LineSegment> = vec![];
    vertices.iter().for_each(|i| {
        vertices.iter().for_each(|j| {
            let ls = line_segment(*i, *j);
            if ls.length() >= f64::EPSILON && !edges.iter().any(|e| ls.crosses(&e)) {
                if !new_edges
                    .iter()
                    .any(|e| e.start == ls.start && e.end == ls.end)
                {
                    new_edges.push(ls);
                }
            }
        })
    });

    new_edges
}

#[cfg(test)]
mod tests {
    use crate::{
        game::{Graph, WalkBox},
        geometry::{point, Polygon},
    };
    fn make_graph() -> WalkBox {
        WalkBox::new(
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
        )
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
