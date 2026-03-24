use byteorder::{ByteOrder, BigEndian};

use std::io::{BufRead, BufReader};
use std::mem;
use std::io::ErrorKind;
use std::collections::HashMap;

use crate::transaction::{BinTransactionHeader, BinTransactionBodyFixed, Transaction, TransactionStatus, TransactionType};
use crate::csv_format::CsvFormatIO;
use crate::text_format::TextFormatIO;
use crate::bin_format::BinFormatIO;
use crate::error::ParserError;

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
    
    fn add_transaction(&mut self, tx_to_add: Transaction) -> () {
        self.transactions.push(tx_to_add)
    }


    /*** Реализации для HashMap, если понадобятся ***/
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

    // TODO: эту секцию перенести в Transaction (?)

    // Возвращаем Result<Option<Transaction>, ..., поскольку Transaction может быть не получена в случае EOF
    fn read_one_bin_transaction<R: std::io::Read>(reader: &mut R) -> Result<Option<Transaction>, ParserError> {
        // Выделяем буфер для header'а
        let mut header_bytes = [0u8; mem::size_of::<BinTransactionHeader>()];

        const BODY_SIZE_NO_DESCR: usize = mem::size_of::<BinTransactionBodyFixed>();

        // Читаем строго количество байт 
        match reader.read_exact(&mut header_bytes) {
            Ok(()) => {},
            // Для обработки EOF
            Err(ref e) if e.kind() == ErrorKind::UnexpectedEof => return Ok(None),
            // Err(e) => return Err(format!("Ошибка чтения header'а: {}", e)),
            Err(_) => return Err(ParserError::BinTxReadError),
        }

        // Используем внешний crate byteorder для переводов из сетевого порядка байт и обратно
        let magic = &header_bytes[0..4];
        // Для чтения используем слайсы - ключевой момент
        let record_size = BigEndian::read_u32(&header_bytes[4..8]) as usize;
        
        // Проверка на 'YPBN'
        const EXPECTED_MAGIC: [u8; 4] = [0x59, 0x50, 0x42, 0x4E]; 
        
        // Позволяет сравнивать таким образом
        if magic != EXPECTED_MAGIC {
            // return Err(format!("Неверное magic: {:?}, должно быть {:?}", magic, EXPECTED_MAGIC));
            return Err(ParserError::BinWrongMagicEncountered);
        }
        
        // Читаем body
        let mut body_bytes = vec![0u8; record_size];
        reader.read_exact(&mut body_bytes)
            // .map_err(|| format!("Не смогли прочитать {}", e))?;
            .map_err(|_| ParserError::BinTxReadError)?;
        
        // Прочитали меньше чем тело записи
        if body_bytes.len() < BODY_SIZE_NO_DESCR {
            // return Err("Слишком короткая запись".to_string());
            return Err(ParserError::BinReadLessThanBody);
        }
        
        // Извлекаем остальные записи
        let mut offset = 0;
        let mut field_len = mem::size_of::<u64>();

        let tx_id = BigEndian::read_u64(&body_bytes[offset..offset + field_len]); 
        offset += field_len;

        field_len = mem::size_of::<u8>();
        let tx_type = body_bytes[offset]; 
        offset += field_len;

        field_len = mem::size_of::<u64>();
        let from_user_id = BigEndian::read_u64(&body_bytes[offset .. offset + field_len]); 
        offset += field_len;

        field_len = mem::size_of::<u64>();
        let to_user_id = BigEndian::read_u64(&body_bytes[offset .. offset + field_len]); 
        offset += field_len;

        field_len = mem::size_of::<i64>();
        let amount = BigEndian::read_i64(&body_bytes[offset .. offset + field_len]); 
        offset += field_len;

        field_len = mem::size_of::<u64>();
        let timestamp = BigEndian::read_u64(&body_bytes[offset .. offset + field_len]); 
        offset += field_len;

        field_len = mem::size_of::<u8>();
        let status = body_bytes[offset]; 
        offset += field_len;

        field_len = mem::size_of::<u32>();
        let desc_len = BigEndian::read_u32(&body_bytes[offset .. offset + field_len]) as usize; 
        offset += field_len;
                
        
        // Проверка на длину Description
        if offset + desc_len > body_bytes.len() {
            // return Err("Указана слишком большая длина Description".to_string());
            return Err(ParserError::BinReadDescLenIsExcessive);
        }
        
        // Забираем description
        let description_bytes = &body_bytes[offset .. offset + desc_len];
        
        // Копируем - плохо
        // let description = String::from_utf8(description_bytes.to_vec())
        //     .map_err(|e| format!("Только UTF-8 символы: {}", e))?;

        // Через слайс
        let description = std::str::from_utf8(description_bytes)
            // .map_err(|e| format!("Только UTF-8 символы: {}", e))?
            .map_err(|_| ParserError::BinReadNonUtf8Symbols)?
            .to_string();

        let new_transaction = Transaction::new(tx_id,
                                                            TransactionType::from_u8(tx_type),
                                                            from_user_id,
                                                            to_user_id,
                                                            amount as u64,
                                                            timestamp,
                                                            TransactionStatus::from_u8(status),
                                                            description); 

        println!("Прочитанная запись: {:?}", new_transaction);

        Ok(Some(new_transaction))

    }

    fn tx_from_tx_hashmap(tx_hashmap: &HashMap<String, String>) -> Result<Transaction, String> {

        let tx_id = tx_hashmap.get("TX_ID")
            .ok_or_else(|| "Отсутствует поле TX_ID в записи".to_string())
            .and_then(|s| s.parse::<u64>().map_err(|e| format!("Не распарсился TX_ID: {}", e)))?;
        
        let tx_type = tx_hashmap.get("TX_TYPE")
            .ok_or_else(|| "Отсутствует поле TX_TYPE в записи".to_string())
            .and_then(|s: &String| Self::parse_transaction_type(s))?;
        
        let from_user_id = tx_hashmap.get("FROM_USER_ID")
            .ok_or_else(|| "Отсутствует поле FROM_USER_ID в записи".to_string())
            .and_then(|s| s.parse::<u64>().map_err(|e| format!("Не распарсился FROM_USER_ID: {}", e)))?;
        
        let to_user_id = tx_hashmap.get("TO_USER_ID")
            .ok_or_else(|| "Отсутствует поле TO_USER_ID в записи".to_string())
            .and_then(|s| s.parse::<u64>().map_err(|e| format!("Не распарсился TO_USER_ID: {}", e)))?;
        
        let amount = tx_hashmap.get("AMOUNT")
            .ok_or_else(|| "Отсутствует поле AMOUNT в записи".to_string())
            .and_then(|s| s.parse::<u64>().map_err(|e| format!("Не распарсился AMOUNT: {}", e)))?;
        
        let timestamp = tx_hashmap.get("TIMESTAMP")
            .ok_or_else(|| "Отсутствует поле TIMESTAMP в записи".to_string())
            .and_then(|s| s.parse::<u64>().map_err(|e| format!("Не распарсился TIMESTAMP: {}", e)))?;
        
        let status = tx_hashmap.get("STATUS")
            .ok_or_else(|| "Отсутствует поле STATUS в записи".to_string())
            .and_then(|s| Self::parse_transaction_status(s))?;
        
        let description = tx_hashmap.get("DESCRIPTION")
            .ok_or_else(|| "Отсутствует поле DESCRIPTION в записи".to_string())
            .map(|s| s.to_string())?;

            Ok(Transaction::new(
            tx_id, tx_type, from_user_id, to_user_id, amount, timestamp, status, description
        ))
    }

    fn parse_transaction_type(type_str: &str) -> Result<TransactionType, String> {
        match type_str {
            "DEPOSIT" => Ok(TransactionType::Deposit),
            "WITHDRAWAL" => Ok(TransactionType::Withdrawal),
            "TRANSFER" => Ok(TransactionType::Transfer),
            _ => Ok(TransactionType::Unknown),
        }
    }

    fn parse_transaction_status(status_str: &str) -> Result<TransactionStatus, String> {
        match status_str {
            "SUCCESS" => Ok(TransactionStatus::Success),
            "FAILURE" => Ok(TransactionStatus::Failure),
            "PENDING" => Ok(TransactionStatus::Pending),
            _ => Ok(TransactionStatus::Unknown),
        }
    }

    // Проверка, что все значения есть в транзакции
    fn tx_has_all_fields(tx_hash_map: &HashMap<String, String>) -> bool {
        const REQUIRED_FIELDS: [&str; 8] = ["TX_ID", "TX_TYPE", "FROM_USER_ID", "TO_USER_ID", "AMOUNT", "TIMESTAMP", "STATUS", "DESCRIPTION"];        

        for &required_field in &REQUIRED_FIELDS {
            if !tx_hash_map.contains_key(required_field) {
                return false;
            }
        }

        true
    }
}


