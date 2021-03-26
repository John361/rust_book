fn main() {
    // one if else
    let number = 3;

    if number < 5 {
        println!("Condition verified");
    } else {
        println!("Condition unverified");
    }

    // multiple if else
    let number = 6;

    if number % 4 == 0 {
        println!("Divisible 4");
    } else if number % 3 == 0 {
        println!("Divisible 3");
    } else if number % 2 == 0 {
        println!("Divisible 2");
    } else {
        println!("Non divisible 4, 3 or 2");
    }

    // expression
    let condition = true;
    let number = if condition { 1 } else { 2 };
    println!("number = {0}", number);

    // loop
    let mut number = 0;

    let result = loop {
        number += 1;

        if number == 10 {
            break number * 2;
        }
    };

    println!("result = {0}", result);

    // while
    let mut number = 3;

    while number != 0 {
        println!("in while, number = {0}", number);

        number -= 1;
    }

    println!("while finished");

    // while with table
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("index = {0}", a[index]);

        index += 1;
    }

    // for
    let a = [10, 20, 30, 40, 50];

    for item in a.iter() {
        println!("item = {0}", item);
    }

    // for with rev
    for number in (1..4).rev() {
        println!("number = {0}", number);
    }
    println!("for rev finished");
}
