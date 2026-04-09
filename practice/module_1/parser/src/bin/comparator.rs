///  Сравнение файлов в форматах: csv, txt, bin
///  --input <INPUT>
///  --input-format <INPUT_FORMAT>
///  --output-format <OUTPUT_FORMAT>

use std::fs::File;
use std::process;
use std::io::{BufReader};

use clap::{Parser};

use parser::report::Report;
use parser::csv_format::CsvFormatIO;
use parser::bin_format::BinFormatIO;
use parser::text_format::TextFormatIO;
// use parser::error::ParserError;

#[derive(Parser, Debug)]
#[command(name = "Сравнение файлов в форматах: csv, txt, bin")]
struct Args {
    #[arg(long)]
    file1: String,

    #[arg(long)]
    format1: String,

    #[arg(long)]
    file2: String,

    #[arg(long)]
    format2: String,
}

fn main() {
    let args = Args::parse();
    
    if let Err(e) = run_comparator(&args) {
        eprintln!("Ошибка: {}", e);
        std::process::exit(1);
    }
}

fn run_comparator(args: &Args) -> Result<(), String> {
    // Проверяем форматы
    validate_format(&args.format1, "format1")?;
    validate_format(&args.format2, "format2")?;
    
    // Открываем файл 1 с проверкой
    let mut file1_opened = File::open(&args.file1)
        .map_err(|e| format!("Не удалось открыть файл '{}': {}", args.file1, e))?;
    let mut buf_reader1 = BufReader::new(&mut file1_opened);
    
    // Открываем файл 2 с проверкой
    let mut file2_opened = File::open(&args.file2)
        .map_err(|e| format!("Не удалось открыть файл '{}': {}", args.file1, e))?;
    let mut buf_reader2 = BufReader::new(&mut file2_opened);
    
    // Читаем файлы по очереди
    let report1 = match args.format1.to_lowercase().as_str() {
        "csv" => Report::new_from_csv_reader(&mut buf_reader1)?,
        "txt" => Report::new_from_text_reader(&mut buf_reader1)?,
        "bin" => Report::new_from_bin_reader(&mut file1_opened)?,
        _ => return Err(format!("Неверный формат: {}", args.format1))
        // Поскольку провалидировали, можно так:
        // _ => unreachable!(),
    };

    // 
    let report2 = match args.format2.to_lowercase().as_str() {
        "csv" => Report::new_from_csv_reader(&mut buf_reader2)?,
        "txt" => Report::new_from_text_reader(&mut buf_reader2)?,
        "bin" => Report::new_from_bin_reader(&mut file2_opened)?,
        _ => return Err(format!("Неверный формат: {}", args.format1))
        // Поскольку провалидировали, можно так:
        // _ => unreachable!(),
    };

    if let Err(error) = report1.compare_full(&report2) {
        println!("Отчёты отличаются: {}", error);
        process::exit(1)
    }

    // TODO: Да, нужно сделать сравнение с streaming

    println!("Отчёты равны!");

    Ok(())
}

fn validate_format(format: &str, in_or_out_type: &str) -> Result<(), String> {
    match format.to_lowercase().as_str() {
        // Можно использовать ИЛИ, удобно
        "csv" | "txt" | "bin" => Ok(()),
        _ => Err(format!("Неверный формат '{}' для {}. Поддерживаемые форматы: csv, txt, bin", 
                        format, in_or_out_type)),
    }
}