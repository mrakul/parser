use std::thread;
use std::sync::{Mutex, Arc, atomic::{AtomicU32, Ordering}};

fn run1 (value: Arc<AtomicU32>, id: u32) -> u32 {
    for _ in 0..10 {
        // эмулируем 'сдвиг по фазе' при доступе из разных потоков
        // (например, один поток находится в начале тела цикла,
        //  а другой - в середине)
        std::thread::sleep(std::time::Duration::from_millis(id as u64));
        value.fetch_add(1, Ordering::SeqCst);
        // здесь очень просто нарваться на обновление со стороны другого потока
        std::thread::sleep(std::time::Duration::from_millis(id as u64));
        value.fetch_add(value.load(Ordering::SeqCst), Ordering::SeqCst);
    }
    value.load(Ordering::SeqCst)
}
fn run2 (value: Arc<Mutex<u32>>, id: u32) -> u32 {
    use std::ops::DerefMut;
    for _ in 0..10 {
        let mut lock = value.lock().unwrap();
        let value = lock.deref_mut();
        // эмулируем 'сдвиг по фазе' при доступе из разных потоков
        // (например, один поток находится в начале тела цикла,
        //  а другой - в середине)
        std::thread::sleep(std::time::Duration::from_millis(id as u64));
        *value += 1;
        std::thread::sleep(std::time::Duration::from_millis(id as u64));
        *value *= 2;
    }
    *value.lock().unwrap().deref_mut()
}

fn main() {
    let value1 = Arc::new(AtomicU32::from(0));
    let value2 = Arc::new(Mutex::new(0u32));
    println!("Hello, world!");

    let (value1_1, value1_2) = (value1.clone(), value1.clone());
    println!("atomics:");
    let t1 = thread::spawn(|| run1(value1_1, 1));
    let t2 = thread::spawn(|| run1(value1_2, 2));
    let v1 = t1.join().unwrap();
    let v2 = t2.join().unwrap();
    // max, чтобы был последний обновлённый,
    // просто пример чтения, обе строки одинаковые
    println!("result1 is {}", std::cmp::max(v1, v2));
    println!("result1 is {}", value1.load(Ordering::SeqCst));

    let (value2_1, value2_2) = (value2.clone(), value2.clone());
    println!("\nmutex:");
    let t1 = thread::spawn(|| run2(value2_1, 1));
    let t2 = thread::spawn(|| run2(value2_2, 2));
    let v1 = t1.join().unwrap();
    let v2 = t2.join().unwrap();
    // у автора результаты возле метки "result1" регулярно отличались
    // от правильных значений "result2"
    println!("result2 is {}", std::cmp::max(v1, v2));
    println!("result2 is {}", *value2.lock().unwrap());
}