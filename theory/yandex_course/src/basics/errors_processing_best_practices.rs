//Императивный стиль 
fn parse_and_double(s: &str) -> Result<i32, std::num::ParseIntError> {
    // Пытаемся распарсить строку в число
    let n = match s.parse::<i32>() {
        Ok(num) => num,
        Err(e) => return Err(e), // явно выходим при ошибке
    };

    // Умножаем результат на 2 и возвращаем
    Ok(n * 2)
}

// Функциональный стиль
// 1. map
pub fn parse_and_double_map(s: &str) -> Result<i32, std::num::ParseIntError> {
    // Берём только успешный результат
    s.parse::<i32>().map(|n| n * 2) 
}

// 2. map_err. То же самое, но применяется к ошибке. Удобно для преобразования типа ошибки
pub fn parse_number_map_err(s: &str) -> Result<i32, String> { 
    s.parse::<i32>().map_err(|e| format!("Ошибка парсинга: {}", e)) 
}

// 3. map_err => and_then
pub fn square_root(num: f64) -> Result<f64, String> {
    if num >= 0.0 {
        Ok(num.sqrt())
    } else {
        Err("Отрицательное число".into())
    }
}

pub fn parse_and_sqrt_map_err_and_then(s: &str) -> Result<f64, String> {
    s.parse::<f64>()
        .map_err(|_| "Не удалось распарсить".into())
        .and_then(square_root)
    // Если parse успешный → вызываем square_root.
    // Если ошибка → она передаётся дальше.
} 

// 4. or_else. Аналог and_then, но для ошибки.
// Позволяет заменить ошибку на другой результат.

// Читаем значение из файла, и если неуспешно (получаем ошибку), то 
// читаем значение из переменной окружения 
// pub fn get_config_value(key: &str) -> Result<String, String> {
//     read_from_file(key).or_else(|_| read_from_env(key))
// }

// 5. unwrap_or, unwrap_or_else, unwrap_or_default.
// Достаём значение или берём значение по умолчанию, если не получилось взять значение.

// Пытаемся взять переменную окружения HOSTTYPE, в случае неудачи ставим по умолчанию 8080
pub fn get_env_var_unwrap_or(env_var: &str) -> i32 {
// Пытаемся взять переменную окружения PORT, в случае неудачи ставим по умолчанию 8080
    let port: i32 = std::env::var(env_var)
        .ok()
        .and_then(|s| s.parse::<i32>().ok())
        .unwrap_or(8080);

    port
}

// Method          Processes Ok?       Explanation
// map             ✅ Yes              Applies function to Ok value, returns Result<U, E>
// map_err         ❌ No               Applies function to Err value (processes errors)
// and_then        ✅ Yes              Applies function to Ok value, function returns Result (chaining)
// or_else         ❌ No               Applies function to Err value (error recovery)
// unwrap_or       ❌ No               Extracts value or provides default (not transformation)

/*** Цепочки значений ***/

// Вспомогательная функция для возведения числа в корень.
// (Такая же определена выше)
// fn square_root(x: f64) -> Result<f64, String> {
//     if x >= 0.0 {
//         Ok(x.sqrt())
//     } else {
//         Err("Отрицательное число".into())
//     }
// }

pub fn parse_str_get_root_and_x10(s: &str) -> Result<f64, String> {
    s.parse::<f64>()                    // Парсим строку в число
        // .map_err(|_| "Парсинг не удался".into()) // Преобразуем ошибку
        // Так оставляем ошибку
        .map_err(|e| format!("Parse error: {}", e))  // ← Unify to String early
        .and_then(square_root)                   // Берём корень
        .map(|r| r * 10.0)                                     // Умножаем результат на 10
}

// .map_err(|_| "Парсинг не удался".into())
// //         ↑
// //         └── Discards the original error value
// Explanation:
// map_err receives the original error (ParseFloatError) as its parameter
// The _ pattern explicitly discards this value (we don't need the specific parsing error details)
// We replace it with a fixed user-friendly message regardless of the original error type

