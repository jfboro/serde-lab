use serde::{Deserialize, Serialize};
use serde_json::json;
use serde_lab::{deserialization_example_print_many, serialization_example_print_many};

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged, rename_all = "kebab-case")]
pub enum DenialReason {
    InsufficientAmount,
    ExpiredCreditCard,
    Other(String),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged, rename_all = "camelCase")]
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

    #[serde(rename_all = "camelCase")]
    Denied {
        reason: DenialReason,
    },
}

fn main() {
    let values = [
        PaymentStatus::Pending,
        PaymentStatus::Denied {
            // Case #02
            reason: DenialReason::ExpiredCreditCard,
        },
        PaymentStatus::Denied {
            // Notice this instance will be serialized to the same JSON string as the Case #02,
            // which illustrates how untagged JSON representation creates ambiguity.
            // Actually it erases the variant's identifier, which makes it impossible to differentiate the unit-variants.
            reason: DenialReason::InsufficientAmount,
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
        json!(null),
        // It is not possible to choose to which unit-variant this JSON representation will be deserialized:
        json!({"reason": null}),
    ];

    deserialization_example_print_many::<PaymentStatus>(&json_values);

    let values = [DenialReason::Other("unknown-error".into())];
    serialization_example_print_many(&values);
}
