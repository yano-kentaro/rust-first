pub fn run() {
    // let a1: [u8; 90_000_000] = [1; 90_000_000];

    // vector型
    let v1 = vec![1, 2, 3, 4, 5];
    let v2 = vec![5, 6, 7, 8, 9];
    let v3 = vec![9, 10];
    println!("Stack address of v1 is: {:p}", &v1 as *const Vec<i32>);
    println!("Stack address of v2 is: {:p}", &v2 as *const Vec<i32>);
    println!("Stack address of v3 is: {:p}", &v3 as *const Vec<i32>);
}
