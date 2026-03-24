use std::fmt::Formatter;
use std::fmt;
use std::mem;

use crate::transaction::fmt::Display;

// Задаём тип (аналог using C++)
pub type ID = u64;

// Структура для чтения/записи транзакции
#[derive(Debug)]
pub struct Transaction {
    pub tx_id:          u64,
    pub tx_type:        TransactionType,
    pub from_user_id:   u64,
    pub to_user_id:     u64,
    pub amount:         u64,
    pub timestamp:      u64,
    pub status:         TransactionStatus,
    pub description:    String,
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

// Header отдельно
#[repr(packed)]
#[derive(Debug)]
pub struct BinTransactionHeader {
    // 'YPBN' = [0x59, 0x50, 0x42, 0x4E]
    magic: [u8; 4],    
    // Прямой порядок байт?
    record_size: u32,
}
#[repr(packed)]
pub struct BinTransactionBodyFixed {
    tx_id: u64, 
    tx_type: u8,  
    from_user_id: u64,
    to_user_id: u64,
    // | `AMOUNT` | 8 байт | знаковое 64-битное | Сумма в наименьшей денежной единице (центах). 
    // Положительное значение для зачислений, отрицательное для списаний. |
    amount: i64,
    timestamp: u64,
    status: u8,
    desc_len: u32,
    // TODO: спросить про возможность иметь указатель (?) переменой aka Flexible array member
}


// В таком случае packed может не сработать, насколько понял, пока лучше не использовать
#[repr(packed)]  
pub struct BinaryTransaction {
    pub header: BinTransactionHeader,
    pub body:   BinTransactionBodyFixed,
}

impl BinaryTransaction {
    pub fn size_without_description() -> usize {
        mem::size_of::<BinTransactionHeader>() +
        mem::size_of::<BinTransactionBodyFixed>()
    }
}

// Тип транзакции
#[derive(Debug)]
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
#[derive(Debug)]
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


impl Transaction {
    // Пребразование в бинарные данные
    // pub fn to_binary_bytes(&self) -> Vec<u8> {
    //     ...
    // }
    
    // Используем буферизированный вывод
    pub fn write_to_binary_writer<W: std::io::Write>(&self, writer: &mut W) -> Result<(), String> {
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
        
        // Пишем в буфер
        writer.write_all(&record_buffer)
            .map_err(|e| format!("Не удалось записать транзакцию: {}", e))?;
        
        Ok(())
    }
}