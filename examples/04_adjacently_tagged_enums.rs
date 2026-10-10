use serde::Serialize;
use serde_lab::serialization_example_print_many;

//
// Uncomment this example to view the compiler diagnostics:
//
// #[derive(Debug, Serialize)]
// #[serde(tag = "abc", rename_all = "camelCase")]
// pub enum Example {
//     Foo,
//     Bar { abc: usize },
// }
//
// When the enum is adjacently tagged, it does not have this problem:
//
// #[derive(Debug, Serialize)]
// #[serde(tag = "abc", content = "xyz" rename_all = "camelCase")]
// pub enum Example {
//     Foo,
//     Bar { abc: usize },
// }
//

#[derive(Debug, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum DenialReason {
    InsufficientAmount,
    ExpiredCreditCard,
    #[serde(untagged)]
    Other(String),
}

#[derive(Debug, Serialize)]
#[serde(tag = "status", content = "info", rename_all = "camelCase")]
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
    let values: [PaymentStatus; _] = [
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
}
