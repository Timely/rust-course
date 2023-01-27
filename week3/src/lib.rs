use rand::prelude::*;
use std::f32::consts::PI;

#[derive(Debug)]
pub enum Shape {
    Circle(f32),
    Square(isize),
    Rectangle(isize, isize),
}

pub fn area(shape: &Shape) -> f32 {
    match shape {
        Shape::Circle(radius) => radius * radius * PI,
        Shape::Square(length) => (length * length) as f32,
        Shape::Rectangle(width, height) => (height * width) as f32,
    }
}

pub fn random_shape() -> Shape {
    let mut rng = rand::thread_rng();
    let random_number: isize = rng.gen_range(0..2);

    match random_number {
        0 => Shape::Circle(rng.gen()),
        1 => Shape::Rectangle(rng.gen(), rng.gen()),
        _ => Shape::Square(rng.gen()),
    }
}

pub fn circle_area(shape: &Shape) -> Option<f32> {
    if let Shape::Circle(shape) = shape {
        Some(area(&Shape::Circle(*shape)))
    } else {
        None
    }
}

#[cfg(test)]
mod test_shape_areas {
    use super::*;

    #[test]
    fn gets_area_of_circle() {
        const RADIUS: f32 = 1.0;
        const ANSWER: f32 = 3.1415927;

        let circle = Shape::Circle(RADIUS);

        assert_eq!(ANSWER, area(&circle));
    }
    #[test]
    fn gets_area_of_rectangle() {
        const WIDTH: isize = 100;
        const HEIGHT: isize = 20;
        const ANSWER: f32 = 2000.0 as f32;
        let rectangle = Shape::Rectangle(WIDTH, HEIGHT);

        assert_eq!(ANSWER, area(&rectangle));
    }
    #[test]
    fn gets_area_of_square() {
        const LENGTH: isize = 100;
        const ANSWER: f32 = 10000.0 as f32;
        let square = Shape::Square(LENGTH);

        assert_eq!(ANSWER, area(&square));
    }
    #[test]
    fn gets_circle_area() {
        const RADIUS: f32 = 1.0;
        const ANSWER: Option<f32> = Some(3.1415927);

        let circle = Shape::Circle(RADIUS);

        assert_eq!(ANSWER, circle_area(&circle));
    }
    #[test]
    fn gets_circle_area_else() {
        const LENGTH: isize = 100;

        let square = Shape::Square(LENGTH);

        assert_eq!(None, circle_area(&square));
    }
}
