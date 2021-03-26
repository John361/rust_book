fn main() {
    // use String like Vec
    let mut s = String::new();
    s.push('s');
    println!("s = {0}", s);

    let mut s =  "你好".to_string(); // same as String::from("你好")  // String = UTF-8
    s.push_str(" 世界!"); // does not take ownership
    println!("s = {0}", s);

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;
    println!("s3 = {0}", s3); // s1 cannot be used anymore -> fn add(self, s: &str) -> String

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{}-{}-{}", s1, s2, s3); // does not take ownership
    println!("s = {0}", s);

    // access to chars // does not work using index access
    for c in s1.chars() {
        println!("c = {0}", c);
    }
}