// Пример 2, с итератором:
pub fn process_vec(data: Vec<&str>) -> Result<Vec<f64>, String> {
    data.into_iter()
        .map(|s| {
            s.parse::<f64>()   // Применяет парсер к каждому элементу вектора 
                .map_err(|_| format!("Не удалось распарсить '{}'", s)) // Преобразуем ошибку
                .and_then(square_root) // Корень каждого элемента вектора
                .map(|r| r * 10.0) // Умножаем каждый элемент на 10
        })
        .collect() // Собираем финальный вектор после всех преобразований
}

// Практическое задание #1
pub fn get_port_config(env_var: Option<String>) -> u16 {
    // 1. Если env_var есть, попробуйте распарсить в u16
    // 2. Если парсинг неудачен или значения нет, верните 8080
    // Используйте функциональный стиль!
    env_var
        .and_then(|s| s.parse().ok())
        .unwrap_or(8080)
}

// Step	Expression	        Input Type	        Output Type	                What Happens
// 0   	env_var	            —	                Option<String>	            Initial value
// 1	.and_then(...)      Option<String>	    Option<u16>	                None → returns None • Some(s) → runs closure
// 1a	s.parse()	        String	            Result<u16, ParseIntError>	**HERE IS THE RESULT** (temporary!)
// 1b	.ok()	            Result<u16, E>	    Option<u16>	                Converts: Ok(v) → Some(v) Err(_) → None
// 2	.unwrap_or(8080)    Option<u16>	        u16	                        Extracts value or uses default


/*** Безопасные методы обработки ошибок ***/
// Безопасные и небезопасные методы работы с Option и Result
// В Rust всё построено на том, чтобы программист явно принимал решения, что делать с отсутствием значения или ошибкой. Для этого есть два пути:

// 1. Безопасные методы, когда мы обрабатываем все варианты:

// - match — полный разбор всех вариантов.
// - if let / while let — удобные сокращения, если нас интересует только один вариант.
// - Методы вроде .map(), .and_then(), .unwrap_or(), .unwrap_or_else(), .ok_or() — позволяют аккуратно преобразовывать и извлекать значения.
// Эти методы не приводят к аварийному завершению программы, так как требуют продумать поведение программы. 
// Если этого не сделать, компилятор просто не соберёт код.

// 2. Небезопасные методы:
//     .unwrap() достаёт значение, но, если его нет (None или Err), программа вызовет аварийное завершение программы (panic).
//     .expect("Сообщение") делает то же самое, но позволяет вывести понятное сообщение, если значение отсутствует.


/*** panic! ***/

use std::fs;

pub fn load_game_config(path: &str) -> Result<String, String> {
    let content = fs::read_to_string(path)
        .map_err(|_| format!("Не удалось прочитать конфиг: {}", path))?;
    Ok(content)
}

// Также panic! означает баг в логике или критическую проблему — например, выход за границы массива:
fn get_item(vec: &Vec<i32>, index: usize) -> i32 {
    vec[index] // если index >= vec.len(), вызовется panic!
} 

// Макрос panic! можно вызвать самостоятельно вручную:
fn only_positive(x: i32) {
    if x < 0 {
        panic!("Ожидалось положительное число, а пришло {}", x); 
        // Программа завершит выполнение и напечатает в консоль причину 
    }
}

// Можно руководствоваться следующими негласными правилами:
//  - В коде библиотек и в продакшен-приложениях — использовать безопасные методы.
//  - В прототипах, быстрых скриптах, тестах — можно unwrap и expect.
//  - panic! — для ошибок, из которых программа точно не должна восстанавливаться: 
//    «невозможное состояние», недоступность ресурсов, настроек и т. п.


/*** Трейт Error: создание собственных типов для ошибок ***/

// В Rust все типы ошибок должны реализовывать стандартный трейт std::error::Error.
// Это создаёт единообразный интерфейс для работы с любыми ошибками в экосистеме Rust:

// pub trait Error: Debug + Display {
//     // Provided methods
//     fn source(&self) -> Option<&(dyn Error + 'static)> { ... }
    
//     fn description(&self) -> &str { ... }   // deprecated 
//     fn cause(&self) -> Option<&dyn Error> { ... } // deprecated 
//     fn provide<'a>(&'a self, request: &mut Request<'a>) { ... } //nightly experimental 
// }

// Плохой подход:
fn parse_config_bad() -> Result<Config, String> {
    // Теряется информация о типе ошибки
    Err("Не удалось найти файл".to_string())
}

