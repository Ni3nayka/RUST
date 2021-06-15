fn main() {
    let n: &str = "qwerty";
    let _a = remove_char(n);
    println!("{}", _a);
}

fn remove_char(s: &str) -> String {
    String::from(&s[1..s.len()-1])
}
