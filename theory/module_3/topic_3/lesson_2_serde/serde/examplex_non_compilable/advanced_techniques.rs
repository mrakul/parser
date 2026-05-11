// Продвинутые техники работы с serde
// Иногда стандартных возможностей мало. Например, нужно встроить одну структуру в другую, работать с тегированными enum'ами или создавать кастомные сериализаторы. Такие задачи будут вставать перед вами не часто, но в реальной разработке вы можете столкнуться с ними. С ними помогут продвинутые техники работы с serde.
// Сразу покажем их на примерах кода.


// flatten — встраивание структур:

#[derive(Serialize, Deserialize)]
struct Address { street: String, city: String, country: String }

#[derive(Serialize, Deserialize)]
struct User {
    id: u32,
    name: String,
    #[serde(flatten)]
    address: Address,
} 

//  untagged enums — без тега:

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
enum Value { String(String), Number(i64), Boolean(bool) } 

// tagged enums — с тегом:

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
enum Event {
    UserCreated { id: u32, name: String },
    UserDeleted { id: u32 },
} 

// Кастомные (де)сериализаторы для сложных типов:

use serde::{Serialize, Deserialize, Serializer, Deserializer};
use std::collections::HashMap;

#[derive(Debug)]
struct CustomMap { data: HashMap<String,String> }

impl Serialize for CustomMap {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where S: Serializer {
        let mut seq = s.serialize_seq(Some(self.data.len()))?;
        for (k, v) in &self.data { seq.serialize_element(&(k, v))?; }
        seq.end()
    }
}

impl<'de> Deserialize<'de> for CustomMap {
    fn deserialize<D>(d: D) -> Result<Self, D::Error>
    where D: Deserializer<'de> {
        let pairs: Vec<(String,String)> = Vec::deserialize(d)?;
        Ok(CustomMap { data: pairs.into_iter().collect() })
    }
} 

// Даты и время:

use chrono::{DateTime, Utc, NaiveDateTime};
use serde::{Serialize, Deserialize, Serializer, Deserializer};

#[derive(Serialize, Deserialize)]
struct Event {
    name: String,

    #[serde(with = "chrono::serde::ts_seconds")]
    created_at: DateTime<Utc>,

    #[serde(serialize_with = "ser_date", deserialize_with = "de_date")]
    updated_at: NaiveDateTime,
}

fn ser_date<S>(d: &NaiveDateTime, s: S) -> Result<S::Ok, S::Error>
where S: Serializer {
    s.serialize_str(&d.format("%Y-%m-%d %H:%M:%S").to_string())
}

fn de_date<'de, D>(d: D) -> Result<NaiveDateTime, D::Error>
where D: Deserializer<'de> {
    let s = String::deserialize(d)?;
    NaiveDateTime::parse_from_str(&s, "%Y-%m-%d %H:%M:%S")
        .map_err(serde::de::Error::custom)
} 