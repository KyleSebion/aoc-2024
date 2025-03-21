#![allow(dead_code)]

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod test {
    use super::*;
    #[test] fn a() {}
}
