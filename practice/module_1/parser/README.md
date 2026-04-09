 # Библиотека parser
 
 Библиотека для парсинга/сериализации/десериализации финансовых данных Yandex Practicum в несколько форматов: CSV, текстовый, бинарный

 ## Консольные утилиты
- converter:
```/// Конвертер из/в форматы: csv, txt, bin
/// Пример запуска: 
///
/// cargo run -- --input ./aux/records_example_wrong_line.csv --input-format csv --output-format csv
/// 
/// Вывод:
/// Ошибка: Встречена транзакция с неверным форматом 1000000000000002,WITHDRAWAL,599094029349995112,0,300,1633036980000,SUCCESS,"Record number 3",,,,
///
/// Или вывод отчёта в stdout
```

- comparator =>  
```///  Сравнение файлов в форматах: csv, txt, bin
///  --input <INPUT>
///  --input-format <INPUT_FORMAT>
///  --output-format <OUTPUT_FORMAT>
///
/// Пример запуска:
/// cargo run --bin comparator --  --file1 ./aux/records_example_2.bin --format1 bin --file2 ./aux/records_example_2.bin --format2 bin
```

 ### Структуры данных
 - [`struct Transaction`] - структура внутреннего представления одной транзакции
 - [`struct Report`] - структура внутреннего представления коллекции транзакций, 
                       используемая для перевода из/в указанные форматы
 
 ### Трейты
 Трейты используются для придания характеристики чтения/записи транзакций в формате Yandex Practicum при работе с указанными форматами 
 use parser::csv_format::CsvFormatIO;
 use parser::bin_format::BinFormatIO;
 use parser::text_format::TextFormatIO;

  
 ### Сериализация/десериализация транзакции
 - `Transaction::new_from_csv_reader<R: std::io::Read>(reader: &mut R) -> Result<InternalType, ParserError>` 
 - `Transaction::new_from_text_reader<R: std::io::BufRead>(reader: &mut R) -> Result<InternalType, ParserError>`
 - `Transaction::new_from_bin_reader<R: std::io::BufRead>(reader: &mut R) -> Result<InternalType, ParserError>`
 - `tx.write_as_csv_to_writer<W: std::io::Write>(&self, writer: &mut W) -> Result<(), ParserError>`
 - `tx.write_as_text_to_writer<W: std::io::Write>(&self, writer: &mut W) -> Result<(), ParserError>`
 - `tx.write_as_bin_to_writer<W: std::io::Write>(&self, writer: &mut W) -> Result<(), ParserError>`
 
 ### Возможные ошибки описаны в error.rs
 
 ### Пример использования отчёта
 
 ```rust
 use parser::report::Report;
 use parser::csv_format::CsvFormatIO;
 use parser::bin_format::BinFormatIO;
 use parser::text_format::TextFormatIO;
 use std::fs::File;
 use std::path::Path;
 
 // Чтение из BIN-файла (или другого источника, релизующего трейт Read)
    let file_to_read = Path::new("aux/records_example.bin");
 
    let mut file_to_read = File::open(file_to_read)
        .unwrap_or_else(|e| {
            eprintln!("Не удалось открыть файл: {}", e);
            std::process::exit(1);
        });

    let mut report = Report::new_from_bin_reader(&mut file_to_read)
        .unwrap_or_else(|e| {
            eprintln!("BIN не прочитан: {}", e);
            std::process::exit(1);
        });
 
 // Запись в CSV (трейт Write)
    let csv_file_to_write_path = Path::new("aux/output.csv");

    let mut csv_file_to_write = File::create(csv_file_to_write_path)
        .unwrap_or_else(|e| {
            eprintln!("Не удалось создать файл: {}", e);
            std::process::exit(1);
        });

    match report.write_as_csv_to_writer(&mut csv_file_to_write) {
        Ok(()) => println!("Записано в файл: {:?}", csv_file_to_write_path),
        Err(error) => println!("Ошибка записи в файл {:?}: {}", csv_file_to_write_path, error),
    }

Для подробного описания функций см.модуль report.rs
 ```
 Используемые функции:
 - `Report::new_from_csv_reader<R: std::io::Read>(reader: &mut R) -> Result<InternalType, ParserError>` 
 - `Report::new_from_text_reader<R: std::io::BufRead>(reader: &mut R) -> Result<InternalType, ParserError>`
 - `Report::new_from_bin_reader<R: std::io::BufRead>(reader: &mut R) -> Result<InternalType, ParserError>`
 - `report.write_as_csv_to_writer<W: std::io::Write>(&self, writer: &mut W) -> Result<(), ParserError>`
 - `report.write_as_text_to_writer<W: std::io::Write>(&self, writer: &mut W) -> Result<(), ParserError>`
 - `report.write_as_bin_to_writer<W: std::io::Write>(&self, writer: &mut W) -> Result<(), ParserError>`
 
 