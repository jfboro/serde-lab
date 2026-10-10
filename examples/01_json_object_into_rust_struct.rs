use serde::Deserialize;
use serde_json::json;
use serde_lab::{ToPrettyStringMany, deserialization_example_print_many};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PaymentStatus {
    pub status: String,
    pub paid_amount: Option<f64>,
    pub refunded_amount: Option<f64>,
    pub reason: Option<String>,
}

fn main() {
    let json_strings: [String; _] = [
        json! ({
            "status": "paid",
            "paidAmount": 2.71,
        }),
        json! ({
            "status": true, // Incorrect type. Our model only accepts strings for the "status" field.
            "paidAmount": 2.71,
        }),
        json! ({
            // Correct type, but the system could have trouble deciding its meaning.
            "status": "invalid",
            // Correct type, but again, what does negative values mean in this field? This is a common confusion in payment APIs design: the sign has ambiguous meaning in different parts of the system.
            // A better approach is discussed in the example 'non_negative_values.rs'
            "refundedAmount": -100.0,
        }),
        json!({
            // Valid status, but information provided. However our model accepts it. We would need to manually validate it.
            "status": "refunded"
        }),
        json!({
            "status": "refunded",
            // Field not existent in the deserialized struct:
            "datetime": "2026-10-10T01:02:03.12345Z",
        }),
    ]
    .to_string_pretty();

    deserialization_example_print_many::<PaymentStatus>(&json_strings)
}
