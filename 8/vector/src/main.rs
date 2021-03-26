use std::fmt::Display;

fn main() {
    // declare and add
    let mut v1: Vec<u8> = Vec::new();
    v1.push(1);

    let mut v2 = vec![1, 2, 3];
    v2.push(4);

    // preferred if panic is obliged
    let v21 = &v2[1];
    println!("v21 = {0}", v21);

    // safe access
    match v2.get(2) {
        Some(v) => println!("v2.2 = {0}", v),
        None => println!("No value found")
    }

    // loop
    for i in &mut v2 {
        *i *= 2
    }

    let cell_line = vec![
        Cell::Int(3),
        Cell::Text(String::from("blue")),
        Cell::Float(10.12)
    ];

    for i in &cell_line {
        println!("i = {:?}", i);
    }
}

#[derive(Debug)]
enum Cell {
    Int(i32),
    Float(f64),
    Text(String)
}
