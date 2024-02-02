use std::fs::File;
use std::io::ErrorKind;
use std::io::Read;
use std::io;


pub fn test() {
    // let mut file1 = get_file_1("test1.txt");
    // let mut file2 = get_file_2("test2.md");
    // let mut file3 = get_file_3("test3.json");
    // let mut buff = String::new();
    // file1.read_to_string(&mut buff).unwrap();
    // println!("{:?}", buff);
    // let mut buff = String::new();
    // file2.read_to_string(&mut buff).unwrap();
    // println!("{:?}", buff);
    // let mut buff = String::new();
    // file3.read_to_string(&mut buff).unwrap();
    // println!("{:?}", buff);

    // let file4 = File::open("test4.txt").unwrap();
    // let file5 = File::open("hello.txt")
    //     .expect("hello.txt should be included in this project");
}

fn get_file_1(file_name: &str) -> File {
    let greeting_file_result = File::open(file_name);
    match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create(file_name) {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error)
            }
        }
    }
}

fn get_file_2(file_name: &str) -> File {
    let greeting_file_result = File::open(file_name);
    greeting_file_result.unwrap_or_else(|error| match error.kind() {
        ErrorKind::NotFound => match File::create(file_name) {
            Ok(fc) => fc,
            Err(e) => panic!("Problem creating the file: {:?}", e),
        },
        other_error => {
            panic!("Problem opening the file: {:?}", other_error)
        }
    })
}

fn get_file_3(file_name: &str) -> File {
    File::open(file_name).unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create(file_name).unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        }
        else {
            panic!("Problem opening the file: {:?}", error);
        }
    })
}

fn read_username_from_file_1() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");
    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e)
    };

    let mut username = String::new();
    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

fn read_username_from_file_2() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;

    let mut username = String::new();
    username_file.read_to_string(&mut username)?;

    Ok(username)
}

fn read_username_from_file_3() -> Result<String, io::Error> {
    let mut username = String::new();
    File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username)
}

fn read_username_from_file_4() -> Result<String, io::Error> {
    use std::fs;

    fs::read_to_string("hello.txt")
}

fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}