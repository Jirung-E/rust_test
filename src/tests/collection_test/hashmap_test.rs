pub fn test() {
    println!(" [ test ] ");

    use std::collections::HashMap;

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    println!("scores: {:#?}", scores);
    let blue_score = scores.get("Blue");
    println!("scores.get(\"Blue\"): {:?}", blue_score);

    let mut map = HashMap::new();
    map.insert(1, String::from("Apple"));
    map.insert(2, String::from("Banana"));
    map.insert(3, String::from("Cherry"));
    for (key, value) in &map {
        println!("{key}: {value}");
    }

    // Overwrite
    map.insert(1, String::from("Apricot"));
    println!("map.insert(1, String::from(\"Apricot\")):");
    for (key, value) in &map {
        println!("{key}: {value}");
    }

    // Insert if not exists
    map.entry(1).or_insert(String::from("Apple"));
    map.entry(4).or_insert(String::from("Durian"));
    println!("map.entry(1).or_insert(String::from(\"Apple\")):");
    println!("map.entry(4).or_insert(String::from(\"Durian\")):");
    for (key, value) in &map {
        println!("{key}: {value}");
    }

    // Update
    let text = "hello world wonderful world";
    let mut word_count = HashMap::new();
    for word in text.split_whitespace() {
        let count = word_count.entry(word).or_insert(0);
        *count += 1;
    }
    println!("text: {}", text);
    println!("word count: {:#?}", word_count);
}