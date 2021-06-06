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

pub fn Qsort(mas: &mut [i32], qwerty: usize) { // Qsort(&mut y, 0); , qwerty: usize
    let mut size = mas.len();
    if (qwerty>0 && qwerty+1<size) { size = qwerty+1; }
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
        Qsort(&mut mas[0..j+1], qwerty);
    }
    if (i < size-1) {
        //"Првый кусок"
        Qsort(&mut mas[i..size], qwerty);
    }
}

pub fn del_zero_mas(mas: &mut [i32]) -> &mut [i32] { // let mut x = del_zero_mas(&mut x);
    // delete "0"
    let mut i: usize = 1;
    while (mas[i]>0) { i+=1; }
    let mut mas = &mut mas[0..i];
    //print_mas(&mas);
    mas
}

pub fn print_mas(mas: &[i32]) { // print_mas(&y);
    for i in 0..mas.len() {
        print!("{}", mas[i]);
        print!(" ");
    }
    println!();
}

/* // recursia + array
fn add(mas: &mut [i32], n: usize, counter: i32) {
    let mut i: usize = 0;
    while (mas[i+1]>0) { i+=1; }
    let mut y = 2 * n + 1;
    let mut z = 3 * n + 1;
    mas[i+1] = y as i32;
    mas[i+2] = z as i32;
    if (counter-1>0) {
        add(mas, y, counter-1);
        add(mas, z, counter-1);
    }
}
*/
