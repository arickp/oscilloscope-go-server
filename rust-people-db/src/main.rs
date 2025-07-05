mod cli;
mod constants;
mod person;

use std::env;
use std::io::{self, Write};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    if cli::should_run_cli() {
        cli::run()?;
    } else {
        // Print current directory and prompt for file path
        let cwd = env::current_dir()?;
        println!("Current directory: {}", cwd.display());
        print!("Enter path to CSV file: ");
        io::stdout().flush()?;
        let mut file = String::new();
        io::stdin().read_line(&mut file)?;
        let file = file.trim().to_string();
        // If file doesn't exist, create it with headers
        if !std::path::Path::new(&file).exists() {
            println!("File '{}' does not exist. Creating new file...", file);
            // Write CSV headers for Person
            let mut writer = csv::Writer::from_path(&file)?;
            writer.write_record(["first_name", "last_name", "date_of_birth", "favorite_sport"])?;
            writer.flush()?;
        }
        cli::interactive_cli(file)?;
    }
    Ok(())
}
