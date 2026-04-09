// Note: этот код уже внесён как ./bin/cli_utils.ru

// Зависимость от библиотеки подключена в Cargo.toml с локальным путём к файлу

// Резолвим имена
use bank_system::storage::{Name};

// Пример: поскольку pub use storage::Storage есть в lib.rs, можно не указывать полный путь
use bank_system::Storage;
use std::env;

// super — означает на один уровень в файловой системе вверх. Аналогично .. в командной строке Linux.
// crate — означает корень проекта, то есть начинает искать с src/. Аналогично / в командной строке Linux.

fn main() {
    // Простой тест
    use_lib_simple();

    /*** Использование с CLI ***/
    
    // Загружаем текущее состояние банка из CSV-файла
    // Здесь демонстрация использования BufRead в методе load_data()
    // Файл читается построчно, и каждая строка преобразуется в (Name, Balance)

    let file_path = String::from("../aux/balance.csv");

    let mut storage = Storage::load_file_to_storage(file_path.as_str());
    
    // Без чтения из файла - создание нового Storage и наполнение:
    // let mut storage = Storage::new();
    // Добавляем пользователей: let users_vec = vec!["John", "Alice", "Bob", "Vasya"];

    // Создаём String in-place на основе &str, &str передаётся по значению
    // Подразумевается user_vec.into_iter() => то есть векто "consumed", дальше использовать нельзя
    // for user_to_add in users_vec {
    //     storage.add_user(user_to_add.to_string());
    // }

    // собираем аргументы
    let args: Vec<String> = env::args().collect();

    // Выводим help в stderr
    if args.len() < 2 {
        eprintln!("Использование:");
        eprintln!("  add <name> <amount>");
        eprintln!("  withdraw <name> <amount>");
        eprintln!("  balance <name>");
        return;
    }

    // Сравниваем как .as_str() (&str)
    match args[1].as_str() {
        
        "deposit" => {
            if args.len() != 4 {
                eprintln!("Пример: add John 200");
                return;
            }

            // Взял ссылку, а не склонировал (как в примере)
            let user= &args[2];
            // Или: let user= &args[2].clone(); => тогда в deposit() передавать &user

            // panic!, если не parse => Err()
            let amount: i64 = args[3].parse().expect("Сумма должна быть числом");

            match storage.deposit(user, amount) {
                Ok(_) => {
                    println!("Пополнено: {} на {}", user, amount);
                    // После изменения баланса сохраняем новое состояние в CSV (полностью o_O)
                    storage.save_storage_to_file(file_path.as_str());
                },
                Err(e) => println!("Ошибка: {}", e),
            }
        }

        "withdraw" => {
            if args.len() != 4 {
                eprintln!("Пример: withdraw John 100");
                return;
            }
            
            // Здесь то же самое, но как в примере
            let user: Name = args[2].clone();
            let amount: i64 = args[3].parse().expect("Сумма должна быть числом");

            match storage.withdraw(&user, amount) {
                Ok(_) => {
                    println!("Снято: {} на {}", user, amount);
                    // После изменения баланса сохраняем новое состояние в CSV (полностью o_O)
                    storage.save_storage_to_file(file_path.as_str());
                },
                Err(e) => println!("Ошибка: {}", e),
            }
        }

        "balance" => {
            if args.len() != 3 {
                eprintln!("Пример: balance John");
                return;
            }

            let user = &args[2];

            match storage.get_balance(user) {
                Some(balance) => println!("Баланс {}: {:?}", user, balance),
                None => println!("Пользователь {} не найден", user),
            }
        }
        // Остальные варианты
        _ => {
            eprintln!("Неизвестная команда");
        }
    }
}

// Простой пример использования библиотеки */
fn use_lib_simple() {
    let mut storage = Storage::new();

    // Создаём пользователя
    let user: Name = "Человек".into();
    // let user: Name = "John".to_string();

    // Копируем пользователя перед созданием, потому что передаём владение
    storage.add_user(user.clone());
    
    // Кладём деньги
    let amount: i64 = 200;
    let _ = storage.deposit(&user, amount);

    // Чтение или по ссылке &user, или по &str.to_string()
    println!("Денег на счету пользователя {} => {:?}", user, storage.get_balance(&user).unwrap());
    println!("{:?}", storage.get_balance(&"Человек".to_string()));
}