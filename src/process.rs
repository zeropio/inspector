use std::fs;
use std::path::PathBuf;
use lazy_static::lazy_static;
use parking_lot::Mutex;
use std::collections::VecDeque;
use crate::utils::{cat, process_search_line, get_username_from_uid};

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
    path.file_name()
        .and_then(|n| n.to_str())
        .and_then(|s| s.parse::<i32>().ok())
        .is_some()
}

// Parser proc
fn parse_proc(path: &PathBuf) {
    // Placeholder: Implement logic to get user, cpu_usage, mem_usage, and command
    let pid: i32;
    let user: String;
    let user = "placeholder_user".to_string(); // Replace this with actual logic
    let cpu_usage = 0.0; // Placeholder
    let mem_usage = 0.0; // Placeholder
    let command = "placeholder_command".to_string(); // Placeholder

    // Get PD
    // Attempt to extract the file name as a string and parse it as i32
    let file_name = match path.file_name().and_then(|n| n.to_str()) {
        Some(name) => name,
        None => return, // Early return if file_name is not found or not a valid UTF-8 string
    };

    pid = match file_name.parse::<i32>() {
        Ok(pid) => pid,
        Err(_) => return, // Early return if file_name cannot be parsed as i32
    };

    // Get username
    let status_path = path.join("status");
    let mut uid_str: String = String::new();

    match cat(&status_path) {
        Ok(content) => {
            uid_str = process_search_line(&content, "Uid");
        },
        Err(e) => {
            println!("Error reading file: {}", e);
            // uid_str already initialized to an empty string, so no need to reassign
        },
    }

    match uid_str.parse::<u32>() {
        Ok(uid) => {
            match get_username_from_uid(uid) {
                Some(username) => user = username,
                None => println!("No user found for UID {}", uid),
            }
        },
        Err(_) => println!("Failed to parse UID from string: {}", uid_str),
    }

    // Add process info
    add_process_info(ProcessInfo {
        pid,
        user,
        cpu_usage,
        mem_usage,
        command,
    });
}

// Access proc
pub fn access_proc() {
    if let Ok(paths) = fs::read_dir("/proc") {
        for path in paths.filter_map(Result::ok) {
            let path_buf = path.path();
            if check_proc(&path_buf) {
                parse_proc(&path_buf);
            }
        }
    }
}
