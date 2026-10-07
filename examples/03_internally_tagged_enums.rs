use serde::{Deserialize, Serialize};
use serde_json::json;
use serde_lab::{deserialization_example_print_many, serialization_example_print_many};

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "kebab-case")]
pub enum DenialReason {
    InsufficientAmount,
    ExpiredCreditCard,
    Other(String),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "status", rename_all = "camelCase")]
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

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[allow(non_camel_case_types)]
pub enum DenialReason_MODIFIED {
    InsufficientAmount,
    ExpiredCreditCard,
    #[serde(untagged)]
    Other(String),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "status", rename_all = "camelCase")]
#[allow(non_camel_case_types)]
pub enum PaymentStatus_MODIFIED {
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
        reason: DenialReason_MODIFIED,
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
            "status": "processing" // Invalid status
        }),
        json!({
            "status": "denied",
            "reason": {
                "type": "insufficient-amount",
            },
        }),
        json!({
            "status": "denied",
            "reason": {
                // Invalid type, this tagging model can't represent the `Other` variant as JSON
                "type": "other"
            },
        }),
        json!({
            "status": "paid",
            // Invalid amount, but our model allows negative values. This is a problem in using `f64` for monetary values.
            "paidAmount": -20.00,
        }),
        json!({
            "status": "refunded",
            // missing field `paidAmount`
            "refundedAmount": 20.00,
        }),
    ];

    deserialization_example_print_many::<PaymentStatus>(&json_values);

    println!("MODIFIED EXAMPLES:");

    let values = [
        PaymentStatus_MODIFIED::Denied {
            reason: DenialReason_MODIFIED::ExpiredCreditCard,
        },
        PaymentStatus_MODIFIED::Denied {
            reason: DenialReason_MODIFIED::Other("system-error".into()),
        },
        PaymentStatus_MODIFIED::Denied {
            // The "UnknownError" string will not be translated into 'kebab-case' style,
            // even if the renaming macro was used for the 'Other' variant.
            // The value inside the 'Other' variant is not an identifier. Only identifiers can be renamed.
            // Here we could use any String value and it should not be altered in the serialization operation.
            reason: DenialReason_MODIFIED::Other("UnknownError".into()),
        },
        PaymentStatus_MODIFIED::Denied {
            // Not the same variant as the modified Case #01, however it serializes into its same JSON string.
            // Untagged variants should be used with caution because they may cause ambiguity.
            reason: DenialReason_MODIFIED::Other("expired-credit-card".into()),
        },
    ];

    serialization_example_print_many(&values);

    let json_values = [
        json!({
            "status": "denied",
            "reason": "insufficient-amount",
        }),
        json!({
            "status": "denied",
            // Because this is none of the DenialReason unit-variants, it is deserialized into the `Other` variant
            "reason": "UnknownError",
        }),
    ];

    deserialization_example_print_many::<PaymentStatus_MODIFIED>(&json_values);
}
