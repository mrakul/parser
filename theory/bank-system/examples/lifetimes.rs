// struct Holder {
//     // В структурах и перечислениях все ссылки нужно аннотировать временем жизни
//     // Так, здесь мы ограничили время жизни данных, на которые ссылается dataref,
//     // концом жизни программы
//     dataref: &'static str,
// }
// fn main() {
//     let owned = String::from("hello, world");
    
//     // let holder = Holder{dataref: &owned}; // error[E0597]: `value` does not live long enough
//     // `owned` dropped here while still borrowed
//     // ( текст ошибки может отличаться в другой версии
//     //   компилятора или rust-analyzer )

//     // А это работает, потому что ссылка ссылка на &'static str
//     let static_str: &'static str = "Test string";
//     let holder = Holder{dataref: &static_str};

//     // И даже так - строковый литерал живёт в RO-области (сколько и программа)
//     let local_str = "Test string without static";
//     let holder = Holder{dataref: &static_str};
// }

/*** Указание времени жизни в явном виде ***/
// struct Holder<'a> {
//     dataref: &'a str,
// }
// // Lifetime 'a inferred as "at least as long as `holder` exists

// fn main() {
//     let owned = String::from("hello, world");
//     let holder = Holder{dataref: &owned};
// } 


// Здесь явные лайфтаймы уже необходимы
// Мы говорим компилятору, как связаны лайфтаймы входа и выхода, а компилятор - проверяет
fn get<'a>(s1: &'a str, s2: &str) -> &'a str {
    s1
}

// В перечислениях могут быть ссылки -> в перечислениях используются лайфтаймы

// (!) Выражение struct User<'a>{name: &’a str} связывает время жизни ссылки внутри User с переменной типа str. 
// Оно не позволяет изменять этот str или брать на str мутабельную ссылку до тех пор, пока жив User
enum Kind<'a> {
    Holder(&'a str),
}

// Лайфтаймы могут зависеть друг от друга!
// Например, мы можем для замены в Kind::Holder использовать ссылку с другим лайфтаймом
// - главное, чтобы значение, на которое она указывает, жило не меньше, чем жило значение предыдущей ссылки
fn set<'shorter, 'same_or_longer: 'shorter>(kind: &mut Kind<'shorter>, value: &'same_or_longer str) {
    match kind {
        Kind::Holder(myref) => *myref = value,
    }
}

fn main() {
    let s1 = String::from("hello");
    let s2 = String::from("world");

    
    let some = get(&s1, &s2);
    println!("got: {}", some);
    
    let s3 = "Test"; 
    let mut holder = Kind::Holder(&s1);
    set(&mut holder, &s2);
} 