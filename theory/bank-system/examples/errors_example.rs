// А так мы их используем
fn find_idx(collection: &[u32], value: u32) -> Option<usize> {
    let mut idx = 0;
    for c in collection {
        if *c == value {
            return Some(idx);
        }
        idx += 1;
    }
    None
}

#[derive(Debug)]
enum WeakPassError {
    PasswordShort(usize)
}

#[derive(Debug)]
enum RegistrationError {
    // при пакетной обработке понадобится знать, который из пользователей уже есть
    UserExists(String),
    WeakPass(WeakPassError),
}

// Давайте сразу напишем From, чтобы позже использовать всю мощь оператора `?`
impl From<WeakPassError> for RegistrationError {
    fn from(from: WeakPassError) -> RegistrationError {
        RegistrationError::WeakPass(from)
    }
}
// Расширим возможности наших ошибок - реализуем трейт Error!
// Правда, для реализации Error надо реализовать Display
impl std::fmt::Display for WeakPassError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::PasswordShort(pass_size)
              => write!(f, "password with length {pass_size} is too short")
        }
    }
}
impl std::fmt::Display for RegistrationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::WeakPass(_) => write!(f, "weak password"),
            Self::UserExists(user) => write!(f, "user '{user}' already registered")
        }
    }
}
impl std::error::Error for WeakPassError {
    // std::error::Error имеет все реализации по умолчанию
    // WeakPassError - достаточно "глубокая" ошибка, ничего для неё
}

// Здесь на помощь приходит оператор ?. Он автоматически проверяет Result и Option, 
// и в случае ошибки пытается с помощью From составить подходящий тип ошибки. 
// Это снимает нагрузку на написание и чтение кода с разработчика.

// Трейт Error тоже помогает в этой ситуации. 
// В коде выше обратите внимание на &(dyn Error + ‘static) — это ссылка на трейт-объект, 
// тип которого неизвестен! То есть Error в качестве источника ошибки возвращает не что-то строгое, 
// а просто нечто, которое можно напечатать. 
// Таким образом, Error через стандартную библиотеку выступает связующим звеном
// между ошибками нашего кода и ошибками наших зависимостей.

impl std::error::Error for RegistrationError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::WeakPass(weak_pass_error) => Some(weak_pass_error),
            Self::UserExists(_) => None,
        }
    }
    // У этого трейта есть и другие методы, но они deprecated.. Богатая история!
}

fn _check_password_short(pass: &str) -> Result<(), WeakPassError> {
    if pass.chars().count() < 10 {
        return Err(WeakPassError::PasswordShort(pass.chars().count()))
    }
    else {
        Ok(())
    }
}
fn _does_username_exists(username: &str, existing: &std::collections::HashSet<String>) -> bool {
    false // не будем усложнять пример
}

fn can_register_v1(
       username:       &str,
       password:       &str,
       existing_users: &std::collections::HashSet<String>
       ) -> Result<(), RegistrationError>
{
    // многословно 😭
    if _does_username_exists(username, existing_users) {
        return Err(RegistrationError::UserExists(username.to_string()));
    }
    if let Err(err) = _check_password_short(password) {
        return Err(RegistrationError::WeakPass(err));
    }
    Ok(())
}

fn can_register_v2(
       username:       &str,
       password:       &str,
       existing_users: &std::collections::HashSet<String>
       ) -> Result<(), RegistrationError>
{
    // Скорее proof-of-concept, здесь всегда вызывается .clone()
    _does_username_exists(username, existing_users)
        .then_some(Err(RegistrationError::UserExists(username.to_string())))
        .unwrap_or(Ok(()))?;
    // А этот пример интереснее! Несмотря на то что возвращается другой тип ошибки,
    // оператор `?` автоматический использует From
    _check_password_short(password)?;
    Ok(())
}

fn main() {
    let existing_users = ["Alice".to_string(), "Bob".into()].into_iter().collect();
    match can_register_v1("Eva", "shortpass", &existing_users) {
        Ok(()) => (),
        Err(err) => {
            use std::error::Error;
            println!("error:  {err}");
            println!( "source: {}",
                      err.source().map_or("-".to_string(), |source| source.to_string()) );
        }
    }
    // Если упадём при ошибке - к сожалению, за нас source никто не напечатает
    can_register_v1("Eva", "shortpass", &existing_users).unwrap();
}