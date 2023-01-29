use std::collections::{BinaryHeap, HashMap};

use crate::geometry::{line_segment, LineSegment, Point};

pub struct EdgeIterator<'a> {
    data: Vec<&'a LineSegment>,
    i: usize,
}
impl<'a> EdgeIterator<'a> {
    pub fn new(data: Vec<&'a LineSegment>) -> Self {
        Self { data, i: 0 }
    }
}
pub trait Graph {
    fn walkable_edges(&self) -> EdgeIterator;
    fn neighbours(&self, point: Point) -> EdgeIterator;
}
impl<'a> Iterator for EdgeIterator<'a> {
    type Item = &'a LineSegment;
    fn next(&mut self) -> Option<Self::Item> {
        if self.i >= self.data.len() {
            None
        } else {
            self.i += 1;
            Some(&self.data[self.i - 1])
        }
    }
}

#[derive(Default, Debug, PartialEq, Clone, PartialOrd, Ord, Hash, Eq, Copy)]
struct UPoint {
    x: usize,
    y: usize,
}
impl From<Point> for UPoint {
    fn from(value: Point) -> Self {
        Self {
            x: value.x as usize,
            y: value.y as usize,
        }
    }
}
impl From<UPoint> for Point {
    fn from(value: UPoint) -> Self {
        Self {
            x: value.x as f64,
            y: value.y as f64,
        }
    }
}

#[derive(Default, Debug, PartialEq, Clone)]
struct State {
    position: UPoint,
    cost: f64,
}
impl Eq for State {}
impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .cost
            .partial_cmp(&self.cost)
            .unwrap()
            .then_with(|| self.position.cmp(&other.position))
    }
}
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(&other))
    }
}

pub fn astar<G>(graph: &G, start: Point, goal: Point) -> Option<ShortestPath>
where
    G: Graph,
{
    let mut frontier = BinaryHeap::new();
    let mut distances: HashMap<UPoint, f64> = graph
        .walkable_edges()
        .map(|e| (e.end.into(), f64::MAX))
        .collect();
    let mut path: HashMap<UPoint, UPoint> = HashMap::new();

    frontier.push(State {
        position: start.into(),
        cost: 0.0,
    });
    distances.insert(start.into(), 0.0);

    while let Some(State {
        position: current_position,
        cost: current_cost,
    }) = frontier.pop()
    {
        if current_position == goal.into() {
            return Some(ShortestPath::new(path, start.into(), goal.into()));
        }
        for edge in graph.neighbours(current_position.into()) {
            let next_position: UPoint = edge.end.into();
            let next_cost = edge.length() + current_cost;

            if next_cost < distances[&next_position] {
                distances.insert(next_position, next_cost);
                let priority = next_cost + (goal - Point::from(current_position)).length();
                frontier.push(State {
                    position: next_position,
                    cost: priority,
                });
                path.insert(next_position, current_position);
            }
        }
    }

    return None;
}

#[derive(Default, Debug, PartialEq, Clone, PartialOrd, Ord, Eq)]
pub struct ShortestPath {
    start: Point,
    end: Point,
    points: Vec<Point>,
}
impl ShortestPath {
    fn new(paths: HashMap<UPoint, UPoint>, start: UPoint, goal: UPoint) -> Self {
        Self {
            start: start.into(),
            end: goal.into(),
            points: assemble_path(paths, start, goal),
        }
    }
    pub fn points(&self) -> impl Iterator<Item = &Point> {
        self.points.iter()
    }
    pub fn lines(&self) -> impl Iterator<Item = LineSegment> + '_ {
        self.points.windows(2).map(|w| line_segment(w[0], w[1]))
    }
}
fn assemble_path(paths: HashMap<UPoint, UPoint>, start: UPoint, goal: UPoint) -> Vec<Point> {
    let mut path = vec![];
    let mut curr = goal;
    while curr != start {
        path.insert(0, curr);
        curr = paths[&curr];
    }
    path.insert(0, start);

    path.iter().map(|e| Point::from(*e)).collect()
}

#[cfg(test)]
mod tests {
    use crate::{
        game::{astar, WalkBox},
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
    fn test_astar() {
        let mut graph = make_graph();

        let start = point(150.0, 150.0);
        let end = point(570.0, 120.0);
        graph.add_temporary_edges(start, end);
        let path = astar(&graph, start, end);
        assert!(path.is_some());

        let start = point(600.0, 290.0);
        let end = point(600.0, 190.0);
        graph.add_temporary_edges(start, end);
        let path = astar(&graph, start, end);
        assert!(path.is_some());
    }
}
