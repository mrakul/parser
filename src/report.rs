use std::collections::HashMap;
use std::io::{BufRead, BufReader};
// use std::path::Path;
// use std::fs::File;
// use std::fs;

use crate::csv_format::CsvFormatIO;
use crate::text_format::TextFormatIO;
use crate::bin_format::BinFormatIO;

// Задаём тип (аналог using C++)
pub type ID = u64;

// Тип транзакции
#[derive(Debug)]
enum TransactionType {
    Deposit,
    Withdrawal,
    Transfer
}

// Статус транзакции
#[derive(Debug)]
enum TransactionStatus {
    Success,
    Failure,
    Pending
}

// Структура для чтения/записи транзакции
#[derive(Debug)]
struct Transaction {
    tx_id: u64,
    tx_type: TransactionType,
    from_user_id: u64,
    to_user_id: u64,
    amount: u64,
    timestamp: u64,
    description: String,
}

// Реализуем трейты для ввода/вывода в различном формате
#[derive(Debug)]
pub struct Report {
   transactions: HashMap<ID, Transaction>
}

impl Report {
    // Конструктор, возвращем структуру Report (by value, Move-семантика)
    pub fn new() -> Self {
        Self {
            transactions: HashMap::new(),
        }
    }
    
    // Добавить транзакцию: важно, что передаём с передачей владения для .insert()
    // С одним проходом => .entry()
    // pub fn add_transaction(&mut self, tx_to_add: Transaction) -> Option<&Transaction> {

    //     match self.transactions.entry(tx_to_add.tx_id) {
            
    //         // TODO: маловероятно, можно обработать
    //         std::collections::hash_map::Entry::Occupied(occupied_entry) => {
    //             Some(occupied_entry.get_mut())
    //         },
        
    //         std::collections::hash_map::Entry::Vacant(entry) => {
    //             // let added_transaction = Transaction::new();     
    //             let added_transaction = entry.insert(tx_to_add);
                
    //             Some(added_transaction)
    //             // Не очень, что копируем: можно пересмотреть API: возвращать bool или Option<&Transaction>
    //             // Entry::Vacant(entry) => Some(entry.insert(Balance::new()))
    //         }
    //     }
    // }

    // Удалить транзакцию: Option<i64> => "забираем" данные (Copy для примитивов)
    pub fn remove_transaction(&mut self, tx_id_to_remove: &ID) -> Option<Transaction> {
        self.transactions.remove(tx_id_to_remove)
    }

}

impl CsvFormatIO<Report> for Report {
    fn new_from_csv_file<R: std::io::Read>(reader: &mut R) -> Result<Report, String> {
        
        let new_report = Report::new();
        // println!("Чтение из CSV-файла: {:?}", 5);
        // Ok(new_report)

        let mut buffer = String::new();

        // Можно весь прочитать
        // match reader.read_to_string(&mut buffer) {

        // Оборачиваем файл в BufReader
        // BufReader читает данные блоками и хранит их в буфере,
        // поэтому построчное чтение (lines()) работает быстрее, чем читать по байту

        // Читаем файл построчно
        // Каждая строка — это Result<String>, поэтому делаем if let Ok
        let buf_reader = BufReader::new(reader);

        // Читаем файл построчно
        for cur_line in buf_reader.lines() {
            match cur_line {
                Ok(ok_line) => {
                    println!("Processing line: {}", ok_line);
                    // report.lines.push(ok_line);  // Store the line
                    // return Some(new_report);
                },
                Err(e) => {
                    return Err(format!("Error reading line: {}", e));
                }
            }
        }

        // Ok(new_report)
        Err("Искусственная ошибка для проверки вызова".to_string())
    }

    fn write_to_csv_file<W: std::io::Write>(&mut self, writer: &mut W) -> Result<(), String> {
        todo!()
    }
}

impl BinFormatIO<Report> for Report {
    fn new_from_bin_file<R: std::io::Read>(reader: &mut R) -> Result<Report, String> {
        todo!()
    }

    fn write_to_bin_file<W: std::io::Write>(&mut self, writer: &mut W) -> Result<(), String> {
        todo!()
    }
}

impl TextFormatIO<Report> for Report {
    fn new_from_text_file<R: std::io::Read>(reader: &mut R) -> Result<Report, String> {
        todo!()
    }

