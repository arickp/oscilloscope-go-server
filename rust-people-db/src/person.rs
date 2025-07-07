use crate::constants::Sport;
use chrono::Local;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt;
use std::fs::File;
use std::path::Path;
use tabled::{Table, Tabled};
use crate::constants::CSV_HEADERS;
use log;
use std::sync::atomic::{AtomicU32, Ordering};

static COUNTER: AtomicU32 = AtomicU32::new(0);
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Person {
    #[serde(skip_deserializing)]
    pub id: u32,
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
            id: COUNTER.fetch_add(1, Ordering::Relaxed),
            first_name,
            last_name,
            date_of_birth,
            favorite_sport,
        }
    }

    pub fn with_id(
        id: u32,
        first_name: String,
        last_name: String,
        date_of_birth: NaiveDate,
        favorite_sport: Sport,
    ) -> Self {
        Person {
            id,
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
        let file = File::open(&path)?; // Open the file. Errors returned immediately.
        let mut reader = csv::Reader::from_reader(file);
        let mut people = Vec::new();

        // Iterate for each record in the CSV file.
        for result in reader.deserialize() {
            // Deserialize the record into a `Person` struct.
            let mut person: Person = result?;
            // Assign a unique ID since it's skipped during deserialization
            person.id = COUNTER.fetch_add(1, Ordering::Relaxed);

            // Add the `Person` struct to the vector.
            people.push(person);
        }

        log::info!("Read {} {} from CSV file: {}", 
            people.len(), 
            if people.len() == 1 {"person" } else { "people" },
            path.as_ref().display()
        );
        Ok(people)
    }

    /// Writes all `Person` records to a CSV file.
    pub fn write_to_csv<P: AsRef<Path>>(path: P, people: &[Person]) -> Result<(), Box<dyn Error>> {
        let file = File::create(&path)?;
        let mut writer = csv::Writer::from_writer(file);

        for person in people {
            writer.serialize(person)?;
        }

        writer.flush()?;
        log::info!("Wrote {} {} to CSV file: {}", 
            people.len(), 
            if people.len() == 1 {"person" } else { "people" },
            path.as_ref().display()
        );
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

/// Creates a new CSV file for people with the correct headers.
pub fn create_new_csv_file<P: AsRef<std::path::Path>>(path: P) -> Result<(), Box<dyn std::error::Error>> {
    let mut writer = csv::Writer::from_path(&path)?;
    writer.write_record(CSV_HEADERS)?;
    writer.flush()?;
    log::info!("Created new CSV file: {:}", path.as_ref().display());
    Ok(())
}
