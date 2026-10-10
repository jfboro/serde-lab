use std::{collections::HashSet, fmt::Display, hash::Hash, marker::PhantomData, str::FromStr};

use serde::{Deserialize, Deserializer, Serialize, Serializer, de::Visitor};
use serde_json::json;
use serde_lab::{
    ToPrettyStringMany, deserialization_example_print_many, serialization_example_print_many,
};

trait SeparatedValuesConfig {
    type Item: FromStr<Err: Display> + ToString;
    const SEPARATOR: &'static str;
}

struct SeparatedValuesToIteratorVisitor<
    V: FromStr<Err: Display>,
    I: FromIterator<V>,
    C: SeparatedValuesConfig<Item = V>,
>(PhantomData<(V, I, C)>);

impl<V, I, C> SeparatedValuesToIteratorVisitor<V, I, C>
where
    V: FromStr<Err: Display>,
    I: FromIterator<V>,
    C: SeparatedValuesConfig<Item = V>,
{
    fn new() -> Self {
        Self(PhantomData)
    }
}

///
/// We use the Visitor pattern to avoid unnecessary memory allocation for reading the string to be deserialized.
///
/// Without the Visitor we would need to write something like `let string_field = String::deserialize(deserializer);`
/// to read the string, which allocated memory.
///
/// The Visitor trait already provides us the `visit_str` method, whose argument `v: &str` is already value of the
/// string field we are deserializing.
///
/// (This has nothing to do with further allocation necessary to actually deserialize the &str value.)
///
impl<'de, V, I, C> Visitor<'de> for SeparatedValuesToIteratorVisitor<V, I, C>
where
    V: FromStr<Err: Display>,
    I: FromIterator<V>,
    C: SeparatedValuesConfig<Item = V>,
{
    type Value = I;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str(
            format!(
                "a list of string-representations of `{}` values separated by `{}`",
                std::any::type_name::<V>(),
                C::SEPARATOR,
            )
            .as_str(),
        )
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        let string_values = v.split(C::SEPARATOR);
        let mut values = vec![];
        for str_value in string_values {
            values.push(V::from_str(str_value).map_err(|e| serde::de::Error::custom(e))?);
        }

        Ok(values.into_iter().collect())
    }
}

trait SeparatedValuesDeserializer: SeparatedValuesConfig<Item: FromStr<Err: Display>> + Sized {
    fn deserialize<'de, D>(deserializer: D) -> Result<Vec<Self::Item>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let visitor = SeparatedValuesToIteratorVisitor::<_, Vec<_>, Self>::new();
        deserializer.deserialize_str(visitor)
    }
}

trait SeparatedValuesSerializer: SeparatedValuesConfig<Item: ToString> {
    fn serialize<S>(item: &Vec<Self::Item>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(
            &item
                .iter()
                .map(Self::Item::to_string)
                .collect::<Vec<String>>()
                .join(Self::SEPARATOR),
        )
    }
}

trait UniqueSeparatedValuesDeserializer:
    SeparatedValuesConfig<Item: FromStr<Err: Display> + Eq + Hash> + Sized
{
    fn deserialize_unique<'de, D>(deserializer: D) -> Result<HashSet<Self::Item>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let visitor = SeparatedValuesToIteratorVisitor::<_, HashSet<_>, Self>::new();
        deserializer.deserialize_str(visitor)
    }
}

trait UniqueSeparatedValuesSerializer: SeparatedValuesConfig<Item: ToString> {
    fn serialize_from_hashset<S>(
        item: &HashSet<Self::Item>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(
            &item
                .iter()
                .map(Self::Item::to_string)
                .collect::<Vec<String>>()
                .join(Self::SEPARATOR),
        )
    }
}

///
/// Since we cannot hold `const` values for `&'static str` inside structs, we can use this macro
/// to create Serializers and Deserializers for each individual separator.
///
macro_rules! define_separated_values {
    ($name:ident, $separator:literal) => {
        pub struct $name<V: FromStr<Err: Display>>(PhantomData<V>);

        impl<V> SeparatedValuesConfig for $name<V>
        where
            V: FromStr<Err: Display> + ToString,
        {
            type Item = V;

            const SEPARATOR: &'static str = $separator;
        }

        impl<V> SeparatedValuesSerializer for $name<V>
        where
            V: FromStr<Err: Display> + ToString,
        {
        }

        impl<V> SeparatedValuesDeserializer for $name<V>
        where
            V: FromStr<Err: Display> + ToString,
        {
        }
    };
}

// And this macro is the same as the above but for unique values deserialization.
macro_rules! define_unique_separated_values {
    ($name:ident, $separator:literal) => {
        pub struct $name<V: FromStr<Err: Display>>(PhantomData<V>);

        impl<V> SeparatedValuesConfig for $name<V>
        where
            V: FromStr<Err: Display> + ToString + Eq + Hash,
        {
            type Item = V;

            const SEPARATOR: &'static str = $separator;
        }

        impl<V> UniqueSeparatedValuesSerializer for $name<V> where
            V: FromStr<Err: Display> + ToString + Eq + Hash
        {
        }

        impl<V> UniqueSeparatedValuesDeserializer for $name<V> where
            V: FromStr<Err: Display> + ToString + Eq + Hash
        {
        }
    };
}

define_separated_values!(CommaSeparatedValues, ",");
define_unique_separated_values!(PipeSeparatedValues, "|");

#[derive(Debug, Serialize, Deserialize)]
pub struct Foo {
    pub id: u64,
    #[serde(
        deserialize_with = "CommaSeparatedValues::deserialize",
        serialize_with = "CommaSeparatedValues::serialize"
    )]
    pub tags: Vec<String>,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Hash)]
pub enum Tag {
    Director,
    Manager,
    Employee,
    Admin,
}

impl ToString for Tag {
    fn to_string(&self) -> String {
        match self {
            Tag::Director => "Director",
            Tag::Manager => "Manager",
            Tag::Employee => "Employee",
            Tag::Admin => "Admin",
        }
        .into()
    }
}

impl FromStr for Tag {
    type Err = serde_json::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_json::from_str(&["\"", s, "\""].concat())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Bar {
    pub id: u64,
    #[serde(
        deserialize_with = "PipeSeparatedValues::deserialize_unique",
        serialize_with = "PipeSeparatedValues::serialize_from_hashset"
    )]
    pub tags: HashSet<Tag>,
}

fn main() {
    let json_strings = [json!({"id": 123, "tags": "qwer,asdf,zxcv"})].to_string_pretty();

    deserialization_example_print_many::<Foo>(&json_strings);

    let json_strings = [json!({"id": 123, "tags": "Manager|Admin|Manager"})].to_string_pretty();

    deserialization_example_print_many::<Bar>(&json_strings);

    let values = [Foo {
        id: 123,
        tags: vec!["abc".into(), "xyz".into()],
    }];

    serialization_example_print_many(&values);

    let values = [Bar {
        id: 123,
        tags: HashSet::from([Tag::Manager, Tag::Admin]),
    }];

    serialization_example_print_many(&values);
}
