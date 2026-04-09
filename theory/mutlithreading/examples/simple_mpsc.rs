fn main() {
    use std::sync::mpsc;

    fn the_sending_thread (tx: mpsc::Sender<u32>) {
        std::thread::sleep(std::time::Duration::from_millis(10));
        tx.send(42).unwrap();
    }
    
    let (tx, rx) = mpsc::channel();
    
    std::thread::spawn(move || {
        the_sending_thread(tx);
    });
    
    assert_eq!(rx.recv().unwrap(), 42);
} 