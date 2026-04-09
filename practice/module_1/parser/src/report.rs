use crate::csv_format::CsvFormatIO;
use crate::text_format::TextFormatIO;
use crate::bin_format::BinFormatIO;

use crate::transaction::{Transaction};
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
    
    fn add_transaction(&mut self, tx_to_add: Transaction) {
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

    /// Получение ссылки на внутреннюю структуру хранения транзакций (сейчас вектор)
    pub fn get_transactions(&self) -> &Vec<Transaction> {
        &self.transactions
    }
    /// Получение мутабельной ссылки на внутреннюю структуру хранения транзакций (сейчас вектор)
    pub fn get_transactions_mut(&mut self) -> &mut Vec<Transaction> {
        &mut self.transactions
    }

    /// Функция сравнения отчёта с передаваемым в функцию
    /// Проверяется длина. Если длины равны, сравнивается контент последовательно.
    /// В случае неравенства отчётов - возвращаются ошибки:
    ///  - ParserError::ReportLengthsAreNotEqual(len_src, len_compared)
    ///  - ParserError::NonEqualTransactionFound(cur_tx_str, compared_tx_str)
    ///
    pub fn compare_full(&self, report_to_compare: &Report) -> Result<(), ParserError> {
        if self.transactions.len() != report_to_compare.transactions.len() {
            return Err(ParserError::ReportLengthsAreNotEqual(self.transactions.len(), report_to_compare.transactions.len()))
        }

        let mut compared_iter = report_to_compare.transactions.iter();

        for source_tx in self.transactions.iter() {
            if let Some(compared_tx) = compared_iter.next() {
                if source_tx != compared_tx {
                    // Переводим в текстовое представление для ошибок
                    let cur_tx_str = source_tx.as_str();
                    let compared_tx_str = compared_tx.as_str();

                    return Err(ParserError::NonEqualTransactionFound(cur_tx_str, compared_tx_str));
                }
            }
            // В теории не должно быть, длины проверены
            else {
                // Пока так
                unreachable!()
            }
        }

        Ok(())
    }

    pub fn compare_streaming(&self, _report_to_compare: &Report) -> Result<(), ParserError> {
        todo!()
    }

}


/// Реализация трейта для парсинга из CSV-формата в Report и обратно
impl CsvFormatIO<Report> for Report {
    /// Получение Report из ввода СSV-формата
    /// Аргументы: <R: std::io::Read>(reader: R)
    /// Результат: Result<Report, ParserError>
    /// Использование:
    ///     let mut report = Report::new_from_csv_reader(&mut reader)?;
    fn new_from_csv_reader<R: std::io::BufRead>(reader: &mut R) -> Result<Report, ParserError> {
        // Можно весь прочитать: match reader.read_to_string(&mut buffer) ...
        // let mut buf_reader = BufReader::new(reader);

        // Первую строку надо пропустить (и можно обработать)
        let mut header_line = String::new();
        let _bytes_read = reader.read_line(&mut header_line)
            .map_err(|_| ParserError::CsvLineReadError)?;

        // // Создаём итератор для пропуска header'а - первой строки 
        // // let mut lines = buf_reader.lines();
        // // let _header = lines.next();

        // Создаём новый Report и читаем файл построчно
        let mut report = Self::new();
        
        // Идём по списку, читая по одному
        loop {
            match Transaction::new_from_csv_reader(reader) {
                Ok(new_transaction) => {
                    report.add_transaction(new_transaction);
                },
                Err(ParserError::EOFEncountered) => {
                    // EOF => заканчиваем чтение
                    // TODO: подумать для Streaming'а
                    break;
                },
                Err(e) => {
                    return Err(e);
                }
            }
        }
        
        Ok(report)
    }

