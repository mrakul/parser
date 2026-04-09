// Коллекции в стандартной библиотеке Rust можно условно разделить на несколько категорий задач:

//     Последовательности: Vec, VecDeque.
//     Отображения: BtreeMap, BtreeMap.
//     Множества: BtreeSet, HashSet.

// По этому критерию (память/производительность) коллекции делятся так:

//     Минимальное потребление памяти: Vec, VecDeque, String.
//     Самобалансирующиеся деревья: BtreeMap, BtreeSet.
//     Хеш-таблицы: HashMap, HashSet.

/*** Vec ***/

pub fn vec_example() {
    // У всех коллекций есть операция созданий 'пустой' коллекции через метод new()
    let mut currencies = Vec::new();

    // Для быстрого создания Vec в Rust есть макрос
    currencies = vec!["ru", "usd", "gbr", "chf"];

    println!("Исходный вектор: {:?}", currencies);

    // Vec реализует Deref от типа slice, как и array,
    // который мы прошли в уроке 'типы данных', а
    // значит, с ним сразу же можно работать как с массивом
    let top2: &[&str] = &currencies[..2];

    println!("Первые валюты: [{}, {}]", top2[0], top2[1]);

    // Когда мы достаём элементы - возвращается Option<T>
    // .pop()
    let Some(last_currency) = currencies.pop() else {
        panic!("Нет валютs!")
    };

    println!("Удалили валюту: '{}'", last_currency);

    // Добавление в конец: .push(arg)
    currencies.push("cny");

    // Методами slice можно пользоваться напрямую, например,
    // получить первый элемент (вернётся Optional<T>)..
    println!("Первая валюта .first(): '{}'", currencies.first().unwrap()); // .last() тоже есть
    println!("После модификаций: {:?}", currencies);

    // ..или разбить мутабельный срез
    //   на два непересекающихся мутабельных среза
    let (first_slice, second_slice) = currencies.split_at_mut(3);
    first_slice[2] = "eur";

    // Или так изменять, проверка индекса
    if let Some(val_ref) = first_slice.get_mut(1) {
        // first_slice.get_mut(100) => выдаст panic!
        *val_ref = "curcur";
    }

    // Note:
    // if let opt = first_slice.get_mut(1) { => это сработает, но оно всегда match (опция всегда возвращается)

    println!("Первая валюта во втором слайсе: '{}'", second_slice[0]);

    // Добавление по индексу будет паниковать, если попытаться
    // вставить в позицию > .len()
    currencies.insert(2, "irr");
    println!(
        "После добавление currencies.insert(2, irr): {:?}",
        currencies
    );

    // К слову, .len() поддерживается во всех коллекциях, но это не метод трейта
    println!("Current supported currencies count: {}", currencies.len());

    // .clear() тоже есть, во всех динамических (это которые в куче данные держат) коллекциях
    currencies.clear();
    assert!(currencies.is_empty());

    // При создании вектора можно выделить ему место заранее
    let mut preallocated_vec: Vec<u32> = Vec::new();
    preallocated_vec.reserve(10);

    println!(
        "Preallocated массив, количество элементов: {}",
        preallocated_vec.len()
    );
    println!("Ёмкость: {}", preallocated_vec.capacity());
}

pub fn vec_deq_example() {
    // Макрос вроде vec! есть только для вектора
    // Для остальных коллекций (и для вектора тоже) можно использовать .into()

    // Преобразование .into(), см.предыдущий урок
    let mut tasks: std::collections::VecDeque<_> = ["register", "pay", "pay", "refund"].into();

    tasks.push_back("withdraw");
    tasks.push_front("due-diligence");

    println!(
        "Исходный VecDeque {:?} Capacity: {}, Len: {}",
        tasks,
        tasks.capacity(),
        tasks.len()
    );

    println!("the task after next: {}", tasks.get(1).unwrap());
    println!("total tasks count: {}", tasks.len());

    println!("undo last task: {}", tasks.pop_back().unwrap());
    println!("undo first task: {}", tasks.pop_front().unwrap());

    println!(
        "Исходный VecDeque после преобразований{:?} Capacity: {}, Len: {}",
        tasks,
        tasks.capacity(),
        tasks.len()
    );

    // Поменять front через mut-ссылку + unwrap
    *tasks.front_mut().unwrap() = "prepare";

    assert_eq!(tasks.front().unwrap(), &"prepare");

    println!(
        "Окончательный VecDeque {:?} Capacity: {}, Len: {}",
        tasks,
        tasks.capacity(),
        tasks.len()
    );
}

