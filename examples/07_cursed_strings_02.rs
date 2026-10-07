use std::marker::PhantomData;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Deserializer, Serialize, de::Visitor};
use serde_json::json;
use serde_lab::deserialization_example_print_many;

struct StringToValueVisitor<V: for<'de> Deserialize<'de>> {
    _marker: PhantomData<V>,
    expected_result: &'static str,
}

impl<'de, V> StringToValueVisitor<V>
where
    V: for<'a> Deserialize<'a>,
{
    fn new(expected_result: &'static str) -> Self {
        Self {
            _marker: PhantomData,
            expected_result,
        }
    }
}

impl<'de, V> Visitor<'de> for StringToValueVisitor<V>
where
    V: for<'a> Deserialize<'a>,
{
    type Value = V;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str(self.expected_result)
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        serde_json::from_str::<V>(v).map_err(|e| serde::de::Error::custom(e))
    }
}

struct GenericDeserializer<V: for<'de> Deserialize<'de>>(PhantomData<V>);

impl<V> GenericDeserializer<V>
where
    V: for<'de> Deserialize<'de>,
{
    fn deserialize_into_value<'de, D>(deserializer: D) -> Result<V, D::Error>
    where
        D: Deserializer<'de>,
    {
        let visitor = StringToValueVisitor::<V>::new(
            "a valid JSON string that could be deserialized to the given type",
        );
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
    let json_values = [
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
    ];

    deserialization_example_print_many::<Payment_ANY_STRING>(&json_values);
    deserialization_example_print_many::<Payment_JSON_FORMAT>(&json_values);
    deserialization_example_print_many::<Payment_STRONG_TYPE>(&json_values);
}
