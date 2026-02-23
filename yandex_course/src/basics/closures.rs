// Синтаксис у замыкания такой:
// |параметры| выражение
// Есть расширенный вариант:
// |параметры| -> ТипРезультата { тело } 
pub fn closure_example() {
    let mut counter = 0;
    
    let mut increment = || {
        counter += 1;
        println!("Счётчик: {}", counter);
    };
    
    increment(); // Счётчик: 1
    increment(); // Счётчик: 2
}

pub fn closure_example_2() {
    // Принимает x: i32, возвращает i32
    let add_one: fn(i32) -> i32 = |x: i32| -> i32 { x + 1 };
 
    println!("{}", add_one(5));  // 6
}

pub fn closure_sum() {
    let x = 2; // внешняя переменная
    let y = 3; // внешняя переменная

    // Замыкание: захватывает x и y из контекста автоматически
    // (! Захватывает по умолчанию по immutable borrow)
    let sum_closure = || x + y;

    println!("sum_closure: {} + {} = s{}", sum_closure(), x, y); // 5
}


/*** Fn-трейты ***/

// - Трейт Fn можно вызывать сколько угодно раз, ничего не меняя.
// - Трейт FnMut можно вызывать сколько угодно раз, но он может менять своё состояние.
// - Трейт FnOnce можно вызвать только один раз, потому что он забирает владение данными.

//  У Fn-трейтов есть иерархия:
//  FnOnce -> FnMut -> Fn
//     - Fn наследуется от FnMut.
//     - FnMut наследуется от FnOnce.

// Всё, что реализует Fn, автоматически реализует FnMut и FnOnce.

fn call_fn_mut<F: FnMut()>(mut f: F) { f(); f(); }
fn call_fn_once<F: FnOnce()>(f: F) { f();}

pub fn fn_traits_example() {
    let mut value = 1;

    // Замыкание, которое может изменять value
    // Captures: value by mutable borrow => при наведении на || показывает
    let mut my_closure = || {
        value += 1;
        println!("Inside closure: value = {}", value);
    };

    println!("--- Используем как FnMut ---");
    call_fn_mut(&mut my_closure); 
    // Первый вызов внутри call_fn_mut: value = 1 -> 2
    // Второй вызов внутри call_fn_mut: value = 2 -> 3
    println!("After FnMut: value = {}", value); // 3

    println!("--- Используем как FnOnce ---");
    
    // Ключевое слово move заставляет замыкание захватывать все используемые 
    // переменные не по ссылке (& или &mut), а по значению (чего?)
    let my_closure_once = move || {
        println!("Final value inside FnOnce: {}", value);
        // value в замыкании move = 3
    };

    call_fn_once(my_closure_once);
    // После вызова FnOnce, переменная value перемещена в замыкание
    // println!("{}", value); 
    // нельзя использовать, так как move забрал владение
}

pub fn borrow_move_closure() {
        let s = String::from("Rust");

    // Захват по ссылке (обычное замыкание)
    let borrow_closure = || {
        println!("Borrowed: {}", s); // использует ссылку на s
    };
    borrow_closure();
    println!("После borrow_closure: {}", s); // s всё ещё доступна

    // Из курса: "захват по значению (move)"
    // И при наведении || показывает:
    // Capture a closure's environment by value.
    // move converts any variables captured by reference or mutable reference to variables captured by value.
    
    // (!) На самом деле это захват с передачей владения "move closure"
    // Для примитивов - будут скопированы и доступны дальше
    // Но для не примитивов (например, для String, которые не реализуют Copy) - будет захват владения

    let move_closure = move || {
        println!("Moved: {}", s); // берёт владение s
    };

    move_closure();
    // println!("После move_closure: {}", s); // ❌ s больше недоступна``
}



/*** Пример с типом fn - указатель на функцию ***/
fn add(x: i32, y: i32) -> i32 {
    x + y
}

fn multiply(x: i32, y: i32) -> i32 {
    x * y
}

// Функция принимает ссылку на функцию (fn type)
fn calculate_with_fn(op: fn(i32, i32) -> i32, a: i32, b: i32) -> i32 {
    op(a, b)
}

// Функция принимает любой Fn-трейт (более гибко)
fn calculate_with_trait<F>(op: F, a: i32, b: i32) -> i32 
where 
    F: Fn(i32, i32) -> i32 
{
    op(a, b)
}

