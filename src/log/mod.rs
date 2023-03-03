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
        
        let current_time = Utc::now();
        let mut begin_num: u32 = 1;
        for dir in std::fs::read_dir("logs/").unwrap_or_else(|e| {
            log(format!("NH Err: {}", e.to_string()));
            save_log();
            panic!();
        }) {
            if let Ok(dir) = dir {
                let name = dir.file_name().to_str().unwrap_or_else(|| {
                    log("CRITICAL ERROR: file name not valid UTF 8");
                    save_log();
                    panic!();
                }).to_string();
                
                if name.contains(current_time.format("%d-%m-%Y").to_string().as_str()) {   
                    begin_num = name.get(0..1).unwrap_or_else(|| {
                        log("CRITICAL ERROR: cant get first letter in log.rs");
                        save_log();
                        panic!();
                    }).parse::<u32>().unwrap_or_else(|e| {
                        log(format!("NH Err: {}", e.to_string()));
                        save_log();
                        panic!();
                    });
                    begin_num += 1;
                }
            }
        }
        
        let path = format!("logs/{}.txt", current_time.format(format!("{} %d-%m-%Y", begin_num).as_str()).to_string());
        let mut log_file = std::fs::File::create(path).unwrap_or_else(|e| {
            log(format!("NH Err: {}", e.to_string()));
                save_log();
                panic!();
        });
        
        unsafe {
            for log_str in &LOGS {
                log_file.write(log_str.as_bytes()).unwrap_or_else(|e| {
                    log(format!("NH Err: {}", e.to_string()));
                    save_log();
                    panic!();
                });
                log_file.write(b"\n").unwrap_or_else(|e| {
                    log(format!("NH Err: {}", e.to_string()));
                    save_log();
                    panic!();
                });
            }

            SAVED = true;
        }
    }
}