fn use_parse_config_bad () {
    // (!) Cложно обрабатывать разные типы ошибок, парсинг строки — плохой подход. 
    match parse_config() {
        Err(msg) => {
            // Как понять, что именно пошло не так?
            // if msg.contains("файл") {
            //     // Проблема с файлом
            // } else if msg.contains("формат") {
            //     // Проблема с форматом
            // }
        }
        Ok(config) => { /* ... */ }
    }
}

// Хороший подход (Код упрощён)

// Объявляем перечисление для возможных ошибок
#[derive(Debug)]
enum ConfigError {
    FileNotFound(String),
    ParseError(String),
    InvalidFormat,
}

fn parse_config() -> Result<Config, ConfigError> {
    // Явно указываем тип ошибки
    Err(ConfigError::FileNotFound("app.conf".to_string()))
}


// Обработка
fn use_parse_config () {
    match parse_config() {
        Err(ConfigError::FileNotFound(filename)) => {
            println!("Файл {} не найден", filename);
        }
        Err(ConfigError::ParseError(details)) => {
            println!("Ошибка парсинга: {}", details);
        }
        Err(ConfigError::InvalidFormat) => {
            println!("Неверный формат файла");
        }
        Ok(config) => { /* используем конфиг */ }
    }
}

// (!!!) Реализация собственного типа ошибок

// 1. Создайте собственное перечисление с возможными типами ошибок:

#[derive(Debug)]
pub enum MyConfigError {
    FileRead(std::io::Error),
    Parse(std::num::ParseIntError),
    InvalidValue { field: String, value: String, expected: String },
    MissingField(String),
    Custom(String),
} 

// 2. Реализуйте трейт Display для красивого вывода:

use std::fmt;

impl fmt::Display for MyConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MyConfigError::FileRead(err) => {
                write!(f, "Не удалось прочитать файл конфигурации: {}", err)
            }
            MyConfigError::Parse(err) => {
                write!(f, "Ошибка парсинга числа в конфиге: {}", err)
            }
            MyConfigError::Custom(err) => {
                write!(f, "Ошибка: {}", err)
            }
            MyConfigError::MissingField(field) => {
                write!(f, "Отсутствует обязательное поле: {}", field)
            }
            MyConfigError::InvalidValue { field, value, expected } => {
                write!(f, "Неверное значение '{}' для поля '{}', ожидается: {}", 
                       value, field, expected)
            }
        }
    }
} 

// 3. Реализуйте трейт Error:
use std::error::Error;

impl Error for MyConfigError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            MyConfigError::FileRead(err) => Some(err),
            MyConfigError::Parse(err) => Some(err),
            MyConfigError::Custom(_) => None,
            MyConfigError::MissingField(_) => None,
            MyConfigError::InvalidValue { .. } => None,
        }
    }
} 

// Вот как это можно использовать на примере загрузки конфигурационного файла:

// Структура конфигурации
#[derive(Debug)]
pub struct Config {
    port: u16,
    host: String,
    debug: bool,
}

// Функция загрузки конфига с нашей ошибкой MyConfigError
pub fn load_config(filename: &str) -> Result<Config, MyConfigError> {
    // Читаем файл
    let content = fs::read_to_string(filename)
        .map_err(MyConfigError::FileRead)?;
    
    let mut port = None;
    let mut host = None;
    let mut debug = None;
    
    // Простой парсер key=value
    for line in content.lines() {
        let line = line.trim();
        // Игнорируем пустые линии и комментарии
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        
        let parts: Vec<&str> = line.split('=').collect();
        if parts.len() != 2 {
            continue;
        }
        
        let key = parts[0].trim();
        let value = parts[1].trim();
        
        match key {
            "port" => {
                port = Some(value.parse::<u16>()
                    .map_err(MyConfigError::Parse)?);
            }
            "host" => {
                host = Some(value.to_string());
            }
            "debug" => {
                debug = Some(match value {
                    "true" => true,
                    "false" => false,
                    _ => return Err(MyConfigError::InvalidValue {
                        field: "debug".to_string(),
                        value: value.to_string(),
                        expected: "true или false".to_string(),
                    }),
                });
            }
            _ => {} // Игнорируем неизвестные поля
        }
    }
    
    // Проверяем обязательные поля
    let port = port.ok_or_else(|| MyConfigError::MissingField("port".to_string()))?;
    let host = host.ok_or_else(|| MyConfigError::MissingField("host".to_string()))?;
    
    // Проверяем необязательные поля
    let debug = debug.unwrap_or(false);  // если None(отсутствует), то значение debug = false 
    
    Ok(Config { port, host, debug })
}


