
// loop - бесконечный цикл, можно выйти только по break

use std::ops::{Add, AddAssign};

pub fn loop_example() {

    let mut my_num: u32 = 0;

    loop {

        println!("Этот цикл будет выполняться долго ...");
        
        if my_num == 10 {
            println!("Цикл завершился!");
            break;
        }

        my_num.add_assign(1);
    }

    // 2. Может возвращать значение при выходе
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    }; 

    println!("Loop_2: result == {} ", result);

    // 3. while
    let mut counter = 5;

    while counter > 0 {
        println!("counter == {counter}");
        counter -= 1;
    }

    println!("Счётчик достиг нуля");

    // 4. while-let
    #[derive(Debug, PartialEq)]
    enum Command {
        Move,
        Attack,
        Defend,
        Wait,
        End,
    }

    // ❌ Without Debug:
    // println!("{:?}", command);  // Error: `Command` doesn't implement `Debug`

    // ❌ Without PartialEq:}
    // if command == Command::Move { }  // Error: `Command` doesn't implement `PartialEq`


    let commands = [Command::Move, Command::Attack, Command::Wait, Command::Defend];
    let mut index = 0;
    
    // Сопоставление с шаблоном
    // Some(&Command) — valid index, returns reference to element
    while let Some(command) = commands.get(index) {
        println!("Выполняется команда: {:?}", command);
        index += 1;
    }
    
    println!("Все команды выполнены");

    // 5. for
    // Scope i ограничен циклом
    for i in 1..=10 {
    // 1..=10 => означает, что включать 10
        println!("Число: {}", i);
    }

}

pub fn matrix_search() {
    
    let matrix = [
        [1, 3, 5],
        [2, 7, 4],
        [9, 0, 6],
    ];
    let needle = 9;

    let mut i = 0;

    'extLoop: loop {
        if i >= matrix.len() {
            println!("{} не найдено.", needle);
            break;
        }

        let mut j = 0;

        'intLoop: loop {
            if j >= matrix[i].len() {
                break;
            }

            if matrix[i][j] == needle {
                println!("{} найдено в позиции ({}, {})!", needle, i, j);

                // Выход сразу из внешнего цикла, как только найдено
                break 'extLoop; 
            }

            j += 1;
        }

        i += 1;
    }

    println!("Конец.");
}


pub fn show_progress(current: u32, total: u32) {
    // Длина прогресс-бара
    const WIDTH: u32 = 20;

    let percentage = current * 100 / total;
    
    let chars_to_fill = percentage * WIDTH / 100;

    let mut bar = String::new();

    for i in 0..WIDTH {
        if i < chars_to_fill { 
            bar.push('█');
        } else { 
            bar.push(' ');
        };
    }

    println!("[{}] {}%", bar, percentage);
} 