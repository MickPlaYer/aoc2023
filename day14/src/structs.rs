#[derive(Debug, Hash, Eq, PartialEq, Clone)]
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

    pub(crate) fn tilt_north(&self) -> Self {
        self.transpose().tilt_west_or_east(true).transpose()
    }

    pub(crate) fn tilt_south(&self) -> Self {
        self.transpose().tilt_west_or_east(false).transpose()
    }

    pub(crate) fn tilt_west(&self) -> Self {
        self.tilt_west_or_east(true)
    }

    pub(crate) fn tilt_east(&self) -> Self {
        self.tilt_west_or_east(false)
    }

    pub(crate) fn tilt_west_or_east(&self, is_west: bool) -> Self {
        let mut data = Vec::new();
        self.data.iter().for_each(|row| {
            let parts: Vec<String> = row
                .split("#")
                .map(|e| {
                    let mut chars: Vec<char> = e.chars().collect();
                    chars.sort();
                    if is_west {
                        chars.reverse();
                    }
                    chars.into_iter().collect()
                })
                .collect();
            data.push(parts.join("#"));
        });
        Self::new(data)
    }

    pub(crate) fn count_total_load(&self) -> usize {
        self.data
            .iter()
            .rev()
            .enumerate()
            .map(|(i, row)| (i + 1) * row.chars().filter(|c| *c == 'O').count())
            .sum()
    }
}
