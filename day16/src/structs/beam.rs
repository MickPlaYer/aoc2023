use super::Map;

pub type Cord = (usize, usize);

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
enum Dir {
    L,
    R,
    U,
    D,
}

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub struct Beam {
    cord: Cord,
    dir: Dir,
}

impl Beam {
    fn new(cord: Cord, dir: Dir) -> Self {
        Self { cord, dir }
    }

    pub(crate) fn left(cord: Cord) -> Self {
        Self::new(cord, Dir::L)
    }

    pub(crate) fn right(cord: Cord) -> Self {
        Self::new(cord, Dir::R)
    }

    pub(crate) fn up(cord: Cord) -> Self {
        Self::new(cord, Dir::U)
    }

    pub(crate) fn down(cord: Cord) -> Self {
        Self::new(cord, Dir::D)
    }

    pub fn fly(&self, map: &Map) -> (Vec<Cord>, Vec<Beam>) {
        let (x, y) = self.cord;
        let mut path = Vec::new();
        let mut new_beams = Vec::new();
        match self.dir {
            Dir::L => {
                if x > 0 {
                    for x in (0..=(x - 1)).rev() {
                        let tile = map.get(&(x, y));
                        if let Some(tile) = tile {
                            path.push((x, y));
                            match tile {
                                '.' => (),
                                '|' => {
                                    new_beams.push(Beam::up((x, y)));
                                    new_beams.push(Beam::down((x, y)));
                                    break;
                                }
                                '-' => (),
                                '\\' => {
                                    new_beams.push(Beam::up((x, y)));
                                    break;
                                }
                                '/' => {
                                    new_beams.push(Beam::down((x, y)));
                                    break;
                                }
                                _ => panic!("wrong tile {:?}", tile),
                            }
                        } else {
                            break;
                        }
                    }
                }
            }
            Dir::R => {
                for x in (x + 1).. {
                    let tile = map.get(&(x, y));
                    if let Some(tile) = tile {
                        path.push((x, y));
                        match tile {
                            '.' => (),
                            '|' => {
                                new_beams.push(Beam::up((x, y)));
                                new_beams.push(Beam::down((x, y)));
                                break;
                            }
                            '-' => (),
                            '\\' => {
                                new_beams.push(Beam::down((x, y)));
                                break;
                            }
                            '/' => {
                                new_beams.push(Beam::up((x, y)));
                                break;
                            }
                            _ => panic!("wrong tile {:?}", tile),
                        }
                    } else {
                        break;
                    }
                }
            }
            Dir::U => {
                if y > 0 {
                    for y in (0..=(y - 1)).rev() {
                        let tile = map.get(&(x, y));
                        if let Some(tile) = tile {
                            path.push((x, y));
                            match tile {
                                '.' => (),
                                '|' => (),
                                '-' => {
                                    new_beams.push(Beam::left((x, y)));
                                    new_beams.push(Beam::right((x, y)));
                                    break;
                                }
                                '\\' => {
                                    new_beams.push(Beam::left((x, y)));
                                    break;
                                }
                                '/' => {
                                    new_beams.push(Beam::right((x, y)));
                                    break;
                                }
                                _ => panic!("wrong tile {:?}", tile),
                            }
                        } else {
                            break;
                        }
                    }
                }
            }
            Dir::D => {
                for y in (y + 1).. {
                    let tile = map.get(&(x, y));
                    if let Some(tile) = tile {
                        path.push((x, y));
                        match tile {
                            '.' => (),
                            '|' => (),
                            '-' => {
                                new_beams.push(Beam::left((x, y)));
                                new_beams.push(Beam::right((x, y)));
                                break;
                            }
                            '\\' => {
                                new_beams.push(Beam::right((x, y)));
                                break;
                            }
                            '/' => {
                                new_beams.push(Beam::left((x, y)));
                                break;
                            }
                            _ => panic!("wrong tile {:?}", tile),
                        }
                    } else {
                        break;
                    }
                }
            }
        };
        (path, new_beams)
    }

    pub(crate) fn get_cord(&self) -> (usize, usize) {
        self.cord
    }
}
