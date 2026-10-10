use serde::Deserialize;
use serde_json::json;
use serde_lab::{ToPrettyStringMany, deserialization_example_print_many};

#[derive(Debug, Deserialize)]
pub struct Person {
    pub id: u64,

    #[serde(flatten)]
    pub personal_info: PersonalInfo,

    #[serde(flatten)]
    pub contact_info: ContactInfo,
}

#[derive(Debug, Deserialize)]
pub struct PersonalInfo {
    pub name: String,
    pub birthday: chrono::NaiveDate,
}

#[derive(Debug, Deserialize)]
pub struct ContactInfo {
    // I'm not going to discuss all possible ways to parse email strings or phone numbers
    // I'll use Strings for both for simplicity
    pub email: String,
    pub phone: String,
}

fn main() {
    let json_strings: [String; _] = [
        json!({
            "id": 123,
            "name": "John",
            "birthday": "2000-01-01",
            "email": "john@example.com",
            "phone": "123456789",
        }),
        json!({
            "id": 123,
            "name": "John",
            "birthday": "2000-01-01",
            "email": "john@example.com",
            // missing field:
            // "phone": "123456789",
        }),
        json!({
            "id": 123,
            "name": "John",
            "birthday": "2000-01-01",
            "email": "john@example.com",
            "phone": "123456789",
            // extra field not present in the deserialized struct:
            "zipcode": "987654321"
        }),
        json!({
            // This will not work out of the box because our model is expecting flattened structs
            // There are some workarounds to make this work, however, it would be better to use a custom deserialization method
            "id": 123,
            "personal_info": {
                "name": "John",
                "birthday": "2000-01-01",
            },
            "contact_info": {
                "email": "john@example.com",
                "phone": "123456789",
            }
        }),
    ]
    .to_string_pretty();

    deserialization_example_print_many::<Person>(&json_strings);
}