/// Реализация трейта для парсинга из CSV-формата в Report и обратно
impl CsvFormatIO<Report> for Report {
    /// Получение структуры Report из ввода СSV-формата
    /// Аргументы: <R: std::io::Read>(reader: R)
    /// Результат: Result<Report, String>
    /// Использование:
    ///     let mut report = Report::new_from_text_file(&mut file_to_read)
    ///         {...}
    fn new_from_csv_file<R: std::io::Read>(reader: R) -> Result<Report, ParserError> {
        // Можно весь прочитать: match reader.read_to_string(&mut buffer) ...

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
                        // eprintln!("Неверный формат транзакции: {}", ok_line);
                        return Err(ParserError::CsvWrongTransactionFormat(ok_line));
                    }
                },
                Err(_e) => {
                    return Err(ParserError::CsvLineReadError);
                }
            }
        }

        // Искусственная ошибка для проверки вызова
        // Err(ParserError::CsvLineReadError)
        Ok(new_report)
    }

    /// Перевод структуры Report в СSV-формата
    /// Аргументы: <W: std::io::Write>(&mut self, writer: &mut W)
    /// Результат: Result<(), String>
    /// Использование:
    ///     let mut report = Report::new_from_text_file(&mut file_to_read)
    ///         {...}
    fn write_to_csv_file<W: std::io::Write>(&mut self, writer: &mut W) -> Result<(), ParserError> {
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
        
        writer.write_all(out_data.as_bytes())
            .map_err(|_| ParserError::CsvTxWriteError)?;

        Ok(())
    }
}


