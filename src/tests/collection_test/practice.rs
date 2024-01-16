use std::collections::HashSet;

pub fn practice() {
    println!(" [ practice ] ");
    // list_count();
    // pig_latin();
    cli();
}


fn list_count() {
    use std::collections::HashMap;

    let list = vec![1, 2, 3, 4, 5, 6, 7, 3, 5, 1, 7, 12, 8, 9, 31, 11, 53, 23, 23, 6, 7, 19, 0, 8, 6, 2, 1, 2];
    println!("list: {list:?}");

    let mut map: HashMap<i32, i32> = HashMap::new();
    for e in &list {
        let count = map.entry(*e).or_insert(0);
        *count += 1;
    }
    // println!("count: {map:#?}");

    let mut max = 0;
    let mut mode_list = Vec::new();
    for (key, value) in map {
        if value > max {
            mode_list.clear();
            mode_list.push(key);
            max = value;
        }
        else if value == max {
            mode_list.push(key);
        }
    }
    println!("mode: {mode_list:?}");

    let mut list = list;
    list.sort();
    println!("sorted list: {list:?}");
    let len = list.len();
    println!("median: {}", list[len/2]);
}

fn pig_latin() {
    use std::io::{self, Write};

    let mut s = String::new();
    print!("Enter a word: ");
    io::stdout().flush().expect("Could not flush stdout");
    io::stdin().read_line(&mut s).expect("Failed to read line");
    let s = s.trim();

    println!("s: {}", s);

    let mut result = String::from(s);
    let first = result.chars().next().unwrap().to_lowercase().next().unwrap();
    result = match first {
        'a' | 'e' | 'i' | 'o' | 'u' => {
            format!("{}-hay", result)
        },
        _ => {
            result.remove(0);
            format!("{}-{}ay", result, first)
        }
    };

    println!("result: {}", result);
}

fn cli() {
    use std::collections::HashMap;
    use std::io::{self, Write};

    let mut departments: HashMap<String, HashSet<String>> = HashMap::new();

    loop {
        print!("> ");
        io::stdout().flush().expect("Could not flush stdout");
        let mut s = String::new();
        match io::stdin().read_line(&mut s) {
            Ok(_) => {
                let command = Vec::from_iter(s.split_whitespace());
                let length = command.len();
                if length == 0 {
                    continue;
                }
                let argc = length - 1;

                match command[0] {
                    "show" => {
                        if argc == 0 {
                            println!("departments: {:#?}", departments);
                        }
                        else {
                            match departments.get(command[1]) {
                                Some(department) => {
                                    println!("{}: {:#?}", command[1], department);
                                },
                                None => {
                                    println!("{}: no such department", command[1]);
                                }
                            }
                        }
                    },
                    "add" => {
                        if argc < 3 {   // add [args] to [department]
                            println!("invalid syntax");
                            continue;
                        }

                        let mut args = Vec::new();
                        let mut to_offset: Option<usize> = None;
                        for (i, arg) in command[1..].iter().enumerate() {
                            if arg.eq(&"to") {
                                to_offset = Some(i);
                                break;
                            }
                            args.push(arg);
                        }

                        match to_offset {
                            Some(offset) => {
                                if offset == 0 || offset == argc-1 {
                                    println!("invalid syntax");
                                    continue;
                                }

                                if argc - args.len() - 2 == 0 && command[argc] == "*" {
                                    for department in departments.values_mut() {
                                        department.extend(args.iter().map(|s| s.to_string()));
                                    }
                                    continue;
                                }

                                for department in &command[offset+2..] {
                                    for arg in &args {
                                        departments.entry(department.to_string()).or_insert(HashSet::new()).insert(arg.to_string());
                                    }
                                }
                            },
                            None => {
                                println!("invalid syntax");
                            }
                        }
                    },
                    "remove" => {
                        if argc == 0 {  // remove [args] from [department], remove [department]
                            println!("invalid syntax");
                            continue;
                        }

                        let mut args: Vec<&str> = Vec::new();
                        let mut from_offset: Option<usize> = None;
                        for (i, arg) in command[1..].iter().enumerate() {
                            if arg.eq(&"from") {
                                from_offset = Some(i);
                                break;
                            }
                            args.push(arg);
                        }

                        match from_offset {
                            Some(offset) => {
                                if offset == 0 || offset == argc-1 {
                                    println!("invalid syntax");
                                    continue;
                                }

                                if args[0].eq("*") && args.len() == 1 {
                                    for department in departments.values_mut() {
                                        department.clear();
                                    }
                                    continue;
                                }

                                if argc - args.len() - 2 == 0 && command[argc] == "*" {
                                    for department in departments.values_mut() {
                                        for arg in args.iter() {
                                            department.remove(*arg);
                                        }
                                    }
                                    continue;
                                }

                                for department in &command[offset+2..] {
                                    for arg in &args {
                                        departments.entry(department.to_string()).or_insert(HashSet::new()).remove(*arg);
                                    }
                                }
                            },
                            None => {
                                if argc == 1 && command[1] == "*" {
                                    departments.clear();
                                }
                                else {
                                    for arg in &command[1..] {
                                        departments.remove(*arg);
                                    }
                                }
                            }
                        }
                    },
                    _ => {
                        println!("invalid command");
                    }
                }
            },
            Err(_) => {
                println!("invalid syntax - try again");
                continue;
            }
        }


    }
}