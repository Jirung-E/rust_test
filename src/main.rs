mod test;
use test::*;

fn main() {
    println!("함수 선언을 아래쪽에 해도 된다.");

    let mut x = 10;
    let y = 2;

    println!("{} + {} = {}", x, y, test_func(10, 2));

    test_remainder();

    x = 10;
    test_match(x);

    test_for();

    test_if_else();

    test_loop();
    test_while();
    test_for_array();
}