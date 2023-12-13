use crate::structs::Map;

pub fn parse(input: &str) -> Vec<Map> {
    let mut maps = Vec::new();
    let mut map = Vec::new();
    input.lines().into_iter().for_each(|line| {
        if line.is_empty() {
            maps.push(Map::new(map.clone()));
            map.clear();
        } else {
            map.push(line.to_string());
        }
    });
    maps.push(Map::new(map.clone()));
    dbg!(&maps);
    maps
}
