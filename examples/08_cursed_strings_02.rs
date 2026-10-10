use std::marker::PhantomData;

use chrono::{DateTime, Utc};
use serde::{
    Deserialize, Deserializer, Serialize,
    de::{DeserializeOwned, Visitor},
};
use serde_json::json;
use serde_lab::{ToPrettyStringMany, deserialization_example_print_many};

struct StringToValueVisitor<T: DeserializeOwned>(PhantomData<T>);

impl<'de, T> StringToValueVisitor<T>
where
    T: DeserializeOwned,
{
    fn new() -> Self {
        Self(PhantomData)
    }
}

impl<'de, T> Visitor<'de> for StringToValueVisitor<T>
where
    T: DeserializeOwned,
{
    type Value = T;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str(
            format!(
                "a valid JSON string representation of {}",
                std::any::type_name::<T>()
            )
            .as_str(),
        )
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        serde_json::from_str::<T>(v).map_err(|e| serde::de::Error::custom(e))
    }
}

struct GenericDeserializer<T: DeserializeOwned>(PhantomData<T>);

impl<T> GenericDeserializer<T>
where
    T: DeserializeOwned,
{
    fn deserialize_into_value<'de, D>(deserializer: D) -> Result<T, D::Error>
    where
        D: Deserializer<'de>,
    {
        let visitor = StringToValueVisitor::new();
        deserializer.deserialize_str(visitor)
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Serialize, Deserialize)]
pub struct Payment_ANY_STRING {
    pub id: u64,
    pub amount: f64,
    pub metadata: String,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Serialize, Deserialize)]
pub struct Payment_JSON_FORMAT {
    pub id: u64,
    pub amount: f64,
    #[serde(deserialize_with = "GenericDeserializer::deserialize_into_value")]
    pub metadata: serde_json::Value,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Serialize, Deserialize)]
pub struct Payment_STRONG_TYPE {
    pub id: u64,
    pub amount: f64,
    #[serde(deserialize_with = "GenericDeserializer::deserialize_into_value")]
    pub metadata: PaymentMetadata,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentMetadata {
    pub location: String,
    pub datetime: DateTime<Utc>,
}

fn main() {
    let json_strings = [
        json!({
            "id": 123,
            "amount": 1000.0,
            "metadata": "{\"location\": \"BRA\", \"datetime\": \"2026-10-12T01:02:03.123456Z\"}",
        }),
        json!({
            "id": 123,
            "amount": 1000.0,
            "metadata": "{\"ip\": \"10.1.100.200\", \"port\": \"443\"}",
        }),
        json!({
            "id": 234,
            "amount": 2000.0,
            "metadata": "This is any metadata in any random format",
        }),
    ]
    .to_string_pretty();

    deserialization_example_print_many::<Payment_ANY_STRING>(&json_strings);
    deserialization_example_print_many::<Payment_JSON_FORMAT>(&json_strings);
    deserialization_example_print_many::<Payment_STRONG_TYPE>(&json_strings);
}
