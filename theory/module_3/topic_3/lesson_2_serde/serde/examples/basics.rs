use serde::{Serialize, Deserialize, Serializer, Deserializer};
use serde_json;

#[derive(Serialize, Deserialize)]
struct User {
    id: u32,
    name: String,
    email: String,
    // is_active: bool,
}

#[derive(Serialize, Deserialize)]
enum Status {
    Active,
    Inactive,
    Suspended,
} 

// Под капотом при этом автоматически генерируется примерно вот такой код:

// impl Serialize for User {
//     fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
//     where S: Serializer {
//         let mut st = s.serialize_struct("User", 4)?;
//         st.serialize_field("id", &self.id)?;
//         st.serialize_field("name", &self.name)?;
//         st.serialize_field("email", &self.email)?;
//         st.serialize_field("is_active", &self.is_active)?;
//         st.end()
//     }
// } 


/*** Варианты использования Serde */
// Переименование полей:

#[derive(Serialize, Deserialize)]
struct User2 {
    #[serde(rename = "user_id")]
    id: u32,
    #[serde(rename = "full_name")]
    name: String,
    #[serde(rename = "email_address")]
    email: String,
} 

// Пропуск полей:
#[derive(Serialize, Deserialize)]
struct User3 {
    id: u32,
    name: String,

    #[serde(skip_serializing)]
    password_hash: String,   

    #[serde(skip_deserializing, default = "default_time")]
    created_at: std::time::SystemTime,

    #[serde(skip)]
    internal_id: u64,        
} 

// Custom function to provide the default value
fn default_time() -> std::time::SystemTime {
    std::time::SystemTime::now()  // Or UNIX_EPOCH, or any logic you want
}

// Значения по умолчанию:
#[derive(Serialize, Deserialize)]
struct Config {
    host: String,
    port: u16,

    #[serde(default)]
    timeout: u64, 

    #[serde(default = "default_retries")]
    retries: u32,
}

fn default_retries() -> u32 { 3 } 

// Алиасы для полей (обратная совместимость):
#[derive(Deserialize)]
struct User4 {
    #[serde(alias = "user_id", alias = "id")]
    user_id: u32,

    #[serde(alias = "full_name", alias = "name")]
    name: String,
} 



#[derive(Serialize, Deserialize)]
struct User5 {
    name: String,
    #[serde(serialize_with = "ser_pwd")]
    #[serde(deserialize_with = "de_pwd")]
    password: String,
}

fn ser_pwd<S>(_: &String, s: S) -> Result<S::Ok, S::Error>
where S: Serializer {
    s.serialize_str("***")
}

fn de_pwd<'de, D>(d: D) -> Result<String, D::Error>
where D: Deserializer<'de> {
    String::deserialize(d)
}

use serde_yaml;
use rmp_serde;
use bincode;

use anyhow::Result;  // anyhow::Result<T> = Result<T, anyhow::Error>

fn main() -> Result<()>
{
    let user = User { id: 1, name: "John".into(), email: "john@example.com".into() };

    let json = serde_json::to_string(&user)?;
    let pretty = serde_json::to_string_pretty(&user)?;
    let u2: User = serde_json::from_str(&json)?; 

    // YAML (конфиги):
    let yaml = serde_yaml::to_string(&user)?;
    let u2: User = serde_yaml::from_str(&yaml)?; 

    // MessagePack:


    let buf = rmp_serde::to_vec(&user)?;
    let u2: User = rmp_serde::from_slice(&buf)?; 

    // Bincode:


    let bin = bincode::serialize(&user)?;
    let u2: User = bincode::deserialize(&bin)?; 

    Ok(())
}

