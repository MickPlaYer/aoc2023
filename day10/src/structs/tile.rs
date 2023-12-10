use super::Direction;

#[derive(Clone)]
pub enum Tile {
    Ground,
    Verticle,
    Horizontal,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
}

impl Tile {
    pub fn to_right(&self) -> bool {
        match self {
            Tile::Horizontal => true,
            Tile::NorthEast => true,
            Tile::SouthEast => true,
            _ => false,
        }
    }

    pub fn to_left(&self) -> bool {
        match self {
            Tile::Horizontal => true,
            Tile::NorthWest => true,
            Tile::SouthWest => true,
            _ => false,
        }
    }

    pub fn to_up(&self) -> bool {
        match self {
            Tile::Verticle => true,
            Tile::NorthEast => true,
            Tile::NorthWest => true,
            _ => false,
        }
    }

    pub fn to_down(&self) -> bool {
        match self {
            Tile::Verticle => true,
            Tile::SouthWest => true,
            Tile::SouthEast => true,
            _ => false,
        }
    }

    pub fn default_direction(&self) -> Direction {
        match self {
            Tile::Verticle | Tile::NorthEast | Tile::NorthWest => Direction::Up,
            Tile::Horizontal | Tile::SouthWest => Direction::Left,
            Tile::SouthEast => Direction::Right,
            Tile::Ground => panic!("no default direction! {:?}", self),
        }
    }

    pub fn connect_to(&self, connect_from: &Direction) -> Direction {
        match (self, connect_from) {
            (Tile::Verticle, Direction::Up) => Direction::Down,
            (Tile::Verticle, Direction::Down) => Direction::Up,
            (Tile::Horizontal, Direction::Left) => Direction::Right,
            (Tile::Horizontal, Direction::Right) => Direction::Left,
            (Tile::NorthEast, Direction::Up) => Direction::Right,
            (Tile::NorthEast, Direction::Right) => Direction::Up,
            (Tile::NorthWest, Direction::Up) => Direction::Left,
            (Tile::NorthWest, Direction::Left) => Direction::Up,
            (Tile::SouthWest, Direction::Down) => Direction::Left,
            (Tile::SouthWest, Direction::Left) => Direction::Down,
            (Tile::SouthEast, Direction::Down) => Direction::Right,
            (Tile::SouthEast, Direction::Right) => Direction::Down,
            _ => panic!("connect to nowhere! {:?} {:?}", self, connect_from),
        }
    }

    pub fn is_pipe(&self) -> bool {
        match self {
            Tile::Ground => false,
            _ => true,
        }
    }

    pub fn rotate(&self) -> Tile {
        match self {
            Tile::Ground => Tile::Ground,
            Tile::Verticle => Tile::Horizontal,
            Tile::Horizontal => Tile::Verticle,
            Tile::NorthEast => Tile::NorthWest,
            Tile::NorthWest => Tile::SouthWest,
            Tile::SouthWest => Tile::SouthEast,
            Tile::SouthEast => Tile::NorthEast,
        }
    }
}

impl std::fmt::Debug for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ground => write!(f, "."),
            Self::Verticle => write!(f, "║"),
            Self::Horizontal => write!(f, "═"),
            Self::NorthEast => write!(f, "╚"),
            Self::NorthWest => write!(f, "╝"),
            Self::SouthWest => write!(f, "╗"),
            Self::SouthEast => write!(f, "╔"),
        }
    }
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '.' | 'S' => Tile::Ground,
            '|' => Tile::Verticle,
            '-' => Tile::Horizontal,
            'L' => Tile::NorthEast,
            'J' => Tile::NorthWest,
            '7' => Tile::SouthWest,
            'F' => Tile::SouthEast,
            _ => panic!("tile error! {:?}", value),
        }
    }
}
