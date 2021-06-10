//https://www.codewars.com/kata/56bc28ad5bdaeb48760009b0/train/rust
use std::io;

fn main() {
    let n: &str = "qwerty";
    println!("answer: {}", remove_char(n));
}

fn remove_char(s: &str) -> String {
    String::from(&s[1..s.len()-1])
}
