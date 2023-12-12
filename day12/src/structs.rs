use std::{collections::HashMap, usize};

enum State {
    Operational,
    Damaged,
    Unknown,
}

impl State {
    fn parse(c: char) -> State {
        match c {
            '.' => Self::Operational,
            '#' => Self::Damaged,
            '?' => Self::Unknown,
            _ => panic!("wrong data! {:?}", c),
        }
    }
}

pub struct Data {
    data: Vec<State>,
}

impl std::fmt::Debug for Data {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.data.iter().for_each(|state| {
            let _ = match state {
                State::Operational => write!(f, "."),
                State::Damaged => write!(f, "#"),
                State::Unknown => write!(f, "?"),
            };
        });
        write!(f, "")
    }
}

impl Data {
    fn parse(input: &str) -> Self {
        let data = input.chars().map(|c| State::parse(c)).collect();
        Self { data }
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    fn try_fill(&self, idp: usize, i: usize, x: usize) -> bool {
        let left_check = if i > 0 {
            if matches!(self.data[i - 1], State::Unknown | State::Operational) {
                for e in idp..i {
                    if matches!(self.data[e], State::Damaged) {
                        return false;
                    }
                }
                true
            } else {
                false
            }
        } else {
            true
        };
        let body_check = (i..(i + x)).all(|j| {
            if j < self.len() {
                matches!(self.data[j], State::Unknown | State::Damaged)
            } else {
                false
            }
        });
        let right_check = if (i + x) < self.len() {
            if matches!(self.data[i + x], State::Unknown | State::Operational) {
                true
            } else if matches!(self.data[i + x], State::Damaged) {
                return false;
            } else {
                false
            }
        } else {
            true
        };
        if left_check && body_check && right_check {
            true
        } else {
            false
        }
    }

    fn is_empty(&self, dp: usize) -> bool {
        self.data
            .iter()
            .skip(dp)
            .all(|e| matches!(e, State::Unknown | State::Operational))
    }
}

#[derive(Debug)]
pub struct Record {
    pub data: Data,
    pub hint: Vec<usize>,
}

impl Record {
    pub(crate) fn new(data: Data, hint: Vec<usize>) -> Self {
        Self { data, hint }
    }

    pub(crate) fn parse(line: &str) -> Self {
        let mut split = line.split(' ');
        let data = split.next().unwrap();
        let hint = split
            .next()
            .unwrap()
            .split(',')
            .map(|c| c.parse().unwrap())
            .collect();
        let data = Data::parse(data);
        Self::new(data, hint)
    }

    pub(crate) fn count_arrangements(&self) -> usize {
        let mut cache = HashMap::new();
        count_arrangements_recursive_and_with_cache(0, 0, &self.data, &self.hint, &mut cache)
    }
}

fn count_arrangements_recursive_and_with_cache(
    idp: usize,
    hdp: usize,
    data: &Data,
    hint: &Vec<usize>,
    cache: &mut HashMap<(usize, usize), usize>,
) -> usize {
    if let Some(ans) = cache.get(&(idp, hdp)) {
        return *ans;
    }
    let mut success_count = 0;
    let x = hint[hdp];
    (idp..data.len()).for_each(|i| {
        let result = data.try_fill(idp, i, x);
        if result {
            let dp = i + x + 1;
            let data_empty = data.is_empty(dp);
            if hint.len() == hdp + 1 {
                if data_empty {
                    success_count += 1;
                }
            } else {
                success_count +=
                    count_arrangements_recursive_and_with_cache(dp, hdp + 1, data, hint, cache);
            }
        };
    });
    cache.insert((idp, hdp), success_count);
    success_count
}
