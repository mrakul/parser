use std::fmt::{Display, Formatter};
use std::fmt;

pub enum ParserError {
    CsvLineReadError,
    // TODO: и в других местах можно добавить доп.информацию
    CsvWrongTransactionFormat(String),
    CsvTxWriteError,
    BinTxWriteError,
    BinTxReadError,
    BinWrongMagicEncountered,
    BinReadLessThanBody,
    BinReadDescLenIsExcessive,
    BinReadNonUtf8Symbols,
    TextLineReadError,
    TextTxWriteError,
}

// Для вывода в виде строки
impl Display for ParserError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            ParserError::CsvLineReadError          => write!(f, "Ошибка чтения csv-строки"),
            ParserError::CsvWrongTransactionFormat(tx_line) => write!(f, "Встречена транзакция с неверным форматом {}", tx_line),
            ParserError::CsvTxWriteError           => write!(f, "Ошибка записи csv-строки"),
            ParserError::BinTxWriteError           => write!(f, "Ошибка записи bin-данных"),
            ParserError::BinTxReadError            => write!(f, "Ошибка чтения bin-данных"),
            // TODO: продолжить обработку после следующего MAGIC
            ParserError::BinWrongMagicEncountered  => write!(f, "Встретилось неверное Magic, конец обработки"),
            ParserError::BinReadLessThanBody       => write!(f, "Прочитано меньше данных, чем указанная длина Body"),
            ParserError::BinReadDescLenIsExcessive => write!(f, "Указана слишком большая длина Description, выходящая за рамки Body"),
            ParserError::BinReadNonUtf8Symbols     => write!(f, "Встречены не UTF-8 символы"),
            ParserError::TextLineReadError         => write!(f, "Ошибка чтения текстовой строки"),
            ParserError::TextTxWriteError          => write!(f, "Ошибка записи текстовой строки"),
        }
    }
}

// Для возможности перевода в строку
impl From<ParserError> for String {
    fn from(parser_error: ParserError) -> String {
        parser_error.to_string()
    }
}
