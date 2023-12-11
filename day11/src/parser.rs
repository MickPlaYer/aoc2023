use crate::structs::{Image, Pixel};

pub fn parse(input: &str) -> Image {
    let mut pixels = Vec::new();
    input.lines().for_each(|line| {
        line.chars().for_each(|c| {
            pixels.push(Pixel::new(c));
        });
    });
    let width = input.lines().next().unwrap().chars().count();
    let height = input.lines().count();
    Image::new(width, height, pixels)
}
