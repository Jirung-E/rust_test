pub struct Point<T, U> {
    pub x: T,
    pub y: U
}

impl<T: std::clone::Clone, U> Point<T, U> {
    pub fn mixup<V, W: std::clone::Clone>(p1: &Point<T, U>, p2: &Point<V, W>) -> Point<T, W> {
        Point {
            x: p1.x.clone(),
            y: p2.y.clone()
        }
    }
}
