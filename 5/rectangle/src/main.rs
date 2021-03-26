fn main() {
    /* let width: u32 = 30;
    let height: u32 = 50;
    println!("rectangle area = {0}", area(width, height)); */

    /* let rectangle = Rectangle {
        width: 30,
        height: 50
    };
    println!("rectangle = {:#?}", rectangle);
    println!("rectangle area = {0}", area(&rectangle)); */

    /* let rectangle = Rectangle {
        width: 30,
        height: 50
    };
    println!("rectangle area = {0}", rectangle.area());

    let rectangle2 = Rectangle {
        width: 10,
        height: 60
    };
    println!("rectangle can handle rectangle2: {0}", rectangle.can_handle(&rectangle2)); */

    let square = Rectangle::square(7);
    println!("square = {:#?}", square);
}

/* fn area(width: u32, height: u32) -> u32 {
    width * height
} */

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

/* fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
} */

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_handle(&self, rectangle: &Rectangle) -> bool {
        self.width > rectangle.width && self.height > rectangle.height
    }

    fn square(nb: u32) -> Rectangle {
        Rectangle {
            width: nb,
            height: nb
        }
    }
}
