// https://www.codewars.com/kata/54d512e62a5e54c96200019e/train/rust

use std::io;
//use std::mem; // for test()

fn main() {
    //println!("Hello, world!");
    /*println!("угадай число");
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("строка не считана");
    println!("вы загадали: {}", guess);*/
    let n = 86240;
    println!("answer: {}", prime_factors(n));
    //test();
}

/*fn foo(a:&[&[i64]])
{
    for i in 0..2 {
        for j in 0..4 {
            println!("{}",a[i][j]);
        }
    }
}

fn main() {
    let a:[&[i64];2]=[&[2,3,5,7],
                       &[0,0,0,0]];
    foo(&a);
}*/

/*
// Эта функция заимствует срез
fn analyze_slice(slice: &[i32]) {
    println!("первый элемент среза: {}", slice[0]);
    println!("второй элемент среза: {}", slice[1]);
    println!("в срезе {} элементов", slice.len());
}

fn test() {
    // Массив фиксированного размера (указывать сигнатуру типа необязательно)
    let xs: [i32; 5] = [1, 2, 3, 4, 5];

    // Все элементы могут быть инициализированы одной и той же переменной
    let ys: [i32; 500] = [0; 500];

    // Индекс начинается с 0
    println!("первый элемент массива: {}", xs[0]);
    println!("второй элемент массива: {}", xs[1]);

    // `len` возвращает длину массива
    println!("размер массива: {}", xs.len());

    // Память для массивов выделяется в стеке
    println!("массив занимает {} байт", mem::size_of_val(&xs));

    // Массивы могут быть автоматически заимствованы как срез
    println!("заимствуем весь массив, используя срез");
    analyze_slice(&xs);

    // Срезы могут указывать на часть массива
    // Они имеют форму [starting_index..ending_index]
    // starting_index - это первая позиция в срезе
    // ending_index - на 1 больше, чем последняя позиция в срезе
    println!("заимствуем часть массива как срез");
    analyze_slice(&ys[1 .. 4]);

    // так для теста
    println!("test {ya}", ya = xs[4]);

    // Выход за границу массива заставит компилятор паниковать.
    // Не надо так.
    //println!("{}", xs[5]);
}*/

fn prime_factors(n: i64) -> String {
    let mut answer = String::new();

    let mut N = n;
    //println!("{}", N);
    let mut i = 2;
    while (N>1) {
        //println!("{}", N);
        let mut ii = 0;
        //println!("{}",N-i!=0);
        while (N%i==0) {
            N = N/i;
            ii+=1;
        }
        // вставка результата в ответ
        //println!("{},{}",i,ii);
        if (ii>0) {
            answer.push_str("(");

            let i: String = i.to_string();
            let i: &str = &i;   // specifying type is necessary for deref coercion to fire
            //let ss = &s[..];     // alternatively, use slicing syntax
            answer.push_str(i);

            if (ii>1) {
                answer.push_str("**");

                let ii: String = ii.to_string();
                let ii: &str = &ii;   // specifying type is necessary for deref coercion to fire
                //let ss = &s[..];     // alternatively, use slicing syntax
                answer.push_str(ii);
            }

            answer.push_str(")");
        }
        i+=1;
        //println!("{}", i, "{}", ii);*/
    }

    answer
}
