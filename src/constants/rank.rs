use serde::{
    de::{self, Visitor},
    Deserialize, Deserializer, Serialize, Serializer,
};
use std::fmt;
use std::str::FromStr;

pub enum Rank {
    I,
    II,
    III,
    IV,
    V,
}

impl Rank {
    pub fn value(&self) -> &'static str {
        match *self {
            Rank::I => "I",
            Rank::II => "II",
            Rank::III => "III",
            Rank::IV => "IV",
            Rank::V => "V",
        }
    }
}

impl fmt::Display for Rank {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.value())
    }
}

impl fmt::Debug for Rank {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.value())
    }
}

impl FromStr for Rank {
    type Err = RankError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_ref() {
            "i" => Ok(Rank::I),
            "ii" => Ok(Rank::II),
            "iii" => Ok(Rank::III),
            "iv" => Ok(Rank::IV),
            "v" => Ok(Rank::V),
            "1" => Ok(Rank::I),
            "2" => Ok(Rank::II),
            "3" => Ok(Rank::III),
            "4" => Ok(Rank::IV),
            "5" => Ok(Rank::V),
            other => Err(RankError::InvalidRank {
                value: other.to_owned(),
            }),
        }
    }
}

struct RankVisitor;

impl<'de> Visitor<'de> for RankVisitor {
    type Value = Rank;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a rank value expected")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        value
            .parse::<Rank>()
            .map_err(|err| de::Error::custom(err.to_string()))
    }

    fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        self.visit_str(value.as_ref())
    }
}

impl Serialize for Rank {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for Rank {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(RankVisitor)
    }
}

#[derive(Debug, Fail)]
pub enum RankError {
    #[fail(display = "invalid rank: {}", value)]
    InvalidRank { value: String },
}
