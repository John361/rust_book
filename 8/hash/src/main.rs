use std::collections::HashMap;

fn main() {
    // create and add
    let mut points: HashMap<String, u8> = HashMap::new();
    points.insert(String::from("Red"), 10);
    points.insert(String::from("Blue"), 50);

    // create from two tuples
    let teams = vec![ // ownership to points
        String::from("Blue"),
        String::from("Red")];
    let initial_points: Vec<_> = vec![10, 50];

    let mut points: HashMap<String, u8> = teams.into_iter()
        .zip(initial_points.into_iter())
        .collect();

    // get value
    match points.get("Blue") {
        Some(points) => println!("Blue team exists with {0} points", points),
        None => println!("Blue team does not exist")
    }

    // loop
    for (key, value) in &points {
        println!("key = {0}, value = {1}", key, value);
    }

    // replace
    points.insert(String::from("Blue"), 70);
    println!("{:?}", points);

    // insert only if there is no value
    let mut points = HashMap::new();
    points.insert(String::from("Blue"), 10);
    points.entry(String::from("Red")).or_insert(50);
    points.entry(String::from("Blue")).or_insert(50);
    println!("{:?}", points);

    // edit depending of old value
    let text = "Hello world beautiful world";
    let mut table = HashMap::new();

    for word in text.split_whitespace() {
        let cpt = table.entry(word).or_insert(0);
        *cpt += 1;
    }

    println!("{:?}", table);
}
