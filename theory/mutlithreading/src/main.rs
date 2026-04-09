use std::sync::{mpsc, Arc, RwLock, atomic::{AtomicUsize, Ordering}};
use std::thread;

pub type ThreadID = usize;

#[derive(Debug)]
enum MessageKind{
    FinishAll,
    NewServer,
    Work{work_spec: String},
    ChangeEpoch{epoch: String},
}

fn server(rx: mpsc::Receiver<(ThreadID, MessageKind)>,
          id: ThreadID,
          success_count: Arc<AtomicUsize>,
          epoch: Arc<RwLock<String>>)
{
    while let Ok((from, msg_kind)) = rx.recv() {
        match msg_kind {
            MessageKind::FinishAll => break,
            MessageKind::NewServer => unreachable!("this message is not for server!"),
            MessageKind::ChangeEpoch { epoch: new_epoch } => {
                // Лок на чтение
                let mut lock = epoch.write().unwrap();
                println!( "worker-{} is being asked by client-{} at epoch '{}' to change epoch into '{}'",
                          id, from, lock, new_epoch );
                // Меняем epoch
                *lock = new_epoch;

            }
            // Чтение epoch через .read().unwrap
            MessageKind::Work { work_spec } => {

                println!( "worker-{} is being asked by client-{} at epoch '{}' to work '{}'",
                          id, from, epoch.read().unwrap(), work_spec );

            },
        }
        success_count.fetch_add(1, Ordering::SeqCst);
    }
    println!("Finishing worker-{}", id);
}

fn make_and_append_server(all_servers: &mut Vec<(mpsc::Sender<(ThreadID, MessageKind)>,
                                                 thread::JoinHandle<()>)>,
                          success_count: Arc<AtomicUsize>,
                          epoch: Arc<RwLock<String>>)
{
    let (tx, rx) = mpsc::channel();
    let new_server_id = all_servers.len();

    // Массивчик sender + JoinHandle
    all_servers.push((tx, thread::spawn(move || server(rx, new_server_id, success_count, epoch))));
}

fn balancer(receiver: mpsc::Receiver<(ThreadID, MessageKind)>,
            servers_count: ThreadID,
            success_count: Arc<AtomicUsize>)
{
    // Один общик epoch => Arc<RwLock<String>>
    let epoch = Arc::new(RwLock::new("epoch-1".into()));
    let mut servers_senders_handlers = Vec::new();

    // Создаём заданное количество серверов
    for _ in 0..servers_count {
        make_and_append_server(&mut servers_senders_handlers, success_count.clone(), epoch.clone());
    }

    let mut next_server = 0usize;

    while let Ok((from, msg_kind)) = receiver.recv() {
        match msg_kind {
            MessageKind::FinishAll => {
                // Закрываем - выход из цикла всех                
                for (tx, _) in &servers_senders_handlers {
                    tx.send((from, MessageKind::FinishAll)).unwrap();
                }
                // Джоинимся и выходим из самого балансера
                for (_, thread) in servers_senders_handlers {
                    thread.join().unwrap();
                }
                break;
            },
            MessageKind::NewServer => {
                make_and_append_server(&mut servers_senders_handlers, success_count.clone(), epoch.clone());
            }

            MessageKind::Work { work_spec } => {
                servers_senders_handlers[next_server].0.send((from, MessageKind::Work{work_spec})).unwrap();
            }

            MessageKind::ChangeEpoch { epoch } => {
                servers_senders_handlers[next_server].0.send((from, MessageKind::ChangeEpoch{epoch})).unwrap();
            }
        }
        next_server = (next_server + 1).rem_euclid(servers_senders_handlers.len());
    }
}

fn my_sleep() {
    thread::sleep(std::time::Duration::from_millis(10))
}

fn client1(sender: mpsc::Sender<(ThreadID, MessageKind)>) {
    let id = 1;
    sender.send((id, MessageKind::Work{work_spec: "prepare".into()})).unwrap();
    my_sleep();
    sender.send((id, MessageKind::Work{work_spec: "work-1".into()})).unwrap();
    my_sleep();
    sender.send((id, MessageKind::Work{work_spec: "work-2".into()})).unwrap();
    my_sleep();
}

fn client2(sender: mpsc::Sender<(ThreadID, MessageKind)>) {
    let id = 2;
    sender.send((id, MessageKind::Work{work_spec: "prepare".into()})).unwrap();
    my_sleep();
    sender.send((id, MessageKind::NewServer)).unwrap();
    my_sleep();
    sender.send((id, MessageKind::Work{work_spec: "work-1".into()})).unwrap();
    my_sleep();
    sender.send((id, MessageKind::Work{work_spec: "work-2".into()})).unwrap();
    my_sleep();
    sender.send((id, MessageKind::Work{work_spec: "work-3".into()})).unwrap();
    my_sleep();
    sender.send((id, MessageKind::Work{work_spec: "work-4".into()})).unwrap();
    my_sleep();
    sender.send((id, MessageKind::Work{work_spec: "work-5".into()})).unwrap();
    my_sleep();
    sender.send((id, MessageKind::Work{work_spec: "work-6".into()})).unwrap();
    my_sleep();
}

fn client3(sender: mpsc::Sender<(ThreadID, MessageKind)>) {
    let id = 3;
    sender.send((id, MessageKind::Work{work_spec: "prepare".into()})).unwrap();
    my_sleep();
    sender.send((id, MessageKind::Work{work_spec: "work-1".into()})).unwrap();
    my_sleep();
    sender.send((id, MessageKind::ChangeEpoch{epoch: "epoch-2".into()})).unwrap();
    my_sleep();
    sender.send((id, MessageKind::Work{work_spec: "work-2".into()})).unwrap();
    my_sleep();
    sender.send((id, MessageKind::Work{work_spec: "work-3".into()})).unwrap();
    my_sleep();
    sender.send((id, MessageKind::Work{work_spec: "work-4".into()})).unwrap();
    my_sleep();
    sender.send((id, MessageKind::Work{work_spec: "work-5".into()})).unwrap();
    my_sleep();
    sender.send((id, MessageKind::Work{work_spec: "work-6".into()})).unwrap();
    my_sleep();
}

fn main () {
    println!("Hello, world!");

    // Создаём канал - один Sender, один Receiver
    let (balancer_sender, balancer_receiver) = mpsc::channel();
    let success_count = Arc::new(AtomicUsize::new(0));

    // Нужно клонировать именно до замыкания
    let success_count_cloned = success_count.clone();
    
    // Один балансировщик, ему отдаётся (move) Receiver
    let server = thread::spawn(move || balancer(
                                           balancer_receiver,
                                           4,
                                           success_count_cloned)
                                     );

    // Создаём ещё три Sender'а, чтобы клиенты могли отправлять на балансировщик задания
    let (balancer_sender_1, balancer_sender_2, balancer_sender_3) = (balancer_sender.clone(), balancer_sender.clone(), balancer_sender.clone());

    // Пущаем
    let clients = [
        thread::spawn(move || client1(balancer_sender_1)),
        thread::spawn(move || client2(balancer_sender_2)),
        thread::spawn(move || client3(balancer_sender_3)),
    ];
    
    for client in clients {
        client.join().unwrap();
    }

    // Main-поток - нулевой
    balancer_sender.send((0, MessageKind::FinishAll)).unwrap();
    
    server.join().unwrap();

    println!("\nDone jobs = {}", success_count.load(Ordering::SeqCst));
} 