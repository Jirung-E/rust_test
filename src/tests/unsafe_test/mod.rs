pub fn test() {
    let mut v = vec![1, 2, 3, 4, 5, 6];

    let r = &mut v[..];

    {
        let (a, b) = r.split_at_mut(3);
        println!("{:?}", a);
        println!("{:?}", b);
    }

    {
        let (a, b) = split_at_mut(r, 3);
        println!("{:?}", a);
        println!("{:?}", b);
    }

    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }
}

use std::slice;

fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();
    let ptr = values.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

extern "C" {
    fn abs(input: i32) -> i32;
}