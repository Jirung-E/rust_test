pub fn test() {
    println!(" [ Pyramid ] ");
    let mut pyramid = Pyramid::new(5);
    pyramid.show();

    println!();

    pyramid.size = 7;
    pyramid.direction = Direction::RT;
    pyramid.stamp = '#';
    pyramid.show();

    println!();

    pyramid.size = 3;
    pyramid.direction = Direction::LB;
    pyramid.stamp = '%';
    pyramid.show();

    println!();

    pyramid.size = 10;
    pyramid.direction = Direction::RB;
    pyramid.stamp = '+';
    pyramid.show();
}

#[derive(Debug)]
enum Direction {
    LT, RT, LB, RB
}

struct Pyramid {
    size: usize,
    stamp: char,
    direction: Direction
}

impl Pyramid {
    fn new(size: usize) -> Pyramid {
        Pyramid {
            size,
            stamp: '*',
            direction: Direction::LT
        }
    }

    fn show(&self) {
        println!("size: {}, direction: {:?}", self.size, self.direction);
        match self.direction {
            Direction::LT => {
                for i in (0..self.size).rev() {
                    for _ in 0..=i {
                        print!("{}", self.stamp);
                    }
                    println!();
                }
            }
            Direction::RT => {
                for i in 0..self.size {
                    for _ in 0..i {
                        print!(" ");
                    }
                    for _ in 0..self.size-i {
                        print!("{}", self.stamp);
                    }
                    println!();
                }
            }
            Direction::LB => {
                for i in 0..self.size {
                    for _ in 0..=i {
                        print!("{}", self.stamp);
                    }
                    println!();
                }
            }
            Direction::RB => {
                for i in (0..self.size).rev() {
                    for _ in 0..i {
                        print!(" ");
                    }
                    for _ in 0..self.size-i {
                        print!("{}", self.stamp);
                    }
                    println!();
                }
            }
        }
    }
}