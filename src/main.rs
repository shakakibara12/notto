use chrono::{Duration, Local};
use std::{
    env, fs, io,
    path::PathBuf,
    process::{Command, exit},
};

const DEFAULT_DIR: &str = "Documents/notes/毎日";

fn get_editor() -> String {
    env::var("EDITOR").unwrap_or_else(|_| "nvim".to_string())
}

fn get_default_dir() -> PathBuf {
    let home = env::var("HOME").expect("Couldn't read HOME env");
    PathBuf::from(home).join(DEFAULT_DIR)
}

fn exec_neovim(note_path: &str) {
    let editor = get_editor();
    Command::new(editor)
        .arg("--")
        .arg(&note_path)
        .status()
        .expect("failed to execute neovim.");
}

fn help() -> String {
    "-n: to open the note in next day \n -p: to open note in previous day \n -h: to show this menu"
        .to_string()
}

fn parse_cli(args: &[String]) -> String {
    let file = match args.get(1).map(|s| s.as_str()) {
        Some("-n") => set_day(1),
        Some("-p") => set_day(-1),
        Some("-h") => help(),
        // If no matches found, continue to open today's note.
        _ => set_day(0),
    };
    file
}

fn set_day(num: i64) -> String {
    let now = Local::now();
    let next_day = now + Duration::days(num);
    let next_day = next_day.format("%Y-%m-%d").to_string();

    let note_path = format!("/home/shaka/Documents/notes/毎日/{next_day}.md");
    note_path
}

fn default_template(note_path: &str) -> Result<(), io::Error> {
    let now = Local::now();

    // Look if the `%Z` can be configured to show the timezone in string like "IST"
    // as compared to numeric like "05:30"
    // Looks like a lot of mess to implement that. See: https://github.com/chronotope/chrono/issues/960
    // ignoring.
    let formatted_date = now.format("%a %b %d %I:%M:%S %p %Z %Y").to_string();

    let contents = format!("# {} \n\n## Intentions \n\n## Logs \n\n", formatted_date);

    fs::write(&note_path, contents)?;

    Ok(())
}

fn main() {
    // Parse cli
    let args: Vec<String> = env::args().collect();

    let note_path = parse_cli(&args);

    // Exit if the file already exists, avoid overwriting it.
    if fs::exists(&note_path).unwrap() {
        exec_neovim(&note_path);
        exit(1)
    }

    default_template(&note_path).expect("Couldn't write the default content!");

    exec_neovim(&note_path);
}
