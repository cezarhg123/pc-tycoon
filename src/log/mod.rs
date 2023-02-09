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

static mut SAVED: bool = false;

pub fn save_log() {
    if unsafe {!SAVED} {
        match std::fs::create_dir("logs/") {
            Ok(_) => {}
            Err(_) => {}
        }
        
        let mut begin_num: u32 = 1;
        for dir in std::fs::read_dir("logs/").unwrap() {
            if let Ok(dir) = dir {
                let name = dir.file_name().to_str().unwrap().to_string();
                begin_num = name.get(0..1).unwrap().parse().unwrap();
                begin_num += 1;
            }
        }
        
        let current_time = Utc::now();
        let path = format!("logs/{}.txt", current_time.format(format!("{} %d-%m-%Y", begin_num).as_str()).to_string());
        let mut log_file = std::fs::File::create(path).unwrap();
        
        unsafe {
            for log in &LOGS {
                log_file.write(log.as_bytes()).unwrap();
                log_file.write(b"\n").unwrap();
            }

            SAVED = true;
        }
    }
}
