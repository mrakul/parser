use crate::operation::Operation;

// Сейчас сделали struct Balance(i6 4) с методами

// Обернём баланс в новый тип, чтобы можно было реализовывать метод
// (не для целого числа ведь метод добавлять, верно? :) )
// и запретим балансу опускаться ниже нуля

// #[derive(Debug, PartialEq, Clone)]
// pub struct Balance(i64);
 
#[derive(Debug, Clone)]
pub struct Balance {
    value: i64,
    applied_operations: Vec<Operation>,
}

impl Balance {
    // Предыдущий конструктор в tuple-like struct
    // pub fn new(value: i64) -> Self {
    //     Self(value)
    // }
    pub fn new() -> Self {
            Balance{value: 0, applied_operations: Vec::new()}
    }

    pub fn new_value_and_operations(value_to_set: i64, operations: Vec<Operation>) -> Self {
        Balance{value: value_to_set, 
                applied_operations: operations}
    }

    // Или смотреть имплементацию From ниже
    pub fn new_from_i64_value(value_to_set: i64) -> Self {
            Balance{value: value_to_set, applied_operations: Vec::new()}
    }

    // Возвращаем значение, но для примитивного типа копируется
    pub fn get_value(&self) -> i64 {
        self.value
    }

    pub fn set_value(&mut self, value_to_set: i64) -> () {
        self.value = value_to_set
    }

    pub fn get_applied_operations_ref(&self) -> &Vec<Operation> {
        &self.applied_operations
    }

    // Опционально: можно пойти ещё дальше и в качестве аргумента принимать любой тип,
    // который может итерироваться по Operation, с помощью дженерика:
    // fn process<'a>(&mut self, impl IntoIterator<Item=&'a Operation>) -> Vec<&'a Operation>
    
    // Реализация из курса
    // fn process<'a>(&mut self, ops: &[&'a Operation]) -> Vec<&'a Operation> {
    pub fn process_operations(&mut self, operations: Vec<Operation>) -> Vec<Operation> {
        
        // Владение вектором в начале операции, передаём владение в итератор remaining
        let mut remaining_operations = operations.into_iter();
        let mut bad_operations = Vec::new();

        // Необходимо взять итератор по ссылке, чтобы он был доступен в конце для сохранения оставшихся
        for op in remaining_operations.by_ref() {
            match op {
                Operation::Deposit(value) => {
                    self.value += value as i64;
                },
                Operation::Withdraw(value) if self.value >= value as i64 => {
                    self.value -= value as i64;
                },
                other @ _ => {
                    bad_operations.push(other);
                    // Выходим сразу после первой плохой операции (итератор сразу после плохой операции)
                    break;
                },
            }
        }

        // В extend() передаём именно итератор, оставшиеся операции
        bad_operations.extend(remaining_operations);
        bad_operations
    }
}

// (!) Создаётся новый одновременно
impl From<i64> for Balance {
    fn from(value: i64) -> Self {
        Balance{value, applied_operations: Vec::new() }
    }
}

// Реализация операторов сравнения (трейтов)
impl PartialEq for Balance {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl PartialOrd for Balance {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.value.partial_cmp(&other.value) // Compare only by value
    }
}

impl std::ops::AddAssign<i64> for Balance {
    fn add_assign(&mut self, assign_value: i64) {
        self.value += assign_value;
    }
}

impl std::ops::SubAssign<i64> for Balance {
    fn sub_assign(&mut self, assign_value: i64) {
        self.value -= assign_value;
    }
}