// Практическое задание 2: создание собственной ошибки
// Создайте тип ошибки для системы аутентификации:

#[derive(Debug)]
pub enum AuthError {
    InvalidPassword,
    UserNotFound(String),
    TokenExpired,
}

// Реализация Display
impl fmt::Display for AuthError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AuthError::InvalidPassword => {
                write!(f, "Неверный пароль")
            }
            AuthError::UserNotFound(user) => {
                write!(f, "Пользователь {} не найден", user)
            }
            AuthError::TokenExpired => {
                write!(f, "Токен истёк")
            }
        }
    }
}

// Реализация Error
impl Error for AuthError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            _ => None
        }
    }
} 

/*** Box<dyn Error> ***/

// В Rust часто приходится иметь дело с функциями, которые могут возвращать разные типы ошибок, 
// и мы по какой-либо причине не хотим реализовывать собственный трейт для ошибок. 
// Например, одна функция может одновременно работать с файлами и парсить числа:

use std::num::ParseIntError;

// fn read_and_parse_file(path: &str) -> Result<i32, ???> {
//     let content = fs::read_to_string(path)?; // Ошибка чтения файла
//     let number = content.trim().parse::<i32>()?; // Ошибка парсинга
//     Ok(number)
// } 

// Возникает вопрос: что указать вместо ???? Ведь fs::read_to_string возвращает std::io::Error, а parse::<i32> — ParseIntError.
// Тут на помощь приходит динамическая диспетчеризация ошибок с помощью Box<dyn Error>:

fn read_and_parse_file(path: &str) -> Result<i32, Box<dyn Error>> {
    let content = std::fs::read_to_string(path)?;      // std::io::Error
    let number = content.trim().parse::<i32>()?;       // ParseIntError
    Ok(number)
} 

// dyn Error — трейтовый объект, который может представлять любую ошибку, реализующую трейт Error.
//     Box — умный указатель, который хранит значение в куче и позволяет работать с динамически выбранным типом ошибки. Подробнее вы узнаете про него дальше, в уроке про умные указатели.
//     Вместе Box<dyn Error> означает: «любая ошибка, реализующая Error, упакованная в Box».

// Такой подход особенно удобен для публичных API или учебных примеров, когда точный тип ошибки заранее неизвестен или их слишком много.
//  Почему это удобно:

//     Единый тип ошибки для функции. Функция может возвращать разные ошибки (io::Error, ParseIntError, свои кастомные и другие) через один Result<T, Box<dyn Error>>.
//     Использование оператора ? без ручного преобразования. Так как все ошибки реализуют трейт Error, ? автоматически превращает их в Box<dyn Error>.
//     Компактность кода. Не нужно писать отдельные enum для всех возможных ошибок.

// Использование Box<dyn Error> рекомендуется, если:

//     Функция может возвращать несколько разных типов ошибок и вы не хотите создавать для них отдельный enum.
//     Вы пишете простые утилиты или прототипы, где точный тип ошибки не критичен.

// Box<dyn Error> так же имеет и минусы:

//     Потеря конкретного типа ошибки. Когда вы используете Box<dyn Error>, компилятор больше не знает, какой конкретно тип ошибки хранится внутри.
//     Потеря производительности. Каждая ошибка оборачивается в Box, а значит, выделяется в куче (heap). Для мелких или часто возникающих ошибок это может привести к лишним аллокациям и небольшому падению производительности.
//     Меньше контроля над API. Использование Box<dyn Error> облегчает возврат разных типов ошибок через один интерфейс, но при этом вы теряете статическую проверку типов, которая есть у конкретных Result<T, MyError>.
//     Сложнее тестировать и обрабатывать специфические ошибки. Так как внутри Box может быть любой тип ошибки, для юнит-тестов или специальных обработчиков иногда нужно делать downcast_ref или is::<Type>(), что добавляет сложности.

// Резюмируя — Box<dyn Error> удобно использовать для упрощённого API, когда важнее гибкость, но пожертвовано спецификой ошибок и частично производительностью.

