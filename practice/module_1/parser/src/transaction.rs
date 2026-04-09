use std::fmt::Formatter;
use std::fmt;
use std::mem;

use crate::error::ParserError;
use crate::transaction::fmt::Display;

use crate::csv_format::CsvFormatIO;
use crate::text_format::TextFormatIO;
use crate::bin_format::BinFormatIO;

use std::io::ErrorKind;
use byteorder::{ByteOrder, BigEndian};

use std::collections::HashMap;

// Задаём тип (аналог using C++)
pub type _ID = u64;

// Структура для чтения/записи транзакции
#[derive(Debug, PartialEq, Clone)]
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

// Отдельная структура для представления транзакции в бинарном виде без alignment'ов
// (пока нужна только для size_of(), если будут добавляться поля)

// Header отдельно (c packed пока надо разбираться)
#[repr(Rust, packed)]
#[derive(Debug)]
pub struct BinTransactionHeader {
    // 'YPBN' = [0x59, 0x50, 0x42, 0x4E]
    _magic: [u8; 4],    
    // Прямой порядок байт?
    _record_size: u32,
}
#[repr(Rust, packed)]
pub struct BinTransactionBodyFixed {
    _tx_id: u64, 
    _tx_type: u8,  
    _from_user_id: u64,
    _to_user_id: u64,
    // | `AMOUNT` | 8 байт | знаковое 64-битное | Сумма в наименьшей денежной единице (центах). 
    // Положительное значение для зачислений, отрицательное для списаний. |
    _amount: i64,
    _timestamp: u64,
    _status: u8,
    _desc_len: u32,
    // TODO: спросить про возможность иметь указатель (?) переменой aka Flexible array member
}


// В таком случае packed может не сработать, насколько понял, пока лучше не использовать
#[repr(Rust, packed)]
pub struct _BinaryTransaction {
    pub header: BinTransactionHeader,
    pub body:   BinTransactionBodyFixed,
}

impl _BinaryTransaction {
    pub fn _size_without_description() -> usize {
        mem::size_of::<BinTransactionHeader>() +
        mem::size_of::<BinTransactionBodyFixed>()
    }
}

// Тип транзакции
#[derive(Debug, PartialEq, Clone)]
pub enum TransactionType {
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
#[derive(Debug, PartialEq, Clone)]
pub enum TransactionStatus {
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

/*** Преобразования типа и статуса для бинарного формата ***/
impl TransactionType {
    pub fn from_u8(bin_value: u8) -> Self {
        match bin_value {
            0 => TransactionType::Deposit,
            1 => TransactionType::Transfer,
            2 => TransactionType::Withdrawal,
            _ => TransactionType::Unknown,
        }
    }
    
    pub fn to_u8(&self) -> u8 {
        match self {
            TransactionType::Deposit => 0,
            TransactionType::Transfer => 1,
            TransactionType::Withdrawal => 2,
            // Подумать над ошибкой
            TransactionType::Unknown => 255,
        }
    }
}

impl TransactionStatus {
    pub fn from_u8(bin_value: u8) -> Self {
        match bin_value {
            0 => TransactionStatus::Success,
            1 => TransactionStatus::Failure,
            2 => TransactionStatus::Pending,
            _ => TransactionStatus::Unknown,
        }
    }
    
    pub fn to_u8(&self) -> u8 {
        match self {
            TransactionStatus::Success => 0,
            TransactionStatus::Failure => 1,
            TransactionStatus::Pending => 2,
            // Подумать над ошибкой
            TransactionStatus::Unknown => 255,
        }
    }
}


impl BinFormatIO<Transaction> for Transaction {
    // Пребразование в бинарные данные
    // pub fn to_binary_bytes(&self) -> Vec<u8> {
    //     ...
    // }
    
