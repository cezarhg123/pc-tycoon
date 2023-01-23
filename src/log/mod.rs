use std::{time::{Instant, Duration, SystemTime}, io::Write};

use chrono::{NaiveDateTime, NaiveDate, Utc};

static mut LOGS: Vec<String> = Vec::new();

pub fn log(log: impl ToString) {
    let mut log = log.to_string();
    let current_time = Utc::now();
    log.insert_str(0, current_time.format("[%H:%M:%S] ").to_string().as_str());
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
    let path = format!("logs/{}.txt", current_time.format("%d-%m-%Y").to_string());
    let mut log_file = std::fs::File::create(path).unwrap();

    unsafe {
        for log in &LOGS {
            log_file.write(log.as_bytes()).unwrap();
            log_file.write(b"\n").unwrap();
        }
    }
}
