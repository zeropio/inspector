use std::fs::File;
use std::io::{self, Read}; // Import the `io` module and `Read` trait
use std::path::Path;
use nix::unistd::User;

// A simple implementation of `% cat path`
pub fn cat(path: &Path) -> io::Result<String> {
    let mut f = File::open(path)?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

// Find str in line from file
pub fn process_search_line(content: &str, search: &str) -> String {
    for line in content.lines() {
        if line.starts_with(search) {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() > 1 {
                return parts[1].to_string(); // Convert to String
            }
            break;
        }
    }
    String::new() // Return an empty string if not found
}

// Get username
pub fn get_username_from_uid(uid: u32) -> Option<String> {
    match User::from_uid(nix::unistd::Uid::from_raw(uid)) {
        Ok(Some(user)) => Some(user.name),
        _ => None,
    }
}