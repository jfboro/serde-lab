use std::marker::PhantomData;

use chrono::{DateTime, Utc};
use serde::{
    Deserialize, Deserializer,
    de::{DeserializeOwned, Error, Visitor},
};
use serde_json::json;
use serde_lab::{ToPrettyStringMany, deserialization_example_print_many};

#[derive(Debug, Deserialize)]
pub struct Document {
    #[serde(rename = "documentId")]
    pub id: u32,

    #[serde(deserialize_with = "deserialize_json_string")]
    pub metadata: Metadata,

    #[serde(deserialize_with = "deserialize_xml_string")]
    pub invoice: Invoice,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata {
    pub save_timestamp: DateTime<Utc>,
    pub file_name: String,
}

#[derive(Debug, Deserialize)]
pub struct Invoice {
    #[serde(rename = "@id")]
    pub id: u32,

    #[serde(rename = "@total")]
    pub total: f64,

    pub items: Items,

    pub discounts: Discounts,
}

#[derive(Debug, Deserialize)]
pub struct Items {
    #[serde(default, rename = "item")]
    pub list: Vec<Item>,
}

#[derive(Debug, Deserialize)]
pub struct Item {
    #[serde(rename = "@id")]
    pub id: u32,

    #[serde(rename = "@price")]
    pub price: f64,

    #[serde(rename = "@qty")]
    pub quantity: u32,

    #[serde(rename = "@name")]
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct Discounts {
    #[serde(default, rename = "discount")]
    pub list: Vec<Discount>,
}

#[derive(Debug, Deserialize)]
pub struct Discount {
    #[serde(rename = "@id")]
    pub id: u32,

    #[serde(rename = "@amount")]
    pub amount: f64,
}

struct XmlStringVisitor<T: DeserializeOwned>(PhantomData<T>);

impl<T: DeserializeOwned> XmlStringVisitor<T> {
    fn new() -> Self {
        Self(PhantomData)
    }
}

impl<'de, T> Visitor<'de> for XmlStringVisitor<T>
where
    T: DeserializeOwned,
{
    type Value = T;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str(
            format!(
                "a valid XML string the represents a `{}` value",
                std::any::type_name::<T>()
            )
            .as_str(),
        )
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        quick_xml::de::from_str(v).map_err(|e| serde::de::Error::custom(e))
    }
}

fn deserialize_json_string<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: DeserializeOwned,
{
    // Ideally we would use a visitor to avoid allocating another String unnecessarily.
    // Reference to the `07_cursed_strings_02` example to see how to avoid this String allocation using a `Visitor`
    let value = String::deserialize(deserializer)?;
    serde_json::from_str(&value).map_err(D::Error::custom)
}

fn deserialize_xml_string<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: DeserializeOwned,
{
    deserializer.deserialize_str(XmlStringVisitor::new())
}

fn main() {
    let json_strings = [json!(
    {
        "documentId": 987,
        "metadata": "{\"saveTimestamp\":\"2026-01-02T12:34:56Z\",\"fileName\":\"important-document.xml\"}",
        "invoice": "<invoice id=\"1234\" total=\"1234.56\"><items><item id=\"999\" price=\"1334.56\" qty=\"1\" name=\"testing\"/></items><discounts><discount id=\"5\" amount=\"100.00\"/></discounts></invoice>"
    })].to_string_pretty();

    deserialization_example_print_many::<Document>(&json_strings);
}
