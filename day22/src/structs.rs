#[derive(Debug)]
pub struct Point {
    x: usize,
    y: usize,
    z: usize,
}
impl Point {
    fn new(x: usize, y: usize, z: usize) -> Point {
        Self { x, y, z }
    }
}
impl From<Vec<usize>> for Point {
    fn from(value: Vec<usize>) -> Self {
        Self::new(value[0], value[1], value[2])
    }
}

#[derive(Debug)]
pub struct Area {
    pub bl: (usize, usize),
    pub tr: (usize, usize),
}
impl Area {
    fn is_cross(&self, other: &Area) -> bool {
        if self.tr.0 < other.bl.0 || self.bl.0 > other.tr.0 {
            return false;
        }
        if self.tr.1 < other.bl.1 || self.bl.1 > other.tr.1 {
            return false;
        }
        true
    }
}

#[derive(Debug)]
pub struct Brick {
    head: Point,
    tail: Point,
}
impl Brick {
    pub(crate) fn new(head: Point, tail: Point) -> Self {
        Self { head, tail }
    }

    pub(crate) fn get_shadow(&self) -> Area {
        let min_x = self.head.x.min(self.tail.x);
        let max_x = self.head.x.max(self.tail.x);
        let min_y = self.head.y.min(self.tail.y);
        let max_y = self.head.y.max(self.tail.y);
        Area {
            bl: (min_x, min_y),
            tr: (max_x, max_y),
        }
    }

    pub(crate) fn get_top(&self) -> usize {
        self.head.z.max(self.tail.z)
    }

    pub(crate) fn get_bottom(&self) -> usize {
        self.head.z.min(self.tail.z)
    }

    pub(crate) fn is_cross(&self, shadow: &Area) -> bool {
        self.get_shadow().is_cross(shadow)
    }

    pub(crate) fn down_to(&self, new_z: usize) -> Self {
        let bottom = self.get_bottom();
        let head = Point::new(self.head.x, self.head.y, self.head.z - bottom + new_z);
        let tail = Point::new(self.tail.x, self.tail.y, self.tail.z - bottom + new_z);
        Self { head, tail }
    }
}
