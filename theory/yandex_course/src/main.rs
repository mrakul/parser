// mod utils;
// Searching priority: 
//     - This file, under the mod utils { ... } section;
//     - Under the current dir, file utils.rs (this is the case where is the code now);
//     - Under the current dir, but deeper: utils/mod.rs.

// Стандартные модули
use time::OffsetDateTime;

// Подключение верхнеуровневого модуля
mod basics;

// Конфигурация
use basics::config::{DEFAULT_COURSE_NAME, CourseConfig, CourseCohort};

// Типы, переменные 
use basics::types::greet;
use basics::types::variables;

// Функции
use basics::functions::print_coordinates;
use basics::functions::is_divisible;
use basics::functions::celsius_to_fahrenheit;

// Циклы
use basics::loops::{loop_example, matrix_search, show_progress};

// Владение, заимствование
use basics::ownership::string_ownership;
use basics::borrowing::borrowing_example;

// Условные операторы
use basics::conditionals::if_let_example_1;

// Borrowing
use basics::borrowing::reference_lifetime;

// Generics, traits
use basics::generics_traits;

// Строки
use basics::output::{*};

// Замыкания (closures) и fn-трейты
use basics::closures::{*};

// Обработка ошибок
use basics::errors_processing::{*};
use basics::errors_processing_best_practices::{*};
use std::error::Error;

// Умные указатели
use basics::smart_pointers::{*};

// Коллекции
use basics::collections::{*};

// Итераторы
use basics::collections::{*};

use crate::basics::iterators::iterators_example;

