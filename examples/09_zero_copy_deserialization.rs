use serde::{
    Deserialize,
    de::{self, Deserializer, Visitor},
};
use serde_json::json;
use serde_lab::{ToPrettyStringMany, deserialization_example_print_many};

#[derive(Debug, Deserialize)]
pub struct Bla<'b> {
    pub a: String,
    #[serde(borrow, deserialize_with = "deserialize_borrowed")]
    pub b: &'b str,
}

struct BlaVisitor;

impl<'de> Visitor<'de> for BlaVisitor {
    type Value = &'de str;

    fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str("a string")
    }

    fn visit_borrowed_str<E>(self, value: &'de str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        println!("visit_borrowed_str called: {value}");
        Ok(value)
    }
}

fn deserialize_borrowed<'de, D>(
    deserializer: D,
) -> Result<&'de str, <D as Deserializer<'de>>::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_str(BlaVisitor)
}

fn main() {
    let json_strings = [
        json!({
            "a": "owned",
            "b": "Calls the `visit_borrowed_str` because this string can be directly borrowed from the original input string without any intermediate allocation.",
        }),
        json!({
            "a": "owned",
            "b": "Cannot call `visit_borrowed_str` with the 'de lifetime when escaped chars are used.\nReturns `Err` if `visit_str` is not implemented.",
        }),
    ]
    .to_string_pretty();

    deserialization_example_print_many::<Bla>(&json_strings);
}
