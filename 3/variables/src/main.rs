const MAX_POINT: u8 = 10;

fn main() {
    // constant
    println!("MAX_POINT value is: {0}", MAX_POINT);

    // immutable
    let x: u8 = 3;
    println!("x value is: {0}", x);

    // mutable
    let mut y: u8 = 4;
    println!("y value is: {0}", y);
    y = 5;
    println!("y value is: {0}", y);

    // mask
    let z = true;
    println!("z value is: {0}", z);
    let z: f32 = 6.7;
    println!("z value is: {0}", z);

    // char
    let c: char = 'd';
    println!("c value is: {0}", c);

    // tuple
    let tuple: (i8, bool, char) = (3, false, 'x');
    let (x, _, z) = tuple;
    println!("x value from tuple is: {0}", x);
    println!("y value from tuple is: {0}", tuple.1);
    println!("z value from tuple is: {0}", z);

    // table
    let table: [i8; 3] = [1, 2, 3];
    let table3 = [3; 5];
}
