use crate::geometry::{line_segment, point, LineSegment, Point, Polygon};

#[derive(Default, Debug, PartialEq, Clone)]
pub struct WalkBox {
    pub exterior: Polygon,
    interior: Vec<Polygon>,
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
        Self {
            exterior,
            interior,
            xmin,
            xmax,
            ymin,
            ymax,
        }
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
    pub fn contains(&self, p: Point) -> bool {
        if p.x <= self.xmin || p.x >= self.xmax || p.y <= self.ymin || p.y >= self.ymax {
            return false;
        }
        let ray_start = point(self.xmin - f64::EPSILON, self.ymin - f64::EPSILON);
        let ls = line_segment(ray_start, p);

        let count = self.edges().fold(0, |mut acc, e| {
            if ls.intersects3(&e) {
                acc += 1;
            }
            acc
        });
        (count % 2) == 0
    }
}
