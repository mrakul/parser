/// Файл для примеров использования - в основном, Report'ы целиком

use parser::report::Report;
use parser::csv_format::CsvFormatIO;
use parser::bin_format::BinFormatIO;
use parser::text_format::TextFormatIO;
use std::fs::File;
use std::path::Path;
use std::io::{BufWriter, Cursor};
use std::io::BufReader;

fn main() {
    // 1. Тест чтения из CSV
    let file_to_read = Path::new("aux/records_example.csv");

    // Открываем файл
    let file_to_read = File::open(file_to_read)
        .unwrap_or_else(|e| {
            eprintln!("Не удалось открыть файл: {}", e);
            std::process::exit(1);
        });

    let mut buf_reader = BufReader::new(file_to_read);
    let report = Report::new_from_csv_reader(&mut buf_reader)
        .unwrap_or_else(|e| {
            eprintln!("Ошибка обработки CSV: {}", e);
            std::process::exit(1);
        });

    // println!("Отчёт: {:?} размер {} ", report, report.get_transactions().len());
    println!("Отчёт: {:?} размер {} ", report.get_transactions()[0], report.get_transactions().len());

    // 2. Запись в CSV
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

    // 3. Тест со строкой
    let csv_content_str = "TX_ID,TX_TYPE,FROM_USER_ID,TO_USER_ID,AMOUNT,TIMESTAMP,STATUS,DESCRIPTION\n1,DEPOSIT,100,200,1000,123456789,SUCCESS,Test transaction\n2,TRANSFER,100,0,500,123456790,FAILURE,Withdrawal";

    // Оборачиваем в курсор для передачи как reader
    let mut cursor = Cursor::new(csv_content_str);

    let report = Report::new_from_csv_reader(&mut cursor)
        .unwrap_or_else(|e| {
            eprintln!("СSV не прочитан: {}", e);
            std::process::exit(1);
        });

        println!("\nЗагружено {} транзакций", report.get_transactions().len());

        match report.write_as_csv_to_writer(&mut csv_file_to_write) {
            Ok(()) => println!("Записано в файл: {:?}", csv_file_to_write_path),
            Err(error) => println!("Ошибка записи в файл {:?}: {}", csv_file_to_write_path, error),
        }

    /*** Секция бинарного формата ***/
    // 1. Тест чтения из BIN
    let file_path_to_read = Path::new("aux/records_example_2.bin");

    // Открываем файл
    let mut file_to_read = File::open(file_path_to_read)
        .unwrap_or_else(|e| {
            eprintln!("Не удалось открыть файл: {}", e);
            std::process::exit(1);
        });

    let report = Report::new_from_bin_reader(&mut file_to_read)
        .unwrap_or_else(|e| {
            eprintln!("Bin не прочитан: {}", e);
            std::process::exit(1);
        });
    
    println!("Отчёт: {:?} размер {} ", report, report.get_transactions().len());

    // 2. Запись в BIN
    let bin_file_to_write_path = Path::new("aux/output.bin");

    let mut bin_file_to_write = File::create(bin_file_to_write_path)
        .unwrap_or_else(|e| {
            eprintln!("Не удалось создать файл: {}", e);
            std::process::exit(1);
        });

    // Оборачиваем для буферизированного вывода
    // let mut bin_buf_writer = BufWriter::new(bin_file_to_write);
    
    match report.write_as_bin_to_writer(&mut bin_file_to_write) {
            Ok(()) => println!("Записано в файл: {:?}", bin_file_to_write_path),
            Err(error) => println!("Ошибка записи в файл {:?}: {}", bin_file_to_write_path, error),
    }

    /*** Секция текстового формата ***/
    let file_path_to_read = Path::new("aux/for_comparator/50_records.txt");
    // let file_path_to_read = Path::new("aux/records_example.txt");


    let file_to_read = File::open(file_path_to_read)
        .unwrap_or_else(|e| {
            eprintln!("Не удалось открыть файл: {}", e);
            std::process::exit(1);
        });

    let mut buf_reader = BufReader::new(file_to_read);

    let mut report = Report::new_from_text_reader(&mut buf_reader)
        .unwrap_or_else(|e| {
            eprintln!("Текстовый файл не прочитан: {}", e);
            std::process::exit(1);
        });
    
    println!("Отчёт: {:?} размер {} ", report, report.get_transactions().len());

    // 2. Запись в текстовом формате
    let txt_file_to_write_path = Path::new("aux/output.txt");

    let txt_file_to_write = File::create(txt_file_to_write_path)
        .unwrap_or_else(|e| {
            eprintln!("Не удалось создать файл: {}", e);
            std::process::exit(1);
        });

    // Оборачиваем для буферизированного вывода
    let mut txt_buf_writer = BufWriter::new(txt_file_to_write);
    
    match report.write_as_text_to_writer(&mut txt_buf_writer) {
        Ok(()) => println!("Записано в файл: {:?}", txt_file_to_write_path),
        Err(error) => println!("Ошибка записи в файл {:?}: {}", txt_file_to_write_path, error),
    }
}