fn main() {

    // // Примеры вывода
    // greet();
    // println!("Сегодня: {}", OffsetDateTime::now_utc().date());
    // println!("Я прохожу курс: {}!", DEFAULT_COURSE_NAME);

    // // Вызов примеров с функциями

    // /*** Типы, переменные ***/
    // // variables();
    
    // /*** Функции ***/
    // print_coordinates(3, 4);
    // let _is_exact_division = is_divisible(10, 3);
    // let _temperature = celsius_to_fahrenheit(23.0);

    // /*** Условные операторы ***/
    // if_let_example_1();
    
    // /*** Циклы ***/
    // // loop_example();
    // // matrix_search();
    // show_progress(5, 15);
    
    // /*** Владение ***/
    // string_ownership();
    // borrowing_example();
    // reference_lifetime();

    // // Создание экземпляра структуры
    // // 1. Без конструктора с передачей значения
    // // let config = CourseConfig {
    // //     cohort: CourseCohort::Start,
    // // };

    // // 2. С конструктором с передачей значения
    // let mut config = CourseConfig::new(CourseCohort::Start);
    // println!("Длительность вашей когорты: {}", config.get_duration());

    // config.upgrade_cohort();
    // println!("Длительность вашей когорты: {}", config.get_duration());

    // /*** Generic'и и trait'ы ***/
    // let field: generics_traits::FieldDerived<i32> = generics_traits::FieldDerived::default();

    // // false (значение по умолчанию для bool)
    // println!("{}", field.is_valid); 

    // /*** Строки и форматирование ***/
    // strings_format();
    // print_student();
    // specificators();

    // // Вывод файла и номера строки, где выполняется
    // let x = dbg!(5 * 4);

    // // Вывод с реализацией трейта Display для конфигурации
    // println!("{}", config);

    /*** Замыкания (closures) и fn-трейты ***/
    // closure_example();
    // closure_example_2();
    // closure_sum();
    // fn_traits_example();
    // borrow_move_closure();
    // calculate_differently();
    // generic_functor();
    // adders_creation();

    /*** Обработка ошибок ***/

    // Примеры c Option
    // find_long_word_example();

    // option_match_check();

    // option_if_let_check();
    // option_if_let_none_check();

    // option_unwrap();
    // option_expect();

    // option_let_else_check();

    // // Задание
    // first_even_check();

    // // Примеры с Result
    // divide_result_example();
    // result_match_example();
    // result_if_let_example();

    // let result = parse_number("abc");
    // println!("{:?}", result);  
    // // Вывод, отладочный из-за {:?}: "Err(ParseIntError { kind: InvalidDigit })"

    // // Практическое задание #2
    // save_divide_example();

    // // Оператор ?: синтаксический сахар для обработки ошибок

    // let mut file_path = "/home/m_rakul/Code/RustYandex/yandex_course/aux/text.txt";

    // let mut read_result = read_file(file_path);

    // match read_result {
    //     Err(error) => println!("Файл не прочитан. {}", error),
    //     Ok(value) => println!("Контент файла: {}", value),
    // }

    // file_path = "/home/m_rakul/Code/RustYandex/yandex_course/aux/not_exists.txt";

    // read_result = read_file(file_path);

    // match read_result {
    //     Err(error) => println!("Файл не прочитан. {}", error),
    //     Ok(value) => println!("Контент файла: {}", value),
    // }

    // // Самый простой вариант
    // // println!("Контент файла: {}", read_result.unwrap());
    
    // /*** Обработка ошибок, Best Practices ***/

    // // Method          Processes Ok?       Explanation
    // // map             ✅ Yes              Applies function to Ok value, returns Result<U, E>
    // // map_err         ❌ No               Applies function to Err value (processes errors)
    // // and_then        ✅                  Yes Applies function to Ok value, function returns Result (chaining)
    // // or_else         ❌                  No Applies function to Err value (error recovery)
    // // unwrap_or       ❌                  No Extracts value or provides default (not transformation)

    // println!("Parsed and double {}", parse_and_double_map("34").unwrap());
    // // panic!, поскольку берём только успешный результат
    // // println!("Parsed and double {}", parse_and_double_map("abc").unwrap());  

    // println!("Parsed and double {}", parse_number_map_err("50").unwrap());
    // // panic!, поскольку unwrap() применяется для Ok успешно (❌.unwrap() on Err always panics)
    // // println!("Parsed and double {}", parse_number_map_err("abc").unwrap());    
    
    // println!("Parsed and double {}", parse_and_sqrt_map_err_and_then("50").unwrap());
    // // Если parse успешный → вызываем square_root.
    // // Если ошибка → она передаётся дальше, то есть не предполагается вызов unwrap()
    // // println!("Parsed and double {}", parse_and_sqrt_map_err_and_then("abc").unwrap());    
    
    // println!("PORT: {}", get_env_var_unwrap_or("PORT"));        // Для примера можно сделать "export PORT=100"
    // println!("PORT: {}", get_env_var_unwrap_or("NOT_EXISTING_ENV_VAR"));    

    // /*** Цепочки значений с ошибками/ ***/
    // match parse_str_get_root_and_x10("Это не число") {
    //     Ok(result) => {println!("Корень * 10: {}", result);}
    //     Err(error) => {println!("{}", error);}
    // };

    // // Создание временных векторов
    // let _unused_result = process_vec(vec!["Test", "Lane", "Boom-boom"]);
    // let _unused_result =process_vec(Vec::from(["Test", "Lane", "Boom-boom"]));
    // let _unused_result =process_vec(["Test", "Lane", "Boom-boom"].to_vec());

    // // Пример с вектором
    // let nums = vec!["9.0", "4.0", "16.0", "4.0"];

    // // Попробуем преобразовать всё в Vec<f64>
    // match process_vec(nums) {
    //     Ok(results) => println!("Результаты: {:?}", results),
    //     Err(err) => println!("Ошибка: {}", err),
    // }
    // // Результаты: [30.0, 20.0, 40.0, 20.0]

    // let nums = vec!["9.0", "4.0", "-16.0", "4.0"];

    // // Попробуем преобразовать всё в Vec<f64>
    // match process_vec(nums) {
    //     Ok(results) => println!("Результаты: {:?}", results),
    //     Err(err) => println!("Ошибка: {}", err),
    // }

    // // Практическое задание
    // assert_eq!(get_port_config(Some("3000".to_string())), 3000);
    // assert_eq!(get_port_config(Some("abc".to_string())), 8080);
    // assert_eq!(get_port_config(None), 8080);
    // println!("Тесты прошли");

    // /*** panic! */
    // let config = load_game_config("./aux/game_config.txt")
    //     .expect("Конфигурация игры не найдена, программа не может продолжить!");
    
    // println!("Конфиг загружен: {}", config);
    // // Если нет файла настроек, то .expect() вызовет panic!

    // /*** Трейт Error ***/
    // // Использование, более сложный пример реализация трейта Display и Error
    // match load_config("./aux/app.conf") {
    //     Ok(config) => {
    //         println!("Конфиг загружен: {:?}", config);
    //     }
    //     Err(err) => {
    //         eprintln!("Ошибка: {}", err);
            
    //         // Показываем цепочку ошибок
    //         let mut source = err.source();
    //         while let Some(err) = source {
    //             eprintln!("  Вызвано: {}", err);
    //             source = err.source();
    //         }
    //     }
    // }

    // // Практическое задание #2
    // // Реализуйте Display и Error
    // let err = AuthError::UserNotFound("john".to_string());
    // assert_eq!(err.to_string(), "Пользователь john не найден");
    
    // let err = AuthError::InvalidPassword;
    // assert_eq!(err.to_string(), "Неверный пароль");
    
    // let err = AuthError::TokenExpired;
    // assert_eq!(err.to_string(), "Токен истёк");
    
    // println!("Тесты прошли");

    // /*** Трейты From и Into ***/
    // let data = vec!["10", "20", "oops", "40"];

    // match read_number_from_vec(data.clone(), 1) {
    //     Ok(n) => println!("Нашли число: {}", n),
    //     Err(e) => println!("Ошибка: {}", e),
    // }

    // match read_number_from_vec(data.clone(), 2) {
    //     Ok(n) => println!("Нашли число: {}", n),
    //     Err(e) => println!("Ошибка: {}", e),
    // }

    // match read_number_from_vec(data.clone(), 10) {
    //     Ok(n) => println!("Нашли число: {}", n),
    //     Err(e) => println!("Ошибка: {}", e),
    // }

    // /*** Итоговое задание по обработкам ошибок ***/
    
    // println!(">>> Создание пользователей <<<");

    // // 1. Успешное создание пользователя
    // match create_user("user@test.com", "password123", 25) {
    //     Ok(user) => println!("Пользователь создан: {:?}", user),
    //     Err(e) => println!("Ошибка: {:?}", e),
    // }
    
    // // 2. Обработка конкретных ошибок
    // match create_user("abc", "123", 16) {
    //     Ok(user) => println!("Пользователь создан: {:?}", user),
    //     Err(ValidationError::EmailTooShort) => println!("Не создан: e-mail слишком короткий!"),
    //     Err(ValidationError::EmailMissingAt) => println!("В e-mail отсутствует @"),
    //     Err(ValidationError::PasswordTooShort) => println!("Пароль слишком короткий!"),
    //     Err(ValidationError::AgeTooYoung) => println!("Пользователь слишком молод!"),
    // }
    
    // // 3. Игнорирование ошибок (не рекомендуется, но возможно)
    // if let Ok(user) = create_user("admin@site.com", "securepass", 30) {
    //     println!("Администратор создан: {:?}", user);
    // }
    
    // // 4. Использование unwrap_or для значения по умолчанию
    // let default_age = validate_age(15).unwrap_or(18);
    // println!("Возраст: {}", default_age);
    
    // // Тесты
    // assert_eq!(validate_email("ab"), Err(ValidationError::EmailTooShort));
    // assert_eq!(validate_email("test.com"), Err(ValidationError::EmailMissingAt));
    // assert_eq!(validate_email("user@test.com"), Ok("user@test.com".to_string()));
    
    // assert_eq!(validate_password("1234567"), Err(ValidationError::PasswordTooShort));
    // assert_eq!(validate_password("password123"), Ok("password123".to_string()));
    
    // assert_eq!(validate_age(17), Err(ValidationError::AgeTooYoung));
    // assert_eq!(validate_age(25), Ok(25));
    
    // println!("Все тесты прошли! Поздравляем!");

    /*** Умные указатели ***/

    // box_using();
    // // list_pointers();
    // rc_using();

    // // RefCell
    // ref_cell_example();
    // ref_cell_panic();
    // ref_cell_add_value();
    // rc_refcell_example();

    /*** Коллекции ***/

    // 1.  Vec / VecDeque
    // vec_example();
    // vec_deq_example();

    // 2. BTreeSet
    // btreeset_example();

    // // 3. BTreeMap
    // btreemap_example();

    // // 4. String
    // string_example();

    // // 5. HashSet, HashMap
    // hashset_example();

    // // 6. BinaryHeap
    // binaryheap_example();

    // // Практическое задание
    // anagrammes();

    /*** Итераторы ***/
    iterators_example();

}