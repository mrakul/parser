/*** Теория ***/

// pub trait Iterator {
//     // Каждая коллекция должна реализовать самостоятельно..
//     type Item;
//     fn next(&mut self) -> Option<Self::Item>;

//     // ..и этого будет достаточно, чтобы автоматически предоставить 60+
//     //   вспомогательных методов - только в стабильном Rust! И
//     //   несколько методов в nightly
//     fn all<F>(&mut self, f: F) -> bool
//     where
//         Self: Sized,
//         F: FnMut(Self::Item) -> bool,
//     {
//         // ..универсальная реализация all()..
//     }
//     // ...
//     // ..60+ методов..
//     // ...
//     fn zip<U>(self, other: U) -> Zip<Self, <U as IntoIterator>::IntoIter>
//     where
//         Self: Sized,
//         U: IntoIterator,
//     {
//         // ..универсальная реализация zip()..
//     }
// }

// Обратите внимание на next() — главный метод трейта. Его предназначение — вернуть следующий элемент:

//     Some, если элементы остались;
//     None — если нет.

// Метод next():

//     Принимает мутабельную ссылку на self, чтобы прогрессировать: обновить внутреннее состояние и в следующий раз вернуть следующий элемент.
//     Возвращает Option<Self::Item>, то есть None, если элементов больше не осталось. Или Some(Self::Item) — если ещё есть элемент, который до сих пор не посетили.

// Quiz: .iter(), .into_iter(), .iter_mut()

// У вас был цикл, но элементы понадобилось дополнительно пофильтровать или преобразовать. На что будете менять источник данных?
// for element in &collection
// .iter() заимствует элементы, а не забирает их себе: элементы доступны для чтения, но менять их нельзя.

// for element in collection
// .into_iter() забирает коллекцию себе, коллекцией больше пользоваться нельзя. Поскольку мы теперь элементами владеем в цикле, их можно менять — надо только добавить mut: for mut element in collection.into_iter().

// for element in &mut collection
// .iter_mut() заимствует элементы мутабельно. Он не забирает элементы себе, но менять их нельзя. Когда мы заимствуем мутабельно, накопленными к этому моменту ссылками на коллекцию и её элементы больше пользоваться нельзя.

// Из Qwen'а:
// Method           What You Get                Ownership Effect                              When to Use
// .iter()        &T (immutable reference)    ✅ Collection stays usable                      Read-only access (most common)
// .into_iter()   T (owned value)             ❌ Collection consumed (moved)                  "Taking ownership (e.g., transforming values)"
// .iter_mut()    &mut T (mutable reference)  ✅ Collection stays usable (mutably borrowed)   Modify elements in place

// Аналоги:

// for x in &vec       → vec.iter()
// for x in &mut vec   → vec.iter_mut()
// for x in vec        → vec.into_iter()

/*** Практика итераторов ***/

fn is_not_odd(a: &u32) -> bool {
    a % 2 != 0
}

