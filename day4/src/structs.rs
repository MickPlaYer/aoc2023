#[derive(Debug)]
pub struct Card {
    id: usize,
    winning_numbers: Vec<usize>,
    numbers: Vec<usize>,
}

impl Card {
    pub fn new(id: usize, winning_numbers: Vec<usize>, numbers: Vec<usize>) -> Self {
        Self {
            id,
            winning_numbers,
            numbers,
        }
    }

    pub fn get_match_count(&self) -> usize {
        let mut match_count = 0;
        let mut winning_numbers = self.winning_numbers.clone();
        winning_numbers.sort();
        self.numbers.iter().for_each(|number| {
            let hit = winning_numbers.binary_search(number);
            if hit.is_ok() {
                match_count += 1;
            }
        });
        match_count
    }

    pub fn get_points(&self) -> usize {
        let match_count = self.get_match_count();
        match match_count {
            0 => 0,
            1 => 1,
            _ => (2 as usize).pow(match_count as u32 - 1),
        }
    }

    pub fn get_id(&self) -> usize {
        self.id
    }
}