    // Используем буферизированный вывод
    fn write_as_bin_to_writer<W: std::io::Write>(&self, writer: &mut W) -> Result<(), ParserError> {
        let description_bytes = self.description.as_bytes();
        let desc_len = description_bytes.len();
        
        // Размер записи
        let record_size = mem::size_of::<BinTransactionBodyFixed>()
                                 + desc_len;
        
        // Буфер на всю запись с предвыделенной памятью в куче
        let mut record_buffer = Vec::with_capacity(record_size);
        
        // Заполняем часть Header'а
        record_buffer.extend_from_slice(&[0x59, 0x50, 0x42, 0x4E]); // 'YPBN'
        record_buffer.extend_from_slice(&(record_size as u32).to_be_bytes());
        
        // Заполняем часть Body
        record_buffer.extend_from_slice(&self.tx_id.to_be_bytes());
        record_buffer.push(self.tx_type.to_u8());
        record_buffer.extend_from_slice(&self.from_user_id.to_be_bytes());
        record_buffer.extend_from_slice(&self.to_user_id.to_be_bytes());
        record_buffer.extend_from_slice(&(self.amount as i64).to_be_bytes());
        record_buffer.extend_from_slice(&self.timestamp.to_be_bytes());
        record_buffer.push(self.status.to_u8());
        record_buffer.extend_from_slice(&(desc_len as u32).to_be_bytes());
        record_buffer.extend_from_slice(description_bytes);
        
        // Пишем, можно через .map_err()
        if let Err(_) = writer.write_all(&record_buffer) {
            return Err(ParserError::BinTxWriteError);
        }
        
        Ok(())
    }

    // Возвращаем Result<Option<Transaction>, ..., поскольку Transaction может быть не получена в случае EOF
    fn new_from_bin_reader<R: std::io::Read>(reader: &mut R) -> Result<Transaction, ParserError> {
        // Выделяем буфер для header'а
        let mut header_bytes = [0u8; mem::size_of::<BinTransactionHeader>()];

        const BODY_SIZE_NO_DESCR: usize = mem::size_of::<BinTransactionBodyFixed>();

        // Читаем строго количество байт 
        match reader.read_exact(&mut header_bytes) {
            Ok(()) => {},
            // Для обработки EOF (грубовато, лучше на Option переделать)
            Err(ref e) if e.kind() == ErrorKind::UnexpectedEof => return Err(ParserError::EOFEncountered),
            
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

        // println!("Прочитанная запись: {:?}", new_transaction);

        Ok(new_transaction)

    }
}

impl CsvFormatIO<Transaction> for Transaction {
    fn new_from_csv_reader<R: std::io::BufRead>(reader: &mut R) -> Result<Transaction, ParserError> {
        // Можно весь прочитать: match reader.read_to_string(&mut buffer) ...

        // let mut buf_reader = BufReader::new(reader);
        // Создаём итератор для пропуска header'а - первой строки 
        // let mut buf_reader = BufReader::new(reader);

        // TODO: HashMap для однократности полей (не нужен, считаем колонки)
        // let mut _cur_tx_hashmap = HashMap::<String, String>::new();
        let mut line_from_buffer = String::new();
            
        // Читаем одну строку
        let bytes_read = reader.read_line(&mut line_from_buffer)
            .map_err(|_| ParserError::CsvLineReadError)?;
            
        // Обработка EOF - пока подумать
        if bytes_read == 0 {
            return Err(ParserError::EOFEncountered);
        }
        
        // print!("Прочитанная строка: {}", line_from_buffer);

        // Разделяем строку по запятым
        let columns: Vec<&str> = line_from_buffer.trim().split(',').collect();

        // Если нужное количество столбцов
        if columns.len() == 8 {

            // Получем поля из вектора
            // 1. Transaction ID
            let tx_id = Transaction::parse_u64(columns[0])?;
            // 2. Transaction Type: сравниваем с &str
            let tx_type= Transaction::parse_tx_type(columns[1])?;
            // 3. From User
            let from_user_id = Transaction::parse_u64(columns[2])?;
            // 4. To User
            let to_user_id = Transaction::parse_u64(columns[3])?;
            // 5. Amount
            let amount = Transaction::parse_u64(columns[4])?;
            // 6. Timestamp
            let timestamp = Transaction::parse_u64(columns[5])?;
            // 7. Status
            let status= Transaction::parse_tx_status(columns[6])?;         
            // 8. Description
            let description = columns[7].to_string();

            // Всё ок, возвращаем транзакцию
            Ok(Transaction{tx_id, tx_type, from_user_id, to_user_id, amount, timestamp, status, description})
        }
        else {
            // eprintln!("Неверный формат транзакции: {}", ok_line);
            Err(ParserError::CsvWrongTransactionFormat(line_from_buffer))
        }

    }