    fn write_to_text_file<W: std::io::Write>(&mut self, writer: &mut W) -> Result<(), String> {
        todo!()
    }
}


//*** Секция тестов для Report ***/

// Child-модуль
// #[cfg(test)]
// mod tests {
//     // Подключаем всё из родительского модуля (использование методов/полей)
//     use super::*; 

//     #[test]
//     fn test_new_storage_is_empty() {
//         let bank = Report::new();
//         // При создании нет пользователей
//         assert_eq!(bank.transactions.len(), 0);
//     }

//     #[test]
//     fn test_add_user() {
//         let mut storage = Report::new();

//         // Проверка уже существующего пользователя
//         assert_eq!(storage.add_transaction("Alice".to_string()), Some(Balance::from(0))); // новый пользователь
//         assert_eq!(storage.add_transaction("Alice".to_string()), None);    // уже существует
//     }

//     #[test]
//     fn test_remove_user() {
//         let mut storage = Report::new();

//         storage.add_transaction("Bob".to_string());
//         storage.deposit(&"Bob".to_string(), 100).unwrap();

//         // Проверка баланса до и после удаления пользователя
//         assert_eq!(storage.remove_transaction(&"Bob".to_string()), Some(Balance::from(100)));
//         assert_eq!(storage.remove_transaction(&"Bob".to_string()), None); 
//     }

//     #[test]
//     fn test_deposit_and_withdraw() {
//         let mut storage = Report::new();
//         storage.add_transaction("Charlie".to_string());

//         // Пополнение
//         assert!(storage.deposit(&"Charlie".to_string(), 200).is_ok());
//         assert_eq!(storage.get_balance(&"Charlie".to_string()), Some(&Balance::from(200)));

//         // Успешное снятие
//         assert!(storage.withdraw(&"Charlie".to_string(), 150).is_ok());
//         assert_eq!(storage.get_balance(&"Charlie".to_string()), Some(&Balance::from(50)));

//         // Ошибка: недостаточно средств
//         assert!(storage.withdraw(&"Charlie".to_string(), 100).is_err());
//         // Но 50 ещё имеется
//         assert_eq!(storage.get_balance(&"Charlie".to_string()), Some(&Balance::from(50)));
//     }

//     #[test]
//     fn test_nonexistent_user() {
//         let mut storage = Report::new();

//         // Депозит несуществующему пользователю
//         assert!(storage.deposit(&"Dana".to_string(), 100).is_err());

//         // Снятие у несуществующего пользователя
//         assert!(storage.withdraw(&"Dana".to_string(), 50).is_err());

//         // Баланс у несуществующего пользователя
//         assert_eq!(storage.get_balance(&"Dana".to_string()), None);
//     }

//     use std::fs::File;
//     use std::io::Write;


//     #[test]
//     fn test_load_data_existing_file() {
//         let file_path = String::from("unit_test_file.csv");

//         // Важно сделать mut file
//         // Не нужно делать open после create (?)
//         if let Ok(mut file) = File::create(&file_path) {
//             writeln!(file, "John,100").unwrap();
//             writeln!(file, "Alice,200").unwrap();
//             writeln!(file, "Bob,50").unwrap();
//             // Здесь закроется, отрытый на чтение (?)
//             // .unwrap() нужен, судя по всему, только чтобы убрать unused warning'и
//         }
//         else {
//             panic!("Файл не создан!");
//         }

//         let storage = Report::load_file_to_storage(&file_path.as_str());     

//         assert_eq!(storage.get_balance(&"John".to_string()), Some(&Balance::from(100)));
//         assert_eq!(storage.get_balance(&"Alice".to_string()), Some(&Balance::from(200)));
//         assert_eq!(storage.get_balance(&"Bob".to_string()), Some(&Balance::from(50)));
//         // Пользователь Vasya не добавлен в файле, поэтому None
//         assert_eq!(storage.get_balance(&"Vasya".to_string()), None);

//         // Удаляем тестовый файл
//         fs::remove_file(file_path).unwrap();
//     }

//     #[test]
//     fn test_save_creates_file_with_correct_data() {
//        let file_path = "test_save.csv";

//         // Создаём Storage и добавляем пользователей
//         let mut storage = Report::new();
//         storage.add_transaction("John".to_string());
//         storage.add_transaction("Alice".to_string());
//         storage.deposit(&"John".to_string(), 150).unwrap();
//         storage.deposit(&"Alice".to_string(), 300).unwrap();

