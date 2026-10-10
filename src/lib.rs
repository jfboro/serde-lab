use std::fmt::Debug;

use serde::{Deserialize, Serialize};

pub const DELIMITER: &'static str = "======================================";

pub fn deserialization_example_print<'de, T>(json_str: &'de str)
where
    T: Deserialize<'de> + Debug,
{
    println!();
    println!("JSON to be deserialized:");
    println!("{}", json_str);
    println!();

    let value_result = serde_json::from_str::<T>(json_str);
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

pub fn deserialization_example_print_many<'de, T>(json_strings: &'de [String])
where
    T: Deserialize<'de> + Debug,
{
    println!("{}", DELIMITER);
    println!("DESERIALIZATION:");
    for jv in json_strings {
        deserialization_example_print::<T>(jv);
    }
}

pub fn serialization_example_print_many<T>(json_strings: &[T])
where
    T: Serialize + Debug,
{
    println!("{}", DELIMITER);
    println!("SERIALIZATION:");
    for (case, jv) in json_strings.iter().enumerate() {
        serialization_example_print::<T>(case + 1, jv);
    }
}

pub trait ToPrettyStringMany<const N: usize> {
    fn to_string_pretty(self) -> [String; N];
}

impl<const N: usize> ToPrettyStringMany<N> for [serde_json::Value; N] {
    fn to_string_pretty(self) -> [String; N] {
        self.map(|x| serde_json::to_string_pretty(&x).unwrap())
    }
}
