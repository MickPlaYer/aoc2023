use crate::structs::Map;

pub fn parse(input: &str) -> Map {
    let mut map = Vec::new();
    input.lines().into_iter().for_each(|line| {
        map.push(line.to_string());
    });
    Map::new(map)
}
