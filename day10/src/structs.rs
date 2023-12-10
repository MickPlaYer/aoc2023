mod tile;

pub use tile::Tile;

#[derive(Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn invert(&self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

pub struct MapSize {
    pub width: usize,
    pub height: usize,
}

impl MapSize {
    pub fn new(width: usize, height: usize) -> Self {
        Self { width, height }
    }
}