    fn write_as_csv_to_writer<W: std::io::Write>(&self, writer: &mut W) -> Result<(), ParserError> {
       // Собираем все данные в текстовом виде в одну строку с newline'ами
       let mut out_data = String::new();

        out_data.push_str(&format!("{},{},{},{},{},{},{},{}\n", self.tx_id,
                                                                        self.tx_type,
                                                                        self.from_user_id,
                                                                        self.to_user_id,
                                                                        self.amount,
                                                                        self.timestamp,
                                                                        self.status,
                                                                        self.description));

        writer.write_all(out_data.as_bytes())
            .map_err(|_| ParserError::CsvTxWriteError)?;

        Ok(())
    }
}

impl TextFormatIO<Transaction> for Transaction {
    fn new_from_text_reader<R: std::io::BufRead>(reader: &mut R) -> Result<Transaction, ParserError> {
        // Чтение всего: match reader.read_to_string(&mut buffer) {
        // Все строки - не очень в данном случае
        // let lines = buf_reader.lines();

        // let mut buf_reader = BufReader::new(reader);
        // HashMap для однократности полей
        let mut cur_tx_hashmap = HashMap::<String, String>::new();
        let mut line_from_buffer = String::new();

        loop {
            line_from_buffer.clear();
            
            // Читаем очередную строку
            let bytes_read = reader.read_line(&mut line_from_buffer)
                .map_err(|_| ParserError::TextLineReadError)?;
            
            // Обработка EOF - пока подумать
            if bytes_read == 0 {
                if !cur_tx_hashmap.is_empty() {
                    // Неполная транзакция
                    return Err(ParserError::TextTxReadError);
                }
                return Err(ParserError::EOFEncountered);
            }

            // Обрабатывам комментарии и записи (.read_line() читает вместе с newline)
            if !line_from_buffer.trim().is_empty() {

                // Пропускаем комменты
                if line_from_buffer.starts_with('#') {
                    continue;
                }

                // println!("Прочитанная строка {}", line_from_buffer);

                // Разбиваем на токены и грубовато (для Description) проверяем, что формат "ключ: значение"
                let line_tokens: Vec<&str> = line_from_buffer.trim().split(": ").collect();        
                
                if line_tokens.len() != 2 {
                    return Err(ParserError::TextWrongLineFormat(line_from_buffer))
                }

                if !Transaction::is_acceptable_field(line_tokens[0]) {
                    return Err(ParserError::TextWrongFieldName(line_tokens[0].to_string()));
                }
                
                // TODO: проверить, что уже было поле
                cur_tx_hashmap.insert(line_tokens[0].to_string(), line_tokens[1].to_string());
            }
            // Пустая строка 
            else {

                if !cur_tx_hashmap.is_empty() {
                    // Если все поля, распарсиваем значения, сохраняем в транзакцию и возвращаем её
                    if Transaction::tx_hashmap_has_all_fields(&cur_tx_hashmap) {
                        // Не забывать, что ? возвращает unwrapped-значение, удобно
                        let transaction = Transaction::tx_from_tx_hashmap(&cur_tx_hashmap)?;
                        return Ok(transaction);
                    }
                    else {
                        println!("Не все поля присутствуют в транзакции {:?}", cur_tx_hashmap);
                        return Err(ParserError::TextMissingRequiredFields)
                    }
                }
                // Пропускаем все пустые строчки (?) (в документе строго одна)
            }
        }

        // Err("Искусственная ошибка для проверки вызова".to_string())
        // Ok(екфт)
    }

