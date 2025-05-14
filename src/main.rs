mod geometrical_shapes;

use geometrical_shapes as gs;
use gs::{Displayable, Drawable};
use raster::{Color, Image};

fn main() {
    let mut image = Image::blank(1000, 1000);

    // let a = gs::Point::new(0, 0);
    // let b = gs::Point::new(50, 0);
    // let c = gs::Point::new(50, 50);
    // a.draw(&mut image);
    // b.draw(&mut image);
    // c.draw(&mut image);

    // A) 50, 50
    // B) 25, 70
    // C) 70, 80

    // gs::Line::new(
    //     &gs::Point::new(50, 50), //a
    //     &gs::Point::new(25, 70), //b
    //     Color::rgb(200, 200, 200),
    // )
    // .draw(&mut image);

    // gs::Line::new(
    //     &gs::Point::new(50, 50), //a
    //     &gs::Point::new(70, 80), //c
    //     Color::rgb(200, 200, 200),
    // )
    // .draw(&mut image);

    // gs::Line::new(
    //     &gs::Point::new(25, 70),  //c
    //     &gs::Point::new(70, 80), //a
    //     Color::rgb(200, 200, 200),
    // )
    // .draw(&mut image);

    // gs::Line::random(image.width, image.height).draw(&mut image);

    // gs::Point::random(image.width, image.height).draw(&mut image);

    // let rectangle = gs::Rectangle::new(&gs::Point::new(150, 150), &gs::Point::new(50, 50));
    // rectangle.draw(&mut image);

    gs::Triangle::new(
        &gs::Point::new(500, 500),
        &gs::Point::new(250, 700),
        &gs::Point::new(700, 800),
    )
    .draw(&mut image);

    for _ in 1..2 {
        gs::Circle::random(image.width, image.height).draw(&mut image);
    }

    raster::save(&image, "image.png").unwrap();
}

impl Displayable for Image {
    fn display(&mut self, x: i32, y: i32, color: Color) {
        if x >= 0 && x < self.width && y >= 0 && y < self.height {
            self.set_pixel(x, y, color).unwrap();
        }
    }
}
