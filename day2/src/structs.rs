#[derive(Debug)]
pub struct CubeSet {
    pub red: usize,
    pub green: usize,
    pub blue: usize,
}

impl CubeSet {
    pub fn empty() -> Self {
        Self {
            red: 0,
            green: 0,
            blue: 0,
        }
    }

    pub fn new(red: usize, green: usize, blue: usize) -> Self {
        Self { red, green, blue }
    }
}

#[derive(Debug)]
pub struct Game {
    pub id: usize,
    pub cube_sets: Vec<CubeSet>,
}

impl Game {
    pub fn new(id: usize, cube_sets: Vec<CubeSet>) -> Self {
        Self { id, cube_sets }
    }
}
