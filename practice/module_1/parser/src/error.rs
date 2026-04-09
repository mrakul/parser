use std::fmt::{Display, Formatter};
use std::fmt;

#[derive(PartialEq, Debug)]
pub enum ParserError {
    CsvLineReadError,
    // TODO: и в других местах можно добавить доп.информацию
    CsvWrongTransactionFormat(String),
    CsvTxWriteError,
    CsvU64IsNotParsed(String),
    CsvUnknownTxType(String),
    CsvUnknownTxStatus(String),
    
    BinTxWriteError,
    BinTxReadError,
    BinWrongMagicEncountered,
    BinReadLessThanBody,
    BinReadDescLenIsExcessive,
    BinReadNonUtf8Symbols,

    TextLineReadError,
    TextTxWriteError,
    TextTxReadError,
    TextMissingField,
    TextMissingFieldValue,
    TextWrongFieldValue,
    TextWrongLineFormat(String),
    TextWrongFieldName(String),
    TextMissingRequiredFields,
    // Для сравнения, не очень хорошо, наверное, но пока положу сюда
    ReportLengthsAreNotEqual(usize, usize),
    NonEqualTransactionFound(String, String),
    // Не очень хорошо, лучше переделать с Option<Transaction>
    EOFEncountered,
}

// Для вывода в виде строки
impl Display for ParserError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            ParserError::CsvLineReadError          => write!(f, "Ошибка чтения csv-строки"),
            ParserError::CsvWrongTransactionFormat(tx_line) => write!(f, "Встречена транзакция с неверным форматом {}", tx_line),
            ParserError::CsvTxWriteError           => write!(f, "Ошибка записи csv-строки"),
            ParserError::CsvU64IsNotParsed(in_str)  => write!(f, "Поле значения не распарсено {}", in_str),
            ParserError::CsvUnknownTxStatus(in_str)  => write!(f, "Поле статуса не распарсено {}", in_str),
            ParserError::CsvUnknownTxType(in_str)  => write!(f, "Поле типа не распарсено {}", in_str),

            ParserError::BinTxWriteError           => write!(f, "Ошибка записи bin-данных"),
            ParserError::BinTxReadError            => write!(f, "Ошибка чтения bin-данных"),
            // TODO: продолжить обработку после следующего MAGIC
            ParserError::BinWrongMagicEncountered  => write!(f, "Встретилось неверное Magic, конец обработки"),
            ParserError::BinReadLessThanBody       => write!(f, "Прочитано меньше данных, чем указанная длина Body"),
            ParserError::BinReadDescLenIsExcessive => write!(f, "Указана слишком большая длина Description, выходящая за рамки Body"),
            ParserError::BinReadNonUtf8Symbols     => write!(f, "Встречены не UTF-8 символы"),
            ParserError::TextLineReadError         => write!(f, "Ошибка чтения текстовой строки"),
            ParserError::TextTxWriteError          => write!(f, "Ошибка записи текстовой строки"),
            ParserError::TextTxReadError           => write!(f, "Ошибка чтения текстовой строки"),
            ParserError::TextMissingField          => write!(f, "Отсутствует поле"),
            ParserError::TextMissingFieldValue     => write!(f, "Отсутствует значение"),
            ParserError::TextWrongFieldValue       => write!(f, "Ошибка парсинга значения"),
            ParserError::TextWrongFieldName(field_read) => write!(f, "Встречено недопустимое слово {}", field_read),
            ParserError::TextWrongLineFormat(cur_line) => write!(f, "Неверный формат строки: {}", cur_line),
            ParserError::TextMissingRequiredFields => write!(f, "Не все поля присутствуют в транзакции"),
            ParserError::ReportLengthsAreNotEqual(rep1_len, rep2_len)               => write!(f, "Разные длины полученных отчётов: {} и {}", rep1_len, rep2_len),
            ParserError::NonEqualTransactionFound(source_tx, compared_tx) => write!(f, "Найдены отличающиеся транзакции:\n Исходный отчёт: \n{} Сравниваемый отчёт: \n{}", source_tx, compared_tx),
            ParserError::EOFEncountered => write!(f, "Конец файла"),
        }
    }
}

// Для возможности перевода в строку
impl From<ParserError> for String {
    fn from(parser_error: ParserError) -> String {
        parser_error.to_string()
    }
}
