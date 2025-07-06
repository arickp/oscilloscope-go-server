mod cli;
mod constants;
mod person;

use rustyline::{Editor, Config, Helper};
use rustyline::completion::FilenameCompleter;
use rustyline::hint::HistoryHinter;
use rustyline::highlight::MatchingBracketHighlighter;
use rustyline::validate::MatchingBracketValidator;

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
    if cli::should_run_cli() {
        cli::run()?;
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
            // Write CSV headers for Person
            let mut writer = csv::Writer::from_path(&file)?;
            writer.write_record(["first_name", "last_name", "date_of_birth", "favorite_sport"])?;
            writer.flush()?;
        }
        cli::interactive_cli(file)?;
    }
    Ok(())
}
