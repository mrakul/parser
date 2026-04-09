use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

/// Наш процедурный макрос: принимает строку и печатает её в коде
#[proc_macro]
pub fn say_hello(input: TokenStream) -> TokenStream {
    let msg = parse_macro_input!(input as syn::LitStr); // ожидаем строковый литерал

    let expanded = quote! {
        println!("{}", #msg);
    };

    expanded.into()
}


#[proc_macro_derive(Transaction, attributes(transaction))]
pub fn transaction_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    // По умолчанию — deposit
    let mut kind = "deposit";

    for attr in &input.attrs {
        if attr.path().is_ident("transaction") {
            // Разбираем атрибут как Meta
            if let Ok(meta) = attr.parse_args::<syn::LitStr>() {
                let val = meta.value();
                if val == "withdraw" {
                    kind = "withdraw";
                } else if val == "transfer" {
                    kind = "transfer";
                }
            }
        }
    }

    let body = match kind {
        "deposit" => quote! {
            *storage.get_accounts_mut()
                // За один проход: возвращает или Occupoed, or_insert() это unwrap'ит
                // Или создаёт новый Balance. если нет записи
                // AddAssign реализовал в Balance
                .entry(self.from_account.clone())
                .or_insert(Balance::from(0)) += self.amount;
        },
        "withdraw" => quote! {
            let from_balance = storage.get_accounts_mut().entry(self.from_account.clone()).or_insert(Balance::from(0));

            if from_balance.get_value() < self.amount {
                return Err(TxError::InsufficientFunds);
            }

            // SubAssign реализовал в Balance
            *from_balance -= self.amount;
        },
        "transfer" => quote! {
            let from_balance = storage.get_accounts_mut().entry(self.from_account.clone()).or_insert(Balance::from(0));

            if from_balance.get_value() < self.amount {
                return Err(TxError::InsufficientFunds);
            }

            // SubAssign реализовал в Balance
            *from_balance -= self.amount;

            *storage.get_accounts_mut().entry(self.to_account.clone()).or_insert(Balance::from(0)) += self.amount;
        },
        _ => panic!("Unknown transaction kind"),
    };

    let expanded = quote! {
        impl Transaction for #name {
            fn apply(&self, storage: &mut Storage) -> Result<(), TxError> {
                #body
                Ok(())
            }
        }
    };

    TokenStream::from(expanded)
}


/*** proc_macro2 */
// Кроме библиотеки proc_macro существует ещё proc_macro2. Она позволяет привнести функциональность в любую часть программы (например, main.rs build.rs) и использовать её даже в тестах. 
// А сгенерированный код можно распечатать с помощью println!.

use proc_macro2::TokenStream as TokenStream2;
use syn::{Data, Fields};

#[proc_macro_derive(ToSql)]
pub fn to_sql_derive(input: TokenStream) -> TokenStream {
    // Парсим вход в proc_macro2 TokenStream
    let input: DeriveInput = parse_macro_input!(input);
    let name = input.ident;

    let (field_names, field_values): (Vec<_>, Vec<_>) = match input.data {
        Data::Struct(ref data) => match &data.fields {
            Fields::Named(ref fields) => fields
                .named
                .iter()
                .map(|f| {
                    let ident = f.ident.as_ref().unwrap();
                    (ident, quote! { self.#ident })
                })
                .unzip(),
            _ => panic!("ToSql can only be derived for structs with named fields"),
        },
        _ => panic!("ToSql can only be derived for structs"),
    };

    // Генерация кода с proc_macro2 + quote
    let expanded: TokenStream2 = quote! {
        impl #name {
            pub fn to_sql(&self, table: &str) -> String {
                let columns = vec![#(stringify!(#field_names)),*].join(", ");
                let values = vec![#(format!("'{}'", #field_values)),*].join(", ");
                format!("INSERT INTO {} ({}) VALUES ({});", table, columns, values)
            }
        }
    };

    println!("{expanded}",);

    // Преобразуем proc_macro2::TokenStream обратно в proc_macro::TokenStream
    TokenStream::from(expanded)
}

// From SQL
#[proc_macro_derive(FromSql)]
pub fn from_sql_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    // Собираем поля структуры
    let fields = if let syn::Data::Struct(data) = &input.data {
        data.fields
            .iter()
            .map(|f| f.ident.clone().unwrap())
            .collect::<Vec<_>>()
    } else {
        panic!("FromSql can only be derived for structs");
    };

    // Генерируем код с итератором по значениям
    let assigns = fields.iter().map(|f| {
        quote! {
            #f: vals.next().unwrap().parse().expect("Cannot parse field"),
        }
    });

    let expanded = quote! {
        impl #name {
            pub fn from_sql(sql: &str) -> Self {
                let mut vals = sql
                    .split("VALUES(")
                    .nth(1)
                    .expect("No VALUES found")
                    .trim_end_matches(");")
                    .split(',')
                    .map(|s| s.trim().trim_matches('\''))
                    .into_iter();

                Self {
                    #(#assigns)*
                }
            }
        }
    };

    println!("{}", expanded);

    TokenStream::from(expanded)
} 