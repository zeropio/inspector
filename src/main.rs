use crate::process::{access_proc, get_all_process_info};
use std::{thread, time::Duration};

mod process;
mod utils;

fn main() {
    let update_interval = Duration::from_secs(5);

    // Monitor
    loop {
        access_proc();
        let processes = get_all_process_info();

        // Testing information returned
        for process in processes {
            println!(
                "PID: {}, User: {}, Command: {}, CPU Usage: {}, \
            Memory Usage: {}",
                process.pid(),
                process.user(),
                process.command(),
                process.cpu_usage(),
                process.mem_usage()
            );
        }

        // Pause the loop for the specified update interval
        thread::sleep(update_interval);
    }
}
