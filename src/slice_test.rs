pub fn test() {
    println!("[slice test]");
    test_string_slice();
    println!();
    test_array_slice();
}

fn test_string_slice() {
    let s = String::from("hello world");
    println!("string: {}", s);
    println!("first word: {}", &s[..space_index(&s)]);
    println!("first word: {}", first_word(&s));
    println!("first word: {}", first_word_match(&s));
    println!("second word: {}", second_word(&s));

    let s = "Hi, rust!";
    println!("string: {}", s);
    println!("first word: {}", &s[..space_index(s)]);
    println!("first word: {}", first_word(s));
    println!("first word: {}", first_word_match(s));
    println!("second word: {}", second_word(s));
}

fn space_index(s: &str) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    s
}

fn first_word_match(s: &str) -> &str {
    let offset = s.find(' ');
    match offset {
        Some(offset) => &s[..offset],
        None => s
    }
}

fn second_word(s: &str) -> &str {
    let offset = s.find(' ');
    match offset {
        Some(offset) => {
            let s = &s[offset+1..];
            let offset = s.find(' ');
            match offset {
                Some(offset) => &s[..offset],
                None => s
            }
        },
        None => ""
    }
}


fn test_array_slice() {
    let a = [1, 2, 3, 4, 5];
    println!("a: {:?}", a);
    let slice = &a[1..3];
    println!("slice: {:?}", slice);
}
