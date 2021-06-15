// https://www.codewars.com/kata/59f81fe146d84322ed00001e
// https://kgv.gitbooks.io/rust_book_ru/content/src/strings.html

//use unicode_segmentation::UnicodeSegmentation;

fn main() {
    let input: [&str;3] = ["╋━━┓", "┃..┃", "┛..┣"];
    println!("{}",check_pipe(&input));
}

fn analyzer(mut pipe_map: &[&str], y: usize, x: usize) -> [i8;4] {
    let mut a: [i8;4] = [0;4]; // верх низ лево право
    if (String::from(pipe_map[y]).chars().nth(x).unwrap()=='┗') { a[0]=1; a[1]=0; a[2]=0; a[3]=1; }
    if (String::from(pipe_map[y]).chars().nth(x).unwrap()=='┓') { a[0]=0; a[1]=1; a[2]=1; a[3]=0; }
    if (String::from(pipe_map[y]).chars().nth(x).unwrap()=='┏') { a[0]=0; a[1]=1; a[2]=0; a[3]=1; }
    if (String::from(pipe_map[y]).chars().nth(x).unwrap()=='┛') { a[0]=1; a[1]=0; a[2]=1; a[3]=0; }
    if (String::from(pipe_map[y]).chars().nth(x).unwrap()=='━') { a[0]=0; a[1]=0; a[2]=1; a[3]=1; }
    if (String::from(pipe_map[y]).chars().nth(x).unwrap()=='┃') { a[0]=1; a[1]=1; a[2]=0; a[3]=0; }
    if (String::from(pipe_map[y]).chars().nth(x).unwrap()=='┣') { a[0]=1; a[1]=1; a[2]=0; a[3]=1; }
    if (String::from(pipe_map[y]).chars().nth(x).unwrap()=='┫') { a[0]=1; a[1]=1; a[2]=1; a[3]=0; }
    if (String::from(pipe_map[y]).chars().nth(x).unwrap()=='┳') { a[0]=0; a[1]=1; a[2]=1; a[3]=1; }
    if (String::from(pipe_map[y]).chars().nth(x).unwrap()=='┻') { a[0]=1; a[1]=0; a[2]=1; a[3]=1; }
    if (String::from(pipe_map[y]).chars().nth(x).unwrap()=='╋') { a[0]=1; a[1]=1; a[2]=1; a[3]=1; }
    a
}

fn check_pipe(mut pipe_map: &[&str]) -> bool {
    //println!("{:?}", analyzer(&pipe_map, 0,1));
    // анализ входного массива и составление матрицы смежностей
    let mut map = vec![vec![0 as i32; 4]; 0]; // map.push(vec![0 as i32; 4]);
    let mut size: usize = 0;
    //let mut sized: i32 = -1;
    for y in 0..pipe_map.len()  {
        //println!("!");
        for x in 0..String::from(pipe_map[y]).chars().count() {
            //println!("!!!");
            let test = analyzer(&pipe_map, y,x);
            println!("{:?}", test);
            if (test[0]==1) { map.push(vec![0 as i32; 4]); size+=1; map[size-1][0] = x as i32; map[size-1][1] = y as i32; map[size-1][2] = x as i32; map[size-1][3] = y-1 as i32; } //
            //println!("{}", String::from(pipe_map[y]).chars().nth(x).unwrap())
        }
    }
    println!("{:?}", map);
    // return
    false
}