    fn write_as_text_to_writer<W: std::io::Write>(&mut self, writer: &mut W) -> Result<(), ParserError> {    
        // Пишем запись: перевод в строку => в байты
        if let Err(_) = writer.write_all(self.as_str().as_bytes()) {
            return Err(ParserError::TextTxWriteError);
        }

        Ok(())
    }
}

impl Transaction {
    pub fn as_str(&self) -> String {
        let mut tx_as_str = String::new();

        tx_as_str.push_str(&format!("# Запись о транзакции\n"));
        tx_as_str.push_str(&format!("TX_ID: {}\n", self.tx_id));
        tx_as_str.push_str(&format!("TX_TYPE: {}\n", self.tx_type));
        tx_as_str.push_str(&format!("FROM_USER_ID: {}\n", self.from_user_id));
        tx_as_str.push_str(&format!("TO_USER_ID: {}\n", self.to_user_id));
        tx_as_str.push_str(&format!("AMOUNT: {}\n", self.amount));
        tx_as_str.push_str(&format!("TIMESTAMP: {}\n", self.timestamp));
        tx_as_str.push_str(&format!("STATUS: {}\n", self.status));
        // Два newline в конце для разделения записей
        tx_as_str.push_str(&format!("DESCRIPTION: {}\n\n", self.description));

        tx_as_str
    }

    
    pub fn tx_from_tx_hashmap(tx_hashmap: &HashMap<String, String>) -> Result<Transaction, ParserError> {
        // Здесь чуть лишние проверки на наличие, tx_has_all_fields должна покрывать
        let tx_id = tx_hashmap.get("TX_ID")
            .ok_or(ParserError::TextMissingField)
            .and_then(|s| {
                s.parse::<u64>()
                    .map_err(|_| ParserError::TextWrongFieldValue)
            })?;
        
        let tx_type = tx_hashmap.get("TX_TYPE")
            .ok_or(ParserError::TextMissingField)
            .and_then(|s: &String| Self::parse_transaction_type(s))
            .map_err(|_| ParserError::TextWrongFieldValue)?;
        
        let from_user_id = tx_hashmap.get("FROM_USER_ID")
            .ok_or(ParserError::TextMissingField)
            .and_then(|s| {
                s.parse::<u64>()
                    .map_err(|_| ParserError::TextWrongFieldValue)
            })?;
        
        let to_user_id = tx_hashmap.get("TO_USER_ID")
            .ok_or(ParserError::TextMissingField)
            .and_then(|s| {
                s.parse::<u64>()
                    .map_err(|_| ParserError::TextWrongFieldValue)
            })?;
        
        let amount = tx_hashmap.get("AMOUNT")
            .ok_or(ParserError::TextMissingField)
            .and_then(|s| {
                s.parse::<u64>()
                    .map_err(|_| ParserError::TextWrongFieldValue)
            })?;
        
        let timestamp = tx_hashmap.get("TIMESTAMP")
            .ok_or(ParserError::TextMissingField)
            .and_then(|s| {
                s.parse::<u64>()
                    .map_err(|_| ParserError::TextWrongFieldValue)
            })?;
        
        let status = tx_hashmap.get("STATUS")
            .ok_or(ParserError::TextMissingField)
            .and_then(|s| Self::parse_transaction_status(s))?;
        
        let description = tx_hashmap.get("DESCRIPTION")
            .ok_or(ParserError::TextMissingField)
            .map(|s| s.to_string())?;

        Ok(Transaction::new(
            tx_id, tx_type, from_user_id, to_user_id, amount, timestamp, status, description
        ))
    }

    fn parse_transaction_type(type_str: &str) -> Result<TransactionType, ParserError> {
        match type_str {
            "DEPOSIT" => Ok(TransactionType::Deposit),
            "WITHDRAWAL" => Ok(TransactionType::Withdrawal),
            "TRANSFER" => Ok(TransactionType::Transfer),
            _ => Err(ParserError::TextWrongFieldValue),
        }
    }

    fn parse_transaction_status(status_str: &str) -> Result<TransactionStatus, ParserError> {
        match status_str {
            "SUCCESS" => Ok(TransactionStatus::Success),
            "FAILURE" => Ok(TransactionStatus::Failure),
            "PENDING" => Ok(TransactionStatus::Pending),
            _ => Err(ParserError::TextWrongFieldValue),
        }
    }

    // Проверка, что все значения есть в транзакции
    fn tx_hashmap_has_all_fields(tx_hash_map: &HashMap<String, String>) -> bool {
        const REQUIRED_FIELDS: [&str; 8] = ["TX_ID", "TX_TYPE", "FROM_USER_ID", "TO_USER_ID", "AMOUNT", "TIMESTAMP", "STATUS", "DESCRIPTION"];        

        for &required_field in &REQUIRED_FIELDS {
            if !tx_hash_map.contains_key(required_field) {
                return false;
            }
        }

        true
    }

