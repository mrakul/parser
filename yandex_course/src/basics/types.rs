// Public HelloWorld
pub fn greet() {
    println!("Hello, world!");
}

pub fn variables() {

    // (!) Черта перед переменной подавляет warning об unused variable
    // Ниже везде следую этому правилу

    const _APP_NAME: &str = "MyApp";
    const _LIMIT: u8 = 100;

    // Mutable variable definition and init with a value
    let mut x = 1;
    
    // Immutable variable definition and init with a value
    let user_name = "Боб";
    println!("Привет, {}!", user_name);
    println!("Ты пользователь №{}", x);

    // Mutable variable change
    x = 2;

    // Variable "Shadowing"
    let user_name = "Миша";

    print!("Привет, {}! ", user_name);
    println!("Ты пользователь №{}", x);


    // несмотря на значение при инициализации, ...
    let mut _inferred_int = 30;

    // ... тип `i64` фактически выводится здесь
    _inferred_int = 3_000_000_000i64;   

    // Пример преобразования типа через `as`
    let _casted_int = 10u8 as i32;

    // Пример экспоненциальной записи - `f64` (1_000_000.0)
    let _exp_float = 1e6;


    // Пример неявной аннотации - `bool`
    let _is_active = true;

    // Пример явной аннотации - `bool`
    let _is_ready: bool = false;

    // Пример выведения типа
    let _is_greater = 2 > 1;

    // Пример преобразования в число через `as`
    let casted_bool = true as i32;
    println!("casted_bool = {}", casted_bool);

    // Пример неявной аннотации - `char`
    let _default_char = 'a';

    // Пример явной аннотации - `char`
    let _ecliptic_char: char = 'ℤ';

    // Вариант используемого значения
    let emoji_char = '🦀';

    // Пример преобразования через `as`
    let casted_char = '🦀' as u32;
    println!("emoji_char: {}, casted_char = {}", emoji_char, casted_char);


    // Проверка адреса при "затенении"
    let x = 5;
    println!("x1 address: {:p}, value: {}", &x, x);
    
    let x = 10;
    println!("x2 address: {:p}, value: {}", &x, x);
    
    let x = 15;
    println!("x3 address: {:p}, value: {}", &x, x);


    /*** Кортежи ***/
    println!(">>> Кортежи <<<");
    // Имеют фиксированную длину

    // Пример неявной аннотации - `(i32, f64, bool)`
    let mut person = (30, 1.85, true);

    // Обращение к первому элементу
    person.0 = 40;
    println!("Person: {:?}", person);

    // Пример явной аннотации - `(i32, bool, char)`
    let team: (i32, bool, char) = (10, true, '🦀');

    // ВАЖНО: кортеж из одного элемента требует запятую!
    let single_tuple = (10,); 
    println!("single_tuple: {} (тип (i32,))", single_tuple.0);      

    // А это НЕ кортеж! это скалярный тип `i32` => предупреждение об этом
    let single_number = (10);
    println!("single_number: {} (тип i32)", single_number);

    // Пример деструктуризации кортежа (аналог structure folding в C++17)
    let (age, height, is_active) = person;
    println!("Возраст: {}, рост: {}, активен: {}", age, height, is_active);

    // Пример доступа по индексу
    let team_icon = team.2;
    println!("Символ команды: {}", team_icon);


    /*** Массивы ***/
    println!(">>> Массивы <<<");

    // Пример неявной аннотации - `[i32; 7]`
    let week_temperatures = [20, 22, 19, 24, 21, 22, 25];

    // Как константа => требует явного указания типа
    const WEEK_TEMP: [u32; 7] = [20, 22, 19, 24, 21, 22, 25];

    // Вывод всего tuple
    println!("Week temp {:?}", WEEK_TEMP);

    // Доступ к одному элементу, не как в tuple
    WEEK_TEMP[0];

    // Пример явной аннотации - `[char; 4]`
    let weather_icons: [char; 4] = ['🌤', '🌧', '☀', '⚡'];

    // Пример инициализации с повторением - [bool; 7]
    let day_monitoring_flags = [true; 7];

    // Пример доступа по индексу
    println!("Понедельник: {}°C {}", week_temperatures[0], weather_icons[0]);

    /*** Структуры ***/
    println!(">>> Массивы <<<");

    // Структура с именованными полями
    struct Person {
        age: u8,
        is_active: bool,
    }

    // Структура-кортежа
    struct Point(i32, i32);

    // Пример юнит-подобной структуры
    struct Logger;

    let person = Person {
        age: 20,
        is_active: true,
    };

    println!("Возраст: {}, активен {}", person.age, person.is_active);

    /*** Перечисления (enum) */
    enum UserAction {
        SignOut,                        // без данных
        _MoveCursor { x: i32, y: i32 },  // с именованными полями
        _SendMessage(String),            // с одним значением
        _ChangeTheme(u8, u8, u8),        // с кортежем
    }

    let _action = UserAction::SignOut;

}