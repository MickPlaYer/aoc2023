use crate::structs::Map;

pub fn parse(input: &str) -> Map {
    let mut data = Vec::new();
    input.lines().for_each(|line| data.push(line.to_string()));
    Map::new(data)
}
