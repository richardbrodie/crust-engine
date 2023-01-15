use std::iter;

use super::{line_segment, LineSegment, Point};

#[derive(Default, Debug, PartialEq, Clone)]
pub struct Polygon {
    pub vertices: Vec<Point>,
}
impl Polygon {
    pub fn new(vertices: Vec<Point>) -> Self {
        Self { vertices }
    }
    pub fn edges(&self) -> impl Iterator<Item = LineSegment> + '_ {
        let l = line_segment(self.vertices[self.vertices.len() - 1], self.vertices[0]);
        self.vertices
            .windows(2)
            .map(|w| line_segment(w[0], w[1]))
            .chain(iter::once(l))
    }
    pub fn concave_vertices(&self) -> impl Iterator<Item = Point> + '_ {
        let v = self.vertices.windows(3).filter_map(|pp| {
            if Point::is_convex(pp[0], pp[1], pp[2]) {
                Some(pp[1])
            } else {
                None
            }
        });
        let o = iter::once([
            self.vertices[self.vertices.len() - 1],
            self.vertices[0],
            self.vertices[1],
        ])
        .filter_map(|pp| {
            if Point::is_convex(pp[0], pp[1], pp[2]) {
                Some(self.vertices[0])
            } else {
                None
            }
        });
        v.chain(o)
    }
    pub fn convex_vertices(&self) -> impl Iterator<Item = Point> + '_ {
        self.vertices.windows(3).filter_map(|pp| {
            if !Point::is_convex(pp[0], pp[1], pp[2]) {
                Some(pp[1])
            } else {
                None
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::geometry::{point, Point, Polygon};

    #[test]
    fn test_polygon_edges() {
        let v = vec![
            point(5.0, 5.0),
            point(6.0, 6.0),
            point(7.0, 7.0),
            point(9.0, 15.0),
            point(13.0, 25.0),
        ];
        let ls = Polygon::new(v);
        let mut ll = ls.edges();

        assert_eq!(ll.next().unwrap().end, point(6.0, 6.0));
        assert_eq!(ll.next().unwrap().end, point(7.0, 7.0));
        ll.next();
        ll.next();
        ll.next();
        let n = ll.next();
        assert!(n.is_none());
    }

    #[test]
    fn test_polygon_vertices() {
        let cv1 = point(25.0, 25.0);
        let cv2 = point(35.0, 35.0);
        let ls = Polygon::new(vec![
            point(5.0, 5.0),
            point(50.0, 5.0),
            cv1,
            point(50.0, 50.0),
            cv2,
            point(5.0, 50.0),
        ]);
        let v = ls.concave_vertices();

        assert_eq!(v.collect::<Vec<Point>>(), vec![cv1, cv2]);

        let cv1 = point(3.0, 3.0);
        let ls = Polygon::new(vec![
            cv1,
            point(3.0, 1.0),
            point(10.0, 1.0),
            point(10.0, 10.0),
            point(1.0, 10.0),
            point(1.0, 3.0),
        ]);
        let v = ls.concave_vertices();
        assert_eq!(v.collect::<Vec<Point>>(), vec![cv1]);
    }
}