    // Проверка, что все значения есть в транзакции
    fn is_acceptable_field(encountered_field: &str) -> bool {
        const REQUIRED_FIELDS: [&str; 8] = ["TX_ID", "TX_TYPE", "FROM_USER_ID", "TO_USER_ID", "AMOUNT", "TIMESTAMP", "STATUS", "DESCRIPTION"];        

        for &cur_field in &REQUIRED_FIELDS {
            if encountered_field == cur_field { 
                return true;
            }
        }

        false
    }

    fn _parse_u64_with_warning(in_str: &str, default_value: u64) -> u64 {
        match in_str.parse::<u64>() {
            Ok(parsed) => parsed,
            Err(_) => {
                eprintln!("Значение не распарсилось {}, устанавливается дефолтное {}", in_str, default_value);
                default_value
            }
        }
    }

    fn parse_u64(in_str: &str) -> Result<u64, ParserError> {
        match in_str.parse::<u64>() {
            Ok(parsed) => Ok(parsed),
            Err(_) => {
                // eprintln!("Значение не распарсилось {}");
                Err(ParserError::CsvU64IsNotParsed(in_str.to_string()))
            }
        }
    }

    // TODO: немного лишнего вышло, надо парсинг полей вынести в общие ошибки, пока оставляю
    // (но здесь с передачей самого поля)
    fn parse_tx_type(in_str: &str) -> Result<TransactionType, ParserError> {
        match in_str {
            "DEPOSIT" => Ok(TransactionType::Deposit),
            "WITHDRAWAL" => Ok(TransactionType::Withdrawal),
            "TRANSFER" => Ok(TransactionType::Transfer),
            _ => Err(ParserError::CsvUnknownTxType(in_str.to_string()))
        }
    }

    fn parse_tx_status(in_str: &str) -> Result<TransactionStatus, ParserError> {
        match in_str {
            "SUCCESS" => Ok(TransactionStatus::Success),
            "FAILURE" => Ok(TransactionStatus::Failure),
            "PENDING" => Ok(TransactionStatus::Pending),
            _ => Err(ParserError::CsvUnknownTxStatus(in_str.to_string()))
        }
    }
}


//*** Секция тестов для Transaction ***/

// Child-модуль
#[cfg(test)]
mod tests {
    use std::io::Cursor;

    // Подключаем всё из родительского модуля (использование методов/полей)
    use super::*; 

    const CSV_CONTENT_STR_BAD: &str = "1,DEPOSIT,100,200,1000,123456789,SUCCESS,Test transaction,,,,,,,,,,,,,,,,,,,,,,,,,,,\n";
    const CSV_CONTENT_STR_BAD_2: &str = "1,UNKONWN OPERATION,100,200,1000,123456789,SUCCESS,Test transaction\n";

    const TEXT_CONTENT_STR_BAD: &str = " 
WRONG FIELD NAME: \"Record number 2\"
TIMESTAMP: 1633036920000
STATUS: PENDING
AMOUNT: 200
TX_ID: 1000000000000001
TX_TYPE: TRANSFER
FROM_USER_ID: 9223372036854775807
TO_USER_ID: 9223372036854775807";

    const TEXT_CONTENT_STR_BAD_2: &str = " 
# Record 2 (TRANSFER)
DESCRIPTION: \"Record number 2\" DESCRIPTION 2: \"Rust is a fantastic language\"
TIMESTAMP: 1633036920000
STATUS: PENDING
AMOUNT: 200
TX_ID: 1000000000000001
TX_TYPE: TRANSFER
FROM_USER_ID: 9223372036854775807
TO_USER_ID: 9223372036854775807";

    // Исходная одна транзакция, чтобы не забивать тесты
    // LazyLock для возможности использовать Transaction::new() для static переменной
    const SOURCE_TX: std::sync::LazyLock<Transaction> = std::sync::LazyLock::new(|| {
        Transaction::new(
            1,
            TransactionType::Deposit,
            3,
            5,
            100,
            1579303033,
            TransactionStatus::Failure,
            "\"Test CSV record\"".to_string(),
        )
    });

