/*** enum для обработки ошибок ***/

// Для обработки ошибок используются два enum:

// enum Option<T>. 
// Используется, когда значение может существовать (Some(T)) или отсутствовать (None). 
// Например, поиск элемента в коллекции возвращает None, если элемент не найден.

// enum Result<T, E>. 
// Используется для операций, которые могут завершиться успешно и вернуть либо результат (Ok(T)), либо ошибку (Err(E)). Это основной способ обработки ошибок в Rust.
// Например, подключение к базе данных может вернуть Err("Неверный пароль").

// enum Result<T, E> {
//     Ok(T),   // успешный результат (значение типа T)
//     Err(E),  // ошибка (значение типа E)
// }


/*** Ошибки в Rust ***/

// Делятся на два типа:
//  - Устранимые (Recoverable) — их можно обработать и продолжить выполнение программы. Для этого используется Result.
//  - Неустранимые (Unrecoverable) — такие ошибки делают дальнейшую работу бессмысленной. 
//    В этих случаях вызывается макрос panic!.
//    Например, если программа не может подключиться к базе данных, выполнение лучше сразу прервать, а не продолжать работать в полусломанном состоянии.
//    panic! как раз предназначен для невосстанавливаемых ошибок, которые говорят о проблеме в самой программе. Их должен исправить разработчик.

/***  Принцип Fail Fast ***/
// Означает, что программа должна немедленно остановиться, если она оказалась в неконсистентном состоянии. Это упрощает отладку и предотвращает возможные серьёзные последствия от замаскированных ошибок. */

/*** Трейт std::error::Error ***/
// Для единообразной работы с ошибками в стандартной библиотеке определён трейт std::error::Error. Его реализуют разные типы ошибок, что позволяет:
//     - возвращать разные ошибки через общий интерфейс;
//     - использовать оператор ? для удобного распространения ошибок;
//     - конвертировать ошибки из одной библиотеки в другую с помощью From и Into.


/*** Секция кода ***/
// use Option::{*};        // Сам придумал(!)

// enum Option<T> {
//     Some(T),
//     None,
// }

// Найти первое слово длиннее 5 символов
// Принимает вектор строк
pub fn find_long_word(words: &[String]) -> Option<&String> {
    for word in words {
        if word.chars().count() > 5 {
            return Some(word); // нашли подходящее слово
        }
    }
    None // ничего не нашли
} 

pub fn find_long_word_example() {
    let words = vec![
        String::from("cat"),       // 3 chars
        String::from("elephant"),  // 8 chars → MATCH
        String::from("dog"),       // 3 chars
    ];
    
    match find_long_word(&words) {
        // Деструктуризация
        Some(word) => println!("Found: {}", word),   // "elephant"
        None => println!("No long word found"),
    }
}

/*** Варианты обработки ошибок с Option */

// 1. match. Полная и безопасная обработка всех вариантов:

pub fn option_match_check() {
    let number = Some(10);
    
    match number {
        // Вывод в консоль: Нашли число: 10 
        Some(n) => println!("Нашли число: {}", n),
        None => println!("Ничего не нашли"),
    }
}


// 2. if let. Удобный синтаксис, если интересует только Some или только None:
pub fn option_if_let_check() {
    let number: Option<i32> = Some(7);

    if let Some(n) = number {
        // Вывод в консоль: Нашли число: 7
        println!("Нашли число: {}", n);
    } else {
        println!("Ничего нет");
    }
}

pub fn option_if_let_none_check() {
    let number: Option<i32> = None;

    if let None = number {
        // Вывод в консоль: Числа нет 
        println!("Числа нет");
    } else {
        println!("Нашли число!");
    }
}

// 3. unwrap. Берёт значение из Some, но упадёт с паникой, если там None:

pub fn option_unwrap() {
    let has_value = Some(42);
    println!("Значение: {}", has_value.unwrap()); // 42

    let none_value: Option<i32> = None;
    // println!("{}", none_value.unwrap()); // 💥 panic 

    // Но можно так:
    println!("Значение опции: {}", none_value.unwrap_or(42));
}

