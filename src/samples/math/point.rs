use std::ops::{Add, AddAssign, Mul};

#[derive(Debug, Copy, Clone)]
pub struct Point {
    x: f64,
    y: f64,
    z: f64
}

impl Point {
    pub fn new(x: f64, y: f64, z: f64) -> Point {
        Point { x, y, z }
    }
    pub fn from_i32(x: i32, y: i32, z: i32) -> Point {
        Point {
            x: x as f64,
            y: y as f64,
            z: z as f64
        }
    }
    pub fn x(&mut self) -> f64 { self.x }
    pub fn y(&mut self) -> f64 { self.y }
    pub fn z(&mut self) -> f64 { self.z }
}

impl Add for Point {
    type Output = Point;
    fn add(self, rhs: Point) -> Point {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z
        }
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, rhs: Point) {
        *self = Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z
        }
    }
}

impl Mul for Point {
    type Output = Point;
    fn mul(self, rhs: Point) -> Point {
        Point {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z
        }
    }
}

impl Mul<f64> for Point {
    type Output = Point;
    fn mul(self, rhs: f64) -> Point {
        Point {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs
        }
    }
}
