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
}
