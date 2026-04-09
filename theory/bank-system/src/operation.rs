
#[derive(Debug, Clone, PartialEq)]
pub enum Operation {
    Deposit(u64),
    Withdraw(u64),
    CloseAccount
} 