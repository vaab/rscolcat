use crate::col;

pub fn run(files: Vec<String>) -> Result<bool, String> {
    match col::concat(files, std::io::stdout()) {
        Ok(()) => Ok(true),
        Err(e) => Err(e.to_string()),
    }
}
