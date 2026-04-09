use std::io::{self, BufRead};
use std::io::{Write};

// Stdin — стандартный ввод
// Позволяет читать данные, которые вводит пользователь с клавиатуры. 
// Имеет два основных метода:
//  -vstdin().read_line(&mut buf) — читает строку с конца строки (\n) и записывает в переменную buf.
//  - lock() — блокирует ввод для эффективного чтения построчно через буфер (BufRead).

fn main() {
    let stdin = io::stdin();
    let mut input = String::new();

    println!("Введите ваше имя: ");

    // Читаем ввод в String -> Result (сколько прочитано?)
    stdin.read_line(&mut input).unwrap();

    // убираем лишние пробелы и переносы 
    let name = input.trim(); 

    println!("Привет, {}!", name);

    // Stdout — стандартный вывод
    // Используется для вывода информации пользователю через консоль. Основные методы:
    //  - println! — вывод с переводом строки (\n).
    //  - print! — вывод без перевода строки.
    //  - stdout().flush() — принудительно сбрасывает буфер на экран, полезно при интерактивных программах.

    print!("Введите число: ");
    io::stdout().flush().unwrap();          // показываем приглашение сразу

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let number: i32 = input.trim().parse().unwrap();

    println!("Вы ввели число {}", number);
} 