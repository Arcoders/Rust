use std::collections::HashMap;

fn main() {
    get_element_in_position(2);
    get_element_in_position(100);
    map_vector();
    vector_variants();
    hash_maps();
    split_words();
}

fn get_element_in_position(position: usize) {
    let v = vec![76, 34, 22];
    match  v.get(position) {
        Some(element) => println!("The element is {}", element),
        None => println!("There is no element."),
    }
}

fn map_vector() {
    let mut v = vec![4, 6, 8];
    for i in &mut v {
        *i *= 2;
    }

    for i in &v {
        println!("{}", i);
    }
}

fn vector_variants() {
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("Yellow")),
        SpreadsheetCell::Float(10.12),
    ];

    match &row[1] {
        SpreadsheetCell::Int(i) => println!("{}", i),
        _=> println!("Not a integer!")
    };
}

fn hash_maps() {
    let blue = String::from("Blue");
    let yellow = String::from("Yellow");

    let mut scores = HashMap::new();

    scores.insert(blue, 10);
    scores.insert(yellow, 50);
    scores.entry(String::from("Red")).or_insert(30);

    for (key, value) in &scores {
        println!("{} {}", key, value);
    }
} 

fn split_words() {
    let text = "Lorem ipsum dolor sit amet";
    let mut map = HashMap::new();

    for word in text.split_ascii_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}