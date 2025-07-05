use crate::constants::Sport;
use chrono::Local;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt;
use std::fs::File;
use std::path::Path;
use tabled::{Table, Tabled};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Person {
    pub first_name: String,
    pub last_name: String,
    #[serde(with = "date_format")]
    pub date_of_birth: NaiveDate,
    pub favorite_sport: Sport,
}

mod date_format {
    use chrono::NaiveDate;
    use serde::{self, Deserialize, Deserializer, Serializer};

    const FORMAT: &str = "%Y-%m-%d";

    pub fn deserialize<'de, D>(deserializer: D) -> Result<NaiveDate, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        NaiveDate::parse_from_str(&s, FORMAT).map_err(serde::de::Error::custom)
    }

    pub fn serialize<S>(date: &NaiveDate, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = date.format(FORMAT).to_string();
        serializer.serialize_str(&s)
    }
}

impl Person {
    pub fn new(
        first_name: String,
        last_name: String,
        date_of_birth: NaiveDate,
        favorite_sport: Sport,
    ) -> Self {
        Person {
            first_name,
            last_name,
            date_of_birth,
            favorite_sport,
        }
    }

    pub fn get_age(&self) -> u32 {
        let today = Local::now().naive_local().date();
        let age = today.signed_duration_since(self.date_of_birth).num_days() / 365;
        age as u32
    }

    pub fn get_favorite_sport_emoji(&self) -> &str {
        self.favorite_sport.emoji()
    }

    /// Reads all `Person` records from a CSV file. Returns a vector of `Person` records.
    pub fn read_from_csv<P: AsRef<Path>>(path: P) -> Result<Vec<Person>, Box<dyn Error>> {
        let file = File::open(path)?; // Open the file. Errors returned immediately.
        let mut reader = csv::Reader::from_reader(file);
        let mut people = Vec::new();

        // Iterate for each record in the CSV file.
        for result in reader.deserialize() {
            // Deserialize the record into a `Person` struct.
            let person: Person = result?;
            // Add the `Person` struct to the vector.
            people.push(person);
        }

        Ok(people)
    }

    /// Writes all `Person` records to a CSV file.
    pub fn write_to_csv<P: AsRef<Path>>(path: P, people: &[Person]) -> Result<(), Box<dyn Error>> {
        let file = File::create(path)?;
        let mut writer = csv::Writer::from_writer(file);

        for person in people {
            writer.serialize(person)?;
        }

        writer.flush()?;
        Ok(())
    }
}

impl fmt::Display for Person {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:<15} {:<15} {:<3} {} {:<16}",
            self.first_name,
            self.last_name,
            self.get_age(),
            self.get_favorite_sport_emoji(),
            self.favorite_sport,
        )
    }
}

pub fn add_person(people: &mut Vec<Person>, person: Person) -> Result<(), Box<dyn Error>> {
    people.push(person);
    Ok(())
}

pub fn delete_person(people: &mut Vec<Person>, index: usize) -> Result<(), Box<dyn Error>> {
    if index < people.len() {
        people.remove(index);
        Ok(())
    } else {
        Err(format!("Index out of bounds: {}", index).into())
    }
}

pub fn edit_person(
    people: &mut Vec<Person>,
    index: usize,
    person: Person,
) -> Result<(), Box<dyn Error>> {
    if index < people.len() {
        people[index] = person;
        Ok(())
    } else {
        Err(format!("Index out of bounds: {}", index).into())
    }
}

#[derive(Tabled)]
pub struct PersonTableRow {
    pub idx: String,
    pub first_name: String,
    pub last_name: String,
    pub age: String,
    pub favorite_sport: String,
}

pub fn print_people(people: &[Person]) {
    let mut rows: Vec<PersonTableRow> = Vec::new();
    for (idx, p) in people.iter().enumerate() {
        let idx_str = idx.to_string();
        let first_name = p.first_name.clone();
        let last_name = p.last_name.clone();
        let age = p.get_age().to_string();
        let favorite_sport = format!("{} {}", p.favorite_sport.emoji(), p.favorite_sport);
        rows.push(PersonTableRow {
            idx: idx_str,
            first_name,
            last_name,
            age,
            favorite_sport,
        });
    }
    let mut base_table = Table::new(rows);
    let table = base_table.with(tabled::settings::Style::rounded());
    println!("{}", table);
}
