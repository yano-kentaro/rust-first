struct Point<T, U> {
    x: T,
    y: U,
}
impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

pub fn run() {
    let number_list = vec![34, 50, 25, 100, 65];
    println!("The largest number is {}", largest_i32(number_list));

    let char_list = vec!['a', 'b', 'c', 'd', 'e'];
    println!("The largest is {}", largest(char_list));

    let p1 = Point { x: 1, y: 2 };
    let p2 = Point { x: 1.0, y: 'a' };
    let p3 = p1.mixup(p2);
    println!("{} {}", p3.x, p3.y);
}

fn largest_i32(list: Vec<i32>) -> i32 {
    let mut largest = list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn largest<T: PartialOrd + Copy>(list: Vec<T>) -> T {
    let mut largest = list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}
