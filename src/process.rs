use crate::utils::{
    cat, format_memory_size, format_process_time, get_username_from_uid, parse_statm_content,
    parse_utime_and_stime, process_search_line,
};
use lazy_static::lazy_static;
use parking_lot::Mutex;
use rayon::prelude::*;
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
    nice_value: i32,
    vm: String,
    res: String,
    shr: String,
    cpu_usage: String,
    mem_usage: String,
    time: String,
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
    pub fn nice_value(&self) -> i32 {
        self.nice_value
    }
    pub fn vm(&self) -> &String {
        &self.vm
    }
    pub fn res(&self) -> &String {
        &self.res
    }
    pub fn shr(&self) -> &String {
        &self.shr
    }
    pub fn cpu_usage(&self) -> &String {
        &self.cpu_usage
    }
    pub fn mem_usage(&self) -> &String {
        &self.mem_usage
    }
    pub fn time(&self) -> &String {
        &self.time
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

    // Get NI
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
    let parts: Vec<&str> = stat_content.split_whitespace().collect();
    let nice_value = parts.get(18).unwrap_or(&"").parse::<i32>().unwrap_or(0) - 20; // Adjusting back from kernel representation

    // Get VIRT, RES and SHR
    let statm_path = path.join("statm");
    let statm_content = match cat(&statm_path) {
        Ok(content) => content,
        Err(e) => {
            println!("Error reading file: {}", e);
            return;
        }
    };

    // Assuming parse_statm_content(statm_content: &str) -> Result<(usize, usize, usize), &'static str>
    let (vm_kb, res_kb, shr_kb) = match parse_statm_content(statm_content) {
        Ok((vm, res, shr)) => (vm, res, shr),
        Err(e) => {
            println!("Error parsing statm content: {}", e);
            return;
        }
    };

    // Convert VIRT, RES, SHR to strings
    let vm = format!("{} KB", vm_kb);
    let res = format!("{} KB", res_kb);
    let shr = format!("{} KB", shr_kb);

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

    let mem_usage = format_memory_size(mem_usage_f32);

    // Get time
    let time = format_process_time(utime, stime);
    println!("utime: {}, stime: {}, time: {}", utime, stime, time);

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

    // Add process info
    add_process_info(ProcessInfo {
        pid,
        user,
        nice_value,
        vm,
        res,
        shr,
        cpu_usage,
        mem_usage,
        time,
        command,
    });
}

// Access proc
pub fn access_proc() {
    if let Ok(paths) = fs::read_dir("/proc") {
        paths
            .filter_map(Result::ok)
            .collect::<Vec<_>>()
            .par_iter()
            .for_each(|entry| {
                let path_buf = entry.path();
                if check_proc(&path_buf) {
                    parse_proc(&path_buf);
                }
            });
    }
}
