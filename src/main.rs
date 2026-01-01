use chrono::{Datelike, Local};
use std::{
    fs, io,
    process::{Command, exit},
};

fn exec_neovim(note_path: &str) {
    Command::new("nvim")
        .arg("--")
        .arg(note_path)
        .status()
        .expect("failed to execute neovim.");
}

// TODO: Refactor note_path to another function, lifting more heavy lifting from main function.

fn main() -> io::Result<()> {
    let now = Local::now();

    let year = now.year();
    let month = now.month();
    let day = now.day();

    let note_path = format!("/home/shaka/Documents/notes/毎日/{year}-{month}-{day}.md");

    // Exit if the file already exists, avoid overwriting it.
    if fs::exists(&note_path).unwrap() {
        exec_neovim(&note_path);
        exit(1)
    }

    // Look if the `%Z` can be configured to show the timezone in string like "IST"
    // as compared to numeric like "05:30"
    let formatted_date = now.format("%a %b %d %I:%M:%S %p %Z %Y").to_string();

    let contents = format!("# {} \n\n ## Intentions \n\n ## Logs \n\n", formatted_date);

    fs::write(&note_path, contents)?;

    exec_neovim(&note_path);

    Ok(())
}