    /// Перевод Report в СSV-формат
    /// Аргументы: <W: std::io::Write>(&mut self, writer: &mut W)
    /// Результат: Result<(), ParserError>
    /// Использование:
    ///    let mut buffer = Vec::new();
    ///    report_from_csv.write_as_csv_to_writer(&mut buffer)?;
    fn write_as_csv_to_writer<W: std::io::Write>(&self, writer: &mut W) -> Result<(), ParserError> {

        // Для симметричности при чтении => пишем при записи строку назад
        writer.write_all("TX_ID,TX_TYPE,FROM_USER_ID,TO_USER_ID,AMOUNT,TIMESTAMP,STATUS,DESCRIPTION\n".as_bytes())
            .map_err(|_| ParserError::CsvTxWriteError)?;

        // Бежим по вектору (по мутабельным ссылкам)
        for cur_tx in self.transactions.iter() {
            cur_tx.write_as_csv_to_writer(writer)?;
        }

        Ok(())
    }
}


/// Реализация трейта для парсинга из Bin-формата в Report и обратно
impl BinFormatIO<Report> for Report {
    /// Получение Report из ввода BIN-формата
    /// Аргументы: <R: std::io::Read>(reader: &mut R)
    /// Результат: Result<Report, ParserError>
    /// Использование:
    ///   let mut report_from_bin = Report::new_from_bin_reader(&mut reader)?;
    fn new_from_bin_reader<R: std::io::Read>(reader: &mut R) -> Result<Report, ParserError> {
        let mut report = Self::new();
        
        // Идём по списку, читая по одному
        loop {
            match Transaction::new_from_bin_reader(reader) {
                Ok(new_transaction) => {
                    report.add_transaction(new_transaction);
                },
                Err(ParserError::EOFEncountered) => {
                    // EOF => заканчиваем чтение
                    // TODO: подумать для Streaming'а
                    break;
                },
                Err(e) => {
                    return Err(e);
                }
            }
        }
        
        Ok(report)
    }
    /// Перевод Report в BIN-формат
    /// Аргументы: <W: std::io::Write>(&mut self, writer: &mut W)
    /// Результат: Result<(), ParserError>
    /// Использование:
    /// Пишем в буфер как текст
    ///    let mut buffer = Vec::new();
    ///    report_from_csv.write_as_bin_to_writer(&mut buffer)?;
    fn write_as_bin_to_writer<W: std::io::Write>(&self, mut writer: &mut W) -> Result<(), ParserError> {

        for cur_tx in self.transactions.iter() {
            // Теперь пишется транзакцией 
            cur_tx.write_as_bin_to_writer(&mut writer)?;

            // Запись каждой транзакции в случае буферизированного вывода
            // writer.flush()
            //     .map_err(|e| format!("Не удалось транзакцию: {:?} => {}", transaction, e))?;
        }
        
        // Сразу все транзакции (или пока буфер не заполнится?)
        // writer.flush()
        //     .map_err(|_| ParserError::BinTxWriteError)?;
        
        Ok(())
    }
}

/// Реализация трейта для парсинга из текстового формата в Report и обратно
impl TextFormatIO<Report> for Report {
    /// Получение структуры Report из ввода текстового формата
    /// Аргументы: <R: std::io::Read>(reader: R)
    /// Результат: Result<Report, String>
    /// Использование:
    ///     let mut report = Report::new_from_text_reader(&mut file_to_read)
    ///         {...}
    fn new_from_text_reader<R: std::io::BufRead>(reader: &mut R) -> Result<Report, ParserError> {
        // Чтение всего
        // match reader.read_to_string(&mut buffer) {

        let mut report = Self::new();
        
        // Идём по списку, читая по одному
        loop {
            match Transaction::new_from_text_reader(reader) {
                Ok(new_transaction) => {
                    report.add_transaction(new_transaction);
                },
                Err(ParserError::EOFEncountered) => {
                    // EOF => заканчиваем чтение
                    // TODO: подумать для Streaming'а
                    break;
                },
                Err(e) => {
                    return Err(e);
                }
            }
        }
        
        Ok(report)
    }
    /// Перевод Report в текстовый
    /// Аргументы: <W: std::io::Write>(&mut self, writer: &mut W)
    /// Результат: Result<(), ParserError>
    /// Использование:
    /// Пишем в буфер как текст
    ///    let mut buffer = Vec::new();
    ///    report_from_csv.write_as_text_to_writer(&mut buffer)?;
    fn write_as_text_to_writer<W: std::io::Write>(&mut self, writer: &mut W) -> Result<(), ParserError> {
        // Бежим по вектору (по ссылке)
        for cur_tx in self.transactions.iter_mut() {
            cur_tx.write_as_text_to_writer(writer)?;
        }

        Ok(())
    }
}

