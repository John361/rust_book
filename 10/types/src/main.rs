fn main() {
    let list = vec![34, 50, 25, 100, 65];
    let mut result = list[0];

    for item in list {
        if item > result {
            result = item;
        }
    }

    println!("greater = {}", result);

    let list = vec![34, 50, 25, 100, 65];
    let result = greater_n(&list);
    println!("greater = {}", result);
    let list = vec!['y', 'm', 'a', 'q'];
    let result = greater_c(&list);
    println!("greater = {}", result);

    let list = vec![34, 50, 25, 100, 65];
    let result = greater(&list);
    println!("greater = {}", result);
    let list = vec!['y', 'm', 'a', 'q'];
    let result = greater(&list);
    println!("greater = {}", result);

    let point = Point { x: 20, y: 30 };
    let point = Point { x: 20.0, y: 30.0 };
    println!("distance = {}", point.distance());
    let point = Point { x: 20, y: 30.0 };
    println!("point.x = {}", point.x());

    let point1 = Point { x: 'a', y: 'b' };
    let point2 = Point { x: 20, y: 30 };
    let point = point1.mix(point2);
    println!("point.x = {0}, point.y = {1}", point.x, point.y)
}

fn greater_n(list: &[i32]) -> i32 {
    let mut greater = list[0];

    for &item in list {
        if item > greater {
            greater = item;
        }
    }

    greater
}

fn greater_c(list: &[char]) -> &char {
    let mut greater = &list[0];

    for item in list.iter() {
        if item > greater {
            greater = item;
        }
    }

    greater
}

fn greater<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut greater = &list[0];

    for item in list {
        if item > greater {
            greater = item;
        }
    }

    greater
}

struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {

    fn x(&self) -> &T {
        &self.x
    }

    fn y(&self) -> &U {
        &self.y
    }
}

impl Point<f32, f32> {
    fn distance(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

impl<T, U> Point<T, U> {
    fn mix<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}
