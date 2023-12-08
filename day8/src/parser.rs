use std::collections::HashMap;

pub fn parse(content: &str) -> (&str, HashMap<&str, (&str, &str)>) {
    let mut lines = content.lines();
    let instruction = lines.next().unwrap();
    lines.next();
    let mut map = HashMap::new();
    lines.for_each(|line| {
        let mut split = line.split(" = ");
        let name = split.next().unwrap();
        let gate = split
            .next()
            .unwrap()
            .trim_matches(|e| matches!(e, '(' | ')'))
            .split(", ")
            .take(2)
            .collect::<Vec<&str>>();
        map.insert(name, (gate[0], gate[1]));
    });
    (instruction, map)
}
