use serde::de::{self, Visitor};
use serde::{Deserializer, Serializer};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Sport {
    Baseball,
    Soccer,
    Basketball,
    Tennis,
    Golf,
    Hockey,
    Cricket,
    Rugby,
    Handball,
    Football,
    Volleyball,
    WaterPolo,
    Equestrian,
    Swimming,
    Running,
    Cycling,
    Skating,
    Skateboarding,
    Surfing,
    Skiing,
    Snowboarding,
    Rowing,
    Wrestling,
    Other(String),
}

impl Sport {
    pub fn emoji(&self) -> &'static str {
        match self {
            Sport::Baseball => "⚾",
            Sport::Soccer => "⚽",
            Sport::Basketball => "🏀",
            Sport::Tennis => "🎾",
            Sport::Golf => "⛳",
            Sport::Hockey => "🏒",
            Sport::Cricket => "🏏",
            Sport::Rugby => "🏉",
            Sport::Handball => "🤾",
            Sport::Football => "🏈",
            Sport::Volleyball => "🏐",
            Sport::WaterPolo => "🤽",
            Sport::Equestrian => "🐎",
            Sport::Swimming => "🏊",
            Sport::Running => "🏃",
            Sport::Cycling => "🚴",
            Sport::Skating => "🛼",
            Sport::Skateboarding => "🛹",
            Sport::Surfing => "🏄",
            Sport::Skiing => "🎿",
            Sport::Snowboarding => "🏂",
            Sport::Rowing => "🚣",
            Sport::Wrestling => "🤼",
            Sport::Other(_) => "",
        }
    }

    pub fn all_known_sports() -> Vec<Sport> {
        vec![
            Sport::Baseball,
            Sport::Soccer,
            Sport::Basketball,
            Sport::Tennis,
            Sport::Golf,
            Sport::Hockey,
            Sport::Cricket,
            Sport::Rugby,
            Sport::Handball,
            Sport::Football,
            Sport::Volleyball,
            Sport::WaterPolo,
            Sport::Equestrian,
            Sport::Swimming,
            Sport::Running,
            Sport::Cycling,
            Sport::Skating,
            Sport::Skateboarding,
            Sport::Surfing,
            Sport::Skiing,
            Sport::Snowboarding,
            Sport::Rowing,
            Sport::Wrestling,
        ]
    }

    pub fn from_string(s: &str) -> Sport {
        match s.trim().to_lowercase().as_str() {
            "baseball" => Sport::Baseball,
            "soccer" => Sport::Soccer,
            "basketball" => Sport::Basketball,
            "tennis" => Sport::Tennis,
            "golf" => Sport::Golf,
            "hockey" => Sport::Hockey,
            "cricket" => Sport::Cricket,
            "rugby" => Sport::Rugby,
            "handball" => Sport::Handball,
            "football" => Sport::Football,
            "volleyball" => Sport::Volleyball,
            "water polo" | "water_polo" => Sport::WaterPolo,
            "equestrian" => Sport::Equestrian,
            "swimming" => Sport::Swimming,
            "running" => Sport::Running,
            "cycling" => Sport::Cycling,
            "skating" => Sport::Skating,
            "skateboarding" => Sport::Skateboarding,
            "surfing" => Sport::Surfing,
            "skiing" => Sport::Skiing,
            "snowboarding" => Sport::Snowboarding,
            "rowing" => Sport::Rowing,
            "wrestling" => Sport::Wrestling,
            other => Sport::Other(other.to_string()),
        }
    }
}

impl fmt::Display for Sport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Sport::Baseball => "baseball",
            Sport::Soccer => "soccer",
            Sport::Basketball => "basketball",
            Sport::Tennis => "tennis",
            Sport::Golf => "golf",
            Sport::Hockey => "hockey",
            Sport::Cricket => "cricket",
            Sport::Rugby => "rugby",
            Sport::Handball => "handball",
            Sport::Football => "football",
            Sport::Volleyball => "volleyball",
            Sport::WaterPolo => "water polo",
            Sport::Equestrian => "equestrian",
            Sport::Swimming => "swimming",
            Sport::Running => "running",
            Sport::Cycling => "cycling",
            Sport::Skating => "skating",
            Sport::Skateboarding => "skateboarding",
            Sport::Surfing => "surfing",
            Sport::Skiing => "skiing",
            Sport::Snowboarding => "snowboarding",
            Sport::Rowing => "rowing",
            Sport::Wrestling => "wrestling",
            Sport::Other(s) => s,
        };
        write!(f, "{}", s)
    }
}

impl serde::Serialize for Sport {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

struct SportVisitor;

impl<'de> Visitor<'de> for SportVisitor {
    type Value = Sport;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid sport string")
    }

    fn visit_str<E>(self, value: &str) -> Result<Sport, E>
    where
        E: de::Error,
    {
        Ok(Sport::from_string(value))
    }
}

impl<'de> serde::Deserialize<'de> for Sport {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(SportVisitor)
    }
}
