pub fn test() {
    let p1 = Point { x: 1, y: 0 };
    let p2 = Point { x: 2, y: 3 };
    println!("{:?}", p1+p2);        // Copy trait 구현에 의해 소유권이 이동하지 않고 p1, p2가 그대로 유지된다.
    println!("{:?}", p1);
    println!("{:?}", p2);

    println!();

    let mm = Millimeters(1000);
    let m = Meters(1);
    println!("mm: {:?}", mm);
    println!("m: {:?}", m);
    println!("mm + m {:?}", mm + m);

    println!();

    let person = Human;
    Pilot::fly(&person);
    Wizard::fly(&person);
    Human::fly(&person);
    person.fly();
}


use std::ops::Add;

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}


#[derive(Debug)]
struct Millimeters(u32);
#[derive(Debug)]
struct Meters(u32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}


trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}

impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*")
    }
}
