fn main() {
    /* let local_v4 = IpAddress {
        family: IpAddressFamily::V4,
        address: String::from("127.0.0.1")
    };
    println!("local_v4 = {:#?}", local_v4); */

    /* let local_v4 = IpAddress::V4(127, 0, 0, 1);
    let local_v6 = IpAddress::V6(String::from("::1"));

    println!("local_v4 = {:?}, local_v6 = {:?}", local_v4, local_v6); */

    /* let message = Message::Write(String::from("Hello world"));
    message.call(); */

    /* let number = Some(42);
    let string = Some("Hello");
    println!("number = {0}, string = {1}", number.unwrap(), string.unwrap()); */

    /* let q1 = UsPiece::Quarter(UsState::Alabama);
    println!("cent value of q1 = {0}", cent_value(q1)); */

    /* let five = Some(5);
    let six = plus_one(five);
    println!("five = {0}, six = {1}", five.unwrap(), six.unwrap()); */

    // if let
    let u8_value = Some(0u8);
    match u8_value {
        Some(3) => println!("three"),
        _ => (),
    }

    if let Some(3) = u8_value {
        println!("three");
    } else {
        println!("not three");
    }
}

/* #[derive(Debug)]
enum IpAddressFamily {
    V4,
    V6
}

#[derive(Debug)]
struct IpAddress {
    family: IpAddressFamily,
    address: String
} */

/* #[derive(Debug)]
enum IpAddress {
    V4(u8, u8, u8, u8),
    V6(String)
} */

/* #[derive(Debug)]
enum Message {
    Quit,
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        match self {
            Message::Quit => println!("Quit is called"),
            Message::Write(s) => println!("Write is called with s = {0}", s),
            Message::ChangeColor(x, y, z) => println!("Change color is called: {0}, {1}, {2}", x, y, z)
        }
    }
} */

/* #[derive(Debug)]
enum UsPiece {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState)
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska
}

fn cent_value(piece: UsPiece) -> u8 {
    match piece {
        UsPiece::Penny => 1,
        UsPiece::Nickel => 5,
        UsPiece::Dime => 10,
        UsPiece::Quarter(state) => {
            println!("Currently in state: {:?}", state);
            25
        }
    }
} */

/* fn plus_one(x: Option<u8>) -> Option<u8> {
    match x {
        None => None,
        Some(i) => Some(i + 1)
    }
} */
