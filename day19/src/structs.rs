use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Label(String);
impl Label {
    pub(crate) fn new(string: String) -> Self {
        Self(string)
    }

    fn is_accept(&self) -> bool {
        self.0 == "A"
    }

    fn is_reject(&self) -> bool {
        self.0 == "R"
    }
}

#[derive(Debug)]
pub enum Condition {
    Greater(char, usize),
    Lesser(char, usize),
    Pass,
}

#[derive(Debug)]
pub struct Rule {
    condition: Condition,
    destination: Label,
}
impl Rule {
    pub(crate) fn new(condition: Condition, destination: Label) -> Self {
        Self {
            condition,
            destination,
        }
    }

    fn test(&self, part: &Part) -> Option<&Label> {
        let get_xmas = |xmas| match xmas {
            'x' => part.x,
            'm' => part.m,
            'a' => part.a,
            's' => part.s,
            _ => panic!("wrong xmas: {:?}", xmas),
        };
        let result = match self.condition {
            Condition::Greater(xmas, value) => get_xmas(xmas) > value,
            Condition::Lesser(xmas, value) => get_xmas(xmas) < value,
            Condition::Pass => true,
        };
        if result {
            Some(&self.destination)
        } else {
            None
        }
    }

    fn analyze(&self, ranges: Ranges) -> (Option<Ranges>, Option<(&Label, Ranges)>) {
        let get_xmas = |xmas_char| {
            let (min, max) = match xmas_char {
                'x' => &ranges.x,
                'm' => &ranges.m,
                'a' => &ranges.a,
                's' => &ranges.s,
                _ => panic!("wrong xmas: {:?}", xmas_char),
            };
            (*min, *max)
        };
        match self.condition {
            Condition::Greater(xmas_char, value) => {
                let (min, max) = get_xmas(xmas_char);
                if value < min {
                    (
                        None,
                        Some((&self.destination, ranges.replace(xmas_char, (min, max)))),
                    )
                } else if value >= min && value < max {
                    (
                        Some(ranges.replace(xmas_char, (min, value))),
                        Some((
                            &self.destination,
                            ranges.replace(xmas_char, ((value + 1), max)),
                        )),
                    )
                } else {
                    (Some(ranges.replace(xmas_char, (min, max))), None)
                }
            }
            Condition::Lesser(xmas_char, value) => {
                let (min, max) = get_xmas(xmas_char);
                if value > max {
                    (
                        None,
                        Some((&self.destination, ranges.replace(xmas_char, (min, max)))),
                    )
                } else if value <= max && value > min {
                    (
                        Some(ranges.replace(xmas_char, (value, max))),
                        Some((
                            &self.destination,
                            ranges.replace(xmas_char, (min, (value - 1))),
                        )),
                    )
                } else {
                    (Some(ranges.replace(xmas_char, (value, max))), None)
                }
            }
            Condition::Pass => (None, Some((&self.destination, ranges))),
        }
    }
}

#[derive(Debug)]
pub struct Workflow {
    name: Label,
    rules: Vec<Rule>,
}
impl Workflow {
    pub(crate) fn new(name: Label, rules: Vec<Rule>) -> Self {
        Self { name, rules }
    }

    fn process(&self, part: &Part) -> &Label {
        for rule in &self.rules {
            if let Some(label) = rule.test(part) {
                return label;
            }
        }
        panic!("not match any rule!: {:?} {:?}", self, part)
    }

    fn analyze(&self, ranges: Ranges) -> Vec<(&Label, Ranges)> {
        let mut todos = Vec::new();
        let mut current_ranges = ranges;
        for rule in &self.rules {
            let (next_ranges, todo) = rule.analyze(current_ranges);
            if let Some(todo) = todo {
                todos.push(todo);
            }
            if next_ranges.is_none() {
                break;
            }
            current_ranges = next_ranges.unwrap();
        }
        todos
    }
}

#[derive(Debug)]
pub struct Part {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}
impl Part {
    pub(crate) fn new(x: usize, m: usize, a: usize, s: usize) -> Self {
        Self { x, m, a, s }
    }

    pub(crate) fn get_score(&self) -> usize {
        self.x + self.m + self.a + self.s
    }
}

#[derive(Debug)]
pub struct Factory {
    workflows: HashMap<Label, Workflow>,
}
impl Factory {
    pub(crate) fn new(workflows: Vec<Workflow>) -> Self {
        let workflows = workflows
            .into_iter()
            .map(|workflow| (workflow.name.clone(), workflow))
            .collect();
        Self { workflows }
    }

    pub(crate) fn process(&self, part: &Part) -> bool {
        let mut current_label = &Label::new("in".to_string());
        loop {
            let workflow = self.workflows.get(&current_label).unwrap();
            let next_label = workflow.process(part);
            if next_label.is_accept() {
                return true;
            } else if next_label.is_reject() {
                return false;
            } else {
                current_label = next_label;
            }
        }
    }
}

type Range = (usize, usize);
#[derive(Debug)]
pub struct Ranges {
    x: Range,
    m: Range,
    a: Range,
    s: Range,
}
impl Ranges {
    pub(crate) fn new(x: Range, m: Range, a: Range, s: Range) -> Self {
        Self { x, m, a, s }
    }

    fn replace(&self, xmas: char, range: Range) -> Self {
        let mut x = self.x.clone();
        let mut m = self.m.clone();
        let mut a = self.a.clone();
        let mut s = self.s.clone();
        match xmas {
            'x' => x = range,
            'm' => m = range,
            'a' => a = range,
            's' => s = range,
            _ => panic!("wrong xmas: {:?}", xmas),
        };
        Self { x, m, a, s }
    }

    pub(crate) fn get_score(&self) -> usize {
        [self.x, self.m, self.a, self.s]
            .iter()
            .map(|(min, max)| (max - min + 1))
            .product()
    }
}

#[derive(Debug)]
pub struct Analyzer {
    workflows: HashMap<Label, Workflow>,
}
impl Analyzer {
    pub(crate) fn new(workflows: Vec<Workflow>) -> Self {
        let workflows = workflows
            .into_iter()
            .map(|workflow| (workflow.name.clone(), workflow))
            .collect();
        Self { workflows }
    }

    pub(crate) fn analyze(&self, ranges: Ranges) -> Vec<Ranges> {
        let label = &Label::new("in".to_string());
        let mut accepts = Vec::new();
        let mut todo_list = vec![(label, ranges)];
        while let Some((current_label, ranges)) = todo_list.pop() {
            let workflow = self.workflows.get(current_label).unwrap();
            let next_todos = workflow.analyze(ranges);
            for next_todo in next_todos {
                let next_label = next_todo.0;
                if next_label.is_accept() {
                    accepts.push(next_todo.1);
                } else if next_label.is_reject() {
                    continue;
                } else {
                    todo_list.push(next_todo);
                }
            }
        }
        accepts
    }
}
