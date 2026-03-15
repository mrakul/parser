/*** Важно: этот файл запустится с cargo run (по умолчанию), поскольку находится в папке ./bin ***/
/*** (сама библиотека не может запускаться) ***/

// use bank_system::Operation::Withdraw;
// use bank_system::balance::balance_manager::BalanceManager;
// use bank_system::users::user_manager::UserManager;
use bank_system::{Name, Storage};
use bank_system::transaction::{Transaction, Deposit, Transfer, Withdraw};
use std::io::{self, BufRead, Write};

fn main() {

    let file_path = String::from("../../aux/balance.csv");

    let mut storage = Storage::load_file_to_storage(file_path.as_str());

    println!("=== Bank CLI Utils ===");
    println!("Команды:");
    println!("  add <name> <balance>      - добавить пользователя");
    println!("  remove <name>             - удалить пользователя");
    println!("  deposit <name> <amount>   - пополнить баланс");
    println!("  transfer <from> <to> <amount>   - перевести деньги");
    println!("  withdraw <name> <amount>  - снять со счёта");
    println!("  balance <name>            - показать баланс");
    println!("  + (deposit + transfer     - несколько команд одновременно");
    println!("  exit                      - выйти");

    let stdin = io::stdin();
    let mut stdout = io::stdout();

    loop {
        print!("> ");
        stdout.flush().unwrap(); // показываем приглашение

        let mut input = String::new();
        if stdin.lock().read_line(&mut input).unwrap() == 0 {
            break; // EOF
        }

        let args: Vec<&str> = input.trim().split_whitespace().collect();
        if args.is_empty() {
            continue;
        }

        match args[0] {
            "add" => {
                if args.len() != 3 {
                    println!("Пример: add John 100");
                    continue;
                }
                let name: Name = args[1].to_string();
                let balance: i64 = match args[2].parse() {
                    Ok(b) => b,
                    Err(_) => {
                        println!("Сумма должна быть числом");
                        continue;
                    }
                };
                if storage.add_user(name.clone()).is_some() {
                    let _ = storage.deposit(&name, balance);
                    println!("Пользователь {} добавлен с балансом {}", name, balance);
                    storage.save_storage_to_file(file_path.as_str());
                } else {
                    println!("Пользователь {} уже существует", name);
                }
            }
            "remove" => {
                if args.len() != 2 {
                    println!("Пример: remove John");
                    continue;
                }
                let name = args[1];
                if storage.remove_user(&name.to_string()).is_some() {
                    println!("Пользователь {} удалён", name);
                    storage.save_storage_to_file(file_path.as_str());
                } else {
                    println!("Пользователь {} не найден", name);
                }
            }
            "deposit" => {
                if args.len() != 3 {
                    println!("Пример: deposit John 100");
                    continue;
                }

                let tx_name = args[1].to_string();

                let tx_amount: i64 = match args[2].parse() {
                    Ok(a) => a,
                    Err(_) => {
                        println!("Сумма должна быть числом");
                        continue;
                    }
                };
                
                // 1. Без использования Trait'а Transaction 
                // match storage.deposit(&name, amount) {
                //     Ok(_) => {
                //         println!("Баланс пользователя {} увеличен на {}", name, amount);
                //         storage.save_storage_to_file(file_path.as_str());
                //     }
                //     Err(e) => println!("Ошибка: {}", e),
                // }

                let deposit_tx = Deposit {
                    from_account: tx_name.clone(),
                    amount: tx_amount,
                };

                // 2. Применяем транзакцию 
                match deposit_tx.apply(&mut storage) {
                    Ok(_) => {
                        println!("Транзакция: депозит {} на {}", tx_name, tx_amount);
                        storage.save_storage_to_file(file_path.as_str());
                    }
                    // TxError
                    Err(e) => println!("Ошибка транзакции: {:?}", e),
                }
            }
            "transfer" => {
                if args.len() != 4 {
                    println!("Пример: transfer Alice Bob 50");
                    continue;
                }

                let tx_from_account = args[1].to_string();
                let tx_to_account = args[2].to_string();
                
                let tx_amount: i64 = match args[3].parse() {
                    Ok(a) => a,
                    Err(_) => {
                        println!("Сумма должна быть числом");
                        continue;
                    }
                };

                let transfer_tx = Transfer {
                    from_account: tx_from_account.clone(),
                    to_account: tx_to_account.clone(),
                    amount: tx_amount,
                };

                // Применяем транзакцию 
                match transfer_tx.apply(&mut storage) {
                    Ok(_) => {
                        println!("Транзакция: перевод от {} к {} на {}", tx_from_account, tx_to_account, tx_amount);
                        storage.save_storage_to_file(file_path.as_str());
                    }
                    // TxError
                    Err(e) => println!("Ошибка транзакции: {:?}", e),
                }

            } 
            "withdraw" => {
                if args.len() != 3 {
                    println!("Пример: withdraw John 100");
                    continue;
                }

                let tx_name = args[1].to_string();

                let tx_amount: i64 = match args[2].parse() {
                    Ok(a) => a,
                    Err(_) => {
                        println!("Сумма должна быть числом");
                        continue;
                    }
                };

                // 1. Без использования транзакции
                // match storage.withdraw(&name, amount) {
                //     Ok(_) => {
                //         println!("С баланса пользователя {} снято {}", name, amount);
                //         storage.save_storage_to_file(file_path.as_str());
                //     }
                //     Err(e) => println!("Ошибка: {}", e),
                // }

                let withdraw_tx = Withdraw {
                    from_account: tx_name.clone(),
                    amount: tx_amount,
                };

                // Здесь контент Moved в Box
                let withdraw_box = Box::new(withdraw_tx);

                // 2. Применяем транзакцию (для Box'а автоматическое dereferencing) 
                match withdraw_box.apply(&mut storage) {
                    Ok(_) => {
                        println!("Транзакция: снятие с {} {} денег", tx_name, tx_amount);
                        storage.save_storage_to_file(file_path.as_str());
                    }
                    // TxError
                    Err(e) => println!("Ошибка транзакции: {:?}", e),
                }
                
            }
            "balance" => {
                            if args.len() != 2 {
                                println!("Пример: balance John");
                                continue;
                            }
                            let name = args[1].to_string();
                            match storage.get_balance(&name) {
                                Some(b) => println!("Баланс {}: {:?}", name, b),
                                None => println!("Пользователь {} не найден", name),
                            }
            }
            "+" => {
                if args.len() != 8 {
                    println!(
                        "Пример: + deposit Alice 100 transfer Alice Bob 30: вы ввели аргументов:  {}",
                        args.len()
                    );
                    continue;
                }

                let deposit_tx = Deposit {
                    from_account: args[2].to_string(),
                    amount: args[3].parse().unwrap_or(0),
                };

                let transfer_tx = Transfer {
                    from_account: args[5].to_string(),
                    to_account: args[6].to_string(),
                    amount: args[7].parse().unwrap_or(0),
                };

                // Здесь мы используем оператор +
                let combined_tx = deposit_tx + transfer_tx;

                match combined_tx.apply(&mut storage) {
                    Ok(_) => println!("Транзакции выполнены!"),
                    Err(e) => println!("Ошибка при выполнении: {:?}", e),
                }

                storage.save_storage_to_file(&file_path);
            } 
            "exit" => break,
            _ => println!("Неизвестная команда"),
        }
    }

    println!("Выход из CLI, все изменения сохранены.");
}