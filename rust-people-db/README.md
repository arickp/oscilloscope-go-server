# rust-people-db

A simple Rust CLI tool to manage a database of people stored in a CSV file.

## Usage

You can use the CLI to add a new person to your database CSV file. For example, to add John Smith, born on 1960-10-10, whose favorite sport is football, run:

```
cargo run -- people.csv new --first-name John --last-name Smith --date-of-birth 1960-10-10 --favorite-sport football
```

### Arguments
- `people.csv`: The path to your CSV file (will be created if it doesn't exist).
- `new`: The subcommand to add a new person.
- `--first-name`: The person's first name (e.g., `John`).
- `--last-name`: The person's last name (e.g., `Smith`).
- `--date-of-birth`: The person's date of birth in `YYYY-MM-DD` format (e.g., `1960-10-10`).
- `--favorite-sport`: The person's favorite sport (e.g., `football`).
    - Valid options: baseball, soccer, basketball, tennis, golf, hockey, cricket, rugby, handball, football, volleyball, water polo, equestrian, swimming, running, cycling, skating, skateboarding, surfing, skiing, snowboarding, rowing

## Other Commands
- `print`: Show all people in the database.
- `edit`: Edit an existing person by index.
- `delete`: Delete a person by index.

For more help, run:

```
cargo run -- --help
``` 

## Author
[Eric Popelka](https://github.com/arickp)