// 4. expect. То же самое, что unwrap, но можно указать сообщение: 
pub fn option_expect() {
    let a = Some("Rust");
    println!("Значение: {}", a.expect("Ожидали строку"));

    let b: Option<&str> = None;
    // println!("{}", b.expect("Значение должно быть!")); 
    // panic с этим сообщением (аварийное завершение программы)  
}

// 5. let-else. 
// Удобный синтаксис, который позволяет сразу извлекать значение из Option, 
// а если его нет — выходить из функции или делать return, break, continue.
fn print_first_even(numbers: &Vec<i32>) {
    
    // Попытка взять первое чётное число
    let Some(n) = numbers.iter().find(|&&x| x % 2 == 0) else {
        println!("Чётных чисел нет");
        return;
    };

    // (!) n ещё здесь существует, как видно
    println!("Первое чётное число: {}", n);
}

pub fn option_let_else_check () {
    
    let numbers1 = vec![1, 3, 5, 7];
    let numbers2 = vec![1, 4, 5, 6];    
    
    print!("Массив #1: ");
    // (!) Чтобы проитерироваться, нужно создать ссылку на массив
    for num in &numbers1 {
        print!("{} ", num);
    }
    println!();
    
    print!("Массив #2: ");
    // (!) Чтобы проитерироваться, нужно создать ссылку на массив
    for num in &numbers2 {
        print!("{} ", num);
    }
    println!();

    print_first_even(&numbers1);
    print_first_even(&numbers2);
}


fn find_first_even(numbers: &[i32]) -> Option<i32> {
    // Попытка взять первое чётное число
    let Some(n) = numbers.iter().find(|&&x| x % 2 == 0) else {
        println!("Чётных чисел нет");
        return Option::None;
    };

    Some(*n)
}

fn find_first_even_2(numbers: &[i32]) -> Option<i32> {
    // Попытка взять первое чётное число
    for &num in numbers {
        if num % 2 == 0 {
            return Some(num)
        }
    }

    // Прошли весь массив, возвращаем None
    return Option::None
}

pub fn first_even_check() {
    // Тесты
    assert_eq!(find_first_even(&[1, 3, 8, 4, 7, 8]), Some(8));
    assert_eq!(find_first_even(&[1, 3, 5, 7]), None);
    println!("На основе кода из примеров: все тесты прошли");

    // Своя функция
    assert_eq!(find_first_even_2(&[1, 3, 4, 7, 8]), Some(4));
    assert_eq!(find_first_even_2(&[1, 3, 5, 7]), None);
    println!("Свой код: все тесты прошли");
} 

/*** Варианты обработки ошибок с Result ***/

// enum Result<T, E> {
//     Ok(T),   // успешный результат (значение типа T)
//     Err(E),  // ошибка (значение типа E)
// }

 // T — это тип успешного значения.
 // E — это тип ошибки. 
//  Ok(T) означает успешное завершение операции и хранит значение результата.
//  Err(E) означает ошибку и хранит информацию о ней.


pub fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err("деление на ноль".to_string())
    } else {
        Ok(a / b)
    }
}

// Мой пример
pub fn divide_result_example() {
    let mut x = 5;
    let mut y = 10;

    // Деструктуризация: if let
    if let Ok(result) = divide(x, y) {
        println!("Деление {} / {} == {}", x, y, result);
    }

    // Проверка деления на 0
    x = 10;
    y = 0;

    match divide(x, y) {
        Ok(result) => {println!("{} / {} == {}", x, y, result);}
        Err(result) => {println!("{}", result);}
    };
}

// Примеры из курсов

// 1. Обработка Result с помощью match
// Попытаемся преобразовать строковый литерал в число типа i32. 
// В случае неудачи мы распечатаем ошибку в консоль и не прервём выполнение программы:

use std::num::ParseIntError;

fn parse_and_double(s: &str) -> Result<i32, ParseIntError> {
    match s.parse::<i32>() {
        Ok(n) => Ok(n * 2),
        Err(e) => Err(e),
    }
}

