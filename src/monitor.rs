use crate::process::{access_proc, get_all_process_info};
use std::{thread, time::Duration};

//--------------------------------------------------------------------------------------------------
// Monitor Functions
//--------------------------------------------------------------------------------------------------

fn collect_process_info() {
    let processes = get_all_process_info();

    for process in processes {
        println!("PID: {}, User: {}, Command: {}", process.pid(),
                 process.user(), process.command());
    }
}

pub fn start_monitoring() {
    let update_interval = Duration::from_secs(5);

    loop {
        access_proc();
        collect_process_info();


        // Pause the loop for the specified update interval
        thread::sleep(update_interval);
    }
}