//*** Секция тестов для Report ***/

#[cfg(test)]
mod tests {
    use std::io::Cursor;
    // Добавляем TransactionType, TransactionStatus отдельной записи транзакций в отчёт
    use crate::transaction::{Transaction, TransactionType, TransactionStatus};

    // Подключаем всё из родительского модуля (использование методов/полей)
    use super::*; 

    // CSV: корректные транзакции
    const CSV_CONTENT_STR: &str = "TX_ID,TX_TYPE,FROM_USER_ID,TO_USER_ID,AMOUNT,TIMESTAMP,STATUS,DESCRIPTION
1,DEPOSIT,100,200,1000,123456789,SUCCESS,Test transaction
2,TRANSFER,100,0,500,123456790,FAILURE,Withdrawal";

    const CSV_CONTENT_STR_BAD: &str = "TX_ID,TX_TYPE,FROM_USER_ID,TO_USER_ID,AMOUNT,TIMESTAMP,STATUS,DESCRIPTION
1,DEPOSIT,100,200,1000,123456789,SUCCESS,Test transaction
2,NON-EXISTING OPERATION,100,0,500,123456790,FAILURE,Withdrawal";

    // TXT: корректные транзакции
    const TEXT_CONTENT_STR: &str = "
# Record 1 (DEPOSIT)
TX_TYPE: DEPOSIT
TO_USER_ID: 9223372036854775807
FROM_USER_ID: 0
TIMESTAMP: 1633036860000
DESCRIPTION: \"Record number 1\"
TX_ID: 1000000000000000
AMOUNT: 100
STATUS: FAILURE

# Record 2 (TRANSFER)
DESCRIPTION: \"Record number 2\"
TIMESTAMP: 1633036920000
STATUS: PENDING
AMOUNT: 200
TX_ID: 1000000000000001
TX_TYPE: TRANSFER
FROM_USER_ID: 9223372036854775807
TO_USER_ID: 9223372036854775807\n\n";

    // TXT: транзакции с ошибками
    const TEXT_CONTENT_STR_BAD: &str = " 
WRONG FIELD NAME: \"Record number 2\"
TIMESTAMP: 1633036920000
STATUS: PENDING
AMOUNT: 200
TX_ID: 1000000000000001
TX_TYPE: TRANSFER
FROM_USER_ID: 9223372036854775807
TO_USER_ID: 9223372036854775807";

    // Исходная одна транзакция, чтобы не забивать тесты
    // LazyLock для возможности использовать Transaction::new() для static переменной
    const SOURCE_TX_1: std::sync::LazyLock<Transaction> = std::sync::LazyLock::new(|| {
        Transaction::new(
            1,
            TransactionType::Deposit,
            3,
            5,
            100,
            1579303033,
            TransactionStatus::Failure,
            "\"TX 1\"".to_string(),
        )
    });

    const SOURCE_TX_2: std::sync::LazyLock<Transaction> = std::sync::LazyLock::new(|| {
        Transaction::new(
            2,
            TransactionType::Transfer,
            3,
            5,
            100,
            1579303033,
            TransactionStatus::Success,
            "\"TX 2\"".to_string(),
        )
    });


