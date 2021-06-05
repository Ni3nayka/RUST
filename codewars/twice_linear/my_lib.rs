/*
https://qna.habr.com/q/366081
https://metanit.com/rust/tutorial/8.3.php
*/

//use std::io;
//use std::mem;

/*
fn test() {
    let mut y: [i32;7] = [3,5,2,5,7,9,2];
    print!("test Qsort 0: ");
    print_mas(&y);
    Qsort(&mut y);
    print!("test Qsort 1: ");
    print_mas(&y);
    println!();
}
*/

pub fn Qsort(mas: &mut [i32]) { // Qsort(&mut y, 0);  , qwerty: i32
    let size = mas.len();
    if (qwerty>0) { size = qwerty; }
    //Указатели в начало и в конец массива
    let mut i = 0;
    let mut j = size - 1;

    //Центральный элемент массива
    let mut mid = mas[size / 2];

    //Делим массив
    while {
        //Пробегаем элементы, ищем те, которые нужно перекинуть в другую часть
        //В левой части массива пропускаем(оставляем на месте) элементы, которые меньше центрального
        while(mas[i] < mid) {
            i+=1;
        }
        //В правой части пропускаем элементы, которые больше центрального
        while(mas[j] > mid) {
            j-=1;
        }

        //Меняем элементы местами
        if (i <= j) {
            let tmp = mas[i];
            mas[i] = mas[j];
            mas[j] = tmp;

            i+=1;
            j-=1;
        }
        i <= j
    } {}

    //Рекурсивные вызовы, если осталось, что сортировать
    if(j > 0) {
        //"Левый кусок"
        Qsort(&mut mas[0..j+1]);
    }
    if (i < size-1) {
        //"Првый кусок"
        Qsort(&mut mas[i..size]);
    }
}

pub fn print_mas(mas: &[i32]) { // print_mas(&y);
    for i in 0..mas.len() {
        print!("{}", mas[i]);
        print!(" ");
    }
    println!();
}
