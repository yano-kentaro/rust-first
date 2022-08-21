use std::str;

pub fn func_a(str: &str) {
    println!("{}", str);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn func_a_test() {
        func_a("Hello, world!");
    }
}