    /// TXT: чтение-запись-чтение-сравнение
    #[test]
    fn test_text_report_ok() -> Result<(), ParserError> {
        let source_transactions_txt = TEXT_CONTENT_STR.to_string();
        let mut source_tx_cursor = Cursor::new(source_transactions_txt);

        // Исходный отчёт во внутреннем представлении из текста
        let mut report_from_txt = Report::new_from_text_reader(&mut source_tx_cursor)?;

        // Пишем в буфер как текст
        let mut buffer = Vec::new();
        report_from_txt.write_as_text_to_writer(&mut buffer)?;

        // Для чтения оборачиваем в курсор
        let mut buf_cursor: Cursor<Vec<u8>> = Cursor::new(buffer);
        let report_from_txt_after_write = Report::new_from_text_reader(&mut buf_cursor)?;

        // Сравниваем отчёты (для вывода в случае успеха: cargo test --no-capture)
        println!("Исходный и прочитанный отчёты: \n{:?} \n {:?}", report_from_txt, report_from_txt_after_write);
        report_from_txt.compare_full(&report_from_txt_after_write)?;

        Ok(())
    }
    
    /// CSV: чтение-запись-чтение-сравнение
    #[test]
    fn test_csv_report_ok() -> Result<(), ParserError> {
        let source_transactions_csv = CSV_CONTENT_STR.to_string();
        let mut source_tx_cursor = Cursor::new(source_transactions_csv);

        // Исходный отчёт во внутреннем представлении из текста
        let report_from_csv = Report::new_from_csv_reader(&mut source_tx_cursor)?;

        // Пишем в буфер как текст
        let mut buffer = Vec::new();
        report_from_csv.write_as_csv_to_writer(&mut buffer)?;

        // Для чтения оборачиваем в курсор
        let mut buf_cursor: Cursor<Vec<u8>> = Cursor::new(buffer);

        let report_from_csv_after_write = Report::new_from_csv_reader(&mut buf_cursor)?;

        // Сравниваем отчёты
        println!("Исходный и прочитанный отчёты: \n{:?} \n {:?}", report_from_csv, report_from_csv_after_write);
        report_from_csv.compare_full(&report_from_csv_after_write)?;

        
        Ok(())
    }

    /// BIN: запись-чтение-сравнение
    #[test]
    fn test_bin_report_ok() -> Result<(), ParserError> {
        // Создаём Report из двух транзакций
        let mut source_report = Report::new();
        source_report.add_transaction(SOURCE_TX_1.clone());
        source_report.add_transaction(SOURCE_TX_2.clone());

        // Пишем в буфер => BIN
        let mut buffer = Vec::new();
        source_report.write_as_bin_to_writer(&mut buffer)?;

        // Читаем отчёт после записи из BIN
        let mut buf_cursor = Cursor::new(buffer);
        let mut report_from_bin = Report::new_from_bin_reader(&mut buf_cursor)?;

        // Сравниваем отчёты
        println!("Исходный и прочитанный отчёты: \n{:?} \n {:?}", source_report, report_from_bin);
        source_report.compare_full(&report_from_bin)?;

        // Добавляем одну транзакцию ко второму => длины не равны
        report_from_bin.add_transaction(SOURCE_TX_1.clone());
        assert_eq!(source_report.compare_full(&report_from_bin), Err(ParserError::ReportLengthsAreNotEqual(2, 3)));
            
        Ok(())
    }

    /// CSV: чтение-запись-чтение-сравнение
    #[test]
    fn test_csv_report_error() -> Result<(), ParserError> {
        let source_transactions_csv = CSV_CONTENT_STR_BAD.to_string();
        let mut source_tx_cursor = Cursor::new(source_transactions_csv);

        // Исходный отчёт во внутреннем представлении из текста
        match Report::new_from_csv_reader(&mut source_tx_cursor) {
            Ok(_) => panic!("Error is expected"),
            Err(error) => assert_eq!(error, ParserError::CsvUnknownTxType("NON-EXISTING OPERATION".to_string()))
            // Err(error) => assert_eq!(error, ParserError::CsvLineReadError)
        }

        Ok(())
    }

