use serde::{
    de::{self, Visitor},
    Deserialize, Deserializer, Serialize, Serializer,
};
use std::fmt;
use std::str::FromStr;

pub enum Division {
    I,
    II,
    III,
    IV,
    V,
}

impl Division {
    pub fn value(&self) -> &'static str {
        match *self {
            Division::I => "I",
            Division::II => "II",
            Division::III => "III",
            Division::IV => "IV",
            Division::V => "V",
        }
    }
}

impl fmt::Display for Division {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.value())
    }
}

impl fmt::Debug for Division {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.value())
    }
}

impl FromStr for Division {
    type Err = DivisionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_ref() {
            "i" => Ok(Division::I),
            "ii" => Ok(Division::II),
            "iii" => Ok(Division::III),
            "iv" => Ok(Division::IV),
            "v" => Ok(Division::V),
            "1" => Ok(Division::I),
            "2" => Ok(Division::II),
            "3" => Ok(Division::III),
            "4" => Ok(Division::IV),
            "5" => Ok(Division::V),
            other => Err(DivisionError::InvalidDivision {
                value: other.to_owned(),
            }),
        }
    }
}

struct DivisionVisitor;

impl<'de> Visitor<'de> for DivisionVisitor {
    type Value = Division;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a rank value expected")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        value
            .parse::<Division>()
            .map_err(|err| de::Error::custom(err.to_string()))
    }

    fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        self.visit_str(value.as_ref())
    }
}

impl Serialize for Division {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for Division {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(DivisionVisitor)
    }
}

#[derive(Debug, Fail)]
pub enum DivisionError {
    #[fail(display = "invalid rank: {}", value)]
    InvalidDivision { value: String },
}
