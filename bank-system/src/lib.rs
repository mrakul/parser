// (!) Очень важно, что файл называется lib.rs, это жёсткая привязка

// pub — доступно везде, где виден модуль.
//       pub fn hello() {} 

// pub(crate) — доступно только в пределах текущего crate: проекта/библиотеки. Удобно, если вы пишете библиотеку и хотите ограничить внутренние детали.
//              pub(crate) fn internal_api() {} 

// pub(super) — доступно только в родительском модуле. Это как «передать наружу ровно на один уровень выше».
// pub(in some::path) — доступно только в указанном модуле и его потомках. Максимально тонкая настройка области видимости.

pub mod storage;

// Так можно не указывать storage
pub use storage::Storage;
pub use storage::Name;
pub use storage::Balance;
pub use storage::Operation;
pub use storage::Operation::{Deposit, Withdraw, CloseAccount};