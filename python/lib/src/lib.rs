use std::thread;

/*fn main() {
    let n = "qwerty";
    println!("rust lib main");
    process();
}*/

#[no_mangle]
pub extern fn process() {
    println!("rust lib fun");
    let handles: Vec<_> = (0..10).map(|_| {
        thread::spawn(|| {
            let mut x = 0;
            for _ in 0..5_000_000 {
                x += 1
            }
            x
        })
    }).collect();

    for h in handles {
        println!("Поток завершился со счётом={}",
        h.join().map_err(|_| "Не удалось соединиться с потоком!").unwrap());
    }
}
