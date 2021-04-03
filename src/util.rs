use super::websocket::Books;
use serde::{de, Deserialize, Deserializer};
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;
use std::str::FromStr;
use std::string::ToString;

pub fn convert_map_to_string<
    K: Debug + Eq + Hash + ToString,
    V: Debug + ToString,
    S: ::std::hash::BuildHasher,
>(
    map: &HashMap<K, V, S>,
) -> String {
    let mut string: String = String::new();
    for (key, value) in map.iter() {
        string.push_str(&key.to_string());
        string.push('=');
        string.push_str(&value.to_string());
        string.push('&');
    }
    string
}

/// Custom deserialization implementation for the [Books] enum.
pub fn deserialize_books<'de, D>(deserializer: D) -> Result<Books, D::Error>
where
    D: Deserializer<'de>,
{
    let book = <&str>::deserialize(deserializer)?;
    Books::from_str(book).map_err(de::Error::custom)
}
