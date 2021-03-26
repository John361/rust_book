fn main() {
    // mutable user from function
    let mut user1 = default_user(String::from("John"), String::from("Doe"));

    println!("first name = {0}, last name = {1}, active = {2}",
             user1.first_name,
             user1.last_name,
             user1.active
    );

    user1.last_name = String::from("Done");
    println!("last name = {0}", user1.last_name);

    // from user1
    let user2 = User {
        active: false,
        ..user1
    };
    println!("first name = {0}, last name = {1}, active = {2}",
             user2.first_name,
             user2.last_name,
             user2.active
    );

    // structure tuple
    let color = Color(3, 2, 1);
    println!("Color 1 = {0}, 2 = {1}, 3 = {2}", color.0, color.1, color.2);
}

struct User {
    first_name: String,
    last_name: String,
    active: bool
}

fn default_user(first_name: String, last_name: String) -> User {
    User {
        first_name,
        last_name,
        active: true
    }
}

struct Color(i32, i32, i32);
