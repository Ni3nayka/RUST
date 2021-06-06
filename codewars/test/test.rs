use std::io;

fn main() {
    let n:u32 = 100;
    println!("answer: {}", dbl_linear(n));
}

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

fn del_dubli(mas: &mut [i32]) -> &mut [i32] {
    let ras: usize = mas.len();
    let mut i: usize = 1;
    while (i < ras-1 && mas[i]!=0) {
        if (mas[i]==mas[i+1] && true) {
            for u in i..(ras-1) { mas[u] = mas[u+1]; }
            mas[ras-1] = 0;
            i+=1;
        }
        else { i+=1; }
    }
    //while (mas[i]>0) { i+=1; }
    //let mut mas = &mut mas[0..i];
    //print_mas(&mas);
    mas
}

fn dbl_linear(n: u32) -> u32{
    const ras: usize = 10000;
    let mut x: [i32;ras] = [0;ras];
    x[0] = 1;

    //let mut h:i32 = f32::powf(n as f32, 0.5) as i32;

    let mut h:i32 = 0;
    let mut a:i32 = 1;
    while (a < n as i32) {
        h += 1;
        a += u32::pow(2, h as u32) as i32;
    }
    h += 1;

    add(&mut x, 1, h); // рекрусивно составляем ряд
    let mut x = del_zero_mas(&mut x);
    Qsort(&mut x,0);
    let mut x = del_dubli(&mut x);
    //print_mas(&x);

    x[n as usize] as u32 // answer
}

fn Qsort(mas: &mut [i32], qwerty: usize) { // Qsort(&mut y, 0); , qwerty: usize
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

fn del_zero_mas(mas: &mut [i32]) -> &mut [i32] { // let mut x = del_zero_mas(&mut x);
    // delete "0"
    let mut i: usize = 1;
    while (mas[i]>0) { i+=1; }
    let mut mas = &mut mas[0..i];
    //print_mas(&mas);
    mas
}
