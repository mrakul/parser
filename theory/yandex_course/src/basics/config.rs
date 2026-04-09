use std::fmt;

pub const DEFAULT_COURSE_NAME: &str = "Rust для действующих разработчиков";

#[derive(Default)]
pub enum CourseCohort {
     #[default]
    Start,
    Base,
    Blockchain,
}

// (!) Один из двух вариантов для Default: или как вверху, или реализация внизу функции default()

// impl Default for CourseCohort {
//     fn default() -> Self {
//         // Когорта по умолчанию
//         CourseCohort::Start
//     }
// } 

pub struct CourseConfig {
    pub cohort: CourseCohort,
}

// Реализация для каждого типа начинается с блока impl (implementation),
// внутри которого и могут определяться различные элементы — чаще всего это функции и константы.
// Таких блоков может быть несколько.
impl CourseConfig {
    // Ассоциированная функция - конструктор    
    // Такое именование негласно используется в Rust для реализаций способа первоначального создания объектов.

    // Ассоциированные функции привязаны к определённому типу данных. Они не работают с конкретным экземпляром этого типа, а принадлежат самому типу как таковому. Такие функции вызываются с использованием уже знакомого нам оператора :: — например, SomeType::some_function().
    // Ассоциированные функции часто используются для:
    //     - Создания новых значений типа (например, конструкторов).
    //     - Выполнения вычислений, связанных с логикой типа.
    //     = Группировки вспомогательных операций, логически относящихся к типу.
    pub fn new(cohort: CourseCohort) -> Self {
        Self {
            cohort
        }
        // Self — удобное сокращение того самого типа, для которого и происходит реализация.
        // В нашем случае Self эквивалентен CourseConfig. 
    }

    // Функция для работы с экземляром класса

    // В методах параметр self может использоваться в следующих формах:
    //      - &self — неизменяемая ссылка. Наиболее распространённый вариант и используется, когда метод только читает данные, не изменяя их.
    //      - &mut self — изменяемая ссылка. Позволяет модифицировать экземпляр, но не забирает владение.
    //      - self — метод забирает владение объектом. Используется достаточно редко, позволяя, например, получить владение полями self в методе.
    pub fn get_duration(&self) -> u8 {
        match self.cohort {
            CourseCohort::Start => 16,
            CourseCohort::Base => 12,
            CourseCohort::Blockchain => 20
        }
    }

    // Пример метода с изменяемой ссылкой
    pub fn upgrade_cohort(&mut self) -> bool {
        match self.cohort {
            CourseCohort::Blockchain => false,
            _ => {
                self.cohort = CourseCohort::Blockchain;
                true
            }
        }
    }
}


// Пример с Display:
// 1. Реализация для CourseCohort
impl fmt::Display for CourseCohort {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            CourseCohort::Start => "Переход с C/С++/Python",
            CourseCohort::Base => "Базовый курс",
            CourseCohort::Blockchain => "Погружение в блокчейн",
        };
        write!(f, "{}", name)
    }
}

// 2. Реализация для CourseConfig
impl fmt::Display for CourseConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Конфигурация: \"{DEFAULT_COURSE_NAME}\", когорта: \"{}\"", self.cohort)
    }
}