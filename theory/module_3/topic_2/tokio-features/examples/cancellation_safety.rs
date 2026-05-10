// Cancellation safety
// Cancellation safety — это свойство future, которое гарантирует, что, если её отменить (вызов drop), это не приведёт к потере данных или нарушению логики программы. К примеру, разберём такой псевдокод:

fn main() {
    todo!()
}

// #[tokio::main]
// fn main() {

//     let tcpstream = TcpStream::connect("localhost:8000").await.unwrap();

//     loop {
//         tokio::select! {
//             v = read_some_data(&mut tcpstream) => {
//                 todo!()
//             },
//             _ => tokio::time::sleep(Duration::from_millis(10)) => {}
//         }
//         println!("Cycle timeout");
//     } 
// }

// Выглядит хорошо: ожидаем чтение, а по тайм-ауту перезапускаем цикл. 
// Но что, если read_some_data уже успела прочитать какие-то данные до тайм-аута, 
// но не успела вернуть их? Тогда эти данные будут утеряны навсегда. 
// Такие функции, отмена выполнения которых приводит к потере данных, не считаются cancellation safe. 

// Примерами таких функций из tokio являются:
    // tokio::io::AsyncReadExt::read_exact
    // tokio::io::AsyncReadExt::read_to_end
    // tokio::io::AsyncReadExt::read_to_string
    // tokio::io::AsyncWriteExt::write_all

// Их нужно использовать с осторожностью.


/*** Заметки джуна по поводу Cancellation Safety ***/
// Method	                    Cancellation-Safe?                      Why
// `read(&mut buf)`	            ✅ Yes	            Returns bytes read; you control buffer
// `write(&buf)`	            ✅ Yes              	Returns bytes written; you can retry
// `read_exact(&mut buf)`	    ❌ No	            Hides partial reads; data lost if cancelled
// `read_to_end(&mut vec)`	    ❌ No	            Appends internally; partial data lost if cancelled
// `read_to_string(&mut str)`	❌ No	            Same as read_to_end + UTF-8 validation
// `write_all(&buf)`	        ❌ No	            Hides partial writes; may leave stream in inconsistent state
// `connect()`	                ✅ Yes	            Either connected or not — no partial TCP state
// `accept()`	                ✅ Yes	            Returns new socket or pending — no partial state
