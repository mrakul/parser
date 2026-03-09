use std::collections::HashMap;
use std::path::Path;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::fs;

// Задаём тип (аналог using C++)
pub type Name = String;
pub type Balance = i64;
// Сейчас сделали struct Balance(i64) с методами

// Обернём баланс в новый тип, чтобы можно было реализовывать метод
// (не для целого числа ведь метод добавлять, верно? :) )
// и запретим балансу опускаться ниже нуля
// pubstruct Balance(i64);

// Хранилище
pub struct Storage {
   accounts: HashMap<Name, Balance>
}


#[derive(Debug, Clone)]
pub enum Operation {
    Deposit(u32),
    Withdraw(u32),
    CloseAccount(String)
} 


impl Storage {
    // Конструктор, возвращем структуру Storage (by value, Move-семантика)
    pub fn new() -> Self {
        Self {
            accounts: HashMap::new(),
        }
    }

    // Добавить пользователя: важно, что передаём user'а с передачей владения для .insert()
    pub fn add_user(&mut self, user_to_add: Name) -> Option<Balance> {
        
        // Важно: передаём по ссылке
        if self.accounts.contains_key(&user_to_add) {
            None
        } else {
            // TODO: Второй проход (?). Использовать .entry() для одного прохода 
            // (интересно, cargo clippy тоже об этом сказал)

            // (!) Добавляем, передавая владение
            self.accounts.insert(user_to_add, 0);
            Some(0)
        }
    }

    // Удалить пользователя: Option<i64> => "забираем" данные (Copy для примитивов)
    pub fn remove_user(&mut self, user_to_remove: &Name) -> Option<Balance> {
        self.accounts.remove(user_to_remove)
    }

    // Получить баланс пользователя: Option<&i64> => читаем по immutable-ссылке
    pub fn get_balance(&self, user_to_read: &Name) -> Option<&Balance> {
        self.accounts.get(user_to_read)
        
        // Можно так: тогда в Option<i64> и в тестах без ссылок, примитивы копируются.
        //            Но get() возвращает immutable-ссылку, поэтому копируем
        // self.accounts.get(user_to_read).copied()
    }

    // Положить деньги, здесь можем передать user как ссылку
    pub fn deposit(&mut self, user: &Name, money_to_add: i64) -> Result<(), &str> {
        // Деструктуризация в Some<&mut i64>, возвращаем как &str - указатель на литерал (можно сделать String как Result)
        if let Some(balance) = self.accounts.get_mut(user.as_str()) {
            // Добавили с разымёныванием
            *balance += money_to_add;
            Ok(())
        }
        else {
            Err("Пользователь не найден")
        }
    }

    pub fn withdraw(&mut self, user: &Name, money_to_withdraw: Balance) -> Result<(), &str> {
        // Деструктуризация в Some<&mut i64>
        if let Some(balance) = self.accounts.get_mut(user) {
            if *balance - money_to_withdraw < 0 {
                Err("Недостаточно средств")
            }
            else {
                // Изымаем
                *balance -= money_to_withdraw;
                Ok(())                
            }
        }
        // None
        else {
            Err("Пользователь не найден")
        }
    }    


    pub fn get_all(&self) -> Vec<(Name, i64)> {
        // Получить все значения в виде вектора: String клонируется, balance читается по ссылке
        // (и автоматом копируется). И собирается .collect()'ом в вектор при возврате
        self.accounts.iter().map(|(name, balance)| (name.clone(), *balance)).collect()
    }

