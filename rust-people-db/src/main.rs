mod constants;
mod person;

use crate::constants::Sport;
use crate::person::{add_person, delete_person, edit_person, print_people, Person, create_new_csv_file};
use chrono::NaiveDate;
use clap::{Parser, Subcommand};
use inquire::{Select, Text};
use rustyline::{history::FileHistory, Editor, Config, Helper};
use rustyline::completion::FilenameCompleter;
use rustyline::hint::HistoryHinter;
use rustyline::highlight::MatchingBracketHighlighter;
use rustyline::validate::MatchingBracketValidator;
use std::io::{self, Write};
use env_logger;

#[derive(Parser)]
#[clap(author, version, about)]
pub struct Cli {
    /// The path to the CSV file containing the database
    file: String,

    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    Print,
    Delete {
        index: usize,
    },
    Edit {
        index: usize,
        #[arg(long)]
        first_name: Option<String>,
        #[arg(long)]
        last_name: Option<String>,
        #[arg(long)]
        date_of_birth: Option<String>,
        #[arg(long)]
        favorite_sport: Option<String>,
    },
    New {
        #[arg(long)]
        first_name: Option<String>,
        #[arg(long)]
        last_name: Option<String>,
        #[arg(long)]
        date_of_birth: Option<String>,
        #[arg(long)]
        favorite_sport: Option<String>,
    },
}

pub fn should_run_cli() -> bool {
    std::env::args().len() > 1
}

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        handle_command(cli.file, command)?;
    } else {
        interactive_cli(cli.file)?;
    }

    Ok(())
}

pub fn handle_command(file: String, command: Commands) -> Result<(), Box<dyn std::error::Error>> {
    let mut people = Person::read_from_csv(&file)?;

    match command {
        Commands::Print => print_people(&people),
        Commands::Delete { index } => {
            delete_person(&mut people, index)?;
            Person::write_to_csv(&file, &people)?;
        }
        Commands::Edit {
            index,
            first_name,
            last_name,
            date_of_birth,
            favorite_sport,
        } => {
            if index >= people.len() {
                return Err("Index out of bounds".into());
            }

            let mut person = people[index].clone();
            if let Some(first_name_val) = first_name {
                person.first_name = first_name_val;
            }
            if let Some(last_name_val) = last_name {
                person.last_name = last_name_val;
            }
            if let Some(dob) = date_of_birth {
                person.date_of_birth = NaiveDate::parse_from_str(&dob, "%Y-%m-%d")?;
            }
            if let Some(sport) = favorite_sport {
                person.favorite_sport = Sport::from_string(&sport);
            }

            edit_person(&mut people, index, person)?;
            Person::write_to_csv(&file, &people)?;
        }
        Commands::New {
            first_name,
            last_name,
            date_of_birth,
            favorite_sport,
        } => {
            let person =
                create_person_from_args(first_name, last_name, date_of_birth, favorite_sport)?;
            add_person(&mut people, person)?;
            Person::write_to_csv(&file, &people)?;
        }
    }

    Ok(())
}

fn create_person_from_args(
    first_name: Option<String>,
    last_name: Option<String>,
    date_of_birth: Option<String>,
    favorite_sport: Option<String>,
) -> Result<Person, Box<dyn std::error::Error>> {
    let first_name = first_name.unwrap_or_else(|| "Unknown".to_string());
    let last_name = last_name.unwrap_or_else(|| "Unknown".to_string());
    let date_of_birth = if let Some(dob) = date_of_birth {
        NaiveDate::parse_from_str(&dob, "%Y-%m-%d")?
    } else {
        NaiveDate::from_ymd_opt(1900, 1, 1).unwrap()
    };
    let favorite_sport = if let Some(s) = favorite_sport {
        Sport::from_string(&s)
    } else {
        Sport::Other("Unknown".to_string())
    };

    Ok(Person::new(
        first_name,
        last_name,
        date_of_birth,
        favorite_sport,
    ))
}

