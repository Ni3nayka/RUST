use std::io;

fn main() {
    //println!("Hello, world!");
    /*println!("угадай число");
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("строка не считана");
    println!("вы загадали: {}", guess);*/
    let n = 86240;
    println!("answer: {}", prime_factors(n));
}

//======================================================================================================================================
//======================================================================================================================================

fn prime_factors(n: i64) -> String {
    let mut N = n;
    println!("{}", N);
    let mut i = 2;
    while (N>1) {
        println!("{}", N);
        let mut ii = 0;
        while (N%i!=0) {
            N = N/i;
        }
        println!("{}", i, "{}", ii);
    }
    let a = "1234";
    a.to_string()
}
