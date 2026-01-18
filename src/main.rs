use chrono::{Duration, Local};
use std::{
    env, fs, io,
    process::{Command, exit},
};

fn exec_neovim(note_path: &str) {
    Command::new("nvim")
        .arg("--")
        .arg(&note_path)
        .status()
        .expect("failed to execute neovim.");
}

fn parse_cli(args: &Vec<String>) -> String {
    let arg = &args[1];
    let file = match arg.as_str() {
        "-n" => next_day(),
        "-p" => prev_day(),
        // If no matches found, continue to open today's note.
        _ => today(),
    };
    file
}

// This will just return the next day's data format, eg, 2026-1-16.md . If today is 15
fn next_day() -> String {
    let now = Local::now();
    let next_day = now + Duration::days(1);
    let next_day = next_day.format("%Y-%m-%d").to_string();

    let note_path = format!("/home/shaka/Documents/notes/毎日/{next_day}.md");
    note_path
}

// See next_day()
fn prev_day() -> String {
    let now = Local::now();
    let prev_day = now - Duration::days(1);
    let prev_day = prev_day.format("%Y-%m-%d").to_string();

    let note_path = format!("/home/shaka/Documents/notes/毎日/{prev_day}.md");
    note_path
}

fn today() -> String {
    let now = Local::now();
    let today = now.format("%Y-%m-%d").to_string();

    let note_path = format!("/home/shaka/Documents/notes/毎日/{today}.md");
    note_path
}

fn main() -> io::Result<()> {
    // Parse cli
    let args: Vec<String> = env::args().collect();

    let note_path = parse_cli(&args);

    // Exit if the file already exists, avoid overwriting it.
    if fs::exists(&note_path).unwrap() {
        exec_neovim(&note_path);
        exit(1)
    }

    // Look if the `%Z` can be configured to show the timezone in string like "IST"
    // as compared to numeric like "05:30"
    let now = Local::now();
    let formatted_date = now.format("%a %b %d %I:%M:%S %p %Z %Y").to_string();

    let contents = format!("# {} \n\n## Intentions \n\n## Logs \n\n", formatted_date);

    fs::write(&note_path, contents)?;

    exec_neovim(&note_path);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_next_day() {
        let now = Local::now() + Duration::days(1);
        assert_eq!(now.format("%Y-%m-%d").to_string(), next_day())
    }
}