    const SOURCE_TX_STR: &str = "1,DEPOSIT,3,5,100,1579303033,FAILURE,\"Test CSV record\"";

    /// Запись в бинарном виде в буфер и чтение, сравнение
    #[test]
    fn test_bin_tx_to_buf_to_tx() -> Result<(), ParserError> {
        let source_tx = SOURCE_TX.clone();

        // Пишем в буфер, Vec<u8> имплементирует Write
        let mut buffer = Vec::new();
        assert_eq!(source_tx.write_as_bin_to_writer(&mut buffer), Ok(()));

        // Для чтения оборачиваем в курсор
        // Ожидаем, что ошибок нет. Иначе возвращаем ошибку (ниже следую этому же подходу)
        let mut buf_cursor: Cursor<Vec<u8>> = Cursor::new(buffer);
        let read_tx = Transaction::new_from_bin_reader(&mut buf_cursor)?;
        // Если испоганить MAGIC: buffer[0] = 0x40;
        // Вывод cargo test в таком случае:
        //         failures:

        // ---- transaction::tests::test_bin_tx_to_buf_to_tx stdout ----
        // Error: BinWrongMagicEncountered

        // Сравниваем исходную с прочитанной
        println!("Исходная и прочитанная транзакции: \n{} {}", source_tx.as_str(), read_tx.as_str());
        assert_eq!(source_tx, read_tx);

        Ok(())
    }
    
    /// Запись в CSV-виде в строку и чтение, сравнение
    #[test]
    fn test_csv_tx_to_string_to_tx() -> Result<(), ParserError> {
        let source_tx = SOURCE_TX.clone();

        let mut buffer = Vec::new();

        // Пишем в буфер, Vec<u8> имплементирует Write
        assert_eq!(source_tx.write_as_csv_to_writer(&mut buffer), Ok(()));

        // Перевод в строку
        let csv_string = match String::from_utf8(buffer) {
            Ok(ok_string) => ok_string,
            Err(_) => return Err(ParserError::BinReadNonUtf8Symbols),
        };

        println!("CSV в строке: {}", csv_string);
        
        // Проверяем прочитанную строку
        assert!(csv_string.starts_with(SOURCE_TX_STR));

        // Читаем из строки и сравниваем
        let mut string_cursor = std::io::Cursor::new(csv_string);
        let read_tx = Transaction::new_from_csv_reader(&mut string_cursor)?;

        // Сравниваем исходную с прочитанной
        assert_eq!(source_tx, read_tx);

        Ok(())
    }

    /// Цепочка: 
    ///     из транзакции -> BIN -> CSV -> Text, сравнение начального с конечным и промежуточные сравнения
    #[test]
    fn test_tx_to_bin_to_csv_roundtrip() -> Result<(), ParserError> {
        let source_tx = SOURCE_TX.clone();

        // Транзакция -> BIN
        let mut bin_buffer = Vec::new();
        source_tx.write_as_bin_to_writer(&mut bin_buffer)?;
        
        // Читаем из BIN-представления
        let mut bin_cursor = std::io::Cursor::new(bin_buffer);
        let mut tx_from_bin = Transaction::new_from_bin_reader(&mut bin_cursor)?;

        // Промежуточное сравнение
        assert_eq!(source_tx, tx_from_bin);

        // BIN -> CSV
        let mut csv_buffer = Vec::new();
        let csv_write_result = tx_from_bin.write_as_csv_to_writer(&mut csv_buffer);
        assert_eq!(csv_write_result, Ok(()));
        
        // Читаем строку из буфера 
        let csv_string = match String::from_utf8(csv_buffer) {
            Ok(ok_string) => ok_string,
            Err(_) => return Err(ParserError::BinReadNonUtf8Symbols),
        };

        // Читаем транзакцию из строки
        let mut csv_cursor = std::io::Cursor::new(csv_string);
        let tx_from_csv = Transaction::new_from_csv_reader(&mut csv_cursor)?;

        // Промежуточное сравнение
        assert_eq!(tx_from_bin, tx_from_csv);

        // CSV -> Text
        let mut txt_buffer = Vec::new();
        let txt_write_result = tx_from_bin.write_as_text_to_writer(&mut txt_buffer);
        assert_eq!(txt_write_result, Ok(()));
        
        // Читаем строку из буфера 
        let txt_string = match String::from_utf8(txt_buffer) {
            Ok(ok_string) => ok_string,
            Err(_) => return Err(ParserError::BinReadNonUtf8Symbols),
        };

        // Читаем транзакцию из строки
        let mut txt_cursor = std::io::Cursor::new(txt_string);
        let tx_from_txt = Transaction::new_from_text_reader(&mut txt_cursor)?;

        // (!) Сравнение начального с конечным из текста
        assert_eq!(source_tx, tx_from_txt);

        Ok(())

    }

