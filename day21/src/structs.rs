use std::collections::{HashMap, HashSet, VecDeque};

pub type Point = (usize, usize);

#[derive(Debug)]
pub struct Map {
    start_point: Point,
    data: Vec<String>,
}

impl Map {
    pub(crate) fn new(start_point: Point, data: Vec<String>) -> Self {
        Self { start_point, data }
    }

    pub(crate) fn find_points(&self, start_point: Option<Point>) -> HashMap<Point, usize> {
        let mut distances = HashMap::new();
        let mut done = HashSet::new();
        let start_point = if let Some(point) = start_point {
            point
        } else {
            self.start_point
        };
        distances.insert(start_point, 0);
        let mut todo = VecDeque::from(vec![(0, start_point)]);
        while let Some((step_count, point)) = todo.pop_front() {
            if done.contains(&point) {
                continue;
            }
            for (step_count, point) in next_steps(point, step_count) {
                if matches!(self.get(&point), Some('.' | 'S')) {
                    let original_distances = distances.get(&point);
                    let new_score = if let Some(origin_distance) = original_distances {
                        *origin_distance > step_count
                    } else {
                        true
                    };
                    if new_score {
                        distances.insert(point, step_count);
                        todo.push_back((step_count, point));
                    }
                }
            }
            todo.as_mut_slices().0.sort_by_key(|e| e.0);
            done.insert(point);
        }
        distances
    }

    pub(crate) fn get(&self, point: &Point) -> Option<char> {
        let (x, y) = point;
        let row = self.data.get(*y)?;
        row.chars().nth(*x)
    }

    pub(crate) fn get_size(&self) -> (usize, usize) {
        (self.data[0].len(), self.data.len())
    }

    pub(crate) fn make_bigger(&self, n: usize) -> Self {
        let (width, height) = self.get_size();
        let mut rows = Vec::new();
        for row in &self.data {
            rows.push(row.repeat(n));
        }
        let mut data = Vec::new();
        for _ in 0..n {
            for row in &rows {
                data.push(row.clone());
            }
        }
        let start_point = ((width * n - 1) / 2, (height * n - 1) / 2);
        Self { start_point, data }
    }
}

fn next_steps(point: Point, step_count: usize) -> Vec<(usize, Point)> {
    let mut points = Vec::new();
    let (x, y) = point;
    let next_step_count = step_count + 1;
    points.push((next_step_count, (x + 1, y)));
    if x > 0 {
        points.push((next_step_count, (x - 1, y)));
    }
    points.push((next_step_count, (x, y + 1)));
    if y > 0 {
        points.push((next_step_count, (x, y - 1)));
    }
    points
}
