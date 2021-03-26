use std::mem::take;

fn main() {
    // does not work
    /* let s1 = String::from("Hello");
    let s2 = s1;
    println!("s1 = {0} and s2 = {1}", s1, s2); */

    // works
    /* let s1 = String::from("Hello");
    let s2 = s1.clone();
    println!("s1 = {0} and s2 = {1}", s1, s2); */

    // does not work
    /* let s1 = String::from("Hello");
    take_ownership(s1);
    println!("s1 = {0}", s1); */

    // works
    /* let n1: u8 = 7;
    create_copy(n1);
    println!("n1 = {0}", n1); */

    // works
    /* let s1 = give_ownership();
    let s2 = take_and_back_ownership(s1.clone());
    println!("s1 = {0} and s2 = {1}", s1, s2); */

    // works
    /* let s1 = String::from("Hello");
    let n1 = get_length(&s1);
    println!("s1 = {0} and n1 = {1}", s1, n1); */

    // works
    /* let mut s1 = String::from("Hello");
    add_world(&mut s1);
    println!("s1 = {0}", s1); */

    // works
    /* let mut s1 = String::from("Hello world");
    let s2 = first_word(&s1);
    println!("s1 = {0} and s2 = {1}", s1, s2);

    // does not works
    s1 = String::from("Goodbye");
    println!("s1 = {0} and s2 = {1}", s1, s2); */
}

fn take_ownership(s: String) {
    println!("s = {0}", s);
}

fn create_copy(nb: u8) {
    println!("nb = {0}", nb);
}

fn give_ownership() -> String {
    String::from("Hello")
}

fn take_and_back_ownership(s: String) -> String {
    s
}

fn get_length(s: &String) -> usize {
    s.len()
}

fn add_world(s: &mut String) {
    s.push_str(" world")
}

fn first_word(s: &str) -> &str {
    let bytes: &[u8] = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}
