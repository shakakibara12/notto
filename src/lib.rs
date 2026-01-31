use std::env;
use std::fs;
use std::path::PathBuf;

const DEFAULT_DIR: &str = "Documents/notes/毎日";

#[derive(Debug)]
pub struct Note {
    pub path: PathBuf,
}

fn get_default_dir() -> PathBuf {
    let home = env::var("HOME").expect("Couldn't read HOME env");
    let path = PathBuf::from(home).join(DEFAULT_DIR);
    if !path.exists() {
        fs::create_dir_all(&path).expect("Couldn't create directory");
    }
    path
}

impl Note {
    pub fn new() -> Note {
        Note {
            path: get_default_dir(),
        }
    }
}

impl Default for Note {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod test {
    use std::env;

    use crate::{DEFAULT_DIR, get_default_dir};

    #[test]
    fn test_default_dir() {
        let mut home = env::var("HOME").unwrap();
        // Because our DEFAULT_DIR doesn't start with a "/"
        home.extend(['/']);
        assert_eq!((home + DEFAULT_DIR), get_default_dir());
    }
}