pub fn result_match_example () {
    let inputs = ["50", "200", "abc"];
    
    for s in &inputs {
        match parse_and_double(s) {
            Ok(v) => println!("Удвоенное число: {}", v),
            Err(e) => println!("Ошибка парсинга: {}", e),   // (!) ошибка от парсера
        }
    }
    // Вывод в консоль: 
    // "Удвоенное число: 100"
    // "Ошибка парсинга: invalid digit found in string" 
    
}

// 2. if let. Здесь мы либо хотим поймать ошибку, либо обработать успешный кейс
// Успешный вариант 


pub fn result_if_let_example() {
    let mut x = 5;
    let mut y = 10;

    if let Ok(value) = divide(x, y) {
        println!("Результат: {} / {} == {}",x, y, value);
    }
    // Опционально
    else {
        println!("Ошибка");  // Нет доступа к тексту ошибки 
    }
    // Вывод в консоль: Результат: 5 

    // Ловим ошибку
    y = 0;

    if let Err(error) = divide(10, 0) {
        println!("Ошибка: {} / {}: {}",x, y, error);
    }
    // Опционально
    else {
        println!("Операция прошла успешно"); // Нет доступа к значению результата операции
    }
}
// Вывод в консоль: Ошибка: деление на ноль 

// 3. .expect() и .unwrap(). Работают идентично тому, как они работают для Option.

// Практическое задание #1
pub fn parse_number(s: &str) -> Result<i32, std::num::ParseIntError> {
    s.parse()
}

// Практическое задание #2

// derive'ы необходимы для корректной работы assert_eq!
#[derive(Debug, PartialEq)]     
enum MathError {
    DivisionByZero,
    UnknownError,
}

fn safe_divide(a: f64, b: f64) -> Result<f64, MathError> {
    if (b == 0.0) {
        Err(MathError::DivisionByZero)
    }
    else {
        Ok(a / b)
    }
}

pub fn save_divide_example() {

    // Тесты
    assert_eq!(safe_divide(10.0, 2.0), Ok(5.0));
    assert_eq!(safe_divide(10.0, 0.0), Err(MathError::DivisionByZero));
    println!("Все тесты прошли!");
}



/*** Оператор ? (Question Mark) ***/
// В Rust оператор ? — это синтаксический «сахар» для упрощённой обработки ошибок.
// Он позволяет избавиться от многословного match при работе с Result и Option:

pub fn read_file(path: &str) -> std::io::Result<String> {
    let content = std::fs::read_to_string(path)?; 
    Ok(content)
}

// (!) Вместо явного match  
// fn read_file(path: &str) -> std::io::Result<String> {
//     match std::fs::read_to_string(path) {
//         Ok(content) => Ok(content),
//         Err(e) => return Err(e),
//     }
// } 

// То есть оператор ? делает две вещи:
//  - Если результат Ok или Some — достаёт значение.
//  - Если результат Err или None — немедленно возвращает из функции эту ошибку/отсутствие значения.

// Важное уточнение:
// Для использования оператора ? функция или метод должны возвращать либо Result, либо Option, иначе этот оператор использовать нельзя. 

// 2. Ещё пример:
// Err(...) возвращается немедленно наверх после получения.
// Если Ok(...), то 
// fn calculate_discount(price_str: &str) -> Result<f64, String> {
//     let base_price = process_str(price_str)?;  // ← Propagates error upward
    
//     if base_price > 100.0 {
//         Ok(base_price * 0.9)  // 10% discount
//     } else {
//         Ok(base_price)
//     }
// }

//                  Можно	                                           Нельзя
// Когда в функции простая логика                      Если при ошибке нужно что-то логировать,
// и вам не нужно ничего особенного делать с ошибкой.  чистить ресурсы или предпринимать действия.

// Когда вы хотите пробросить ошибку дальше по стеку.  Если хочется дать более информативное сообщение (expect)
//                                                     или fallback-значение (unwrap_or).

// Когда это внутренние функции и обработка ошибок 
// будет выше.	