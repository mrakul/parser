use bank_system::{storage::*, };
use bank_system::operation::Operation;
use bank_system::operation::Operation::{*};
use bank_system::balance::Balance;

fn check_storage_best_ratio(storage: &Storage) {
    if let Some(best_ratio_name_value) = storage.find_best() {
        println!(r#"Best factor for {} == {} "!"#, best_ratio_name_value.0, best_ratio_name_value.1);
    }
    else {
        println!("Пользователей нет");
    }
}

fn process_operation_by_manager(storage: &mut Storage, user: &String, apply_operation: Operation) -> Result<(), BalanceManagerError> {

    match apply_operation {
        Deposit(value) => storage.deposit_by_manager(user, value as i64)?,
        Withdraw(value) => storage.withdraw_by_manager(user, value as i64)?,
        _            => return Ok(())
    }

    Ok(())
}

fn main () {
    let mut storage = Storage::new();

    storage.add_user_with_balance("Dad".to_string(), 
                                Balance::new_value_and_operations(1000,
                                vec![Operation::Deposit(200000),
                                                 Operation::Withdraw(100000)]));

    storage.add_user_with_balance("Dad".to_string(), 
                   Balance::new_value_and_operations(0,
                    vec![Operation::Deposit(200000),
                                    Operation::Withdraw(100000)]));

    storage.add_user_with_balance("Mom".to_string(), 
                   Balance::new_value_and_operations(0,
                   vec![Operation::Deposit(120000),
                                   Operation::Withdraw(50000),
                                   Operation::Withdraw(20000)]));


    storage.add_user_with_balance("Son".to_string(), 
                   Balance::new_value_and_operations(0,
                   vec![Operation::Deposit(5000),
                                   Operation::Withdraw(500),
                                   Operation::Withdraw(1000),
                                   Operation::Withdraw(700)]));                      

    check_storage_best_ratio(&storage);

    // Самый простой вариант, на None - паника
    // let best_ratio_name_value = storage.find_best().unwrap();
    // println!(r#"Best factor for {} == {} "!"#, best_ratio_name_value.0, best_ratio_name_value.1);

    storage.remove_user(&"Son".to_string());
    check_storage_best_ratio(&storage);

    // 1. Здесь используем вызов из структуры
    match storage.withdraw(&"Jack".to_string(), 100) {
        Ok(()) => println!("Ok"),
        Err(result) => {println!("Ошибка: {:?}", result);}
    }

    // 2. Здесь используем вызов из Manager'а
    if let Err(error) = process_operation_by_manager(&mut storage, &"Non-existing User".to_string(), Deposit(100)) {
        // Реализовать Display для ошибок для вывода
        println!("{:?}", error);
    }

    if let Err(error) = process_operation_by_manager(&mut storage, &"Dad".to_string(), Withdraw(1000000000)) {
        // Реализовать Display для ошибок для вывода
        println!("{:?}", error);
    }

} 


