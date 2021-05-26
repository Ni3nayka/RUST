use std::io;

fn main() {
    //println!("Hello, world!");
    println!("угадай число");
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("строка не считана");
    println!("вы загадали: {}", guess);
}