    /// Загружает данные из CSV-файла или создаёт хранилище с дефолтными пользователями
    // (!) Важно, что функция не принимает &self
    pub fn load_file_to_storage(file_path: &str) -> Storage {
        let mut storage = Storage::new();

        // Проверяем, существует ли файл
        if Path::new(file_path).exists() {
            // Открываем файл
            let file = File::open(file_path).unwrap();

            // Оборачиваем файл в BufReader
            // BufReader читает данные блоками и хранит их в буфере,
            // поэтому построчное чтение (lines()) работает быстрее, чем читать по байту
            let reader = BufReader::new(file);

            // Читаем файл построчно
            for cur_line in reader.lines() {
                // Каждая строка — это Result<String>, поэтому делаем if let Ok
                if let Ok(ok_line) = cur_line {
                    // Разделяем строку по запятой: "Name,Balance"
                    let columns: Vec<&str> = ok_line.trim().split(',').collect();

                    // Если два столбца
                    if columns.len() == 2 {
                        let name = columns[0].to_string();
                        // Пробуем преобразовать баланс из строки в число
                        let balance: i64 = columns[1].parse().unwrap_or(0);

                        // Добавляем пользователя и выставляем баланс
                        storage.add_user(name.clone());
                        let _ = storage.deposit(&name, balance);
                    }
                }
            }
        } else {
            // если файла нет, создаём пользователей с нуля
            for user in ["John", "Alice", "Bob", "Vasya"] {
                storage.add_user(user.to_string());
            }
        }

        storage
    }

    /// Сохраняет текущее состояние Storage в CSV-файл
    pub fn save_storage_to_file(&self, file_path: &str) {
        let mut data = String::new();

        // Собираем все данные в одну строку формата "Name,Balance"
        // Бежим по вектору
        for (name, balance) in self.get_all() {
            // Разделяем newline'ом записи, всё по классике
            data.push_str(&format!("{},{}\n", name, balance));
        }

        // Записываем в файл
        // Здесь мы не используем BufWriter, потому что сразу пишем всю строку целиком.
        
        // Создаём родительские директории
        if let Some(parent) = Path::new(file_path).parent() {
            fs::create_dir_all(parent).unwrap();
        }

        fs::write(file_path, data).expect("Не удалось записать файл");
    }

    // pub pub fn apply_operations(&mut self, user: &Name, operations: &Vec<Operation>) -> Vec<Operation> {
        
    //     // Пустой вектор
    //     pub let mut bad_operations: Vec<Operation> = vec![];

    //     for op in operations {
    //         match op {
    //             pub Operation::Deposit(value) => {
    //                 if let Err(errorMsg) = self.deposit(user, *value as i64) {
    //                     println!("{}", errorMsg);
    //                     bad_operations.push(op.clone());
    //                 }
    //             },
    //             pub Operation::Withdraw(value) => {
    //                 if let Err(errorMsg) = self.withdraw(user, *value as i64) {
    //                     println!("{}", errorMsg);
    //                     bad_operations.push(op.clone());
    //                 }
    //             },
    //             pub Operation::CloseAccount(user) => {
    //                 if let None = self.remove_user(user) {
    //                     println!("Пользователь не найден {}", user);
    //                     bad_operations.push(op.clone());
    //                 }
    //             }
    //         }
    //     }

        // bad_operations
    // }

}

/*** Секция тестов ***/

// Child-модуль
#[cfg(test)]
mod tests {
    // Подключаем всё из родительского модуля (использование методов/полей)
    use super::*; 

    #[test]
    fn test_new_storage_is_empty() {
        let bank = Storage::new();
        // При создании нет пользователей
        assert_eq!(bank.accounts.len(), 0);
    }

    #[test]
    fn test_add_user() {
        let mut storage = Storage::new();

        // Проверка уже существующего пользователя
        assert_eq!(storage.add_user("Alice".to_string()), Some(0)); // новый пользователь
        assert_eq!(storage.add_user("Alice".to_string()), None);    // уже существует
    }

    #[test]
    fn test_remove_user() {
        let mut storage = Storage::new();

        storage.add_user("Bob".to_string());
        storage.deposit(&"Bob".to_string(), 100).unwrap();

        // Проверка баланса до и после удаления пользователя
        assert_eq!(storage.remove_user(&"Bob".to_string()), Some(100));
        assert_eq!(storage.remove_user(&"Bob".to_string()), None); 
    }

