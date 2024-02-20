use std::fs;
use std::path::PathBuf;
use lazy_static::lazy_static;
use parking_lot::Mutex;
use std::collections::VecDeque;
use crate::utils::cat;

// Create global structure
struct ProcessInfo {
    pid: i32,
    user: String,
    cpu_usage: f32,
    mem_usage: f32,
    command: String,
}

lazy_static! {
    static ref PROCESS_INFO: Mutex<VecDeque<ProcessInfo>> = Mutex::new(VecDeque::new());
}

// Function to add a new process info
fn add_process_info(info: ProcessInfo) {
    let mut data = PROCESS_INFO.lock();
    data.push_back(info);
}

// Check if path is a PID
fn check_proc(path: &PathBuf) -> bool {
    if let Some(file_name) = path.file_name() {
        // Convert OsStr to a &str
        if let Some(file_name_str) = file_name.to_str() {
            // Attempt to parse the file name as an integer
            match file_name_str.parse::<i32>() {
                Ok(num) => return true,
                Err(_) => return false,
            }
        }
    }

    return false
}

// Parser proc
fn parse_proc(path: &PathBuf) {
    let pid: &str;
    let user: &str;
    let cpu_usage: &str;
    let mem_usage: &str;
    let command: &str;

    // Get PID
    if let Some(file_name) = path.file_name() {
        pid = file_name.to_str();
    }

    // Get User
    cat(path)
}

// Access proc
pub fn access_proc() {
    match fs::read_dir("/proc") {
        Err(why) => println!("! {:?}", why.kind()),
        Ok(paths) => for path in paths {
            let path = match path {
                Ok(p) => p.path(),
                Err(_) => continue, // If the path cannot be read, skip to the next one
            };

            if check_proc(&path) {
                parse_proc(&path)
            }
        },
    }
}
