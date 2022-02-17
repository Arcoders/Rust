fn main() {
    get_element_in_position(2);
    get_element_in_position(100);
    map_vector();
    vector_variants();
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