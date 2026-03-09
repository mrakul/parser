use std::{collections::HashMap, ops::Add};

// наследоваться от Sized нужно, чтобы вернуть Self,
// иначе говоря - "чтобы было что разместить на стеке"

// Выражение trait B: A не значит, что если тип будет реализовывать B, то он автоматически реализует A. 
// Здесь указано, что если тип реализует B, то этот же тип обязан реализовывать A. 

// Сами реализации (impl) A и B записываются независимо. 
//  Здесь же — «исключение» из правила: мы не писали impl Sized for Wallet, 
//  потому что Sized реализуется автоматически. В Rust такими автоматическими типами также являются:
//   - Send — тип можно перемещать из одного потока в другой.
//   - Sync — к типу можно обращаться из разных потоков одновременно, или «ссылка на тип реализует Send».
trait CurrencyConvertable : Sized {
    fn convert(self, into: &str, currencies: &HashMap<String, f64>) -> Self;
}

#[derive(Clone)]
struct Wallet {
    account: u64,
}

impl Wallet {
    // да, в Rust можно использовать тип до его объявления - лишь бы в файле был
    fn apply(&mut self, op: Operation) {
        match op {
            Operation::Add(v) => self.account += v as u64,
            Operation::Sub(v) => self.account -= v as u64,
        }
    }
}
#[derive(Clone, Debug)]
enum Operation {
    Add(u32),
    Sub(u32),
}

// 🚫 The Critical Misconception
// ❌ "If I own a value, I can always mutate it."
// ✅ Reality: Ownership grants right to mutate, but binding must be explicitly declared mutable to exercise that right.
// This is by design — Rust separates ownership (lifetime control) from mutability (content modification).

// ❓ "If initial owner is not mutable, impossible to pass owning as mutable?"
// ✅ NO — it's absolutely possible and common!
// Moving a value does not require the source binding to be mutable. The new binding can be mutable regardless of the source.

impl Operation {
    fn is_significant(&self) -> bool {
        match self {
            Self::Add(value) | Self::Sub(value) if *value >= 100 => true,
            _ => false,
        }
    }
}

// (!) Коэффициент на состояние счёта
// (2!) Передача в конверт владения с мутабельностью и возвращение назад
impl CurrencyConvertable for Wallet {
    fn convert(mut self, into: &str, currencies: &HashMap<String, f64>) -> Self {
        self.account = ((self.account as f64) * currencies.get(into).unwrap()) as u64;
        self
    }
}

// Оба типа используются в конвертации - полиморфизм

// (!) Возвращает другую операцию с коэффициентом, коэффициент на операции
impl CurrencyConvertable for Operation {
    fn convert(mut self, into: &str, currencies: &HashMap<String, f64>) -> Self {
        match &mut self {
            // Важно: нет дефолтного случая _, поскольку перечислены все варианты
            // Называется Exhaustive Matching
            Self::Add(value) | Self::Sub(value) => {
                *value = ((*value as f64) * currencies.get(into).unwrap()) as u32;
            }
        }
        self
    }
}

fn main() {
    
    let operations = vec![Operation::Add(200), Operation::Sub(50), Operation::Add(30), Operation::Sub(100)];
    
    let maps_to = HashMap::from([("eur".to_string(), 0.84f64), ("cny".into(), 7.12f64)]);
    
    // Важно: мутабельный
    let mut wallet = Wallet{account: 0};

    // let string = String::from("Test");
    // let mut string2 = &string;
    // string2 = &"Test".to_string();

    for op in operations {
        // Клонируется для использования в других операциях 
        // (?) Почему тогда не по ссылке
        wallet.apply(op.clone());

        println!(
            // а ещё Rust умеет удалять пробелы от начала строки,
            // если для удобства чтения нужно одну строку в коде изобразить несколькими
            "After applying {:?}$ 
            (or {:?}eur
            or {:?}cny) account \
            is {}$ 
            (or {}eur 
            or {}cny), 
            significant - {}",
            op, 
            op.clone().convert("eur", &maps_to), 
            op.clone().convert("cny", &maps_to),
            wallet.account,
            wallet.clone().convert("eur", &maps_to).account,
            wallet.clone().convert("cny", &maps_to).account,
            op.is_significant()
        );
    }
} 