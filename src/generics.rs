pub fn run() {
    let number_list = vec![34, 50, 25, 100, 65];
    println!("The largest number is {}", largest_i32(number_list));

    let char_list = vec!['a', 'b', 'c', 'd', 'e'];
    println!("The largest is {}", largest(char_list));
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
