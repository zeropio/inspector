use nix::unistd::User;
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

// Parse bytes
pub fn format_memory_size(kilobytes: f32) -> String {
    const KB_IN_MB: f32 = 1024.0;
    const KB_IN_GB: f32 = 1024.0 * 1024.0;

    if kilobytes >= KB_IN_GB {
        format!("{:.2} GB", kilobytes / KB_IN_GB)
    } else if kilobytes >= KB_IN_MB {
        format!("{:.2} MB", kilobytes / KB_IN_MB)
    } else {
        format!("{:.2} kB", kilobytes)
    }
}

// Parsing utime and stime
pub fn parse_utime_and_stime(stat_content: String) -> (f64, f64) {
    let parts: Vec<&str> = stat_content.split_whitespace().collect();

    // Common value, but it's better to check this for your system
    const TICKS_PER_SECOND: f64 = 100.0;

    let utime_ticks = parts
        .get(13)
        .and_then(|&s| s.parse::<f64>().ok())
        .unwrap_or(0.0);
    let stime_ticks = parts
        .get(14)
        .and_then(|&s| s.parse::<f64>().ok())
        .unwrap_or(0.0);

    let utime_seconds = utime_ticks / TICKS_PER_SECOND;
    let stime_seconds = stime_ticks / TICKS_PER_SECOND;

    (utime_seconds, stime_seconds)
}

// Parse statm
pub fn parse_statm_content(statm_content: String) -> Result<(usize, usize, usize), &'static str> {
    let parts: Vec<&str> = statm_content.split_whitespace().collect();

    // Assuming a system page size of 4096 bytes (4 KB).
    // You might want to dynamically fetch this value for accuracy.
    let page_size_kb = 4;

    let virtual_memory_kb = parts
        .get(0)
        .ok_or("Failed to read virtual memory size")
        .and_then(|v| {
            v.parse::<usize>()
                .map_err(|_| "Failed to parse virtual memory size")
        })?
        * page_size_kb;

    let resident_set_size_kb = parts
        .get(1)
        .ok_or("Failed to read resident set size")
        .and_then(|v| {
            v.parse::<usize>()
                .map_err(|_| "Failed to parse resident set size")
        })?
        * page_size_kb;

    let shared_memory_kb = parts
        .get(2)
        .ok_or("Failed to read shared memory size")
        .and_then(|v| {
            v.parse::<usize>()
                .map_err(|_| "Failed to parse shared memory size")
        })?
        * page_size_kb;

    Ok((virtual_memory_kb, resident_set_size_kb, shared_memory_kb))
}

// Format time
pub fn format_process_time(utime: f64, stime: f64) -> String {
    let total_time_in_secs = ((utime + stime) / 100.0).round() as i64;
    let hours = total_time_in_secs / 3600;
    let minutes = (total_time_in_secs % 3600) / 60;
    let seconds = total_time_in_secs % 60;
    format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
}

