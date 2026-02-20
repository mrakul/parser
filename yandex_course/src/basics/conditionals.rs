pub fn heart_rate_status(bpm: u8) -> char {

    // 1. if выступает как полноценное выражение, возвращающее значение
    // 2. Условия не заключаются в скобки (можно заключить)
    // 3. Возвращает один и тот же тип
    let result = if bpm < 50 {
        '🐢'
    } else if bpm < 100 {
        '🙂'
    } else {
        '🔥'
    };

    return result;
} 

enum Command {
    Quit,
    Move { x: i32, y: i32 },
}

pub fn if_let_example_1() {
    let cmd = Command::Move{x: 5, y: 10};

    if let Command::Move{ x, y } = cmd {
        println!("Координаты перемещения: ({x},{y})");
    }
}


enum UserRole {
    Guest,
    Moderator,
    Admin
}

struct User {
    name: String,
    role: UserRole,
    active: bool,
}


fn if_let_example_2() {
    // if let проверяет на compile-time, что может быть деструктуризирована.
    // И в runtime проверяет конкретные значения указанных полей, если нужно.
    // Принято, что проверка одного значения. Если надо много, то match

    // Пример использования с кортежем
    let transaction = (1500.0, "USD", 200);
    
    // Явно указываем, каким должен быть 2-й и 3-й элемент ...
    // (!) Только если `USD` и `status_code == 200`
    if let (amount, "USD", 200) = transaction {
        println!("Успешный платёж: ${} USD", amount);
    }

    // Пример использования с массивами
    let download_progress = [0, 25, 50, 75, 100];
    
    // Загрузка должна начинаться с 0% !
    if let [0, .., completion] = download_progress {
        if completion == 100 {
            println!("Загрузка успешно завершена!");
        }
    };

    // Пример использования со структурами
    let new_user = User {
        name: "Алиса".to_string(),
        role: UserRole::Admin,
        active: true,
    };

    // Только активный Admin !
    if let User {
        name,
        role: UserRole::Admin,
        active: true,
    } = new_user
    {
        println!("У нас новый Администратор: {}", name);
    }
}

pub fn let_else_example() {
    // let else: раннее завершение потока управления

    // Имитация значения в виде кортежа
    let config = ("localhost", 8080);

    // Ранняя проверка
    let ("localhost", port) = config else {
        // Возврат `never` (паника)
        panic!("Ожидался хост 'localhost', получен: {}", config.0);
    };

    println!("Сервер запущен на localhost:{}", port);


    // Имитация значения в виде массива
    let upload_progress = [0, 25, 50, 75, 100];

    // Ранняя проверка
    let [0, .., 100] = upload_progress else {
        println!("Загрузка не завершилась");
        // Возврат `never`
        return;
    };

    println!("Загрузка успешно завершена!");
}

pub fn match_example () {
    let dice_roll = 9;
    
    // Без дефолтного _ - не скомпилируется, нужно полное покрытие
    match dice_roll {
        3 => println!("Получен бонус!"),
        7 => println!("Потеря хода."),
        _ => println!("Продолжайте игру."),
    }


    // Можно использовать диапазоны
    let age = 25;

    match age {
        0..=12 => println!("Ребёнок"),
        13..=19 => println!("Подросток"),
        20..=64 => println!("Взрослый"),
        _ => println!("Пожилой"),
    }

    // Или с деструктуризацией
    let some_point = (1, 20);

    match some_point {
        (0, 0) => println!("Начало координат"),
        (x, 0) => println!("На оси X: x = {}", x),
        (0, y) => println!("На оси Y: y = {}", y),
        (x, y) => println!("Точка: ({}, {})", x, y),
    }

    // Каждая ветвь является выражением, поэтому 
    // match может возвращать значение, 
    // тип которого соответствует возвращаемому типу всех используемых ветвей.

    let experience: u32 = 80;

    // Присвоение через возврат от match
    let level = match experience {
        0..=100 => "Новичок",
        101..=500 => "Опытный",
        _ => "Эксперт",
    };

}