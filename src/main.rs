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
                "PID: {}, User: {}, NI: {}, Virt: {}, RES: {}, SHR: {}, CPU Usage: {}, \
            Memory Usage: {}, Time: {}, Command: {}",
                process.pid(),
                process.user(),
                process.nice_value(),
                process.vm(),
                process.res(),
                process.shr(),
                process.cpu_usage(),
                process.mem_usage(),
                process.time(),
                process.command(),
            );
        }

        // Pause the loop for the specified update interval
        thread::sleep(update_interval);
    }
}
