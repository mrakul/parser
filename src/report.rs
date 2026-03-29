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
    /// Получение структуры Report из ввода СSV-формата
    /// Аргументы: <R: std::io::Read>(reader: R)
    /// Результат: Result<Report, String>
    /// Использование:
    ///     let mut report = Report::new_from_text_file(&mut file_to_read)
    ///         {...}
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

    /// Перевод структуры Report в СSV-формат
    /// Аргументы: <W: std::io::Write>(&mut self, writer: &mut W)
    /// Результат: Result<(), ParserError>
    /// Использование:
    ///     let mut report = Report::new_from_text_reader(&mut file_to_read)
    ///         {...}
    fn write_as_csv_to_writer<W: std::io::Write>(&self, writer: &mut W) -> Result<(), ParserError> {

        // Бежим по вектору (по мутабельным ссылкам)
        for cur_tx in self.transactions.iter() {
            cur_tx.write_as_csv_to_writer(writer)?;
        }

        Ok(())
    }
}


/// Реализация трейта для парсинга из Bin-формата в Report и обратно
impl BinFormatIO<Report> for Report {
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

    fn write_as_text_to_writer<W: std::io::Write>(&mut self, writer: &mut W) -> Result<(), ParserError> {
        // Бежим по вектору (по ссылке)
        for cur_tx in self.transactions.iter_mut() {
            cur_tx.write_as_text_to_writer(writer)?;
        }

        Ok(())
    }
}

//*** Секция тестов для Report ***/