pub fn iterators_example() {
    // 1. any(), .all()

    let mut nums = vec![2u32, 4, 6];

    // Можно передавать замыкание,..
    assert_eq!(nums.iter().all(|n| n % 2 == 0), true);

    // ..а можно - указатель на функцию
    assert_eq!(nums.iter().any(is_not_odd), false);

    // Вставили нечётное число - поменялись проверки на противоположные
    nums.insert(2, 3u32);

    assert_eq!(nums.iter().all(|n| n % 2 == 0), false);
    assert_eq!(nums.iter().any(is_not_odd), true);

    let symbols_vec = vec!["a", "3"];
    // let symbols_vec = ["a", "3"];                      // А просто массив не будет moved, поскольку реализует Copy

    for mut symbol in symbols_vec.into_iter() {     // symbols_vec is moved (!) and can't be used further
        symbol = "z";
        println!("test {} ", symbol);
    }

    // println!("{}", symbols_vec.len());

    // 2.
    // .map() создаёт итератор, в котором текущий элемент превращается во что-нибудь другое.
    // .enumerate() создаёт итератор, который возвращает элементы в виде (порядковый_номер_текущего_элемента, текущий_элемент).
    // метод .collect() собирает элементы из итератора в новую коллекцию.

    let nums = vec![0, 5, 10, 15];
    let mut it_into_x10 = nums.iter().map(|n| n * 10);
    
    // Работает на той же коллекции
    assert_eq!(it_into_x10.next(), Some(0));
    assert_eq!(it_into_x10.next(), Some(50));
    assert_eq!(it_into_x10.next(), Some(100));
    assert_eq!(it_into_x10.next(), Some(150));
    assert_eq!(it_into_x10.next(), None);

    let mut it_enumerated = nums.iter().enumerate();
    assert_eq!(it_enumerated.next(), Some((0, &0)));
    assert_eq!(it_enumerated.next(), Some((1, &5)));
    assert_eq!(it_enumerated.next(), Some((2, &10)));

    // Взять элементы, умножить на 10, взять пару-tuple и поместить в вектор пар
    // (!) Возвращает уже коллекцию
    let it_collected = nums.iter().map(|n| n * 10).enumerate().collect::<Vec<_>>();
    assert_eq!(it_collected, vec![(0, 0), (1, 50), (2, 100), (3, 150)]);

    // Поместить в ту же коллекци (?)
    
    // Какой-то хитрый случай
    // Arrays don't implement Iterator directly — .into_iter() converts the array into an iterator that yields the correct tuple type
    let kv = [("key0", "value0"), ("key1", "value1")];
    let collected = kv.into_iter().collect::<std::collections::HashMap<_, _>>();

    assert_eq!(collected,std::collections::HashMap::from([("key0", "value0"), ("key1", "value1")]));

    // 3. Метод count(), адаптеры chain() и zip(), метод unzip()
    //     Метод count() выдаёт количество элементов. 
    //     Итератор при этом будет потерян: вызовами next() он пройдёт через все Some(_), пока не дойдёт до None.
    //     Адаптер chain() склеивает два итератора с элементами одного типа в один итератор.
    //     Адаптер zip() и метод unzip(). 
    //     zip() склеивает два итератора в итератор по кортежу. Последовательности выравниваются по началу: если одна окажется длиннее другой, то лишние элементы будут просто отброшены и вместо них вернётся None. 
    //     unzip() превращает итератор по кортежам из 2 элементов в 2 коллекции из этих элементов.

    use std::collections::BTreeSet;

    let ordinary_transactions = vec![0, 100, 200];
    let promotion_transactions = BTreeSet::from([1, 150]);

    // Для .chain() нужен один тип

    // TODO: написать
    let mut all_transactions = ordinary_transactions.iter().chain(promotion_transactions.iter());
    
    assert_eq!(all_transactions.clone().count(), 5);        // Здесь делаем .clone(), поскольку итератор "потерян", дошли до конца

    assert_eq!(all_transactions.next(), Some(&0));
    assert_eq!(all_transactions.next(), Some(&100));
    assert_eq!(all_transactions.next(), Some(&200));
    assert_eq!(all_transactions.next(), Some(&1));
    assert_eq!(all_transactions.next(), Some(&150));
    assert_eq!(all_transactions.next(), None);

    let users_queue = ["user2", "user1", "user2"];
    let operations_queue = vec![200, 100, 222];
    
    let zipped = users_queue.into_iter().zip(operations_queue.into_iter());
    let mut to_show = zipped.clone();

    // zip две коллекции - array и вектор
    assert_eq!(to_show.next(), Some(("user2", 200)));
    assert_eq!(to_show.next(), Some(("user1", 100)));
    assert_eq!(to_show.next(), Some(("user2", 222)));
    assert_eq!(to_show.next(), None);

    // unzip в BTreeSet - остаются только уникальные и отсортированные
    let (unique_users, operations): (BTreeSet<_>, Vec<_>) = zipped.unzip();
    assert_eq!(unique_users, BTreeSet::from(["user1", "user2"]));
    // assert_eq!(operations, vec![200, 100, 222] 

    // 4. Адаптеры skip(M), take(N) и step_by(K)
    // Продолжают перечисление после элемента номер M, используют только первые N элементов, перечисляют каждый K-й элемент:

    let decision_weights = [0, 1, 2, 3, 4].into_iter();
    let mut larges_weights = decision_weights.clone().skip(3);
    let mut smallest_weights = decision_weights.clone().take(3);
    let mut more_different_weights = decision_weights.clone().step_by(2);

    assert_eq!(larges_weights.next(), Some(3));
    assert_eq!(larges_weights.next(), Some(4));
    assert_eq!(larges_weights.next(), None);

    assert_eq!(smallest_weights.next(), Some(0));
    assert_eq!(smallest_weights.next(), Some(1));
    assert_eq!(smallest_weights.next(), Some(2));
    assert_eq!(smallest_weights.next(), None);

    assert_eq!(more_different_weights.next(), Some(0));
    assert_eq!(more_different_weights.next(), Some(2));
    assert_eq!(more_different_weights.next(), Some(4));
    assert_eq!(more_different_weights.next(), None); 

    // 5. Методы sum(), product(), max(), min()
    // Их названия говорят сами за себя: сложить элементы, перемножить, найти максимальный и минимальный элементы соответственно. Есть варианты:

    let nums_it = [3u32, 1, 4, 2].iter();

    assert_eq!(nums_it.clone().sum::<u32>(), 10);
    assert_eq!(nums_it.clone().product::<u32>(), 24);
    assert_eq!(nums_it.clone().max(), Some(&4));
    assert_eq!(nums_it.clone().min(), Some(&1));

    // max_by(F) и min_by(F), чтобы передать функцию сравнения F, которая вернёт std::cmp::Ordering — enum со значениями Less, Equal и Greater.
    // max_by_key(M) и min_by_key(M), чтобы передать отображение M, которое вернёт ключ, по которому можно сравнивать.

    let ages_and_names = [("Bob", 33), ("Alice", 25), ("Eva", 30)].into_iter();
    // cmp() - метод чисел, который мы можем использовать, чтобы не писать операцию с std::cmp::Ordering самим
    
    // Получить максимум по кастомной функции сравнения
    assert_eq!(ages_and_names.clone().max_by(|left, right| left.0.cmp(right.0)).unwrap(), ("Eva", 30));
    
    // Максимум по сравнению "ключей" - сравнивает по второму элементу, он больше у Боба.
    assert_eq!(ages_and_names.clone().max_by_key(|element| element.1).unwrap(), ("Bob", 33)); 

    // 6. cloned(), copied(), cycle() и fold()
    //  - cloned() и copied() — адаптеры, элементы которых склонированы и скопированы из оригинального итератора соответственно. 
    //    Доступны только для итераторов, элементы которых реализуют Clone и Copy соответственно. 
    //    Это требование обеспечивается механизмом trait bounds, который знаком вам из темы про трейты.
    //  - cycle() — адаптер, который позволяет замкнуть последовательность — после последнего элемента будет идти первый. Такой итератор никогда не вернёт None при вызове next(). Требует, чтобы итератор и его элементы удовлетворяли Clone.
    //  - fold() — адаптер, который позволяет накапливать результат по пользовательской функции. Очень гибкий инструмент.

    use std::collections::{HashSet};

    // (!!!) Это строки, поэтому их клонируем - deep copy
    let last_users_did_ops = vec!["Alice".to_string(), "Bob".into(), "Alice".into(), "Eva".into()];
    let unique = last_users_did_ops.iter().cloned().collect::<HashSet<_>>(); 

    // (!!!) Это строковые литералы, то есть указатели на них. Поэтому их КОПИРУЕМ, но не deep copy
    let last_currencies = vec!["BTC", "ETH", "XDG", "BTC"];
    let unique = last_currencies.iter().copied().collect::<HashSet<_>>();

    // .cycle
    let transaction_ops = ["pre_check", "apply", "post_check"];
    let mut user_state_cursor = transaction_ops.into_iter().cycle();

    // Зациклированный итератор
    assert_eq!(user_state_cursor.next(), Some("pre_check"));
    assert_eq!(user_state_cursor.next(), Some("apply"));
    assert_eq!(user_state_cursor.next(), Some("post_check"));
    assert_eq!(user_state_cursor.next(), Some("pre_check"));
    assert_eq!(user_state_cursor.next(), Some("apply"));

    // .fold()
    let payed_sums = [100u32, 200, 50, 300];
    // let platform_bonus_points = payed_sums.iter().fold(0, |next, collected| collected + next * 1.5 + 2);
    // println!("collected bonus for purchases: {}", platform_bonus_points); 

    // Адаптер filter() и методы find()/position()

    //     filter() принимает вызываемый объект, который из ссылки на аргумент принимает решение, оставлять ли этот элемент, когда у адаптера будет вызван метод next(). true, если оставлять, и false — если нет.
    //     find() принимает вызываемый объект, который также решает, является ли текущий элемент искомым.
    //     position() работает как find(), но если find() возвращает сам элемент, то position() возвращает его позицию.

    // Кстати, в стандартной библиотеке есть удобное сокращение для частой задачи фильтрации и отображения: filter_map() и find_map(). То есть вместо того, чтобы писать цепочки вида filter(Fn(&T1)->bool).map(Fn(T1)->T2) и Fn(find(T1)->bool)->T2, можно использовать filter_map(Fn(T1)->Option<T2>) и find_map(Fn(T1)->Option<T2).
    // position() для подсчёта использует next() итератора, на котором вызван этот position(). То есть если итератор будет обёрнут в filter(), исключённые элементы не будут участвовать в подсчёте. Но иногда бывает нужна позиция в исходном итераторе — например, чтобы получить индекс, по которому элемент можно найти в Vec. В таком случае можно на базовом итераторе использовать enumerate(), а в конце вместо position() — find_map():
    let numbers = [1, 2, 3, 4, 5, 21];

    let mut odd = numbers.iter().filter(|num| 0 == *num % 2);
    let mut odd_cloned_it = odd.clone();

    // assert_eq!(odd.next(), Some(&0));
    assert_eq!(odd.next(), Some(&2));
    assert_eq!(odd.next(), Some(&4));
    assert_eq!(odd.next(), None);

    // position() учитывает только склонированный (?) итератор!
    assert_eq!(odd_cloned_it.clone().position(|num| *num == 4), Some(1));
    assert_eq!(odd_cloned_it.clone().find(|num| *num % 3 == 0), Some(&10));

    let percents = [10i32, 15, 20, 40, 60, 80];
    // find_map позволяет не только проверять истинность, но и сразу преобразовывать результат
    // Дополнительно, здесь мы заиспользовали метод bool::then_some(V), который при истинности
    // bool возвращает Some(V), и None иначе

    // То есть find + map одновременно
    let larger_fraction = percents.iter().find_map(|v| (*v > 50).then_some(*v as f64 / 100.0));
    assert_eq!(larger_fraction, Some(0.6));

    let comparisons = [1.22, 1., 1.15, 2., 1., 0.8, 1.4];

    // То есть filter + map одновременно
    let mut differences = comparisons.iter().filter_map(|f| (*f != 1.).then_some(((f - 1.)*100.0) as i32));
    
    assert_eq!(differences.next(), Some(22));
    assert_eq!(differences.next(), Some(15));
    assert_eq!(differences.next(), Some(100));
    assert_eq!(differences.next(), Some(-20));
    assert_eq!(differences.next(), Some(40));
    assert_eq!(differences.next(), None); 

}
