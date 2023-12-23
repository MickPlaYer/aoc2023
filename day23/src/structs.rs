use std::collections::{HashMap, HashSet};

type Point = (usize, usize);

#[derive(Debug)]
pub struct Map {
    data: Vec<String>,
}

impl Map {
    pub(crate) fn new(data: Vec<String>) -> Self {
        Self { data }
    }

    pub(crate) fn find_paths(
        &self,
    ) -> (usize, HashMap<usize, usize>, HashMap<usize, HashSet<usize>>) {
        let exit_y = self.data.len() - 1;
        let start_point = self.get_start_point();
        let mut connections = HashMap::new();
        let mut paths = HashMap::new();
        let mut done = HashMap::new();
        let mut last_path_id = 0;
        let mut todo = vec![(last_path_id, 0, start_point)];
        let mut end_path_id = None;
        while let Some((path_id, count, point)) = todo.pop() {
            if done.contains_key(&point) {
                continue;
            }
            if point.1 == exit_y {
                end_path_id = Some(path_id);
                paths.insert(path_id, count + 1);
                done.insert(point, path_id);
                continue;
            }
            let mut is_slop = false;
            if let Some(tile) = self.get(&point) {
                if matches!(tile, '^' | 'v' | '<' | '>') {
                    let (x, y) = point;
                    let next_point = match &tile {
                        '^' => (x, y - 1),
                        'v' => (x, y + 1),
                        '<' => (x - 1, y),
                        '>' => (x + 1, y),
                        _ => panic!(),
                    };
                    paths.insert(path_id, count);
                    let next_path_id = get_next_path_id(&done, next_point, &mut last_path_id);
                    let list = connections.entry(path_id).or_insert(HashSet::new());
                    list.insert(next_path_id);
                    todo.push((next_path_id, 0, next_point));
                    is_slop = true;
                }
            }
            if !is_slop {
                for (dir, next_point) in get_next_points(&point) {
                    if let Some(tile) = self.get(&next_point) {
                        if tile == '.' || dir == tile {
                            todo.push((path_id, count + 1, next_point))
                        }
                    }
                }
            }
            done.insert(point, path_id);
        }
        (end_path_id.unwrap(), paths, connections)
    }

    fn get_start_point(&self) -> Point {
        let start_y = 0;
        let start_x = self.data[0].chars().position(|e| e == '.').unwrap();
        let start_point = (start_x, start_y);
        start_point
    }

    fn get(&self, point: &Point) -> Option<char> {
        let (x, y) = point;
        let row = self.data.get(*y)?;
        row.chars().nth(*x)
    }
}

fn get_next_path_id(
    done: &HashMap<Point, usize>,
    next_point: Point,
    last_path_id: &mut usize,
) -> usize {
    if let Some(id) = done.get(&next_point) {
        *id
    } else {
        *last_path_id += 1;
        *last_path_id
    }
}

fn get_next_points(point: &Point) -> Vec<(char, Point)> {
    let (x, y) = point;
    let (x, y) = (*x, *y);
    let mut points = Vec::new();
    if x > 0 {
        points.push(('<', (x - 1, y)));
    }
    points.push(('>', (x + 1, y)));
    if y > 0 {
        points.push(('^', (x, y - 1)));
    }
    points.push(('v', (x, y + 1)));
    points
}
