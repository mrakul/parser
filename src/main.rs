use parser::report::Report;
use parser::csv_format::CsvFormatIO;
use std::fs::File;

fn main() {
    let file_path = String::from("/home/m_rakul/Code/RustYandex/bank-system/aux/balance.csv");

    // Открываем файл
    let mut file = File::open(file_path)
        .unwrap_or_else(|e| {
            eprintln!("Не удалось открыть файл: {}", e);
            std::process::exit(1);
        });

    let report = Report::new_from_csv_file(&mut file)
        .unwrap_or_else(|e| {
            eprintln!("СSV не прочитан: {}", e);
            std::process::exit(1);
        });

    println!("Отчёт: {:?} ", report);

    // let report = Report::new_from_csv_file();
}