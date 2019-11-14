use serde::{
    de::{self, Visitor},
    Deserialize, Deserializer, Serialize, Serializer,
};
use std::fmt;
use std::str::FromStr;

pub struct Rank {}

pub enum Tier {
    Iron,
    Bronze,
    Silver,
    Gold,
    Platinum,
    Diamond,
    Master,
    GrandMaster,
    Challenger,
}

impl Tier {
    pub fn value(&self) -> &'static str {
        match *self {
            Tier::Iron => "IRON",
            Tier::Bronze => "BRONZE",
            Tier::Silver => "SILVER",
            Tier::Gold => "GOLD",
            Tier::Platinum => "PLATINUM",
            Tier::Diamond => "DIAMOND",
            Tier::Master => "MASTER",
            Tier::GrandMaster => "GRAND_MASTER",
            Tier::Challenger => "CHALLENGER",
        }
    }
}

impl fmt::Display for Tier {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.value())
    }
}

impl fmt::Debug for Tier {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.value())
    }
}

impl FromStr for Tier {
    type Err = TierError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_ref() {
            "iron" => Ok(Tier::Iron),
            "bronze" => Ok(Tier::Bronze),
            "silver" => Ok(Tier::Silver),
            "gold" => Ok(Tier::Gold),
            "platinum" => Ok(Tier::Platinum),
            "diamond" => Ok(Tier::Diamond),
            "master" => Ok(Tier::Master),
            "grand_master" => Ok(Tier::GrandMaster),
            "challenger" => Ok(Tier::Challenger),
            other => Err(TierError::InvalidTier {
                value: other.to_owned(),
            }),
        }
    }
}

struct TierVisitor;

impl<'de> Visitor<'de> for TierVisitor {
    type Value = Tier;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a tier value expected")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        value
            .parse::<Tier>()
            .map_err(|err| de::Error::custom(err.to_string()))
    }

    fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        self.visit_str(value.as_ref())
    }
}

impl Serialize for Tier {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for Tier {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(TierVisitor)
    }
}

#[derive(Debug, Fail)]
pub enum TierError {
    #[fail(display = "invalid tier: {}", value)]
    InvalidTier { value: String },
}
