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
    static ref PROCESS_INFO: Mutex<VecDeque<ProcessInfo>> =
        Mutex::new(VecDeque::new());
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
    // Attempt to extract the file name as a string and parse it as i32 for PID
    let pid = match path.file_name().and_then(|n|
            n.to_str()).and_then(|name| name.parse::<i32>().ok()) {
        Some(pid) => pid,
        None => {
            println!("Failed to parse PID");
            return;
        },
    };

    // Get username by UID
    let status_path = path.join("status");
    let uid_str = match cat(&status_path) {
        Ok(content) => process_search_line(&content, "Uid"),
        Err(e) => {
            println!("Error reading file: {}", e);
            return;
        },
    };

    let user = match uid_str.parse::<u32>().ok().and_then(|uid|
            get_username_from_uid(uid)) {
        Some(username) => username,
        None => {
            println!("No user found for UID {}", uid_str);
            return;
        },
    };

    // Get command
    let cmdline_path = path.join("cmdline");
    let command = match cat(&cmdline_path) {
        Ok(content) => {
            // Replace null bytes with spaces for readability
            content.replace("\0", " ").trim_end().to_string()
        },
        Err(e) => {
            println!("Error reading file: {}", e);
            return;
        },
    };

    // Placeholder values for CPU usage, memory usage, and command
    let cpu_usage = 0.0; // Placeholder
    let mem_usage = 0.0; // Placeholder

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
