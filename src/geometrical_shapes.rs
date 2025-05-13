use rand::Rng;
use raster::{Color, Image};

pub trait Drawable {
    fn draw(&mut self, image: &mut Image);
    fn color() -> Color {
        let r = rand::rng().random_range(0..255);
        let g = rand::rng().random_range(0..255);
        let b = rand::rng().random_range(0..255);
        Color::rgb(r, g, b)
    }
}

pub trait Displayable {
    fn display(&mut self, x: i32, y: i32, color: Color);
}

// point struct
#[derive(Debug, Clone)]
pub struct Point(pub i32, pub i32);

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self(x, y)
    }
    pub fn random(width: i32, height: i32) -> Self {
        let x = rand::rng().random_range(0..width);
        let y = rand::rng().random_range(0..height);
        Self(x, y)
    }
}

impl Drawable for Point {
    fn draw(&mut self, image: &mut Image) {
        let _ = image.set_pixel(self.0, self.1, Self::color());
    }
}

// line struct
#[derive(Debug, Clone)]
pub struct Line(Point, Point);

impl Line {
    pub fn new(p1: &Point, p2: &Point) -> Self {
        Self(p1.clone(), p2.clone())
    }
    pub fn random(width: i32, height: i32) -> Self {
        // point 1
        let x1 = rand::rng().random_range(0..width);
        let y1 = rand::rng().random_range(0..height);

        // point 2
        let x2 = rand::rng().random_range(0..width);
        let y2 = rand::rng().random_range(0..height);

        Self(Point(x1, y1), Point(x2, y2))
    }
}

impl Drawable for Line {
    fn draw(&mut self, image: &mut Image) {
        let point_1 = &self.0;
        let point_2 = &self.0;


        // let _ = image.set_pixel(point_1, self.1, Self::color());
    }
}

// triangle struct
#[derive(Debug, Clone)]
pub struct Triangle(Point, Point, Point);

impl Triangle {
    pub fn new(p1: &Point, p2: &Point, p3: &Point) -> Self {
        Self(p1.clone(), p2.clone(), p3.clone())
    }
    pub fn random(width: i32, height: i32) -> Self {
        // point 1
        let x1: i32 = rand::rng().random_range(0..width);
        let y1: i32 = rand::rng().random_range(0..height);

        // point 2
        let x2: i32 = rand::rng().random_range(0..width);
        let y2: i32 = rand::rng().random_range(0..height);

        // point 3
        let x3: i32 = rand::rng().random_range(0..width);
        let y3: i32 = rand::rng().random_range(0..height);

        Self(Point(x1, y1), Point(x2, y2), Point(x3, y3))
    }
}

// rectangle struct
#[derive(Debug, Clone)]
pub struct Rectangle(Point, Point);

impl Rectangle {
    pub fn new(p1: &Point, p2: &Point) -> Self {
        Self(p1.clone(), p2.clone())
    }
    pub fn random(x: i32, y: i32) -> i32 {
        rand::rng().random_range(x..y)
    }
}
