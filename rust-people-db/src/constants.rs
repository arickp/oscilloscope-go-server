use serde::de::{self, Visitor};
use serde::{Deserializer, Serializer};
use std::fmt;
use gettextrs::gettext;

// TRANSLATABLE STRINGS (for .po files):
// File, People, New, Open, Exit, Add, Edit, Delete
// ID, First Name, Last Name, Age, Favorite Sport
// No people loaded
// Baseball, Soccer, Basketball, Tennis, Golf, Hockey, Cricket, Rugby, Handball, Football, Volleyball, Water polo, Equestrian, Swimming, Running, Cycling, Skating, Skateboarding, Surfing, Skiing, Snowboarding, Rowing, Wrestling
// (and any other user-facing string)

pub const APP_NAME: &str = "People DB";
pub const APP_ID: &str = "com.github.arickp.rustpeopledb";
pub const APP_VERSION: &str = env!("CARGO_PKG_VERSION");
pub const CSV_HEADERS: &[&str] = &["first_name", "last_name", "date_of_birth", "favorite_sport"];
pub const GUI_TABLE_HEADER_COLUMNS: &[&str] = &["ID", "First Name", "Last Name", "Age", "Favorite Sport"];

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
            Sport::Baseball => gettext("Baseball"),
            Sport::Soccer => gettext("Soccer"),
            Sport::Basketball => gettext("Basketball"),
            Sport::Tennis => gettext("Tennis"),
            Sport::Golf => gettext("Golf"),
            Sport::Hockey => gettext("Hockey"),
            Sport::Cricket => gettext("Cricket"),
            Sport::Rugby => gettext("Rugby"),
            Sport::Handball => gettext("Handball"),
            Sport::Football => gettext("Football"),
            Sport::Volleyball => gettext("Volleyball"),
            Sport::WaterPolo => gettext("Water polo"),
            Sport::Equestrian => gettext("Equestrian"),
            Sport::Swimming => gettext("Swimming"),
            Sport::Running => gettext("Running"),
            Sport::Cycling => gettext("Cycling"),
            Sport::Skating => gettext("Skating"),
            Sport::Skateboarding => gettext("Skateboarding"),
            Sport::Surfing => gettext("Surfing"),
            Sport::Skiing => gettext("Skiing"),
            Sport::Snowboarding => gettext("Snowboarding"),
            Sport::Rowing => gettext("Rowing"),
            Sport::Wrestling => gettext("Wrestling"),
            Sport::Other(s) => s.to_string(),
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
