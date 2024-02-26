use crate::utils::{
    cat, format_memory_size, get_username_from_uid, parse_utime_and_stime, process_search_line,
};
use lazy_static::lazy_static;
use parking_lot::Mutex;
use std::collections::VecDeque;
use std::fs;
use std::path::{Path, PathBuf};

//--------------------------------------------------------------------------------------------------
// Variables
//--------------------------------------------------------------------------------------------------

// Create global structure
#[derive(Clone)]
pub struct ProcessInfo {
    pid: i32,
    user: String,
    cpu_usage: String,
    mem_usage: String,
    command: String,
}

// Make values accessible
impl ProcessInfo {
    pub fn pid(&self) -> i32 {
        self.pid
    }
    pub fn user(&self) -> &String {
        &self.user
    }
    pub fn cpu_usage(&self) -> &String {
        &self.cpu_usage
    }
    pub fn mem_usage(&self) -> &String {
        &self.mem_usage
    }
    pub fn command(&self) -> &String {
        &self.command
    }
}

lazy_static! {
    static ref PROCESS_INFO: Mutex<VecDeque<ProcessInfo>> = Mutex::new(VecDeque::new());
}

//--------------------------------------------------------------------------------------------------
// ProcessInfo Manipulation Functions
//--------------------------------------------------------------------------------------------------

// Public function to get a snapshot of all current ProcessInfo
pub fn get_all_process_info() -> Vec<ProcessInfo> {
    let data = PROCESS_INFO.lock();
    data.iter().cloned().collect()
}

//--------------------------------------------------------------------------------------------------
// Access Process Functions
//--------------------------------------------------------------------------------------------------

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
    let pid = match path
        .file_name()
        .and_then(|n| n.to_str())
        .and_then(|name| name.parse::<i32>().ok())
    {
        Some(pid) => pid,
        None => {
            println!("Failed to parse PID");
            return;
        }
    };

    // Get username by UID
    let status_path = path.join("status");
    let uid_str = match cat(&status_path) {
        Ok(content) => process_search_line(&content, "Uid"),
        Err(e) => {
            println!("Error reading file: {}", e);
            return;
        }
    };

    let user = match uid_str
        .parse::<u32>()
        .ok()
        .and_then(|uid| get_username_from_uid(uid))
    {
        Some(username) => username,
        None => {
            println!("No user found for UID {}", uid_str);
            return;
        }
    };

    // Get command
    let cmdline_path = path.join("cmdline");
    let command = match cat(&cmdline_path) {
        Ok(content) => {
            // Replace null bytes with spaces for readability
            content.replace("\0", " ").trim_end().to_string()
        }
        Err(e) => {
            println!("Error reading file: {}", e);
            return;
        }
    };

    // Get CPU
    let uptime_path = Path::new("/proc/uptime");
    let uptime = match cat(&uptime_path) {
        Ok(content) => {
            let uptime_str = content.split_whitespace().next().unwrap_or("0");
            let uptime_seconds: f64 = uptime_str.parse().unwrap_or(0.0);
            uptime_seconds
        }
        Err(e) => {
            println!("Error reading file: {}", e);
            return;
        }
    };

    let stat_path = path.join("stat");
    let stat_content = match cat(&stat_path) {
        Ok(content) => {
            // Replace null bytes with spaces for readability
            content.replace("\0", " ").trim_end().to_string()
        }
        Err(e) => {
            println!("Error reading file: {}", e);
            return;
        }
    };
    let (utime, stime) = parse_utime_and_stime(stat_content);
    let cpus = num_cpus::get();

    let cpu_usage = format!(
        "{:.2}%",
        ((utime + stime) as f64 / uptime) * 100.0 / cpus as f64
    );

    // Get memory usage
    // we wil use status_path
    let mem_usage_string = match cat(&status_path) {
        Ok(content) => process_search_line(&content, "VmRSS"),
        Err(e) => {
            println!("Error reading file: {}", e);
            return;
        }
    };
    let mem_usage_f32 = mem_usage_string.parse::<f32>().unwrap_or(0.0);

    // Formatting mem_usage
    let mem_usage = format_memory_size(mem_usage_f32);

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
