use crate::structs::Map;

pub fn parse(input: &str) -> Map {
    let data = input.lines().map(|e| e.to_string()).collect();
    Map::new(data)
}
