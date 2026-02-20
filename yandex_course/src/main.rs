// mod utils;
// Searching priority: 
//     - This file, under the mod utils { ... } section;
//     - Under the current dir, file utils.rs (this is the case where is the code now);
//     - Under the current dir, but deeper: utils/mod.rs.

// Подключение верхнеуровневого модуля
mod basics;

use basics::config::DEFAULT_COURSE_NAME;

use basics::types::greet;
// use basics::types::variables;
use basics::functions::print_coordinates;
use basics::functions::is_divisible;
use basics::functions::celsius_to_fahrenheit;

// Условные операторы
use basics::conditionals::if_let_example_1;

// Циклы
use basics::loops::{loop_example, matrix_search, show_progress};

use time::OffsetDateTime;

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

}
