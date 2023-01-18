use std::{time::{Instant, Duration, SystemTime}, io::Write};

use chrono::{NaiveDateTime, NaiveDate, Utc};

static mut LOGS: Vec<String> = Vec::new();

pub fn log(log: String) {
    unsafe {
        LOGS.push(log);
    }
}

pub fn save_log() {
    match std::fs::create_dir("logs/") {
        Ok(_) => {}
        Err(_) => {}
    }

    let current_time = Utc::now();
    let path = format!("logs/{}.txt", current_time.format("%Y-%m-%d-%H-%M").to_string());
    let mut log_file = std::fs::File::create(path).unwrap();

    unsafe {
        for log in &LOGS {
            log_file.write(log.as_bytes()).unwrap();
            log_file.write(b"\n").unwrap();
        }
    }
}
