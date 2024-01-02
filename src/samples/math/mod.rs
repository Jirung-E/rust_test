mod point;
mod vector3;

// use point::Point;
// type Point = vector3::Vector3;
// use vector3::Vector3 as Point;
use vector3::Point;

pub fn test() {
    let mut p1 = Point::from_i32(10, 2, 12);
    let p2 = Point::from_i32(1, 2, 3);
    p1.x = -8.0;
    // let mut p3 = p1 + p2;
    // p3 += p3;
    println!("p1: {:?}", p1);
    println!("p2: {:?}", p2);
    // println!("p3: {:?}", p3);
    // println!("p1 * p2: {:?}", p1 * p2);
}