pub fn interactive_cli(file: String) -> Result<(), Box<dyn std::error::Error>> {
    let mut people = Person::read_from_csv(&file)?;
    let mut unsaved_changes = false;
    let mut rl = Editor::<(), FileHistory>::new()?;

    loop {
        let prompt = if unsaved_changes {
            "> (unsaved) "
        } else {
            "> "
        };
        let readline = rl.readline(prompt);

        match readline {
            Ok(line) => {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.is_empty() {
                    continue;
                }

                let command = parts[0];
                let args = &parts[1..];

                match command {
                    "exit" | "quit" | "q" => {
                        if unsaved_changes {
                            print!(
                                "You have unsaved changes. Are you sure you want to exit? (y/N): "
                            );
                            io::stdout().flush()?;

                            let mut response = String::new();
                            io::stdin().read_line(&mut response)?;

                            if response.trim().to_lowercase() == "y"
                                || response.trim().to_lowercase() == "yes"
                            {
                                break;
                            }
                        } else {
                            break;
                        }
                    }
                    "save" | "write" | "s" | "w" => {
                        Person::write_to_csv(&file, &people)?;
                        unsaved_changes = false;
                        println!("Saved to {}", file);
                    }
                    "print" | "p" => print_people(&people),
                    "delete" | "d" => {
                        if let Some(index) = args.first().and_then(|s| s.parse::<usize>().ok()) {
                            if delete_person(&mut people, index).is_ok() {
                                unsaved_changes = true;
                                println!("Person at index {} deleted", index);
                            } else {
                                println!("Error: Index out of bounds");
                            }
                        } else {
                            println!("Usage: delete <index>");
                        }
                    }
                    "edit" | "e" => {
                        if let Some(index) = args.first().and_then(|s| s.parse::<usize>().ok()) {
                            if index >= people.len() {
                                println!("Error: Index out of bounds");
                                continue;
                            }

                            let mut person = people[index].clone();
                            println!("Editing person at index {}: {}", index, person.first_name);

                            // Interactive editing
                            print!("Enter new first name (or leave blank): ");
                            io::stdout().flush()?;
                            let mut input = String::new();
                            io::stdin().read_line(&mut input)?;
                            let input_trimmed = input.trim();
                            if !input_trimmed.is_empty() {
                                person.first_name = input_trimmed.to_string();
                            }

                            print!("Enter new last name (or leave blank): ");
                            io::stdout().flush()?;
                            input.clear();
                            io::stdin().read_line(&mut input)?;
                            let input_trimmed = input.trim();
                            if !input_trimmed.is_empty() {
                                person.last_name = input_trimmed.to_string();
                            }

                            print!("Enter new date of birth (YYYY-MM-DD) (or leave blank): ");
                            io::stdout().flush()?;
                            input.clear();
                            io::stdin().read_line(&mut input)?;
                            let input_trimmed = input.trim();
                            if !input_trimmed.is_empty() {
                                if let Ok(date) =
                                    NaiveDate::parse_from_str(input_trimmed, "%Y-%m-%d")
                                {
                                    person.date_of_birth = date;
                                } else {
                                    println!("Invalid date format. Keeping existing date.");
                                }
                            }

                            // Use sport menu with current sport as default
                            println!("Edit favorite sport (or leave blank to keep current):");
                            let sport_input = prompt_for_sport_with_default(Some(&person.favorite_sport));
                            if let Some(sport) = sport_input {
                                person.favorite_sport = sport;
                            }

                            if edit_person(&mut people, index, person).is_ok() {
                                unsaved_changes = true;
                                println!("Person updated successfully");
                            }
                        } else {
                            println!("Usage: edit <index>");
                        }
                    }
                    "new" | "n" => {
                        println!("Adding new person:");

                        print!("Enter first name: ");
                        io::stdout().flush()?;
                        let mut first_name = String::new();
                        io::stdin().read_line(&mut first_name)?;
                        let first_name = first_name.trim().to_string();

                        print!("Enter last name: ");
                        io::stdout().flush()?;
                        let mut last_name = String::new();
                        io::stdin().read_line(&mut last_name)?;
                        let last_name = last_name.trim().to_string();

                        print!("Enter date of birth (YYYY-MM-DD): ");
                        io::stdout().flush()?;
                        let mut date_input = String::new();
                        io::stdin().read_line(&mut date_input)?;
                        let date_input = date_input.trim();

                        let date_of_birth =
                            if let Ok(date) = NaiveDate::parse_from_str(date_input, "%Y-%m-%d") {
                                date
                            } else {
                                println!("Invalid date format. Using 1900-01-01 as default.");
                                NaiveDate::from_ymd_opt(1900, 1, 1).unwrap()
                            };

                        // Use sport menu
                        let favorite_sport =
                            prompt_for_sport().unwrap_or(Sport::Other("Unknown".to_string()));
                        let person =
                            Person::new(first_name, last_name, date_of_birth, favorite_sport);
                        if add_person(&mut people, person).is_ok() {
                            unsaved_changes = true;
                            println!("Person added successfully");
                        }
                    }
                    "help" | "h" => {
                        println!("Available commands:");
                        println!("  print, p          - Display all people");
                        println!("  new, n            - Add a new person");
                        println!("  edit <index>, e   - Edit person at index");
                        println!("  delete <index>, d - Delete person at index");
                        println!("  save/write, s/w   - Save changes to file");
                        println!("  exit, quit        - Exit the program");
                        println!("  help, h           - Show this help");
                        println!("  Note: favorite_sport only accepts known values.");
                        let valid_sports = Sport::all_known_sports();
                        println!(
                            "  Valid options: {}",
                            valid_sports
                                .iter()
                                .map(|s| s.to_string())
                                .collect::<Vec<_>>()
                                .join(", ")
                        );
                    }
                    _ => {
                        println!(
                            "Unknown command: {}. Type 'help' for available commands.",
                            command
                        );
                    }
                }
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }

    Ok(())
}

fn prompt_for_sport() -> Option<Sport> {
    prompt_for_sport_with_default(None)
}

fn prompt_for_sport_with_default(default_sport: Option<&Sport>) -> Option<Sport> {
    let mut sport_variants = Sport::all_known_sports();
    sport_variants.sort_by(|a, b| a.to_string().cmp(&b.to_string()));
    let mut options: Vec<String> = sport_variants
        .iter()
        .map(|sport| format!("{} {}", capitalize_first(&sport.to_string()), sport.emoji()))
        .collect();
    options.push("Other...".to_string());
    
    // Try a different approach - manually set the default by reordering options
    let mut reordered_options = options.clone();
    if let Some(default) = default_sport {
        if let Sport::Other(_) = default {
            // For "Other" sports, move "Other..." to the top
            if let Some(other_pos) = reordered_options.iter().position(|opt| opt == "Other...") {
                let other_option = reordered_options.remove(other_pos);
                reordered_options.insert(0, other_option);
            }
        } else {
            // Move the default sport to the top
            let default_display = format!("{} {}", capitalize_first(&default.to_string()), default.emoji());
            if let Some(default_pos) = reordered_options.iter().position(|opt| opt == &default_display) {
                let default_option = reordered_options.remove(default_pos);
                reordered_options.insert(0, default_option);
            }
        }
    }
    
    println!("Choose a favorite sport:");
    let ans = Select::new("Select a sport:", reordered_options.clone())
        .prompt();
    match ans {
        Ok(choice) => {
            if choice == "Other..." {
                let custom = Text::new("Enter custom sport:").prompt();
                match custom {
                    Ok(val) => {
                        let trimmed = val.trim();
                        if trimmed.is_empty() {
                            None
                        } else {
                            Some(Sport::Other(trimmed.to_string()))
                        }
                    }
                    Err(_) => None,
                }
            } else {
                // Find the original sport variant by matching the display string
                let selected = sport_variants.iter().find(|sport| {
                    format!("{} {}", capitalize_first(&sport.to_string()), sport.emoji()) == choice
                });
                selected.cloned()
            }
        }
        Err(_) => None,
    }
}

fn capitalize_first(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

struct MyHelper {
    completer: FilenameCompleter,
    hinter: HistoryHinter,
    _highlighter: MatchingBracketHighlighter,
    _validator: MatchingBracketValidator,
}

impl Helper for MyHelper {}
impl rustyline::completion::Completer for MyHelper {
    type Candidate = rustyline::completion::Pair;
    fn complete(
        &self,
        line: &str,
        pos: usize,
        ctx: &rustyline::Context<'_>,
    ) -> rustyline::Result<(usize, Vec<Self::Candidate>)> {
        self.completer.complete(line, pos, ctx)
    }
}
impl rustyline::hint::Hinter for MyHelper {
    type Hint = String;
    fn hint(&self, line: &str, pos: usize, ctx: &rustyline::Context<'_>) -> Option<String> {
        self.hinter.hint(line, pos, ctx)
    }
}
impl rustyline::highlight::Highlighter for MyHelper {}
impl rustyline::validate::Validator for MyHelper {}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    
    if should_run_cli() {
        run()?;
    } else {
        // Print current directory and prompt for file path
        let cwd = std::env::current_dir()?;
        println!("Current directory: {}", cwd.display());
        
        // Use rustyline for file path input with tab completion
        let config = Config::builder().build();
        let h = MyHelper {
            completer: FilenameCompleter::new(),
            hinter: HistoryHinter {},
            _highlighter: MatchingBracketHighlighter::new(),
            _validator: MatchingBracketValidator::new(),
        };
        let mut rl = Editor::with_config(config)?;
        rl.set_helper(Some(h));
        let file = match rl.readline("Enter path to CSV file: ") {
            Ok(line) => line.trim().to_string(),
            Err(e) => {
                eprintln!("Error reading input: {}", e);
                return Err(e.into());
            }
        };
        // If file doesn't exist, create it with headers
        if !std::path::Path::new(&file).exists() {
            println!("File '{}' does not exist. Creating new file...", file);
            create_new_csv_file(&file)?;
        }
        interactive_cli(file)?;
    }
    Ok(())
}
