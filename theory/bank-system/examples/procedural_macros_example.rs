// Здесь только toSql

use my_macros::{say_hello, ToSql};

#[derive(ToSql)]
struct User {
    id: i32,
    name: String,
    age: i32,
}

fn main() {
    // proc_macro
    say_hello!("Привет из процедурного макроса!");

    // proc_macro2
    let user = User {
        id: 1,
        name: "Alice".into(),
        age: 30,
    };
    
    println!("{}", user.to_sql("users"));
} 

// > cargo expand --example procedural_macros_example 

// Вывод:
// #![feature(prelude_import)]
// #[macro_use]
// extern crate std;
// #[prelude_import]
// use std::prelude::rust_2024::*;
// use my_macros::say_hello;
// fn main() {
//     {
//         ::std::io::_print(
//             format_args!(
//                 "{0}\n",
//                 "Привет из процедурного макроса!",
//             ),
//         );
//     };
// }


// Два типа макросов:
// Декларативные макросы (macro_rules!) позволяют задавать правила подстановки кода по определённым шаблонам. Они используются для создания DSL (языков предметной области) — например, vec![1,2,3].
// Процедурные макросы принимают токены на вход и генерируют новый код. Они бывают трёх типов:

//     Функциональные (#[proc_macro]) — работают как функции, создающие код.
//     Атрибутные (#[proc_macro_attribute]) — модифицируют существующие элементы.
//     Derive-макросы (#[proc_macro_derive]) — автоматически реализуют трейты для структур и enum-ов.