    // Примечание: чтение нескольких транзакций и сравнение покрыто в тестах Report.rs

    /// Плохие варианты
    /// Неверное MAGIC в BIN
    #[test]
    fn test_bin_bad_magic() -> Result<(), ParserError> {
        let source_tx = SOURCE_TX.clone();

        // Пишем в буфер, Vec<u8> имплементирующего Write
        let mut buffer = Vec::new();
        assert_eq!(source_tx.write_as_bin_to_writer(&mut buffer), Ok(()));

        // Испоганиваю MAGIC (@)
        buffer[0] = 0x40;      

        // Для чтения оборачиваем в курсор
        let mut buf_cursor: Cursor<Vec<u8>> = Cursor::new(buffer);
        assert_eq!(Transaction::new_from_bin_reader(&mut buf_cursor), Err(ParserError::BinWrongMagicEncountered));

        Ok(())
    }

    /// Испорченная длина Description
    #[test]
    fn test_bin_bad_description_len() -> Result<(), ParserError> {
        let source_tx = SOURCE_TX.clone();

        // Пишем в буфер, Vec<u8> имплементирующего Write
        let mut buffer = Vec::new();
        assert_eq!(source_tx.write_as_bin_to_writer(&mut buffer), Ok(()));

        // Испоганиваю длину
        buffer[50] = 255;
        buffer[51] = 255;
        buffer[52] = 255;
        buffer[53] = 255;

        // Для чтения оборачиваем в курсор
        let mut buf_cursor: Cursor<Vec<u8>> = Cursor::new(buffer);
        assert_eq!(Transaction::new_from_bin_reader(&mut buf_cursor), Err(ParserError::BinReadDescLenIsExcessive));

        Ok(())
    }

    /// Неверный формат записи CSV, возвращается плохая строка
    #[test]
    fn test_csv_too_much_commas() -> Result<(), ParserError> {
        
        let mut csv_cursor = std::io::Cursor::new(CSV_CONTENT_STR_BAD.to_string());
        
        assert_eq!(Transaction::new_from_csv_reader(&mut csv_cursor), Err(ParserError::CsvWrongTransactionFormat(CSV_CONTENT_STR_BAD.to_string())));
    
        Ok(())
    }

    /// Неверный формат записи CSV, возвращается плохая строка
    #[test]
    fn test_csv_wrong_tx_type() {
        let mut csv_cursor = std::io::Cursor::new(CSV_CONTENT_STR_BAD_2.to_string());
        
        assert_eq!(Transaction::new_from_csv_reader(&mut csv_cursor), Err(ParserError::CsvUnknownTxType("UNKONWN OPERATION".to_string())));
    }

    /// Текст: неверное имя поля
    #[test]
    fn test_text_wrong_record_key() -> Result<(), ParserError> {
        
        let mut txt_cursor = std::io::Cursor::new(TEXT_CONTENT_STR_BAD.to_string());
        
        assert_eq!(Transaction::new_from_text_reader(&mut txt_cursor), Err(ParserError::TextWrongFieldName("WRONG FIELD NAME".to_string())));
    
        Ok(())
    }

    /// Текст: неверный формат строки (несколько ключ: значение на одной строке)
    #[test]
    fn test_text_record_format() -> Result<(), ParserError> {
        
        let mut txt_cursor = std::io::Cursor::new(TEXT_CONTENT_STR_BAD_2.to_string());
        
        assert_eq!(Transaction::new_from_text_reader(&mut txt_cursor), Err(ParserError::TextWrongLineFormat("DESCRIPTION: \"Record number 2\" DESCRIPTION 2: \"Rust is a fantastic language\"\n".to_string())));
    
        Ok(())
    }
}