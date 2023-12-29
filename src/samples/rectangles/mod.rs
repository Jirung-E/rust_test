mod point;
use point::Point;
mod rectangle;
use rectangle::Rectangle;
mod canvas;
use canvas::Canvas;

pub fn test() {
    print!("{}[2J", 27 as char);

    println!(" [ rectangles ] ");
    let mut canvas = Canvas::new();
    let rect = Rectangle::new(5, 5);
    canvas.draw(&rect, &Point::new(2, 2));
    // canvas.print();

    canvas.brush = '#';
    canvas.draw(&Rectangle::new(2, 4), &Point::new(10, 10));
    // canvas.print();

    canvas.brush = '*';
    canvas.draw(&Rectangle::new(20, 40), &Point::new(52, -30));
    // canvas.print();

    canvas.brush = '+';
    canvas.draw(&Rectangle::new(30, 23), &Point::new(25, 28));

    let stamp = Rectangle::new(10, 10);
    canvas.brush = '%';
    canvas.draw(&stamp, &Point::new(30, 14));
    canvas.draw(&stamp, &Point::new(28, 10));
    canvas.brush = ' ';
    canvas.draw(&stamp, &Point::new(24, 16));
    canvas.brush = '.';
    canvas.draw(&Rectangle::new(6, 6), &Point::new(26, 18));

    canvas.print();
}
