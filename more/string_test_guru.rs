use std::collections::{HashMap, VecDeque};

fn main() {
    veq();
    deq();
    hmap();
    
}

fn veq() {
    let mut vec1:Vec<u32> =  Vec::<u32>::new();
    println!("vec1 new {:?}", vec1);
    vec1.push(1);
    vec1.push(2);
    vec1.push(3);
    println!("vec1 three {:#?}", vec1);
    //-------------------------------------
    let mut vec2 : Vec<u32> =  vec![];
    vec2 = (1..10).collect();
    println!("vec2 filled {:?}", vec2);
    println!("vec2 {:?} element no 5 `{:?}`",vec2,vec2.get(5));
    //-------------------------------------
    let mut sum_arr : u32 = 0;
    for val in vec2.iter() {
        sum_arr += val; // sum_arr = sum_arr + val
    }
    println!("vec2 {:?} sum `{:?}`",vec2,sum_arr);
    //------------------------------------
    for val in vec2.iter_mut() {
        *val += 1; 
    }
    println!("new vec2 {:?}",vec2);
    //------------------------------------
    vec2.iter_mut().for_each(|val| {*val -= 1});
    println!("corr vec2 {:?}",vec2);
    //------------------------------------
    let mut sum_arr2 : u32 = vec2.iter().fold(0, |mut sum, &val| {sum += val; sum});
    println!("vec2 {:?} sum again `{:?}`",vec2,sum_arr2);
    //-----------------------------------
    let str = "Шашки логическая настольная игра".to_string();
    let mut chars1 :  Vec<char> = vec![];
    for c in str.chars() {
        chars1.push(c);
    }
    println!("Chars as vec {:?}",chars1);

    let chars2 = str.to_string().chars().collect::<Vec<char>>();

    println!("Chars as vec 2 {:?}",chars2);

    let words = str.split(" ").map(|item| item.to_string()).collect::<Vec<String>>();
    
    println!("words in {} ->  {:?}",str,words);

}

fn deq() {
    let mut buf = VecDeque::<u32>::new();

    println!("Young Deq {:?}",buf);

    buf.push_back(1);
    buf.push_back(2);
    buf.push_back(3);
    buf.push_back(4);

    println!("1-5 Deq {:?}",buf);

    let front_value = buf.pop_front();

    println!("value from front `{:?}` now Deq {:?}",front_value,buf);

    let back_value = buf.pop_back();

    println!("value from back `{:?}` now Deq {:?}",back_value,buf);

    buf.drain(..).for_each(move |item| {println!("  Item {}",item)});
    
    //Можно было 
    // for item in buf.drain(..) {
    //     println!("  Item {}",item);
    // }
    
     println!("Deq after drain {:?}",buf);

    let front_value_empty = buf.pop_front();

    println!("value from front of empty Deq `{:?}` ",front_value_empty);

    buf = (0..10).collect();

    println!("Make deq great again {:?}",buf);

    buf.rotate_left(2);

    println!("Rotate to Left by 2 {:?}",buf);

    buf.rotate_right(2);

    println!("Rotate to Right by 2 {:?}",buf);

    buf.insert(5, 555);

    println!("Insert 555 at position 5 {:?}",buf);

    buf.remove(5);

    println!("Remove elem at position 5 {:?}",buf);

    buf.truncate(3);

    println!("Truncate by 3 {:?}",buf);

    buf.clear();

    println!("Goodbye {:?}",buf);
}

fn hmap() {
    let mut contacts = HashMap::new();

    contacts.insert("Карлсон", "322-223-322");
    contacts.insert("Мосгаз", "04");
    contacts.insert("Милиция", "02");
    contacts.insert("Скорая", "03");

    println!("Телефоны {:?}",contacts);

    let men_on_roof = contacts.get("Карлсон");

    println!("Карлсон `{:?}`",men_on_roof);

    println!("Пожарная `{:?}`",contacts.get("Пожарная"));
 
    let fireguard = contacts.get("Пожарная");

    if fireguard.is_none() {
        println!("Пожарные `{}`",fireguard.unwrap_or(&"НЕТ НОМЕРА"));
    }

    let ambulance = contacts.get("Скорая");

    if ambulance.is_some() {        
        println!("Скорая `{}`",ambulance.unwrap_or(&"НЕТ НОМЕРА"));
    }


    match contacts.get("Пожарная") {
        Some(&number) => println!("Пожарные: {}", number),
        _ => println!("Пожарных нет !"),
    }

    let who = "Мосгаз";

    match contacts.get(who) {
        Some(&number) => println!("{} `{}`", who, number),
        _ => println!("Нет '{}'",who),
    }

    contacts.remove(who);
    
    match contacts.get(who) {
        Some(&number) => println!("{} `{}`", who, number),
        _ => println!("Нет '{}'",who),
    }

    contacts.keys().for_each(|key| {println!("{}",key)});
}