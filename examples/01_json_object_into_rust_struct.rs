use serde::Deserialize;
use serde_json::json;
use serde_lab::deserialization_example_print_many;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PaymentStatus {
    pub status: String,
    pub paid_amount: Option<f64>,
    pub refunded_amount: Option<f64>,
    pub reason: Option<String>,
}

fn main() {
    let json_values = [
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
    ];

    deserialization_example_print_many::<PaymentStatus>(&json_values)
}
