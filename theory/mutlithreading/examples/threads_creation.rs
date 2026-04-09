fn my_thread_fn(label: &str) -> usize {
    let mut total = 0usize;
    for i in 0..5 {
        println!("my_thread_fn({}): {}", label, i);
        std::thread::sleep(std::time::Duration::from_millis(500));
        total += 1;
    }
    total
}

// fn main() {
//     println!("Hello, world!");
//     println!("main thread begins");

//     let mut label = String::from("hello");
//     let thread = std::thread::spawn(move||my_thread_fn(&label));
//     let total = thread.join().unwrap();
    
       // Ошибка - переместили данные
//     println!("{}", label);   
    
//     label = "Test".to_string();
//     println!("{}", label);

//     println!("main thread ends, other thread sleeped {} times", total);
// } 

// Суть scoped-потоков
// В отличие от обычных потоков, создаваемых через std::thread::spawn, scoped-потоки:
//     гарантированно завершаются до выхода из области видимости, где они были созданы;
//     могут безопасно использовать не-Send данные (например, ссылки на локальные переменные), поскольку компилятор «знает», что поток не переживёт родительскую область видимости.

fn main() {
    println!("Hello, world!");
    println!("main thread begins");

    let label1 = String::from("String 1");
    let label2 = String::from("String 2");
    
    let (res1, res2) = std::thread::scope(
                     |scope| {
                         let r1 = scope.spawn(|| my_thread_fn(&label1));
                         let r2 = scope.spawn(|| my_thread_fn(&label2));
                         // нельзя обратиться по &mut self, когда выше есть просто &
                         //label1.clear();
                         //label2.clear();
                         println!("from main thread");
                         (r1.join().unwrap(), r2.join().unwrap())
                     }
                 );
    println!("main thread ends, other thread sleeped {:?} times", (res1, res2));
} 