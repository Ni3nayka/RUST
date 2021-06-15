// https://www.codewars.com/kata/534e01fbbb17187c7e0000c6/train/rust

fn main() {
    let array = spiralize(10);
    //println!("{:?}",array);
    for a in 0..array.len() { println!("{:?}",array[a]); }
}

fn spiralize(size: usize) -> Vec<Vec<i8>> {
    //let mut array = vec![vec![0 as i8; width]; height];
    let mut array = vec![vec![0 as i8; size]; size];
    // обрабатываем края
    for i in 0..size   { array[0][i] = 1; }
    for i in 1..size   { array[i][size-1] = 1; }
    for i in 0..size-1 { array[size-1][i] = 1; }
    for i in 2..size-1 { array[i][0] = 1; }
    // обрабатываем центр
    let mut x = 0 as usize;
    let mut y = 2 as usize;
    let mut gg: i8 = 4;
    while (gg==4) {
        gg = 0;
        if (gg==0) { while (array[y][x+2]==0 && array[y+1][x+1]==0 && array[y-1][x+1]==0) { array[y][x+1] = 1; x+=1; gg = 1; } } // gg+=1;
        if (gg==1) { while (array[y+2][x]==0 && array[y+1][x-1]==0 && array[y+1][x+1]==0) { array[y+1][x] = 1; y+=1; gg = 2; } }
        if (gg==2) { while (array[y][x-2]==0 && array[y-1][x-1]==0 && array[y+1][x-1]==0) { array[y][x-1] = 1; x-=1; gg = 3; } }
        if (gg==3) { while (array[y-2][x]==0 && array[y-1][x-1]==0 && array[y-1][x+1]==0) { array[y-1][x] = 1; y-=1; gg = 4; } }
    }
    // return
    array
}
