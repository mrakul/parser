use chrono::{DateTime, Utc};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Product {
    pub id: Uuid,
    pub name: String,
    #[serde(serialize_with = "serialize_price")]
    #[serde(deserialize_with = "deserialize_price")]
    pub price: f64,
    pub category: Option<String>,
    pub in_stock: bool,
    #[serde(skip_serializing)]
    #[serde(default)]
    pub internal_id: u64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OrderStatus {
    Pending,
    Processing,
    Shipped,
    Delivered,
    Cancelled,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrderItem {
    pub product: Product,
    pub quantity: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Order {
    pub id: Uuid,
    #[serde(skip_serializing)]
    #[serde(default)]
    pub internal_id: Uuid,
    pub user_id: Uuid,
    #[serde(deserialize_with = "validate_email")]
    pub customer_email: String,
    pub items: Vec<OrderItem>,
    #[serde(serialize_with = "serialize_price")]
    #[serde(deserialize_with = "deserialize_price")]
    pub total: f64,
    pub status: OrderStatus,
    #[serde(skip_serializing)]
    #[serde(deserialize_with = "chrono::serde::ts_seconds::deserialize")]
    #[serde(default = "Utc::now")]
    pub created_at: DateTime<Utc>,
}

fn serialize_price<S>(price: &f64, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_i64((price * 100.0).round() as i64)
}

fn deserialize_price<'de, D>(deserializer: D) -> Result<f64, D::Error>
where
    D: Deserializer<'de>,
{
    let cents = i64::deserialize(deserializer)?;
    Ok(cents as f64 / 100.0)
}

fn validate_email<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let email = String::deserialize(deserializer)?;
    let valid = email.contains('@') && email.contains('.') && email.len() > 5;
    if valid {
        Ok(email)
    } else {
        Err(serde::de::Error::custom("invalid email format"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn product_serializes_price_in_cents() {
        let product = Product {
            id: Uuid::nil(),
            name: "MacBook Pro".into(),
            price: 1999.99,
            category: Some("Electronics".into()),
            in_stock: true,
            internal_id: 42,
        };

        let json = serde_json::to_string(&product).unwrap();
        assert!(json.contains("\"price\":199999"));
        assert!(!json.contains("internal_id"));

        let decoded: Product = serde_json::from_str(&json).unwrap();
        assert!((decoded.price - 1999.99).abs() < f64::EPSILON);
        assert_eq!(decoded.internal_id, 0);
    }

    #[test]
    fn order_serialization_hides_internal_fields() {
        let product = Product {
            id: Uuid::new_v4(),
            name: "Keyboard".into(),
            price: 99.95,
            category: None,
            in_stock: true,
            internal_id: 1,
        };
        let order = Order {
            id: Uuid::new_v4(),
            internal_id: Uuid::new_v4(),
            user_id: Uuid::new_v4(),
            customer_email: "user@example.com".into(),
            items: vec![OrderItem {
                product: product.clone(),
                quantity: 2,
            }],
            total: 199.90,
            status: OrderStatus::Processing,
            created_at: Utc::now(),
        };

        let json = serde_json::to_string(&order).unwrap();
        assert!(!json.contains("internal_id"));
        assert!(json.contains("\"status\":\"processing\""));
        assert!(json.contains("\"price\":9995"));

        let decoded: Order = serde_json::from_str(&json).unwrap();
        assert_eq!(decoded.customer_email, "user@example.com");
        assert_eq!(decoded.items.len(), 1);
        assert_eq!(decoded.internal_id, Uuid::nil());
    }

    #[test]
    fn invalid_email_fails_to_deserialize() {
        let json = serde_json::json!({
            "id": Uuid::new_v4(),
            "internal_id": Uuid::new_v4(),
            "user_id": Uuid::new_v4(),
            "customer_email": "invalid",
            "items": [],
            "total": 0,
            "status": "pending",
            "created_at": 1_600_000_000
        })
        .to_string();

        let err = serde_json::from_str::<Order>(&json).unwrap_err();
        assert!(err.to_string().contains("invalid email format"));
    }
}