    #[test]
    fn test_deposit_and_withdraw() {
        let mut storage = Storage::new();
        storage.add_user("Charlie".to_string());

        // Пополнение
        assert!(storage.deposit(&"Charlie".to_string(), 200).is_ok());
        assert_eq!(storage.get_balance(&"Charlie".to_string()), Some(&200));

        // Успешное снятие
        assert!(storage.withdraw(&"Charlie".to_string(), 150).is_ok());
        assert_eq!(storage.get_balance(&"Charlie".to_string()), Some(&50));

        // Ошибка: недостаточно средств
        assert!(storage.withdraw(&"Charlie".to_string(), 100).is_err());
        // Но 50 ещё имеется
        assert_eq!(storage.get_balance(&"Charlie".to_string()), Some(&50));
    }

    #[test]
    fn test_nonexistent_user() {
        let mut storage = Storage::new();

        // Депозит несуществующему пользователю
        assert!(storage.deposit(&"Dana".to_string(), 100).is_err());

        // Снятие у несуществующего пользователя
        assert!(storage.withdraw(&"Dana".to_string(), 50).is_err());

        // Баланс у несуществующего пользователя
        assert_eq!(storage.get_balance(&"Dana".to_string()), None);
    }

    use std::fs::File;
    use std::io::Write;


    #[test]
    fn test_load_data_existing_file() {
        let file_path = String::from("unit_test_file.csv");

        // Важно сделать mut file
        // Не нужно делать open после create (?)
        if let Ok(mut file) = File::create(&file_path) {
            writeln!(file, "John,100").unwrap();
            writeln!(file, "Alice,200").unwrap();
            writeln!(file, "Bob,50").unwrap();
            // Здесь закроется, отрытый на чтение (?)
            // .unwrap() нужен, судя по всему, только чтобы убрать unused warning'и
        }
        else {
            panic!("Файл не создан!");
        }

        let storage = Storage::load_file_to_storage(&file_path.as_str());     

        assert_eq!(storage.get_balance(&"John".to_string()), Some(&100));
        assert_eq!(storage.get_balance(&"Alice".to_string()), Some(&200));
        assert_eq!(storage.get_balance(&"Bob".to_string()), Some(&50));
        // Пользователь Vasya не добавлен в файле, поэтому None
        assert_eq!(storage.get_balance(&"Vasya".to_string()), None);

        // Удаляем тестовый файл
        fs::remove_file(file_path).unwrap();
    }

    #[test]
    fn test_save_creates_file_with_correct_data() {
       let file_path = "test_save.csv";

        // Создаём Storage и добавляем пользователей
        let mut storage = Storage::new();
        storage.add_user("John".to_string());
        storage.add_user("Alice".to_string());
        storage.deposit(&"John".to_string(), 150).unwrap();
        storage.deposit(&"Alice".to_string(), 300).unwrap();

        // Сохраняем в файл
        storage.save_storage_to_file(file_path);

        // Читаем файл обратно и проверяем содержимое

        // Это читает полнстью в String, можно как сделано: let reader = BufReader::new(file);

        // Фактически, открывает open() и читает read() одновременно
        let contents = fs::read_to_string(file_path).unwrap();

        let mut lines: Vec<&str> = contents.lines().collect();
        // Сортируем, так как get_all() может возвращать в любом порядке, для проверки в тесте
        lines.sort(); 

        assert_eq!(lines, vec!["Alice,300", "John,150"]);

        // Удаляем тестовый файл
        fs::remove_file(file_path).unwrap();
    }

