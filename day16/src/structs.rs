mod beam;

use self::beam::{Beam, Cord};
use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
};

pub(crate) struct Map {
    data: Vec<String>,
    memo: Option<RefCell<HashMap<Beam, (Vec<Cord>, Vec<Beam>)>>>,
}

impl Map {
    pub(crate) fn new(data: Vec<String>) -> Self {
        Self { data, memo: None }
    }

    pub(crate) fn use_memo(&mut self) {
        self.memo = Some(RefCell::new(HashMap::new()));
    }

    pub(crate) fn generate_init_beam(&self) -> Beam {
        Beam::right((0, 0))
    }

    pub(crate) fn generate_init_beams(&self) -> Vec<Beam> {
        let y_end = self.data[0].len() - 1;
        let x_end = self.data.len() - 1;
        let mut beams = vec![
            Beam::right((0, 0)),
            Beam::down((0, 0)),
            Beam::left((x_end, 0)),
            Beam::down((x_end, 0)),
            Beam::left((x_end, y_end)),
            Beam::up((x_end, y_end)),
            Beam::right((0, y_end)),
            Beam::up((0, y_end)),
        ];
        for x in 1..y_end {
            beams.push(Beam::left((x, 0)));
            beams.push(Beam::down((x, 0)));
            beams.push(Beam::right((x, 0)));
            beams.push(Beam::left((x, x_end)));
            beams.push(Beam::up((x, x_end)));
            beams.push(Beam::right((x, x_end)));
        }
        for y in 1..x_end {
            beams.push(Beam::up((0, y)));
            beams.push(Beam::right((0, y)));
            beams.push(Beam::down((0, y)));
            beams.push(Beam::up((y_end, y)));
            beams.push(Beam::right((y_end, y)));
            beams.push(Beam::down((y_end, y)));
        }
        beams
    }

    pub(crate) fn count_energized(&self, beam: Beam) -> usize {
        let mut energized_path = HashSet::new();
        energized_path.insert(beam.get_cord());
        let mut done_beams = HashSet::new();
        let mut beams = vec![beam];
        while let Some(beam) = beams.pop() {
            if done_beams.contains(&beam) {
                continue;
            }
            let (path, new_beams) = if let Some(memo) = &self.memo {
                if let Some(from_memo) = memo.borrow().get(&beam) {
                    (from_memo.0.clone(), from_memo.1.clone())
                } else {
                    beam.fly(self)
                }
            } else {
                beam.fly(self)
            };
            if let Some(memo) = &self.memo {
                memo.borrow_mut()
                    .insert(beam.clone(), (path.clone(), new_beams.clone()));
            }
            for tile in path {
                energized_path.insert(tile);
            }
            for new_beam in new_beams {
                beams.push(new_beam);
            }
            done_beams.insert(beam);
        }
        energized_path.iter().count()
    }

    fn get(&self, cord: &Cord) -> Option<char> {
        let (x, y) = cord;
        let row = self.data.get(*y)?;
        row.chars().nth(*x)
    }
}
