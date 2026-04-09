/*** Транзакции: трейты и дженерики ***/

use crate::storage::{*};
use crate::balance::{*};

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
pub struct Deposit {
    // Get'теры, set'теры
    pub from_account: String,
    pub amount: i64,
}

impl Transaction for Deposit {    
    fn apply(&self, storage: &mut Storage) -> Result<(), TxError> {
        *storage.get_accounts_mut()
            // За один проход: возвращает или Occupoed, or_insert() это unwrap'ит
            // Или создаёт новый Balance. если нет записи
            // AddAssign реализовал в Balance
            .entry(self.from_account.clone())
            .or_insert(Balance::from(0)) += self.amount;
        
        Ok(())
    }
}

// 2. Транзакция Transfer
pub struct Transfer {
    pub from_account: String,
    pub to_account: String,
    pub amount: i64,
}

impl Transaction for Transfer {
    fn apply(&self, storage: &mut Storage) -> Result<(), TxError> {
        let from_balance = storage.get_accounts_mut().entry(self.from_account.clone()).or_insert(Balance::from(0));

        if from_balance.get_value() < self.amount {
            return Err(TxError::InsufficientFunds);
        }

        // SubAssign реализовал в Balance
        *from_balance -= self.amount;

        *storage.get_accounts_mut().entry(self.to_account.clone()).or_insert(Balance::from(0)) += self.amount;

        Ok(())
    }
} 

// 3. Транзакция Withdraw
pub struct Withdraw {
    pub from_account: String,
    pub amount: i64,
}

impl Transaction for Withdraw {    
    fn apply(&self, storage: &mut Storage) -> Result<(), TxError> {
        let from_balance = storage.get_accounts_mut().entry(self.from_account.clone()).or_insert(Balance::from(0));

        if from_balance.get_value() < self.amount {
            return Err(TxError::InsufficientFunds);
        }

        // SubAssign реализовал в Balance
        *from_balance -= self.amount;

        Ok(())
    }
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
            let tx = $crate::transaction::TxCombinator{ t1: tx, t2: $rest };
        )*
        tx
    }};
}

