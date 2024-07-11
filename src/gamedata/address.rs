use serde::{de::Error, Deserialize};
use std::{fmt, num::ParseIntError};

#[derive(Debug)]
pub struct Address(u32);

impl<'de> Deserialize<'de> for Address {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Address::try_from(s.as_str())
            .map_err(|e| D::Error::custom(format!("invalid hex address: {}", e)))
    }
}

impl TryFrom<&str> for Address {
    type Error = ParseIntError;

    fn try_from(value: &str) -> Result<Address, ParseIntError> {
        Ok(Address(
            u32::from_str_radix(value.trim_start_matches("0x"), 16)? & 0xffffff,
        ))
    }
}

impl fmt::Display for Address {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "0x{:06X}", self.0)
    }
}
