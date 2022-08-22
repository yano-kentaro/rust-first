pub fn run() {
    let s1 = String::from("hello");
    let s2 = s1;
    println!("{}", s2);

    let i1 = 1;
    let i2 = i1;
    println!("{} {}", i1, i2);
    println!("Stack address of i1 is: {:p}", &i1 as *const i32);
    println!("Stack address of i2 is: {:p}", &i2 as *const i32);

    let sl1 = "hello";
    let sl2 = sl1;
    println!("{} {}", sl1, sl2);
    println!("Stack address of sl1 is: {:p}", &sl1 as *const &str);
    println!("Stack address of sl2 is: {:p}", &sl2 as *const &str);
}
