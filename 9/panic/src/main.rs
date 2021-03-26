use std::fs::File;
use std::io;
use std::io::Read;

fn main() {
    // causing panic
    // panic!("Crash and burn!");
    // let v = vec![1, 2, 3];
    // v[99]; // RUST_BACKTRACE=1 cargo run

    // panic if does not exist
    // let f: File = File::open("hello.txt").expect("Cannot open file");

    // using multiple match
    // let f: Result<File, io::Error> = File::open("hello.txt");
    // let f = match f {
    //     Ok(file) => file,
    //     Err(error) => match error.kind() {
    //         io::ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(fc) => {
    //                 println!("File does not exists, created it");
    //                 fc
    //             },
    //             Err(error) => panic!("Cannot create file {:?}", error)
    //         },
    //         other => panic!("Error occurred {:?}", other)
    //     }
    // };

    // more concise
    // let f: File = File::open("hello.txt").unwrap_or_else(|error| {
    //     if error.kind() == io::ErrorKind::NotFound {
    //         File::create("hello.txt").unwrap_or_else(|error| {
    //             panic!("Cannot create file {:?}", error);
    //         })
    //     } else {
    //         panic!("Error occurred {:?}", error);
    //     }
    // });

    match read_username_from_file() {
        Ok(s) => println!("username = {0}", s),
        Err(error) => println!("Error while reading username: {:?}", error)
    }
}

// does not manage error, let caller manage it
// fn read_username_from_file() -> Result<String, io::Error> {
//     let f = File::open("username.txt");
//     let mut f = match f {
//         Ok(file) => file,
//         Err(e) => return Err(e),
//     };
//
//     let mut s = String::new();
//
//     match f.read_to_string(&mut s) {
//         Ok(_) => Ok(s),
//         Err(e) => Err(e),
//     }
// }

// more concise way
fn read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open("username.txt")
        ?.read_to_string(&mut s); // ?. cannot be used only in function that returns Result or Option

    Ok(s)
}
