use std::net::{IpAddr, Ipv4Addr};

use serde::{Deserialize, Serialize};
use serde_bool::True;
use serde_json::json;
use serde_lab::{
    DELIMITER, ToPrettyStringMany, deserialization_example_print_many,
    serialization_example_print_many,
};
use serde_repr::Serialize_repr;

#[derive(Debug, Serialize_repr)]
#[repr(u16)]
#[allow(non_camel_case_types)]
pub enum MachineState_1 {
    Off = 100,
    Working = 200,
    Idle = 300,
    Error = 400,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
#[allow(non_camel_case_types)]
pub struct MachineConfig_1 {
    pub ip: IpAddr,
    pub port: u16,
    pub auth_required: bool,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
#[allow(non_camel_case_types)]
pub struct Machine_1 {
    pub id: u64,
    #[serde(flatten)]
    pub config: MachineConfig_1,

    // Enums with integer representation can't be flattened.
    // #[serde(flatten)] // Serialization returns Err.
    pub state: MachineState_1,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(non_camel_case_types)]
pub enum MachineState_2 {
    Off(True),
    Working(True),
    Idle(True),
    Error(True),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(non_camel_case_types)]
pub struct MachineConfig_2 {
    pub ip: IpAddr,
    pub port: u16,
    pub auth_required: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(non_camel_case_types)]
pub struct Machine_2 {
    pub id: u64,
    #[serde(flatten)]
    pub config: MachineConfig_2,
    #[serde(flatten)]
    // MachineState_2 enum has externally tagged serialization, hence it can be flattened.
    pub state: MachineState_2,
}

fn main() {
    println!("{}", DELIMITER);
    println!("Using integer representation and flattened struct:");
    let values = [Machine_1 {
        id: 123,
        config: MachineConfig_1 {
            ip: IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
            port: 80,
            auth_required: true,
        },
        state: MachineState_1::Idle,
    }];

    serialization_example_print_many(&values);

    println!("Using boolean flags and all fields flattened:");

    let values = [Machine_2 {
        id: 123,
        config: MachineConfig_2 {
            ip: IpAddr::V6("::".parse().unwrap()),
            port: 80,
            auth_required: false,
        },
        state: MachineState_2::Idle(True),
    }];

    serialization_example_print_many(&values);

    println!("Deserialization examples:");

    let json_strings = [
        json!({
            "id": 123,
            "ip": "2001:0db8:85a3:0000:0000:8a2e:0370:7334",
            "port": 443,
            "authRequired": true,
            "working": true,
        }),
        json!({
            "id": 123,
            "ip": "2001:0db8:85a3:0000:0000:8a2e:0370:7334",
            "port": 443,
            "authRequired": true,
            // Invalid state, because at least one of the variants must be selected, here the only one that appears is false, which cannot happen.
            "working": false,
        }),
        json!({
            "id": 123,
            "ip": "2001:0db8:85a3:0000:0000:8a2e:0370:7334",
            "port": 443,
            "authRequired": true,
            // Invalid state, but our deserializer picks the first variant in alphabetical order and ignores the other ones.
            // This model does not enforce invalid states like this and does not avoid ambiguity.
            "working": true,
            "idle": true,
        }),
        json!({
            "id": 123,
            "ip": "2001:0db8:85a3:0000:0000:8a2e:0370:7334",
            "port": 443,
            "authRequired": true,
            // Although this is a valid state, our model cannot deserialize it because it picks the first variant in alphabetical order.
            // It will pick the "idle" variant, which holds "false", but our model only accepts "true" values.
            // we could relax this model making our MachineState_2 enum variants hold `bool` values, but then it now accepts
            // another kind of inconsistent inputs: only one variant hold "false" value, which does not meaning anything.
            // This deserialization strategy must be used with caution, preferably only when the JSON input is assured to be always well-formed.
            "off": true,
            "idle": false,
        }),
    ]
    .to_string_pretty();

    deserialization_example_print_many::<Machine_2>(&json_strings);
}
