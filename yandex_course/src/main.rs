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


fn main() {

    // Примеры вывода
    greet();
    println!("Сегодня: {}", OffsetDateTime::now_utc().date());
    println!("Я прохожу курс: {}!", DEFAULT_COURSE_NAME);

    // Вызов примеров с функциями

    /*** Типы, переменные ***/
    // variables();
    
    /*** Функции ***/
    print_coordinates(3, 4);
    let _is_exact_division = is_divisible(10, 3);
    let _temperature = celsius_to_fahrenheit(23.0);

    /*** Условные операторы ***/
    if_let_example_1();
    
    /*** Циклы ***/
    // loop_example();
    // matrix_search();
    show_progress(5, 15);
    
    /*** Владение ***/
    string_ownership();
    borrowing_example();
    reference_lifetime();

    // Создание экземпляра структуры
    // 1. Без конструктора с передачей значения
    // let config = CourseConfig {
    //     cohort: CourseCohort::Start,
    // };

    // 2. С конструктором с передачей значения
    let mut config = CourseConfig::new(CourseCohort::Start);
    println!("Длительность вашей когорты: {}", config.get_duration());

    config.upgrade_cohort();
    println!("Длительность вашей когорты: {}", config.get_duration());

    /* Generic'и и trait'ы */
    let field: generics_traits::FieldDerived<i32> = generics_traits::FieldDerived::default();

    // false (значение по умолчанию для bool)
    println!("{}", field.is_valid); 

} 