/// Реализация трейта для парсинга из Bin-формата в Report и обратно
impl BinFormatIO<Report> for Report {
    fn new_from_bin_file<R: std::io::Read>(mut reader: R) -> Result<Report, ParserError> {
        let mut report = Report::new();
        
        // Идём по списку, читая по одному
        loop {
            match Report::read_one_bin_transaction(&mut reader) {
                Ok(Some(transaction)) => {
                    report.add_transaction(transaction);
                },
                Ok(None) => {
                    // EOF => заканчиваем чтение
                    break;
                },
                Err(e) => {
                    return Err(e);
                }
            }
        }
        
        Ok(report)
    }

    fn write_to_bin_file<W: std::io::Write>(&mut self, mut writer: &mut W) -> Result<(), ParserError> {
        
        for transaction in &self.transactions {
            transaction.write_to_binary_writer(&mut writer)?;

            // Запись каждой транзакции в случае буферизированного вывода
            // writer.flush()
            //     .map_err(|e| format!("Не удалось транзакцию: {:?} => {}", transaction, e))?;
        }
        
        // Сразу все транзакции (или пока буфер не заполнится?)
        writer.flush()
            .map_err(|_| ParserError::BinTxWriteError)?;
        
        Ok(())
    }
}

/// Реализация трейта для парсинга из текстового формата в Report и обратно
impl TextFormatIO<Report> for Report {
    fn new_from_text_file<R: std::io::Read>(reader:  R) -> Result<Report, ParserError> {
        // match reader.read_to_string(&mut buffer) {

        let buf_reader = BufReader::new(reader);
        let lines = buf_reader.lines();

        // Создаём новый Report и читаем файл построчно
        let mut new_report = Self::new();

        // HashMap для однократности ключа
        let mut cur_tx_hashmap = HashMap::<String, String>::new();

        for cur_line in lines {
            match cur_line {
                Ok(ok_line) => {
                    println!("Прочитанная строка: {}", ok_line);

                    // Пропускаем комментарии
                    if ok_line.starts_with("#") == true {
                        continue;
                    }
                    // Или пустая строка (должна быть одна?)
                    else if ok_line.is_empty() {
                        if Report::tx_has_all_fields(&cur_tx_hashmap) {
                            if let Ok(transaction) = Report::tx_from_tx_hashmap(&cur_tx_hashmap) {
                                new_report.add_transaction(transaction);
                                cur_tx_hashmap.clear();
                            }
                            else {
                                eprintln!("Не все поля распарсились");
                                cur_tx_hashmap.clear();
                            }
                        }
                        else {
                            eprintln!("В транзакции не все поля");
                            cur_tx_hashmap.clear();
                        }

                        continue;
                    } 
                    // Обработка записи - чуть грубовато, разделение по ": " на два токена
                    else {
                        let line_tokens: Vec<&str> = ok_line.trim().split(": ").collect();
                        
                        if line_tokens.len() != 2 {
                            continue;
                        }

                        // TODO: проверить, что уже было значение
                        cur_tx_hashmap.insert(line_tokens[0].to_string(), line_tokens[1].to_string());
                    }
                },
                Err(_) => {
                    return Err(ParserError::TextLineReadError);
                }
            }
        }

        // Err("Искусственная ошибка для проверки вызова".to_string())
        Ok(new_report)
    }

    fn write_to_text_file<W: std::io::Write>(&mut self, writer: &mut W) -> Result<(), ParserError> {
        // Собираем все данные в текстовом виде в одну строку с newline'ами
       let mut out_data = String::new();

        // Бежим по вектору (по ссылке)
        for cur_tx in &self.transactions {
            // Разделяем newline'ом записи
            out_data.push_str(&format!("# Запись о транзакции\n"));
            out_data.push_str(&format!("TX_ID: {}\n", cur_tx.tx_id));
            out_data.push_str(&format!("TX_TYPE: {}\n", cur_tx.tx_type));
            out_data.push_str(&format!("FROM_USER_ID: {}\n", cur_tx.from_user_id));
            out_data.push_str(&format!("TO_USER_ID: {}\n", cur_tx.to_user_id));
            out_data.push_str(&format!("AMOUNT: {}\n", cur_tx.amount));
            out_data.push_str(&format!("TIMESTAMP: {}\n", cur_tx.timestamp));
            out_data.push_str(&format!("STATUS: {}\n", cur_tx.status));
            // Два newline в конце для разделения записей
            out_data.push_str(&format!("DESCRIPTION: {}\n\n", cur_tx.description));
        }

        if let Err(_) = writer.write_all(out_data.as_bytes()) {
            // .map_err(|e| FormatError::IoError(e.to_string()))?;
            return Err(ParserError::TextTxWriteError);
        }

        Ok(())
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