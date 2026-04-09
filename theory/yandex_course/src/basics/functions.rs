// Пример функции, принимающей параметры. Им нужны явные аннотации!
pub fn print_coordinates(x: i32, y: i32) {
    println!("Координаты: ({}, {})", x, y);
}

// Пример функции, возвращающей скалярный тип. Обратите внимание, как задаётся имя.
pub fn is_divisible(numerator: u32, denominator: u32) -> bool {
    // Последнее значение служит как return, (!) без точки с запятой
    denominator != 0 && numerator % denominator == 0
}

// Пример функции, возвращающей составной тип
pub fn celsius_to_fahrenheit(celsius: f64) -> (f64, f64) {
    let fahrenheit = celsius * 9.0 / 5.0 + 32.0;

    return (celsius, fahrenheit);
}