/*** Трейты From и Into ***/

// Трейты From и Into для преобразования ошибок
// Когда вы пишете функцию, в которой могут возникать разные типы ошибок, вам нужно как-то привести их к одному типу, чтобы вернуть из функции Result<T, E>.
// Вместо того чтобы вручную писать map_err(...) каждый раз для конвертации ошибки, можно использовать трейт From и оператор ?. Так компилятор сам преобразует ошибки в нужный тип:

// pub trait From<T>: Sized {
//     // Required method
//     fn from(value: T) -> Self;
// }

// pub trait Into<T>: Sized {
//     // Required method
//     fn into(self) -> T;
// } 

// From<A> for B означает: «Как из A сделать B».
// Обычно реализуется только From, а Into автоматически появляется с помощью макросов.
// В Rust достаточно реализовать From<T> for U, чтобы автоматически появилась реализация Into<U> for T. То есть, если написать:

// Рассмотрим более подробную реализацию трейта From для MyError:

// Объявляем перечисление с возможными ошибками
#[derive(Debug)]
pub enum MyError {
    NotFound,
    Parse(ParseIntError),
}

// Реализуем трейт Display для красивого отображения
impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            // write! пишет в форматтер
            MyError::NotFound => write!(f, "Элемент не найден"),
            MyError::Parse(e) => write!(f, "Ошибка парсинга числа: {}", e),
        }
    }
}

// Реализуем трейт Error 
// В большинстве простых случаев методы трейта реализовывать не обязательно
impl Error for MyError {}

// Реализуем трейт From 
impl From<ParseIntError> for MyError {
    // Преобразование из ошибки типа ParseIntError в MyError::Parse
    fn from(err: ParseIntError) -> Self {
        MyError::Parse(err)
    }
}

// Используем наш enum с ошибкой и автоматическим преобразованием ошибки 
pub fn read_number_from_vec(data: Vec<&str>, index: usize) -> Result<i32, MyError> {
    let s = data.get(index).ok_or(MyError::NotFound)?; // тут может вернуться ошибка
    
    // Тут ошибка парсинга строги ParseIntError преобразуется в MyError::Parse автоматически через From
    let number = s.parse::<i32>()?; 
    
    Ok(number)
}

// Вывод в консоль 
// Нашли число: 20
// Ошибка: Ошибка парсинга числа: invalid digit found in string
// Ошибка: Элемент не найден 

/*** Итоговое задание ***/

#[derive(Debug, PartialEq)]
pub enum ValidationError {
    EmailTooShort,
    EmailMissingAt,
    PasswordTooShort,
    AgeTooYoung,
}

#[derive(Debug, PartialEq)]
pub struct User {
    email: String,
    password: String,
    age: u8,
}

// Напишите функции, возвращающие Result:

pub fn validate_email(email: &str) -> Result<String, ValidationError> {
    // 1. Email должен быть минимум 5 символов
    // 2. Email должен содержать символ '@'

    if email.chars().count() < 5{
        return Err(ValidationError::EmailTooShort)
    }
    else if !email.contains('@')
    {
        return Err(ValidationError::EmailMissingAt)
    }
    else {
        return Ok(email.to_string());
    }
}

pub fn validate_password(password: &str) -> Result<String, ValidationError> {
    // Пароль должен быть минимум 8 символов
    if password.len() < 8 {
        return Err(ValidationError::PasswordTooShort);
    }

    Ok(password.to_string())
}

pub fn validate_age(age: u8) -> Result<u8, ValidationError> {
    // Возраст должен быть минимум 18 лет
    if age < 18 {
        return Err(ValidationError::AgeTooYoung)
    }
    
    return Ok(age)
}

pub fn create_user(email: &str, password: &str, age: u8) -> Result<User, ValidationError> {
    // Используйте функции валидации выше
    // Если все проверки прошли - создайте User

    println!("\tПопытка создать {} {} {}", email, password, age);

    let email_validated = validate_email(email)?;
    let password_validated = validate_password(password)?;
    let age_validated = validate_age(age)?;
    
    // Создаётся пользователь внутри и возвращается (Move-семантикой?)
    Ok(User{email: email_validated, password: password_validated, age: age_validated})
}