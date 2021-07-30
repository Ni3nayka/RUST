

fn main() {
    let input = "Шашки логическая настольная игра".to_string();
    println!("start line: {:?}",input);
    a_counter(input.clone());
    up_register(input.clone());
    checkers_count(input.clone());
}

fn a_counter(input: String) {

    println!();

    let mut i:u32 = 0;
    for c in input.chars() { if c=='а' {i+=1;} }
    println!("a_count, variant 1: {}",i);
    
    i = 0;
    for a in 0..input.chars().count() { if input.chars().nth(a).unwrap()=='а' {i+=1;} }
    println!("a_count, variant 2: {}",i);
    
    let mut my_vec = Vec::new();
    for c in input.chars() { if c=='а' { my_vec.push(c); } }
    println!("a_count, variant 3: {}",my_vec.len());
}

fn up_register(input: String) {

    println!();

    let test_str = &*input;
    println!("up_register, variant 1: {:?}", test_str.to_uppercase().as_str());

    println!("up_register, variant 2: {:?}", test_str.to_ascii_uppercase()); // не работает, т.к. русский язык не ascii

    let mut output:String = "".to_string();
    for c in input.chars() {
        match c {
            'ш' => output.push('Ш'),
            'а' => output.push('А'),
            'к' => output.push('К'),
            'и' => output.push('И'),
            'н' => output.push('Н'),
            'с' => output.push('С'),
            'т' => output.push('Т'),
            'о' => output.push('О'),
            'л' => output.push('Л'),
            'ь' => output.push('Ь'),
            'я' => output.push('Я'),
            'г' => output.push('Г'),
            'р' => output.push('Р'),
            'ч' => output.push('Ч'),
            'е' => output.push('Е'),
            _ => output.push(c),
        }
    }
    println!("up_register, variant 3: {:?}", output);
}

fn checkers_count(input_1: String) {

    println!();
    let input = (&*input_1).to_uppercase();

    let mut i:u32 = 0;
    let mut f:u32 = 0;
    for c in input.chars() { 
        if c=='Ш' && (f==0 || f==2) { f+=1; }
        else if c=='А' && f==1 { f+=1; }
        else if c=='К' && f==3 { f+=1; }
        else if c=='И' && f==4 { f+=1; }
        else { 
            f = 0;
            if c=='Ш' { f+=1; } 
        }
        if f==5 { f = 0; i+=1; } 
    }
    println!("checkers_count, variant 1: {}", i);

    i = 0;
    let mut output:String = "".to_string();
    for a in 0..input.chars().count()-4 {
        for u in a..a+5 {
            output.push(input.chars().nth(u).unwrap());
            if output.chars().count()==5 {
                if output=="ШАШКИ".to_string() { i+=1; }
                output = "".to_string(); 
            }
        }
    }
    println!("checkers_count, variant 2: {}", i);

    //for c in input.chars()
    i = 0;
    let mut my_vec = Vec::new();
    my_vec.push("".to_string());
    for c in input.chars() {
        let a = my_vec.len()-1;
        if c==' ' { my_vec.push("".to_string()); }
        else { my_vec[a].push(c); }
    }
    for c in my_vec { if c=="ШАШКИ" {i+=1;} }
    println!("checkers_count, variant 3: {}", i);
}