#[derive(Debug)]
pub(crate) struct Map {
    data: Vec<String>,
}

impl Map {
    pub(crate) fn new(data: Vec<String>) -> Self {
        Self { data }
    }

    pub(crate) fn transpose(&self) -> Self {
        let width = self.data[0].len();
        let mut data = Vec::new();
        for j in 0..width {
            let mut new_row = Vec::new();
            for i in 0..self.data.len() {
                let row = &self.data[i];
                let char = row.chars().nth(j);
                if let Some(char) = char {
                    new_row.push(char);
                }
            }
            let new_row = new_row.into_iter().collect();
            data.push(new_row);
        }
        Self { data }
    }

    pub(crate) fn find_horizontal_mirror(&self) -> Vec<usize> {
        let mut answers = Vec::new();
        let length = self.data.len();
        for i in 0..self.data.len() {
            let mut top = self.data[0..i].to_vec();
            let mut bottom = self.data[i..length].to_vec();
            if top.len() == 0 {
                continue;
            }
            top.reverse();
            if top.len() > bottom.len() {
                (top, bottom) = (bottom, top)
            }
            bottom = bottom[0..top.len()].to_vec();
            if top == bottom {
                answers.push(i);
            }
        }
        answers
    }

    pub(crate) fn find_horizontal_mirror_with_smudge(&mut self) -> Vec<usize> {
        let mut new_answers = Vec::new();
        let binding = self.find_horizontal_mirror();
        let origin = binding.get(0).unwrap_or(&0);
        let width = self.data[0].len();
        for j in 0..width {
            for i in 0..self.data.len() {
                self.flip_one_spot(i, j);
                let new = self.find_horizontal_mirror();
                self.flip_one_spot(i, j);
                new.iter().for_each(|new| {
                    if new != origin {
                        new_answers.push(*new);
                    }
                });
            }
        }
        new_answers
    }

    fn flip_one_spot(&mut self, i: usize, j: usize) {
        let mut char_vector: Vec<char> = self.data[i].chars().collect();
        if let Some(char_to_change) = char_vector.get_mut(j) {
            if *char_to_change == '.' {
                *char_to_change = '#';
            } else if *char_to_change == '#' {
                *char_to_change = '.';
            } else {
                panic!("error spot! => {:?}", char_to_change)
            }
        }
        self.data[i] = char_vector.into_iter().collect();
    }
}
