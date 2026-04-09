use bank_system::operation::Operation;
use bank_system::balance::Balance;

fn main () {
    let operations_to_apply = vec![Operation::Deposit(100), Operation::Withdraw(150), Operation::CloseAccount];
    
    // Сделал конструктор
    let mut balance = Balance::from(100);
    
    // Первая должна выполниться, остаться -250 и закрытие аккаунта
    let bad_ops = balance.process_operations(operations_to_apply);
    assert_eq!(bad_ops.len(), 1);
    println!("{:?}", bad_ops);

    // Проверка адреса переменной на стеке
    // let x = String::from("first");
    // println!("x addr: {:p}", &x as *const _);
    
    // // Shadowing
    // // let x = std::rc::Rc::new(x);
    // let x = String::from("Second");
    // println!("x addr: {:p}", &x as *const _);
} 