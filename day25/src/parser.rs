use std::collections::{HashMap, HashSet};

pub(crate) fn parse(input: &str) -> HashMap<String, HashSet<String>> {
    let mut mapping = HashMap::new();
    for line in input.lines() {
        let mut split = line.split(": ");
        let name = split.next().unwrap();
        for connected in split.next().unwrap().split(' ') {
            let list = mapping.entry(name.to_string()).or_insert(HashSet::new());
            list.insert(connected.to_string());
            let list = mapping
                .entry(connected.to_string())
                .or_insert(HashSet::new());
            list.insert(name.to_string());
        }
    }
    mapping
}