/*** BtreeSet */

// Надо их подключить
use std::collections::BTreeSet;

// Самобалансирующиеся деревья со своими структурами - не проблема

// Но свой тип должен реализовывать Eq. А Eq зависит от PartialEq.
// Eq - про равенство. Для порядка в дереве аналогично нужны PartialOrd и Ord

#[derive(Ord, PartialOrd, PartialEq, Eq)]
struct Subscription {
    user: &'static str,
    service: &'static str,
}

pub fn btreeset_example() {
    let mut students_class_a: BTreeSet<&'static str> = ["Bob", "Alice"].into();
    let next_student = "Eva";

    // Добавление: .insert() вернёт, был ли элемент Новым
    if students_class_a.insert(next_student) {
        println!("Добавлен новый пользователь: {}", next_student);
    } else {
        println!("Пользователь существует: {}", next_student);
    }

    // Второе дерево
    let mut students_class_b = BTreeSet::new();

    students_class_b.insert("Bob");

    // Получить между классами A и B
    for in_a_but_not_in_b in students_class_a.difference(&students_class_b) {
        println!("'{}' не в классе B", in_a_but_not_in_b);
    }

    // Элементы отсортированы по возрастанию, в каком бы порядке ни добавлялись
    println!(
        "По сортировке первый: {}",
        students_class_a.first().unwrap()
    );
    println!(
        "По сортировке последний: {}",
        students_class_a.last().unwrap()
    );
    assert_eq!(students_class_a.first().unwrap(), &"Alice");

    // Удаление
    students_class_a.remove("Bob");
    assert!(!students_class_a.contains("Bob"));

    // Вытащить (и удалить) последний по сортировке
    students_class_a.pop_last();
    assert!(!students_class_a.contains("Eva"));

    // Вытащить (и удалить) первый по сортировке
    students_class_a.pop_first();
    assert!(students_class_a.is_empty());

    // Для структуры с двумя полями:
    // Оба поля участвуют в сортироке, как и обычно
    // Длина таким же образом получается: .len()
    let mut subscriptions = BTreeSet::new();
    subscriptions.insert(Subscription {
        user: "he",
        service: "practicum",
    });
    subscriptions.insert(Subscription {
        user: "she",
        service: "practicum",
    });
    // Это другое пользователь, по service будет сравнение
    subscriptions.insert(Subscription {
        user: "he",
        service: "other",
    });
    assert_eq!(subscriptions.len(), 3);

    // Проверка наличия
    assert_eq!(
        subscriptions.contains(&Subscription {
            user: "he",
            service: "practicum"
        }),
        true
    );
}

/*** BTreeMap ***/

use std::collections::btree_map::Entry;
use std::collections::BTreeMap;

#[derive(Debug)]
struct User {
    email: &'static str,
    name: &'static str,
}

