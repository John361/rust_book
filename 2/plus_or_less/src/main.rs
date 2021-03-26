use std::io;
use rand::Rng;
use rand::rngs::ThreadRng;
use std::cmp::{Ordering};

const MIN: u8 = 0;
const MAX: u8 = 100;

fn main() {
    println!("Guess number!");

    let mut max_try: u8 = 3;
    let computer_number: u8 = computer_number();
    println!("computer number is {0}", computer_number);

    while max_try > 0 {
        let user_number: u8 = ask_user_number();
        let result: bool = compare_numbers(user_number, computer_number);

        if result {
            break;
        }

        max_try = max_try - 1;
    }

    if max_try == 0 {
        println!("Try again...");
    }
}

fn computer_number() -> u8 {
    let mut random_generator: ThreadRng = rand::thread_rng();
    random_generator.gen_range(MIN..MAX + 1)
}

fn ask_user_number() -> u8 {
    let mut guess: String = String::new();
    let mut number: u8 = 101;

    while number < MIN || number > MAX {
        println!("Please enter a number between {0} and {1}", MIN, MAX);

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read user input");

        let trimmed: &str = guess.trim();
        match trimmed.parse::<u8>() {
            Ok(i) => {
                number = i;
            },
            Err(..) => println!("This is not a number between {0} and {1}", MIN, MAX)
        };
    }

    number
}

fn compare_numbers(user_number: u8, computer_number: u8) -> bool {
    match user_number.cmp(&computer_number) {
        Ordering::Less => {
            println!("It is more");
            false
        },
        Ordering::Greater => {
            println!("It is less");
            false
        },
        Ordering::Equal => {
            println!("You won!");
            true
        }
    }
}