    #[test]
    fn test_text_report_error() -> Result<(), ParserError> {
        let source_transactions_txt = TEXT_CONTENT_STR_BAD.to_string();
        let mut source_tx_cursor = Cursor::new(source_transactions_txt);

        // Исходный отчёт во внутреннем представлении из текста
        match Report::new_from_text_reader(&mut source_tx_cursor) {
            Ok(_) => panic!("Error is expected"),
            Err(error) => assert_eq!(error, ParserError::TextWrongFieldName("WRONG FIELD NAME".to_string()))
            // Err(error) => assert_eq!(error, ParserError::CsvLineReadError)
        }        
            
        Ok(())
    }

    /// Неверное MAGIC в BIN
    #[test]
    fn test_bin_bad_magic() -> Result<(), ParserError> {
        // Создаём Report из двух транзакций
        let mut source_report = Report::new();
        source_report.add_transaction(SOURCE_TX_1.clone());
        source_report.add_transaction(SOURCE_TX_2.clone());

        // Пишем в буфер, Vec<u8> имплементирующего Write
        let mut buffer = Vec::new();
        source_report.write_as_bin_to_writer(&mut buffer)?;

        // Испоганиваю первый MAGIC (@)
        buffer[0] = 0x40;      

        // Для чтения оборачиваем в курсор
        let mut buf_cursor: Cursor<Vec<u8>> = Cursor::new(buffer);
        assert_eq!(Transaction::new_from_bin_reader(&mut buf_cursor), Err(ParserError::BinWrongMagicEncountered));
        // assert_eq!(Transaction::new_from_bin_reader(&mut buf_cursor), Err(ParserError::BinTxReadError));
    
        Ok(())
    }

    /// Сравнение отчётов по длине
    #[test]
    fn test_report_compare_len() -> Result<(), ParserError> {
        // Создаём Report из двух транзакций
        let mut report_1 = Report::new();
        report_1.add_transaction(SOURCE_TX_1.clone());
        report_1.add_transaction(SOURCE_TX_2.clone());

        // report_2 с "лишней" транзакцией в конце
        let mut report_2 = Report::new();
        report_2.add_transaction(SOURCE_TX_1.clone());
        report_2.add_transaction(SOURCE_TX_2.clone());
        report_2.add_transaction(SOURCE_TX_1.clone());
    
        // Ошибка с указанием длин
        assert_eq!(report_1.compare_full(&report_2), Err(ParserError::ReportLengthsAreNotEqual(2, 3)));
        // assert_eq!(report_1.compare_full(&report_2), Err(ParserError::ReportLengthsAreNotEqual(2, 4)));
    
        // Добавляем к первому отчёту транзакцию, отчёты должны быть равны
        report_1.add_transaction(SOURCE_TX_1.clone());
        assert_eq!(report_1.compare_full(&report_2), Ok(()));
        // assert_eq!(report_1.compare_full(&report_2), Err(ParserError::ReportLengthsAreNotEqual(2, 3)));
            
        Ok(())
    }

    /// Сравнение отчётов по транзакциям
    #[test]
    fn test_report_compare_content() -> Result<(), ParserError> {
        // Создаём Report из двух транзакций
        let mut report_1 = Report::new();
        report_1.add_transaction(SOURCE_TX_1.clone());
        report_1.add_transaction(SOURCE_TX_2.clone());

        // report_2 с "лишней" транзакцией в конце
        let mut report_2 = Report::new();
        report_2.add_transaction(SOURCE_TX_1.clone());
        report_2.add_transaction(SOURCE_TX_2.clone());

        // Одинаковые отчёты
        assert_eq!(report_1.compare_full(&report_2), Ok(()));
    
        // Добавляем разные транзакции в конец, но длины одинаковые
        report_1.add_transaction(SOURCE_TX_1.clone());
        report_2.add_transaction(SOURCE_TX_2.clone());
        // Отличаются последние добавленные транзакции
        assert_eq!(report_1.compare_full(&report_2), Err(ParserError::NonEqualTransactionFound(SOURCE_TX_1.as_str(), SOURCE_TX_2.as_str())));
        // assert_eq!(report_1.compare_full(&report_2), Err(ParserError::NonEqualTransactionFound(SOURCE_TX_1.as_str(), SOURCE_TX_1.as_str())));
    
        Ok(())
    }
}