pub fn btreemap_example() {
    let mut users_bmap = BTreeMap::new();

    users_bmap.insert(
        "User #1",
        User {
            email: "rust@yandex.ru",
            name: "Victor",
        },
    );
    users_bmap.insert(
        "User #2",
        User {
            email: "edu@yandex.ru",
            name: "Tesla",
        },
    );

    assert!(users_bmap.contains_key("User #1"));

    println!("Исходный BTreeMap {:?}", users_bmap);

    for user in &users_bmap {
        println!("Пользователь: {} {:?}", user.0, user.1);
    }

    // У BTreeMap есть интересный метод entry() - в него можно передать ключ
    // и сразу же вставить элемент, не выполняя поиск в коллекции повторно
    match users_bmap.entry("Other") {
        Entry::Vacant(v) => {
            v.insert(User {
                email: "other@mail.ru",
                name: "Bin",
            });
        }

        Entry::Occupied(o) => {
            println!("existing email: {}", o.get().email);
        }
    }

    // После добавления
    println!("После добавления BTreeMap {:?}", users_bmap);

    // Длина
    assert_eq!(users_bmap.len(), 3);

    // Получить значение, связанное с ключом в виде Option
    assert!(users_bmap.get("User #1").is_some());
    // Можно проверить .is_none()
    assert!(users_bmap.get("User NotExist").is_none());

    // Поменять значение через мутабельную ссылку (только e-mail)
    users_bmap.get_mut("User #1").unwrap().email = "new-edu@yandex.ru";
    assert_eq!(
        users_bmap.get("User #1").unwrap().email,
        "new-edu@yandex.ru"
    );

    // Проверка наличия
    assert_eq!(users_bmap.contains_key("User #1"), true);
}

/*** Strings ***/

pub fn string_example() {
    // Перевод литерала времени компиляции str в контейнер String
    let mut s1 = "hello, world!".to_string();

    println!("Исходная строка: {}", s1);

    // Вставка в _байтовую_ позицию 0 новой строчки
    s1.insert_str(0, "Super ");
    // Вытащить последнюю букву
    s1.pop();

    assert_eq!(s1, "Super hello, world");

    s1.replace_range(13.., "student");
    s1.push_str(" of Practicum");
    assert_eq!(s1, "Super hello, student of Practicum");

    let mut s2 = String::from("Добрый день!");
    println!("Исходная строка: {}", s2);

    // 22 - в байтах! Будет паника, если попасть на 'середину' UTF-8 символа
    assert_eq!(s2.len(), 22); // длина - в байтах!

    // ' ' и '!' - по одному байту, русские буквы - по 2 байта

    let str_str = "Test";
    println!("Строка {} типа &str имеет длину {}", str_str, str_str.len());

    // Когда речь заходит о не английском тексте - полагаться на индексы нельзя
    // Мы воспользуемся методом str find(), который вернёт позицию в байтах для символа
    // Под капотом find() ходит по итератору chars(). Итераторы рассмотрим в следующем уроке

    // (!!!) .find() возвращает количество именно байтов
    let new_len = s2.find(' ').unwrap();

    println!("Урезать до позиции байта {}", new_len);
    // s2.truncate(s2.find(' ').unwrap());     // => Найти ' ', взять индекс и урезать до этого места
    s2.truncate(new_len);
    println!("Строка после преобразования: {}", s2);

    // (!) Важно:
    // 1. Для пришедших из C/C++ важное замечание: это не Си-строки, нулевого байта на конце там нет!
    // Для таких строк следует использовать std::ffi::CString, который в new() принимает значимые байты — и сам добавляет ‘\0’ в конце.

    // 2. Позицию символов использовать нельзя: s2[2] ❌ Нет
    // Хотя String использует Vec<u8> внутри, индексация по байтам (s[i]) может вызвать панику при не-ASCII символах.

    // 3. Замена символа может изменить количество байт (например, 'a' → '🚀'), что требует реаллокации.
    // Даже замена символа на символ той же длины в UTF-8 не поддерживается напрямую.
}

/*** HashSet, HashMap ***/

// В Rust внедрена реализация хеш-таблицы с открытой адресацией Swisstable и хеш-функцией SipHash,
// но Rust оставляет за собой право поменять реализацию, когда будут изобретены более совершенные реализации.

use std::collections::hash_map;
use std::collections::{HashMap, HashSet};

// Для использования своего типа нужно реализовать Hash
#[derive(Hash, PartialEq, Eq, Debug)]
struct Transaction {
    country: &'static str,
    id: u32,
}

