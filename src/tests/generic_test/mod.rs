mod point;
use point::Point;


pub fn test() {
    println!(" [ generic test ] ");

    let numbers = [-34, 50, 25, -100, 65];
    print!("type: "); print_type_of(numbers[0]);
    println!("numbers: {:?}", numbers);
    println!("largest: {}", largest(&numbers));

    let numbers: Vec<u32> = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    print!("type: "); print_type_of(numbers[0]);
    println!("numbers: {:?}", numbers);
    println!("largest: {}", largest(&numbers));

    let str = vec!["Hello", "Hi", "world", "rust"];
    print!("type: "); print_type_of(str[0]);
    println!("numbers: {:?}", str);
    println!("largest: {}", largest(&str));

    println!();

    let int_point = Point { x: 1, y: 2 };
    let float_point = Point { x: 10.8, y: 4.23 };
    let mix = Point::mixup(&int_point, &float_point);
    println!("x: {}, y: {}", int_point.x, int_point.y);
    println!("x: {}, y: {}", float_point.x, float_point.y);
    println!("x: {}, y: {}", mix.x, mix.y);
}

fn print_type_of<T>(_: T) {
    println!("{}", std::any::type_name::<T>());
}

fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for e in list.iter() {
        if e > largest {
            largest = e;
        }
    }
    largest
}