pub fn calculate_differently() {
    // Оба способа работают с обычными функциями
    println!("{}", calculate_with_fn(add, 2, 3));      // 5
    println!("{}", calculate_with_trait(add, 2, 3));   // 5
    
    // Но только trait-версия работает с замыканиями
    let closure = |x, y| x + y + 1;
    
    // calculate_with_fn(closure, 2, 3);    //  Ошибка!
    println!("{}", calculate_with_trait(closure, 2, 3)); // 6
    
    let increment: fn(i32) -> i32 = |x| x + 1;
    println!("Closure result: {}", increment(3));
}

// Дженерик-функтор с конкретными типами
fn transform<F>(value: i32, func: F) -> String 
where 
    F: FnOnce(i32) -> String 
{
    func(value)
}

// Полностью дженерик функтор
// fn apply<T, U, F>(value: T, func: F) -> U 
// where 
//     F: FnOnce(T) -> U 
// {
//     func(value)
// }

// F может быть замыканием, обычной функцией или любым типом с Fn
fn apply<T, F>(value: T, func: F) -> T 
where 
    F: FnOnce(T) -> T  // <-- ограничение по Fn-трейту
{
    func(value)  // вызываем как функцию
}

pub fn generic_functor() {
    // Передаём замыкание в дженерик функтор
    let result = apply(5, |x| x * 2);  // 10
    
    // Или обычную функцию
    fn double(x: i32) -> i32 { x * 2 }
    let result2 = apply(5, double);    // 10
}


// Разберём пример реализации простой обёртки итератора MyMap. Она:
//    - принимает любой итератор (Iterator);
//    - применяет к каждому элементу переданную функцию (FnMut);
//    - возвращает новый итератор с преобразованными элементами.

struct MyMap<I, F> {
    iter: I,
    f: F,
}

impl<I, F, B> Iterator for MyMap<I, F>
where
    I: Iterator,
    F: FnMut(I::Item) -> B,
{
    type Item = B;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(&mut self.f)
    }
}

pub fn map_iterator() {
    let nums = vec![1, 2, 3, 4, 5];

    // создаём свой map
    let doubled: Vec<_> = MyMap {
        iter: nums.into_iter(),
        f: |x| x * 2,
    }
    .collect();

    println!("{:?}", doubled); // [2, 4, 6, 8, 10]
}  

// По сути, это аналог Iterator::map. Более подробно мы рассмотрим итераторы в следующих уроках.

// Принимает число n и возвращает замыкание, прибавляющее n к любому аргументу.
fn make_adder(n: i32) -> impl Fn(i32) -> i32 {
  move|x| -> i32 {x + n}
}

// В Rust функция может возвращать замыкание через impl Fn(...) -> ...
// Замыкание захватывает переменную из окружающей области (n).
// Замыкание можно сохранить в переменной и вызывать как обычную функцию.

pub fn adders_creation() {
    let add5 = make_adder(5);
    let add10 = make_adder(10);

    println!("{}", add5(3));  // 8
    println!("{}", add10(3)); // 13
} 


/*** Выбор Fn-трейта, инструкция с курсов ***/

// Шаг 1: Определите, сколько раз будет вызываться функция. Если один раз — можно использовать FnOnce:

// fn execute_task<F>(task: F) -> String
// where
//     F: FnOnce() -> String
// {
//     task() // Вызываем только один раз
// } 

//  Если много раз — нужен FnMut или Fn.

// fn repeat_action<F>(action: F, times: usize)
// where
//     F: Fn() // Вызываем много раз
// {
//     for _ in 0..times {
//         action();
//     }
// } 

// Шаг 2: Определите, нужно ли изменять состояние. Если замыкание должно изменять состояние — используйте FnMut.

// fn accumulate<F>(mut accumulator: F, values: Vec<i32>) -> i32
// where
//     F: FnMut(i32) -> i32
// {
//     let mut result = 0;
//     for value in values {
//         result = accumulator(value);
//     }
//     result
// } 

// Для удобства выбора мы собрали таблицу — по ней наглядно понятна разница между трейтами: 
// Трейт     	Владение	                    Изменение	Кол-во вызовов
// Fn	        по ссылке (&T)	                нет	        многократно
// FnMut	    по изменяемой ссылке (&mut T)	да	        многократно
// FnOnce	    по значению (T)             	да	        один раз