use std::io::{BufRead, BufReader};
use std::fmt::Formatter;
use std::fmt;
use std::path::Path;
// use std::fs::File;
// use std::fs;

use crate::report::fmt::Display;
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
    Transfer,
    Unknown
}

// Для вывода в виде строки
impl Display for TransactionType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            TransactionType::Deposit => write!(f, "DEPOSIT"),
            TransactionType::Withdrawal=> write!(f, "WITHDRAWAL"),
            TransactionType::Transfer=> write!(f, "TRANSFER"),
            TransactionType::Unknown => write!(f, "UNKNOWN"),
        }
    }
}

// Статус транзакции
#[derive(Debug)]
enum TransactionStatus {
    Success,
    Failure,
    Pending,
    Unknown,
}

// Для вывода в виде строки
impl Display for TransactionStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            TransactionStatus::Success => write!(f, "SUCCESS"),
            TransactionStatus::Failure => write!(f, "FAILURE"),
            TransactionStatus::Pending => write!(f, "PENDING"),
            TransactionStatus::Unknown => write!(f, "UNKONWN"),
        }
    }
}

// Структура для чтения/записи транзакции
#[derive(Debug)]
pub struct Transaction {
    tx_id:          u64,
    tx_type:        TransactionType,
    from_user_id:   u64,
    to_user_id:     u64,
    amount:         u64,
    timestamp:      u64,
    status:         TransactionStatus,
    description:    String,
}

// Конструктор из значений
impl Transaction {
    pub fn new(tx_id:        u64,
               tx_type:      TransactionType,
               from_user_id: u64,
               to_user_id:   u64,
               amount:       u64,
               timestamp:    u64,
               status:       TransactionStatus,
               description:  String) -> Self 
            {
                Transaction {tx_id, tx_type, from_user_id, to_user_id, amount, timestamp, status, description}
            }
}

// Хранение отчёта в виде вектора транзакций
#[derive(Debug)]
pub struct Report {
    // Примечание: начал с HashMap, но, судя по всему, для поставленных задач проекта
    // чтения/записи/сравнения подходит больше вектор
    // transactions: HashMap<ID, Transaction>

    transactions: Vec<Transaction>
}

impl Report {
    // Конструктор, возвращем структуру Report (by value, Move-семантика)
    pub fn new() -> Self {
        Self {
            transactions: Vec::new(),
        }
    }
    
    // Добавить транзакцию (ДЛЯ HASHMAP): важно, что передаём с передачей владения для .insert()
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

    pub fn add_transaction(&mut self, tx_to_add: Transaction) -> () {
        self.transactions.push(tx_to_add)
    }

    // Удалить транзакцию (ДЛЯ HASHMAP): Option<i64> => "забираем" данные (Copy для примитивов)
    // pub fn remove_transaction(&mut self, tx_id_to_remove: &ID) -> Option<Transaction> {
    //     self.transactions.remove(tx_id_to_remove)
    // }

    pub fn get_transactions(&self) -> &Vec<Transaction> {
        &self.transactions
    }

    pub fn get_transactions_mut(&mut self) -> &mut Vec<Transaction> {
        &mut self.transactions
    }

    fn parse_u64_with_warning(in_str: &str, default_value: u64) -> u64 {
        match in_str.parse::<u64>() {
            Ok(parsed) => parsed,
            Err(_) => {
                eprintln!("Значение не распарсилось {}, устанавливается дефолтное {}", in_str, default_value);
                default_value
            }
        }
    }

    // /// Загружает данные из CSV-файла или создаёт хранилище с дефолтными пользователями
    // // (!) Важно, что функция не принимает &self
    // pub fn load_file_to_storage(file_path: &str) -> Report {
    //     let mut storage = Report::new();

    //     // Проверяем, существует ли файл
    //     if Path::new(file_path).exists() {
    //         // Открываем файл
    //         let file = File::open(file_path).unwrap();

    //         // Оборачиваем файл в BufReader
    //         // BufReader читает данные блоками и хранит их в буфере,
    //         // поэтому построчное чтение (lines()) работает быстрее, чем читать по байту
    //         let reader = BufReader::new(file);

    //         // Читаем файл построчно
    //         for cur_line in reader.lines() {
    //             // Каждая строка — это Result<String>, поэтому делаем if let Ok
    //             if let Ok(ok_line) = cur_line {
    //                 // Разделяем строку по запятой: "Name,Balance"
    //                 let columns: Vec<&str> = ok_line.trim().split(',').collect();

    //                 // Если два столбца
    //                 if columns.len() == 2 {
    //                     let name = columns[0].to_string();
    //                     // Пробуем преобразовать баланс из строки в число
    //                     let balance: i64 = columns[1].parse().unwrap_or(0);

