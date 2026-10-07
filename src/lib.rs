use std::fmt::Debug;

use serde::{Serialize, de::DeserializeOwned};

pub const DELIMITER: &'static str = "======================================";

pub fn deserialization_example_print<T>(case_number: usize, json_value: &serde_json::Value)
where
    T: DeserializeOwned + Debug,
{
    if case_number == 1 {
        println!("{}", DELIMITER);
    }
    println!("Case #{:02}:", case_number);
    println!();

    let json_string = serde_json::to_string_pretty(&json_value).unwrap();

    println!("JSON to be deserialized:");
    println!("{}", json_string);
    println!();

    let value_result = serde_json::from_str::<T>(&json_string);
    match value_result {
        Ok(t) => {
            println!("Deserialized Rust value:");
            println!("{:#?}", t);
        }
        Err(e) => println!("Error: {}", e),
    };
    println!();
    println!("{}", DELIMITER);
}

pub fn serialization_example_print<T>(case_number: usize, value: &T)
where
    T: Serialize + Debug,
{
    if case_number == 1 {
        println!("{}", DELIMITER);
    }
    println!("Case #{:02}:", case_number);
    println!();

    println!("Rust value to be serialized:");
    println!("{:#?}", value);
    println!();

    let json_result = serde_json::to_string_pretty(&value);
    match json_result {
        Ok(t) => {
            println!("JSON serialization:");
            println!("{}", t);
        }
        Err(e) => println!("Error: {}", e),
    };
    println!();
    println!("{}", DELIMITER);
}

pub fn deserialization_example_print_many<T>(json_values: &[serde_json::Value])
where
    T: DeserializeOwned + Debug,
{
    println!("{}", DELIMITER);
    println!("DESERIALIZATION:");
    for (case, jv) in json_values.iter().enumerate() {
        deserialization_example_print::<T>(case + 1, jv);
    }
}

pub fn serialization_example_print_many<T>(json_values: &[T])
where
    T: Serialize + Debug,
{
    println!("{}", DELIMITER);
    println!("SERIALIZATION:");
    for (case, jv) in json_values.iter().enumerate() {
        serialization_example_print::<T>(case + 1, jv);
    }
}
