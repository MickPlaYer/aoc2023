use crate::structs::Map;

pub fn parse(input: &str) -> Map {
    let mut data = Vec::new();
    let mut start_point = None;
    for (y, line) in input.lines().enumerate() {
        if start_point.is_none() {
            let start_x = line.chars().position(|e| e == 'S');
            if let Some(x) = start_x {
                start_point = Some((x, y));
            }
        }
        data.push(line.to_string())
    }
    if start_point.is_none() {
        panic!("can not find start_point! {:?}", input)
    }
    Map::new(start_point.unwrap(), data)
}
