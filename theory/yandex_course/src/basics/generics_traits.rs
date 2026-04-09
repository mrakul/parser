// Generic swap
pub fn swap<T>(first_arg: T, second_arg: T) -> (T, T) {
    return (second_arg, first_arg);
}

// Структура с generic

#[derive(Default)]
struct Field<T> {
    value: T,
    is_valid: bool,
} 

// Enum с generic - два типа
enum Result<T, E> {
    Ok(T),
    Err(E),
} 


// Пример trait'а
trait Active {
    // Ассоциированный тип - без указания конкретного типа
    // Каждый тип, реализующий трейт, может сам решить, как представлять своё состояние:
    // как строку, символ, число и т. д.
    type Status;

    // Ассоциированная константа, тип устройства или сущности
    // Тип обязательно указывать, как у любой константы, а (!) значение - в каждой реализации
    const ENTITY_TYPE: &'static str;

    // Static-переменные запрещены

    /* Методы */

    // Возвращает текущее состояние, используя ассоциированный тип
    // (например, true, '🔴', "Active")
    fn status(&self) -> Self::Status;

    // Метод-флаг активности
    fn is_active(&self) -> bool;

    // Метод активации
    fn activate(&mut self);

    // Метод деактивации
    fn deactivate(&mut self);

    // Метод по умолчанию (с реализацией по умолчанию)
    fn toggle(&mut self) {
        if self.is_active() {
            self.deactivate();
        } else {
            self.activate();
        }
    }
}

/* Реализация трейта различными типами */

// Тип Sensor
struct Sensor {
    active: bool,
}

impl Active for Sensor {
    type Status = char;

    const ENTITY_TYPE: &'static str = "SmartDevice";

    fn status(&self) -> Self::Status {
        if self.active { '🟢' } else { '🔴' }
    }

    fn is_active(&self) -> bool {
        self.active
    }

    fn activate(&mut self) {
        self.active = true;
    }

    fn deactivate(&mut self) {
        self.active = false;
    }

    // Переопределение реализации по умолчанию
    fn toggle(&mut self) {
        if self.is_active() {
            self.deactivate();
        } else {
            self.activate();
        }
    }
}

// Тип Feature
struct Feature {
    enabled: bool,
}

// (!) В структуре можно использовать дженерик-параметр с трейт-ограничениями в реализации методов,
// даже если сама структура не ограничивает тип трейтом

impl Active for Feature {
    type Status = &'static str;

    const ENTITY_TYPE: &'static str = "Feature";

    fn status(&self) -> Self::Status {
        if self.enabled { "enabled" } else { "disabled" }
    }

    fn is_active(&self) -> bool {
        self.enabled
    }

    fn activate(&mut self) {
        self.enabled = true;
    }

    fn deactivate(&mut self) {
        self.enabled = false;
    }
}

/* Совмещение generic'ов и trait'ов */

// Нужно ограничить тип T и сказать компилятору:
// «Мы хотим, чтобы T был любым типом, но при условии, что он реализует поведение Active».
// В Rust это делается с помощью ограничений трейтов (trait bounds):
fn start<T: Active>(component: &mut T) {
    component.activate();
} 


// Более сложные ограничения
fn configure_and_start<T: Active + Clone>(mut component: T) -> T {
   // ...
   component
} 

// Мы используем + для указания всех нужных нам трейтов.
// Если же такие ограничения становятся громоздкими, удобнее использовать ключевое слово where:
fn configure_and_start_2<T>(mut component: T) -> T
where
    T: Active + Clone,
{
    component
}


// Помимо ручной реализации трейтов, можно использовать удобный атрибут #[derive(...)].
// Этот механизм позволяет быстро добавить типичное поведение к пользовательским типам данных.
#[derive(Default)]
pub struct FieldDerived<T: Default> {
    value: T,
    pub is_valid: bool,
}

// Формально такая реализация эквивалентна вручную написанному блоку:
// impl<T: Default> Default for FieldDerived<T> {
//     fn default() -> Self {
//         Self {
//             value: T::default(),
//             is_valid: bool::default(),
//         }
//     }
// } 