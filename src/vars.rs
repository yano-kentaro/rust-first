pub mod sub_a;
mod sub_b;

pub fn run() {
    println!("Here is vars module!!");
    sub_a::func_a("Hello, world!");
    sub_b::func_b();
}
