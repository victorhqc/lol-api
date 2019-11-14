use serde::{
    de::{self, Visitor},
    Deserialize, Deserializer, Serialize, Serializer,
};
use std::fmt;
use std::str::FromStr;

pub enum Queue {
    RankedSolo5x5,
    RankedTft,
    RankedFlexSr,
    RankedFlexIt,
}

impl Queue {
    pub fn value(&self) -> &'static str {
        match *self {
            Queue::RankedSolo5x5 => "RANKED_SOLO_5x5",
            Queue::RankedTft => "RANKED_TFT",
            Queue::RankedFlexSr => "RANKED_FLEX_SR",
            Queue::RankedFlexIt => "RANKED_FLEX_IT",
        }
    }
}

impl fmt::Display for Queue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.value())
    }
}

impl fmt::Debug for Queue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.value())
    }
}

impl FromStr for Queue {
    type Err = QueueError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_ref() {
            "RANKED_SOLO_5x5" => Ok(Queue::RankedSolo5x5),
            "RANKED_TFT" => Ok(Queue::RankedTft),
            "RANKED_FLEX_SR" => Ok(Queue::RankedFlexSr),
            "RANKED_FLEX_IT" => Ok(Queue::RankedFlexIt),
            other => Err(QueueError::InvalidQueue {
                value: other.to_owned(),
            }),
        }
    }
}

struct QueueVisitor;

impl<'de> Visitor<'de> for QueueVisitor {
    type Value = Queue;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a queue value expected")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        value
            .parse::<Queue>()
            .map_err(|err| de::Error::custom(err.to_string()))
    }

    fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        self.visit_str(value.as_ref())
    }
}

impl Serialize for Queue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for Queue {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(QueueVisitor)
    }
}

#[derive(Debug, Fail)]
pub enum QueueError {
    #[fail(display = "invalid queue: {}", value)]
    InvalidQueue { value: String },
}
