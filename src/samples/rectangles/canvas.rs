use super::point::Point;
use super::rectangle::Rectangle;

const WIDTH: usize = 64;
const HEIGHT: usize = 32;

pub struct Canvas {
    pub brush: char,
    pixel: [[char; WIDTH]; HEIGHT],
}

impl Canvas {
    pub fn new() -> Canvas {
        Canvas {
            brush: '@',
            pixel: [['.'; WIDTH]; HEIGHT],
        }
    }

    pub fn print(&self) {
        for row in &self.pixel {
            for p in row {
                print!("{} ", p);
            }
            println!();
        }
    }

    pub fn draw(&mut self, rect: &Rectangle, position: &Point) {
        let mut start_x = position.x;
        let mut start_y = position.y;
        let mut end_x = start_x + rect.width as i32;
        let mut end_y = start_y + rect.height as i32;

        if start_x >= WIDTH as i32 { return }
        if start_y >= HEIGHT as i32 { return };
        if end_x <= 0 { return };
        if end_y <= 0 { return };

        if start_x < 0 { start_x = 0 }
        if start_y < 0 { start_y = 0 }
        if end_x > WIDTH as i32 { end_x = WIDTH as i32 };
        if end_y > HEIGHT as i32 { end_y = HEIGHT as i32 };

        let start_x = start_x as usize;
        let start_y = start_y as usize;
        let end_x = end_x as usize;
        let end_y = end_y as usize;

        for y in start_y..end_y {
            for x in start_x..end_x {
                self.pixel[y][x] = self.brush;
            }
        }
    }
}