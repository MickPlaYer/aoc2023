#[derive(Clone, Debug)]
pub enum Pixel {
    EmptySpace,
    EmptyDistance(usize),
    Galaxies,
}

impl Pixel {
    pub(crate) fn new(c: char) -> Self {
        match c {
            '.' => Pixel::EmptySpace,
            '#' => Pixel::Galaxies,
            _ => panic!("error pixel: {:?}", c),
        }
    }

    fn expand(&self, expand_speed: usize) -> Self {
        match self {
            Pixel::EmptySpace => Pixel::EmptyDistance(expand_speed),
            Pixel::EmptyDistance(distance) => Pixel::EmptyDistance(expand_speed * distance),
            Pixel::Galaxies => panic!("error galaxies expand"),
        }
    }

    fn get_distance(&self) -> usize {
        match self {
            Pixel::EmptySpace => 1,
            Pixel::EmptyDistance(distance) => *distance,
            Pixel::Galaxies => 1,
        }
    }

    fn is_galaxies(&self) -> bool {
        matches!(self, Pixel::Galaxies)
    }

    fn is_empty(&self) -> bool {
        matches!(self, Pixel::EmptySpace | Pixel::EmptyDistance(_))
    }
}

#[derive(Debug)]
pub struct Image {
    expend_speed: usize,
    width: usize,
    height: usize,
    pixels: Vec<Pixel>,
}

impl Image {
    pub(crate) fn new(width: usize, height: usize, pixels: Vec<Pixel>) -> Self {
        Self {
            expend_speed: 2,
            width,
            height,
            pixels,
        }
    }

    pub(crate) fn set_expand_speed(&mut self, expend_speed: usize) {
        self.expend_speed = expend_speed;
    }

    pub(crate) fn find_galaxies_points(&self) -> Vec<(usize, usize)> {
        let mut points = Vec::new();
        self.pixels
            .chunks(self.width)
            .enumerate()
            .for_each(|(y, row)| {
                row.iter().enumerate().for_each(|(x, pixel)| {
                    if pixel.is_galaxies() {
                        points.push((x, y));
                    }
                });
            });
        points
    }

    pub(crate) fn simulate_space_expands(self) -> Image {
        self.space_expand_rows()
            .print_pixels()
            .transpose_pixels()
            .print_pixels()
            .space_expand_rows()
            .print_pixels()
            .transpose_pixels()
            .print_pixels()
    }

    fn print_pixels(self) -> Self {
        self.pixels.chunks(self.width).for_each(|row| {
            row.iter().for_each(|pixel| {
                match pixel {
                    Pixel::EmptySpace => print!("."),
                    Pixel::Galaxies => print!("#"),
                    Pixel::EmptyDistance(_) => print!("~"),
                };
            });
            println!();
        });
        println!();
        self
    }

    fn transpose_pixels(self) -> Self {
        let pixels = &self.pixels;
        let pixels = (0..self.width)
            .flat_map(|row| {
                (0..self.height).map(move |column| pixels[column * self.width + row].clone())
            })
            .collect();
        Self {
            pixels,
            width: self.height,
            height: self.width,
            ..self
        }
    }

    fn space_expand_rows(self) -> Self {
        let expend_speed = self.expend_speed;
        let mut pixels = Vec::new();
        self.pixels.chunks(self.width).for_each(|row| {
            pixels.extend_from_slice(row);
            if row.iter().all(|pixel| pixel.is_empty()) {
                let new_row: Vec<Pixel> = row
                    .iter()
                    .map(|pixel| pixel.expand(expend_speed - 1))
                    .collect();
                pixels.extend_from_slice(&new_row);
            }
        });
        let height = pixels.len() / self.width;
        Self {
            pixels,
            height,
            ..self
        }
    }

    pub(crate) fn get_distance(&self, point_a: (usize, usize), point_b: (usize, usize)) -> usize {
        let horizontal_distance = self.calculate_distance(point_a.0, point_b.0, |x| (x, point_a.1));
        let vertical_distance = self.calculate_distance(point_a.1, point_b.1, |y| (point_a.0, y));
        vertical_distance + horizontal_distance
    }

    fn calculate_distance<F>(&self, from: usize, to: usize, create_point: F) -> usize
    where
        F: Fn(usize) -> (usize, usize),
    {
        let mut distance = 0;
        let mut start = from;
        let mut end = to;
        if start > end {
            (start, end) = (end, start);
        }
        for i in (start + 1)..=end {
            let point = create_point(i);
            let pixel = self.get_pixel(&point);
            distance += pixel.get_distance();
        }
        distance
    }

    fn get_pixel(&self, point: &(usize, usize)) -> &Pixel {
        let (x, y) = point;
        self.pixels.get(y * self.width + x).unwrap()
    }
}