        #[test]
    fn test_bufwriter() {
        use std::fs::File;
        use std::io::{BufWriter, Write};

        let file_path = "./aux/data.csv";

        let f = File::create(file_path).unwrap();
        let mut writer = BufWriter::new(f);

        writeln!(writer, "John,100").unwrap();   // пока в буфере
        writeln!(writer, "Alice,200").unwrap();  // пока в буфере

        let mut contents = fs::read_to_string(file_path).unwrap();
        
        assert_eq!(contents.len(), 0);

        // Записываем (flush the buffer), читаем ещё раз и проверяем длину (грубовато)
        writer.flush().unwrap(); 
        contents = fs::read_to_string(file_path).unwrap();

        assert_eq!(contents.len(), 19);

        // Удаляем файл
        fs::remove_file(file_path).unwrap();
    }

    use std::io::{Cursor, BufWriter};

    #[test]
    fn test_load_data_existing_cursor() {
        
        // (!) Создаём данные в памяти, как будто это CSV-файл
        let data = b"John,100\nAlice,200\nBob,50\n";
        let mut cursor = Cursor::new(&data[..]);

        // Читаем данные из Cursor
        let mut storage = Storage::new();
        let reader = BufReader::new(&mut cursor);

        // То же самое, но читаем из буфера в памяти с помощью Cursor
        for line in reader.lines() {
            let line = line.unwrap();
            let columns: Vec<&str> = line.trim().split(',').collect();
    
            if columns.len() == 2 {
                let name = columns[0].to_string();
                let balance: i64 = columns[1].parse().unwrap_or(0);
                storage.add_user(name.clone());
                storage.deposit(&name, balance).unwrap();
            }
        }

        assert_eq!(storage.get_balance(&"John".to_string()), Some(&100));
        assert_eq!(storage.get_balance(&"Alice".to_string()), Some(&200));
        assert_eq!(storage.get_balance(&"Bob".to_string()), Some(&50));
        assert_eq!(storage.get_balance(&"Vasya".to_string()), None);
    }

    #[test]
    fn test_save_writes_to_cursor_correctly() {
        // Создаём Storage и добавляем пользователей
        let mut storage = Storage::new();
        storage.add_user("John".to_string());
        storage.add_user("Alice".to_string());
        storage.deposit(&"John".to_string(), 150).unwrap();
        storage.deposit(&"Alice".to_string(), 300).unwrap();

        // Сохраняем в память через BufWriter

        // (!) и при чтении, и при записи используется Vec<u8>
        let buffer = Vec::new();

        // Буфер оборачивается в Cursor
        let mut cursor = Cursor::new(buffer);
        {
            // Cursor оборачивается в BufWriter
            let mut writer = BufWriter::new(&mut cursor);
            // Получение всех записей в векторе запись в буфер как в файл
            for (name, balance) in storage.get_all() {
                writeln!(writer, "{},{}", name, balance).unwrap();
            }
            
            writer.flush().unwrap();
        }

        // Читаем обратно из памяти - обязательно нужно сбросить позицию в 0
        cursor.set_position(0);

        let mut lines: Vec<String> = BufReader::new(cursor).lines().map(|l| l.unwrap()).collect();
        lines.sort(); // сортируем для сравнения

        assert_eq!(lines, vec!["Alice,300", "John,150"]);
    }

}


// impl Balance {
//     // Можно пойти ещё дальше и в качестве аргумента принимать любой тип,
//     // который может итерироваться по OpKind, с помощью дженерика:
//     // fn process<'a>(&mut self, impl IntoIterator<Item=&'a OpKind>) -> Vec<&'a OpKind>
//     // Пробуйте, дерзайте!
//     fn process<'a>(&mut self, ops: &[&'a OpKind]) -> Vec<&'a OpKind> {
//         let mut remaining = ops.into_iter();
//         let mut bad_ops = Vec::new();
//         for op in &mut remaining {
//             match op {
//                 OpKind::Deposit(value) => {
//                     self.0 += *value as u64;
//                 },
//                 OpKind::Withdraw(value) if self.0 > *value as u64 => {
//                     self.0 -= *value as u64;
//                 },
//                 other @ _ => {
//                     bad_ops.push(*other);
//                     break;
//                 },
//             }
//         }
//         bad_ops.extend(remaining);
//         bad_ops
//     }
// }