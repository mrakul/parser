use bank_system::Storage;
use bank_system::transaction_macros::{*};

// Задание #1
macro_rules! say_hello {
    () => {
        println!("Hello, world!");
    };
}

macro_rules! say {
    ($msg:expr) => {
        println!("{}", $msg);
    };
} 

fn main() {
    let mut storage = Storage::new();
    storage.add_user("Alice".into());
    storage.add_user("Bob".into());

    let tx_chain_variable = bank_system::tx_chain!(
        Deposit{from_account: "Alice".into(), amount: 500},
        Transfer{from_account: "Alice".into(), to_account: "Bob".into(), amount: 50},
        Withdraw{from_account: "Alice".into(), amount: 100}
    );

    // Тип переменной `tx` будет таким:
    //
    // TxCombinator<
    //     Deposit,
    //     TxCombinator<
    //         Transfer,
    //         Withdraw
    //     >
    // >
    //
    // То есть макрос раскладывает цепочку транзакций
    // в дерево вложенных TxCombinator'ов.

    println!("Выполняем транзакции через макрос...");
    match tx_chain_variable.apply(&mut storage) {
        Ok(_) => println!("Успешно"),
        Err(e) => println!("Ошибка: {:?}", e),
    }

    println!("Итоговые балансы:");
    for (name, balance) in storage.get_all() {
        println!("{} -> {:?}", name, balance);
    }

    // Проверка макросов выше:
    say_hello!();
    say!("Test output");
} 