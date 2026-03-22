use parser::report::Report;
use parser::csv_format::CsvFormatIO;
use std::fs::File;
use std::path::Path;
use std::io::Cursor;

fn main() {
    // 1. Тест чтения из CSV
    let file_to_read = Path::new("aux/records_example.csv");

    // Открываем файл
    let mut file_to_read = File::open(file_to_read)
        .unwrap_or_else(|e| {
            eprintln!("Не удалось открыть файл: {}", e);
            std::process::exit(1);
        });

    let mut report = Report::new_from_csv_file(&mut file_to_read)
        .unwrap_or_else(|e| {
            eprintln!("СSV не прочитан: {}", e);
            std::process::exit(1);
        });

    // println!("Отчёт: {:?} размер {} ", report, report.get_transactions().len());
    println!("Отчёт: {:?} размер {} ", report.get_transactions()[0], report.get_transactions().len());

    // 2. Запись в CSV
    let file_to_write_path = Path::new("aux/output.csv");

    let mut file_to_write = File::create(file_to_write_path)
        .unwrap_or_else(|e| {
            eprintln!("Не удалось создать файл: {}", e);
            std::process::exit(1);
        });

    match report.write_to_csv_file(&mut file_to_write) {
        Ok(()) => println!("Записано в файл: {:?}", file_to_write_path),
        Err(error) => println!("Ошибка записи в файл {:?}: {}", file_to_write_path, error),
    }


    // 3. Тест со строкой
    let csv_content_str = "TX_ID,TX_TYPE,FROM_USER_ID,TO_USER_ID,AMOUNT,TIMESTAMP,STATUS,DESCRIPTION
                                 1,DEPOSIT,100,200,1000,123456789,SUCCESS,Test transaction
                                 2,TRANSFER,100,0,500,123456790,FAILURE,Withdrawal";

    // Wrap string in Cursor to make it behave like a file
    let mut cursor = Cursor::new(csv_content_str);

    let mut report = Report::new_from_csv_file(&mut cursor)
        .unwrap_or_else(|e| {
            eprintln!("СSV не прочитан: {}", e);
            std::process::exit(1);
        });

    println!("Загужено {} транзакций", report.get_transactions().len());

        match report.write_to_csv_file(&mut file_to_write) {
        Ok(()) => println!("Записано в файл: {:?}", file_to_write_path),
        Err(error) => println!("Ошибка записи в файл {:?}: {}", file_to_write_path, error),
    }

}