pub fn hashset_example() {
    // HashSet
    let mut countries_hashset = HashSet::new();

    countries_hashset.insert(Transaction {
        country: "ru",
        id: 1,
    });
    countries_hashset.insert(Transaction {
        country: "kz",
        id: 1,
    });

    println!("HashSet countires: {:?}", countries_hashset);

    assert_eq!(countries_hashset.len(), 2);

    // HashMap
    let mut ballances: HashMap<_, _> = [("user0", 0), ("user1", 500)].into();

    match ballances.entry("user3") {
        hash_map::Entry::Vacant(v) => {
            v.insert(10000);
        }

        hash_map::Entry::Occupied(mut o) => {
            *o.get_mut() = 300;
        }
    }

    println!("HashMap ballances: {:?}", ballances);

    // Access by mutable reference + unwrap()
    *ballances.get_mut("user1").unwrap() = 200;

    assert_eq!(ballances.get("user1").unwrap(), &200);

    println!("HashMap ballances after changes: {:?}", ballances);
}

// Два главных отличия BTree* от Hash*:
// BTree хранит элементы отсортированными, Hash — нет.
// При учёте сложности BTree используют ради O(ln(n)) в худшем случае, когда сложность O(n) недопустима. Hash используют ради среднего O(1), когда редкими O(n) можно пренебречь.

/*** BinaryHeap ***/

pub fn binaryheap_example() {
    let mut heap = std::collections::BinaryHeap::new();

    heap.push(3);
    heap.push(4);
    heap.push(1);
    heap.push(5);
    heap.push(2);
    heap.push(4);

    // Проверяем, что вытаскивается всегда максимальынй элемент
    assert_eq!(heap.pop(), Some(5));
    assert_eq!(heap.pop(), Some(4));
    assert_eq!(heap.pop(), Some(4));
    assert_eq!(heap.pop(), Some(3));
    assert_eq!(heap.len(), 2);

    // Поменять значение на вершине (?)
    *heap.peek_mut().unwrap() = 0; // Тут хитро, что вершина поменяется на 1
    assert_eq!(heap.peek(), Some(&1));

    // Reverse-версия, тип выводится из того, как добавляем элементы ниже
    let mut min_heap = std::collections::BinaryHeap::new();

    min_heap.push(std::cmp::Reverse(3));
    min_heap.push(std::cmp::Reverse(2));
    assert_eq!(min_heap.peek(), Some(&std::cmp::Reverse(2)));

    min_heap.push(std::cmp::Reverse(4));
    assert_eq!(min_heap.peek(), Some(&std::cmp::Reverse(2)));

    min_heap.push(std::cmp::Reverse(1));
    assert_eq!(min_heap.peek(), Some(&std::cmp::Reverse(1)));
    assert_eq!(min_heap.pop().unwrap().0, 1);
}

// Практическое задание

// Возвращает массив из 128 элементов
fn make_key(word: &str) -> [usize; 128] {
    let mut key = [0usize; 128];

    for cur_char in word.chars() {
        key[cur_char as usize] += 1;
    }

    return key;
}

pub fn anagrammes() {
    let input = vec!["aba", "cab", "baa", "aab", "acb", "abc", "xyz", "yzx"];

    let mut groups: HashMap<[usize; 128], BTreeSet<&str>> = Default::default();

    // Идём по словам
    for word in input.iter() {
        // word имеет тип &str

        let key = make_key(word);

        match groups.entry(key) {
            std::collections::hash_map::Entry::Vacant(v) => {
                let mut group = BTreeSet::new();
                group.insert(*word);
                v.insert(group);
            }
            std::collections::hash_map::Entry::Occupied(mut o) => {
                o.get_mut().insert(*word);
            }
        }
    }

    let mut group_num = 0;

    for (_, words) in groups.iter() {
        println!("{}:", group_num);

        for word in words.iter() {
            println!("    {}", word);
        }

        group_num += 1;
    }
}
