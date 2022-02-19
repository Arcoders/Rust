use std::fs::File;
use std::io;
use std::io::{ErrorKind, Read};

fn main() {
    get_file_using_match();
    get_file_using_closure();
    get_file_expect();
    let file = get_file_content();
    println!("{:?}", file);
}

fn get_file_using_match() {
    let file = File::open("src/secret.txt");
    
    match file {
        Ok(file) => println!("The file: {:?}", file),
        Err(error) => match  error.kind() {
            ErrorKind::NotFound => match File::create("src/secret.txt") {
                Ok(new_file) => println!("{:?}", new_file),
                Err(error) => panic!("Problem creating the file: {:?}", error),
            },
            other_err => {
                panic!("Problem opening the file: {:?}", other_err);
            }
        },
    };
}

fn get_file_using_closure() {

    let file = File::open("src/secret.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("secret.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error)
        }
    });

    println!("The file: {:?}", file);
}

fn get_file_expect() {
    let file = File::open("src/secret.txt").expect("Filed to open the file");
    println!("The file: {:?}", file);

}

fn get_file_content() -> Result<String, io::Error> {
    let mut file_content = String::new();
    File::open("src/secret.txt")?.read_to_string(&mut file_content)?;
    Ok(file_content)
}

