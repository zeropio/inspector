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
    path.file_name()
        .and_then(|n| n.to_str())
        .and_then(|s| s.parse::<i32>().ok())
        .is_some()
}

// Parser proc
fn parse_proc(path: &PathBuf) {
    // Attempt to extract the file name as a string and parse it as i32
    let file_name = match path.file_name().and_then(|n| n.to_str()) {
        Some(name) => name,
        None => return, // Early return if file_name is not found or not a valid UTF-8 string
    };

    let pid = match file_name.parse::<i32>() {
        Ok(pid) => pid,
        Err(_) => return, // Early return if file_name cannot be parsed as i32
    };

    // At this point, you have a valid PID
    // Placeholder: Implement logic to get user, cpu_usage, mem_usage, and command
    let user = "placeholder_user".to_string(); // Replace this with actual logic
    let cpu_usage = 0.0; // Placeholder
    let mem_usage = 0.0; // Placeholder
    let command = "placeholder_command".to_string(); // Placeholder

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
