/// Конвертер из/в форматы: csv, txt, bin
/// Пример вызова: 
///
/// cargo run -- --input ./aux/records_example_wrong_line.csv --input-format csv --output-format csv
/// 
/// Вывод:
/// Ошибка: Встречена транзакция с неверным форматом 1000000000000002,WITHDRAWAL,599094029349995112,0,300,1633036980000,SUCCESS,"Record number 3",,,,
///
/// Или вывод отчёта в stdout

use std::fs::File;
use std::io::BufReader;
use std::{io::{self, Write}};

use clap::{Parser};

use parser::report::Report;
use parser::csv_format::CsvFormatIO;
use parser::bin_format::BinFormatIO;
use parser::text_format::TextFormatIO;
// use parser::error::ParserError;

#[derive(Parser, Debug)]
#[command(name = "Конвертер из/в форматы: csv, txt, bin")]
struct Args {
    // -i или --input
    #[arg(short, long)]
    input: String,

    #[arg(long)]
    input_format: String,

    // #[arg(long, default_value = "csv")]  => можно задать дефолтное значение
    #[arg(long)]
    output_format: String,
}

fn main() {
    // Парсим строку
    let args = Args::parse();
    
    if let Err(e) = run_converter(&args) {
        eprintln!("Ошибка: {}", e);
        std::process::exit(1);
    }
}

fn run_converter(args: &Args) -> Result<(), String> {
    // Проверяем форматы
    validate_format(&args.input_format, "input")?;
    validate_format(&args.output_format, "output")?;
    
    // Открываем файл с проверкой
    let mut input_file = File::open(&args.input)
        .map_err(|e| format!("Не удалось открыть файл '{}': {}", args.input, e))?;
    let mut buf_reader = BufReader::new(&mut input_file);

    // Читаем отчёт
    let mut report = match args.input_format.to_lowercase().as_str() {
        "csv" => Report::new_from_csv_reader(&mut buf_reader)?,
        "txt" => Report::new_from_text_reader(&mut buf_reader)?,
        "bin" => Report::new_from_bin_reader(&mut input_file)?,
        _ => return Err(format!("Неверный формат: {}", args.input_format))
        // Поскольку провалидировали, можно так:
        // _ => unreachable!(),
    };

    // Пишет в stdout, передавая в необходимый формат
    let mut stdout = io::stdout();
    
    match args.output_format.as_str() {
        "csv" => report.write_as_csv_to_writer(&mut stdout)?,
        "txt" => report.write_as_text_to_writer(&mut stdout)?,
        "bin" => report.write_as_bin_to_writer(&mut stdout)?,
        _ => return Err(format!("Неверный формат: {}", args.input_format))
        // Поскольку провалидировали, можно так:
        // _ => unreachable!(),
    }
    
    // "Спускаем" буфер
    stdout.flush()
        .map_err(|e| format!("Не удалось вывести в stdout: {}", e))?;
    
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

fn _print_format() {
    println!("Формат запуска утилиты перевода из одного формата в другой: 
            ypbank_converter \
            --input <input_file> \
            --input-format <format> \
            --output-format <format> \
            > output_file.txt ")
}