use std::collections::{HashMap, HashSet};

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
enum Dir {
    None,
    L,
    R,
    U,
    D,
}

type Cord = (usize, usize);

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
struct Step {
    cord: Cord,
    dir: Dir,
    momentum: u8,
}

impl Step {
    pub(crate) fn new(cord: Cord, dir: Dir, momentum: u8) -> Self {
        Self {
            cord,
            dir,
            momentum,
        }
    }

    pub(crate) fn init(cord: Cord) -> Self {
        Self {
            cord,
            dir: Dir::None,
            momentum: 0,
        }
    }

    fn get_ultra_next_step(&self, map: &Map) -> Vec<Step> {
        let (x, y) = self.cord;
        let (width, height) = (map.data[0].len(), map.data.len());
        let mut next_steps = Vec::new();
        if x > 0 && !matches!(self.dir, Dir::R) {
            if matches!(self.dir, Dir::L) {
                let momentum = self.momentum + 1;
                if momentum <= 10 {
                    next_steps.push(Self::new(((x - 1), y), Dir::L, momentum));
                }
            } else if x >= 4 {
                next_steps.push(Self::new(((x - 4), y), Dir::L, 4));
            };
        }
        if x < (width - 1) && !matches!(self.dir, Dir::L) {
            if matches!(self.dir, Dir::R) {
                let momentum = self.momentum + 1;
                if momentum <= 10 {
                    next_steps.push(Self::new(((x + 1), y), Dir::R, momentum));
                }
            } else if x <= (width - 1 - 4) {
                next_steps.push(Self::new(((x + 4), y), Dir::R, 4));
            };
        }
        if y > 0 && !matches!(self.dir, Dir::D) {
            if matches!(self.dir, Dir::U) {
                let momentum = self.momentum + 1;
                if momentum <= 10 {
                    next_steps.push(Self::new((x, (y - 1)), Dir::U, momentum));
                }
            } else if y >= 4 {
                next_steps.push(Self::new((x, (y - 4)), Dir::U, 4));
            };
        }
        if y < (height - 1) && !matches!(self.dir, Dir::U) {
            if matches!(self.dir, Dir::D) {
                let momentum = self.momentum + 1;
                if momentum <= 10 {
                    next_steps.push(Self::new((x, (y + 1)), Dir::D, momentum));
                }
            } else if y <= (height - 1 - 4) {
                next_steps.push(Self::new((x, (y + 4)), Dir::D, 4));
            };
        }
        next_steps
    }

    fn get_next_steps(&self, map_size: (usize, usize)) -> Vec<Step> {
        let (x, y) = self.cord;
        let (width, height) = map_size;
        let mut next_steps = Vec::new();
        if x > 0 && !matches!(self.dir, Dir::R) {
            let momentum = if matches!(self.dir, Dir::L) {
                self.momentum + 1
            } else {
                1
            };
            if momentum <= 3 {
                next_steps.push(Self::new(((x - 1), y), Dir::L, momentum));
            }
        }
        if x < (width - 1) && !matches!(self.dir, Dir::L) {
            let momentum = if matches!(self.dir, Dir::R) {
                self.momentum + 1
            } else {
                1
            };
            if momentum <= 3 {
                next_steps.push(Self::new(((x + 1), y), Dir::R, momentum));
            }
        }
        if y > 0 && !matches!(self.dir, Dir::D) {
            let momentum = if matches!(self.dir, Dir::U) {
                self.momentum + 1
            } else {
                1
            };
            if momentum <= 3 {
                next_steps.push(Self::new((x, (y - 1)), Dir::U, momentum));
            }
        }
        if y < (height - 1) && !matches!(self.dir, Dir::U) {
            let momentum = if matches!(self.dir, Dir::D) {
                self.momentum + 1
            } else {
                1
            };
            if momentum <= 3 {
                next_steps.push(Self::new((x, (y + 1)), Dir::D, momentum));
            }
        }
        next_steps
    }
}

pub struct Map {
    data: Vec<String>,
}

impl Map {
    pub(crate) fn new(data: Vec<String>) -> Self {
        Self { data }
    }

    pub(crate) fn get_minimize_heat_loss(&self, ultra: bool) -> usize {
        let width = self.data[0].len();
        let height = self.data.len();
        let target = ((width - 1), (height - 1));
        let mut done = HashSet::new();
        let mut heat_losses = HashMap::new();
        let first_step = Step::init((0, 0));
        heat_losses.insert(first_step.clone(), 0);
        let mut steps = vec![(first_step, 0)];
        while let Some((step, _)) = steps.pop() {
            if done.contains(&step) {
                continue;
            }
            let next_steps = if ultra {
                step.get_ultra_next_step(&self)
            } else {
                step.get_next_steps((width, height))
            };
            for next_step in next_steps {
                let heat_loss = self.get_heat_loss(&step.cord, &next_step.cord);
                let next_step_heat_loss = heat_losses.get(&step).unwrap() + heat_loss;
                let next_step_origin_heat_loss = heat_losses.get(&next_step);
                let less_hest_loss_path_find =
                    if let Some(next_step_origin_heat_loss) = next_step_origin_heat_loss {
                        next_step_heat_loss < *next_step_origin_heat_loss
                    } else {
                        true
                    };
                if less_hest_loss_path_find {
                    heat_losses.insert(next_step.clone(), next_step_heat_loss);
                    steps.push((next_step, next_step_heat_loss));
                }
            }
            done.insert(step);
            steps.sort_by(|a, b| b.1.cmp(&a.1));
        }
        heat_losses
            .iter()
            .filter_map(|(step, loss)| {
                if step.cord == target {
                    Some(*loss)
                } else {
                    None
                }
            })
            .min()
            .unwrap()
    }

    fn get_heat_loss(&self, from: &Cord, to: &Cord) -> usize {
        let (x, y) = from;
        let x_eq = from.0 == to.0;
        let y_eq = from.1 == to.1;
        match (x_eq, y_eq) {
            (true, true) => 0,
            (true, false) => {
                let y_range = if to.1 > from.1 {
                    (from.1 + 1)..=(to.1)
                } else {
                    (to.1)..=(from.1 - 1)
                };
                y_range.map(|y| self.heat_loss_at(&(*x, y))).sum()
            }
            (false, true) => {
                let x_range = if to.0 > from.0 {
                    (from.0 + 1)..=(to.0)
                } else {
                    (to.0)..=(from.0 - 1)
                };
                x_range.map(|x| self.heat_loss_at(&(x, *y))).sum()
            }
            (false, false) => panic!("can not get heat loss from {:?} to {:?}", from, to),
        }
    }

    fn heat_loss_at(&self, cord: &Cord) -> usize {
        let (x, y) = cord;
        let row = self.data.get(*y).unwrap();
        let c = row.chars().nth(*x).unwrap();
        match c {
            '1' => 1,
            '2' => 2,
            '3' => 3,
            '4' => 4,
            '5' => 5,
            '6' => 6,
            '7' => 7,
            '8' => 8,
            '9' => 9,
            _ => panic!("error data: {:?}", c),
        }
    }
}
