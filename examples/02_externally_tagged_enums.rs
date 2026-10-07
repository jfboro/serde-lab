use serde::{Deserialize, Serialize};
use serde_json::json;
use serde_lab::{deserialization_example_print_many, serialization_example_print_many};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum DenialReason {
    InsufficientAmount,
    ExpiredCreditCard,
    Other(String),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")] // only renames the variants identifiers
pub enum PaymentStatus {
    Pending,

    #[serde(rename_all = "camelCase")]
    Paid {
        paid_amount: f64,
    },

    #[serde(rename_all = "camelCase")]
    Refunded {
        paid_amount: f64,
        refunded_amount: f64,
    },

    #[serde(rename_all = "camelCase")] // struct-like variant fields, need their own renaming rule
    Denied {
        reason: DenialReason,
    },
}

fn main() {
    let values = [
        PaymentStatus::Pending,
        PaymentStatus::Denied {
            reason: DenialReason::ExpiredCreditCard,
        },
        PaymentStatus::Denied {
            reason: DenialReason::Other("system-error".into()),
        },
        PaymentStatus::Paid { paid_amount: 100.0 },
        PaymentStatus::Refunded {
            paid_amount: 100.0,
            refunded_amount: 90.0,
        },
    ];

    serialization_example_print_many(&values);

    let json_values = [
        json!({
            "denied": {
                "reason": "expired-credit-card",
            }
        }),
        json!({
            "denied": {
                "reason": {
                    // Our model can't infer this meaning because the structure enforces it to deserialize into the "Other" variant.
                    // Who built this JSON is responsible for using the correct structure.
                    "other": "expired-credit-card"
                },
            }
        }),
    ];

    deserialization_example_print_many::<PaymentStatus>(&json_values);
}
