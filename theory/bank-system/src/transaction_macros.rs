/*** Транзакции: трейты и дженерики ***/
/*
*   В этом файле генерация имплементаций с помощью макросов из crate my_macros.
*   Реализация вручную находится в transactions.rs
*/


use crate::storage::{*};
use crate::balance::{*};
use my_macros::Transaction;

pub trait Transaction {
    fn apply(&self, storage: &mut Storage) -> Result<(), TxError>;
} 

#[derive(Debug)]
pub enum TxError {
    InsufficientFunds,
    // Пока не используется, вставляется с дефолтным значением 0
    InvalidAccount,
}

// 1. Транзакция Deposit
#[derive(Transaction)]
// тут не нужно указывать #[transaction("deposit")], так как это значение по умолчанию
pub struct Deposit {
    pub from_account: String,
    pub amount: i64,
}

// 2. Транзакция Withdraw
#[derive(Transaction)]
#[transaction("withdraw")]
pub struct Withdraw {
    pub from_account: String,
    pub amount: i64,
}

// 3. Транзакция Transfer
#[derive(Transaction)]
#[transaction("transfer")]
pub struct Transfer {
    pub from_account: String,
    pub to_account: String,
    pub amount: i64,
} 


// Переопределение "+"

pub struct TxCombinator<T1, T2>
where
    T1: Transaction,
    T2: Transaction,
{
   pub t1: T1,
   pub t2: T2,
}

// Реализация apply для двух транзакций
impl<T1: Transaction, T2: Transaction> Transaction for TxCombinator<T1, T2> {
    fn apply(&self, storage: &mut Storage) -> Result<(), TxError> {
        self.t1.apply(storage)?;
        self.t2.apply(storage)?;
        Ok(())
    }
} 

// Реализация Add для всех вариантов:

// Реализация Add для Deposit + Transfer
// impl std::ops::Add<Transfer> for Deposit {
//     type Output = TxCombinator<Deposit, Transfer>;

//     fn add(self, rhs: Transfer) -> Self::Output {
//         TxCombinator { t1: self, t2: rhs }
//     }
// }

// impl std::ops::Add<Deposit> for Transfer {
//     type Output = TxCombinator<Transfer, Deposit>;

//     fn add(self, rhs: Deposit) -> Self::Output {
//         TxCombinator { t1: self, t2: rhs }
//     }
// }

// // Реализация Add для Deposit + Deposit
// impl std::ops::Add<Deposit> for Deposit {
//     type Output = TxCombinator<Deposit, Deposit>;

//     fn add(self, rhs: Deposit) -> Self::Output {
//         TxCombinator { t1: self, t2: rhs }
//     }
// }

// // Реализация Add для Transfer + Transfer
// impl std::ops::Add<Transfer> for Transfer {
//     type Output = TxCombinator<Transfer, Transfer>;

//     fn add(self, rhs: Transfer) -> Self::Output {
//         TxCombinator { t1: self, t2: rhs }
//     }
// }


/*** Макросы ***/

// (!) Или с помощью макроса все варианты
#[macro_export]
macro_rules! impl_add {
    ( $( ($lhs:ty, $rhs:ty) ),* ) => {
        $(
            impl std::ops::Add<$rhs> for $lhs {
                type Output = $crate::TxCombinator<$lhs, $rhs>;

                fn add(self, rhs: $rhs) -> Self::Output {
                    $crate::TxCombinator { t1: self, t2: rhs }
                }
            }
        )*
    };
} 

// Теперь никаких вручную объявленных трейтов для оператора Add — достаточно написать макрос, а остальной код оставить как было. 

// transaction.rs

impl_add! {
    (Deposit, Transfer),
    (Transfer, Deposit),
    (Deposit, Deposit),
    (Transfer, Transfer)
} 



// Декларативный макрос
#[macro_export]
macro_rules! tx_chain {
    ( $first:expr $(, $rest:expr )* $(,)? ) => {{
        let tx = $first;
        $(
            let tx = $crate::transaction_macros::TxCombinator{ t1: tx, t2: $rest };
        )*
        tx
    }};
}

