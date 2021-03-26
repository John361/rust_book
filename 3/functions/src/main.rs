fn main() {
    say_hello_to('R');
    let x = add_one(1);
    println!("x = {0}", x);
}

fn say_hello_to(initial: char) {
    println!("Hello {0}", initial)
}

fn add_one(nb: i32) -> i32 {
    nb + 1
}