    //                     // Добавляем пользователя и выставляем баланс
    //                     storage.add_transaction(name.clone());
    //                     let _ = storage.deposit(&name, balance);
    //                 }
    //             }
    //         }
    //     } else {
    //         // если файла нет, создаём пользователей с нуля
    //         for user in ["John", "Alice", "Bob", "Vasya"] {
    //             storage.add_transaction(user.to_string());
    //         }
    //     }

    //     storage
    // }

    // /// Сохраняет текущее состояние Storage в CSV-файл
    // pub fn save_storage_to_file(&self, file_path: &str) {
    //     let mut data = String::new();

    //     // Собираем все данные в одну строку формата "Name,Balance"
    //     // Бежим по вектору
    //     for (name, balance) in self.get_all() {
    //         // Разделяем newline'ом записи, всё по классике
    //         data.push_str(&format!("{},{}\n", name, balance.get_value()));
    //     }

    //     // Записываем в файл
    //     // Здесь мы не используем BufWriter, потому что сразу пишем всю строку целиком.
        
    //     // Создаём родительские директории
    //     if let Some(parent) = Path::new(file_path).parent() {
    //         fs::create_dir_all(parent).unwrap();
    //     }

    //     fs::write(file_path, data).expect("Не удалось записать файл");
    // }
}

impl CsvFormatIO<Report> for Report {
    fn new_from_csv_file<R: std::io::Read>(reader: R) -> Result<Report, String> {
        // Можно весь прочитать
        // match reader.read_to_string(&mut buffer) {

        let buf_reader = BufReader::new(reader);
        // Создаём итератор для пропуска header'а - первой строки 
        let mut lines = buf_reader.lines();
        let _header = lines.next();

        // Создаём новый Report и читаем файл построчно
        let mut new_report = Self::new();

        for cur_line in lines {
            match cur_line {
                Ok(ok_line) => {
                    println!("Прочитанная строка: {}", ok_line);
                    // Разделяем строку по запятым
                    let columns: Vec<&str> = ok_line.trim().split(',').collect();

                    // Если два столбца
                    if columns.len() == 8 {

                        // Получем поля из вектора:
                        // 1. Transaction ID
                        let tx_id = Report::parse_u64_with_warning(columns[0], 0);

                        // 2. Transaction Type: сравниваем с &str
                        let tx_type = match columns[1] {
                            "DEPOSIT" => TransactionType::Deposit,
                            "WITHDRAWAL" => TransactionType::Withdrawal,
                            "TRANSFER" => TransactionType::Transfer,
                            _ => TransactionType::Unknown
                        };
                        
                        // 3. From User
                        let from_user_id = Report::parse_u64_with_warning(columns[2], 0);
                        // 4. To User
                        let to_user_id = Report::parse_u64_with_warning(columns[3], 0);
                        // 5. Amount
                        let amount = Report::parse_u64_with_warning(columns[4], 0);
                        // 6. Timestamp
                        let timestamp = Report::parse_u64_with_warning(columns[5], 0);

                        // 7. Status
                        let status = match columns[6] {
                            "SUCCESS" => TransactionStatus::Success,
                            "FAILURE" => TransactionStatus::Failure,
                            "PENDING" => TransactionStatus::Pending,
                            _ => TransactionStatus::Unknown,
                        };

                        // 8. Description
                        let description = columns[7].to_string();

                        // Добавляем транзакцию в вектор          
                        new_report.add_transaction(Transaction { tx_id, tx_type, from_user_id, to_user_id, amount, timestamp, status, description });

                    }
                    else {
                        println!("Неверный формат транзакции: {}", ok_line);
                    }
                },
                Err(e) => {
                    return Err(format!("Ошибка чтения строки: {}", e));
                }
            }
        }

        // Err("Искусственная ошибка для проверки вызова".to_string())
        Ok(new_report)
    }

    fn write_to_csv_file<W: std::io::Write>(&mut self, writer: &mut W) -> Result<(), String> {
        // Собираем все данные в текстовом виде в одну строку с newline'ами
       let mut out_data = String::new();

        // Бежим по вектору (по ссылке)
        for cur_tx in &self.transactions {
            // Разделяем newline'ом записи, всё по классике
            out_data.push_str(&format!("{},{},{},{},{},{},{},{}\n", cur_tx.tx_id,
                                                                            cur_tx.tx_type,
                                                                            cur_tx.from_user_id,
                                                                            cur_tx.to_user_id,
                                                                            cur_tx.amount,
                                                                            cur_tx.timestamp,
                                                                            cur_tx.status,
                                                                            cur_tx.description));
        }

        // Не используем BufWriter, потому что сразу пишем всю строку целиком.
        // Создаём родительские директории
        // let file_path = Path::new("aux/")
        // if let Some(parent) = Path::new(file_path).parent() {
        //     fs::create_dir_all(parent).unwrap();
        // }
        
        if let Err(error) = writer.write_all(out_data.as_bytes()) {
            // .map_err(|e| FormatError::IoError(e.to_string()))?;
            return Err(error.to_string());
        }

        Ok(())
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