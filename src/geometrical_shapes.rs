use rand::Rng;
use raster::{Color, Image};
use std::cmp::max;

pub trait Drawable {
    fn draw(&self, image: &mut Image);
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
    fn draw(&self, image: &mut Image) {
        image.display(self.0, self.1, Self::color());
    }
}

// line struct
#[derive(Debug, Clone)]
pub struct Line(Point, Point, Color);

impl Line {
    pub fn new(p1: &Point, p2: &Point, color: Color) -> Self {
        Self(p1.clone(), p2.clone(), color.clone())
    }
    pub fn random(width: i32, height: i32) -> Self {
        // get a random color
        let color: Color = Self::color();

        // point 1
        let x1: i32 = rand::rng().random_range(0..width);
        let y1: i32 = rand::rng().random_range(0..height);

        // poi:i32nt 2
        let x2: i32 = rand::rng().random_range(0..width);
        let y2: i32 = rand::rng().random_range(0..height);

        Self(Point(x1, y1), Point(x2, y2), color)
    }
}

impl Drawable for Line {
    fn draw(&self, image: &mut Image) {
        // get a random color
        let color: Color = self.2.clone();

        let start_x: i32 = self.0.0;
        let start_y: i32 = self.0.1;

        let end_x: i32 = self.1.0;
        let end_y: i32 = self.1.1;

        let dis_x: i32 = end_x - start_x; // distance between the x of start & end points
        let dis_y: i32 = end_y - start_y; // distance between the y of start & end points

        let steps = max(dis_x, dis_y);

        let mut new_x: f32 = start_x as f32;
        let mut new_y: f32 = start_y as f32;

        let x_inc = dis_x as f32 / steps as f32;
        let y_inc = dis_y as f32 / steps as f32;

        for _ in 0..=steps {
            image.display(new_x.round() as i32, new_y.round() as i32, color.clone());
            new_x += x_inc;
            new_y += y_inc;
        }
    }
}

// triangle struct
#[derive(Debug, Clone)]
pub struct Triangle(Point, Point, Point);

impl Triangle {
    pub fn new(p1: &Point, p2: &Point, p3: &Point) -> Self {
        Self(p1.clone(), p2.clone(), p3.clone())
    }
}

impl Drawable for Triangle {
    fn draw(&self, image: &mut Image) {
        // get a random color
        let color: Color = Color::rgb(255, 255, 255);

        let point_a = &self.0;
        let point_b = &self.1;
        let point_c = &self.2;

        Line::new(point_a, point_b, color.clone()).draw(image);
        Line::new(point_b, point_c, color.clone()).draw(image);
        Line::new(point_a, point_c, color.clone()).draw(image);
    }
}

// rectangle struct
#[derive(Debug, Clone)]
pub struct Rectangle(Point, Point);

impl Rectangle {
    pub fn new(p1: &Point, p2: &Point) -> Self {
        Self(p1.clone(), p2.clone())
    }
}

impl Drawable for Rectangle {
    fn draw(&self, image: &mut Image) {
        // get a random color
        let color: Color = Color::rgb(255, 255, 255);

        let point_a = &Point(self.1.0, self.1.0);
        let point_b = &Point(self.0.1, self.1.1);
        let point_c = &Point(self.0.0, self.0.0);
        let point_d = &Point(self.1.0, self.0.1);

        Point::new(point_c.0, point_c.1).draw(image);
        Point::new(point_b.0, point_b.1).draw(image);
        Point::new(point_a.0, point_a.1).draw(image);
        Point::new(point_d.0, point_d.1).draw(image);

        Line::new(point_a, point_b, color.clone()).draw(image);
        Line::new(point_b, point_c, color.clone()).draw(image);
        Line::new(point_d, point_c, color.clone()).draw(image);
        Line::new(point_a, point_d, color.clone()).draw(image);
    }
}

// circle struct
#[derive(Debug, Clone)]
pub struct Circle(Point, i32);

impl Circle {
    pub fn new(c: &Point, r: i32) -> Self {
        Self(c.clone(), r)
    }
    pub fn random(width: i32, height: i32) -> Self {
        let x = rand::rng().random_range(0..width);
        let y = rand::rng().random_range(0..height);
        let r = rand::rng().random_range(0..height);
        Self::new(&Point(x, y), r)
    }
}

impl Drawable for Circle {
    fn draw(&self, image: &mut Image) {
        // get a random color
        let color: Color = Self::color();

        let center_x: i32 = self.0.0;
        let center_y: i32 = self.0.1;
        let raduis: i32 = self.1;

        let mut x: i32 = 0;
        let mut y: i32 = -raduis;
        let mut p: i32 = -raduis;

        while x < (-y) {
            if p > 0 {
                y += 1;
                p += 2 * (x + y) + 1;
            } else {
                p += 2 * x + 1;
            }

            image.display(center_x + x, center_y + y, color.clone());
            image.display(center_x - x, center_y + y, color.clone());
            image.display(center_x + x, center_y - y, color.clone());
            image.display(center_x - x, center_y - y, color.clone());
            image.display(center_x + y, center_y + x, color.clone());
            image.display(center_x - y, center_y + x, color.clone());
            image.display(center_x + y, center_y - x, color.clone());
            image.display(center_x - y, center_y - x, color.clone());

            x += 1;
        }
    }
}
