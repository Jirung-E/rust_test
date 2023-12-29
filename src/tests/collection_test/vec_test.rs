pub fn test() {
    println!(" [ Vec test ] ");

    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);
    println!("Vec::new(), push(1), push(2), push(3): ");
    println!("  {:?}", v);

    let v = vec![1, 2, 3];
    println!("vec![1, 2, 3]: ");
    println!("  {:?}", v);

    let v1: &i32 = &v[1];
    println!("v[1]: {}", v1);
    let v_get1: Option<&i32> = v.get(1);
    println!("v.get(1): {:?}", v_get1);
    println!("v.get(4): {:?}", v.get(4));

    let mut v = v;
    for i in &mut v {
        *i += 10;
    }
    println!("v += 10:");
    println!("  {:?}", v);

    println!(" -- Store multiple types -- ");

    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    println!("  {:?}", row);

    println!("pop(): ");
    v.pop();
    println!("  {:?}", v);

    v.push(1);
    v.push(82);
    v.push(24);
    println!("push(..):");
    println!("  {:?}", v);

    v.remove(3);
    v.remove(1);
    println!("remove(1, 3):");
    println!("  {:?}", v);

    v.insert(2, 100);
    println!("insert(2, 100): ");
    println!("  {:?}", v);
}