#[derive(Debug)]
pub struct Record {
    time: usize,
    distance: usize,
}

impl Record {
    pub fn new(time: usize, distance: usize) -> Self {
        Self { time, distance }
    }

    pub fn count_successes(&self) -> usize {
        let range = 0..=self.time;
        let range_back = (0..=self.time).rev();
        let min = range
            .into_iter()
            .find(|hold_time| {
                let travel_time = self.time - hold_time;
                let travel_distance = travel_time * hold_time;
                travel_distance > self.distance
            })
            .unwrap();
        let max = range_back
            .into_iter()
            .find(|hold_time| {
                let travel_time = self.time - hold_time;
                let travel_distance = travel_time * hold_time;
                travel_distance > self.distance
            })
            .unwrap();
        (min..=max).count()
    }
}