//         // Сохраняем в файл
//         storage.save_storage_to_file(file_path);

//         // Читаем файл обратно и проверяем содержимое

//         // Это читает полнстью в String, можно как сделано: let reader = BufReader::new(file);

//         // Фактически, открывает open() и читает read() одновременно
//         let contents = fs::read_to_string(file_path).unwrap();

//         let mut lines: Vec<&str> = contents.lines().collect();
//         // Сортируем, так как get_all() может возвращать в любом порядке, для проверки в тесте
//         lines.sort(); 

//         assert_eq!(lines, vec!["Alice,300", "John,150"]);

//         // Удаляем тестовый файл
//         fs::remove_file(file_path).unwrap();
//     }

//         #[test]
//     fn test_bufwriter() {
//         use std::fs::File;
//         use std::io::{BufWriter, Write};

//         let file_path = "./aux/data.csv";

//         let f = File::create(file_path).unwrap();
//         let mut writer = BufWriter::new(f);

//         writeln!(writer, "John,100").unwrap();   // пока в буфере
//         writeln!(writer, "Alice,200").unwrap();  // пока в буфере

//         let mut contents = fs::read_to_string(file_path).unwrap();
        
//         assert_eq!(contents.len(), 0);

//         // Записываем (flush the buffer), читаем ещё раз и проверяем длину (грубовато)
//         writer.flush().unwrap(); 
//         contents = fs::read_to_string(file_path).unwrap();

//         assert_eq!(contents.len(), 19);

//         // Удаляем файл
//         fs::remove_file(file_path).unwrap();
//     }

//     use std::io::{Cursor, BufWriter};

//     #[test]
//     fn test_load_data_existing_cursor() {
        
//         // (!) Создаём данные в памяти, как будто это CSV-файл
//         let data = b"John,100\nAlice,200\nBob,50\n";
//         let mut cursor = Cursor::new(&data[..]);

//         // Читаем данные из Cursor
//         let mut storage = Report::new();
//         let reader = BufReader::new(&mut cursor);

//         // То же самое, но читаем из буфера в памяти с помощью Cursor
//         for line in reader.lines() {
//             let line = line.unwrap();
//             let columns: Vec<&str> = line.trim().split(',').collect();
    
//             if columns.len() == 2 {
//                 let name = columns[0].to_string();
//                 let balance: i64 = columns[1].parse().unwrap_or(0);
//                 storage.add_transaction(name.clone());
//                 storage.deposit(&name, balance).unwrap();
//             }
//         }

//         assert_eq!(storage.get_balance(&"John".to_string()), Some(&Balance::from(100)));
//         assert_eq!(storage.get_balance(&"Alice".to_string()), Some(&Balance::from(200)));
//         assert_eq!(storage.get_balance(&"Bob".to_string()), Some(&Balance::from(50)));
//         assert_eq!(storage.get_balance(&"Vasya".to_string()), None);
//     }

//     #[test]
//     fn test_save_writes_to_cursor_correctly() {
//         // Создаём Storage и добавляем пользователей
//         let mut storage = Report::new();
//         storage.add_transaction("John".to_string());
//         storage.add_transaction("Alice".to_string());
//         storage.deposit(&"John".to_string(), 150).unwrap();
//         storage.deposit(&"Alice".to_string(), 300).unwrap();

//         // Сохраняем в память через BufWriter

//         // (!) и при чтении, и при записи используется Vec<u8>
//         let buffer = Vec::new();

//         // Буфер оборачивается в Cursor
//         let mut cursor = Cursor::new(buffer);
//         {
//             // Cursor оборачивается в BufWriter
//             let mut writer = BufWriter::new(&mut cursor);
//             // Получение всех записей в векторе запись в буфер как в файл
//             for (name, balance) in storage.get_all() {
//                 writeln!(writer, "{},{}", name, balance.get_value()).unwrap();
//             }
            
//             writer.flush().unwrap();
//         }

//         // Читаем обратно из памяти - обязательно нужно сбросить позицию в 0
//         cursor.set_position(0);

//         let mut lines: Vec<String> = BufReader::new(cursor).lines().map(|l| l.unwrap()).collect();
//         lines.sort(); // сортируем для сравнения

//         assert_eq!(lines, vec!["Alice,300", "John,150"]);
//     }

// }