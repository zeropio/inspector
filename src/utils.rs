use std::fs::File;
use std::io::{self, Read}; // Import the `io` module and `Read` trait
use std::path::Path;

// A simple implementation of `% cat path`
pub fn cat(path: &Path) -> io::Result<String> {
    let mut f = File::open(path)?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}
