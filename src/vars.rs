const MAX_POINTS: i32 = 100_000;

pub fn run() {
    println!("Here is vars module!!");

    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    let _i1 = 3;
    let _f1 = 0.1;

    println!("{}", usize::BITS);

    println!(
        "Memory address of MAX_POINTS is: {:p}",
        &MAX_POINTS as *const i32
    );

    let i2: i64 = 1;
    let i3: i64 = 2;

    println!("Stack address of i2 is: {:p}", &i2 as *const i64);
    println!("Stack address of i3 is: {:p}", &i3 as *const i64);

    let y = 5;
    println!("Stack address of y is: {:p}", &y as *const i32);
    let y = y + 1;
    println!("Stack address of y is: {:p}", &y as *const i32);
    let y = y * 2;
    println!("Stack address of y is: {:p}", &y as *const i32);
    println!("The value of y is: {}", y);
    {
        let y = 0;
        println!("Stack address of y is: {:p}", &y as *const i32);
        println!("The value of y is: {}", y);
    }
    println!("The value of y is: {}", y);

    let t1 = (500, 6.4, "hello");
    let (x, y, z) = t1;
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
    println!("The value of z is: {}", z);
    println!("The value of t1 is: {} {} {}", t1.0, t1.1, t1.2);

    let mut t2 = ((0, 1), (2, 3));
    println!("{:?}", t2);
    let ((ref mut x1_ptr, ref mut y1_ptr), _) = t2;
    *x1_ptr = 5;
    *y1_ptr = 6;
    println!("{:#?}", t2);

    let a1 = [1, 2, 3, 4, 5];
    let a2 = [0; 10];
    println!("{:?} {:?} {} {}", a1, a2, a1[2], a1[3]);

    let s1 = "helloこんにちは挨拶"; // 1byte * 5 + 3bytes * 7 = 26bytes
    let s2 = "hello"; // 1byte * 5 = 5bytes
    println!("Stack address of s1 is: {:p}", &s1 as *const &str);
    println!("Stack address of s2 is: {:p}", &s2 as *const &str);
    println!("Static memory address of s1 is: {:?}", s1.as_ptr());
    println!("Static memory address of s2 is: {:?}", s2.as_ptr());
    println!("Len of s1 is: {}", s1.len());
    println!("Len of s2 is: {}", s2.len());

    let mut s3 = String::from("hello");
    let mut s4 = String::from("Hello, World!");
    println!("Stack address of s3 is: {:p}", &s3 as *const String);
    println!("Stack address of s4 is: {:p}", &s4 as *const String);
    s3.push_str(", World!");
    s4.push_str(stringify!(__FILE__));
    println!("{}", s3);
    println!("{}", s4);
}
