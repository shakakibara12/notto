use chrono::{Datelike, Local};
use std::{fs::write, path::PathBuf};

// TODO: Optimize chrono crate, so that less dependencies are pulled.

fn main() -> std::io::Result<()> {
    let now = Local::now();

    let year = now.year();
    let month = now.month();
    let day = now.day();

    let note_today = format!("{year}-{month}-{day}.md");

    // use PathBuf instead of plain format!. Maybe this is the better way?
    let note_path = PathBuf::from("/home/shaka/Documents/notes/毎日/").join(&note_today);

    // Look if the `%Z` can be configured to show the timezone in string like "IST"
    // as compared to numeric like "05:30"

    let formatted_date = now.format("%a %b %d %I:%M:%S %p %Z %Y").to_string();

    let contents = format!("# {} \n\n ## Intentions \n\n ## Logs \n\n", formatted_date);

    write(&note_path, contents)?;

    Ok(())
}
