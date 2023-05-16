use std::{fs::File, io::Write};
use chrono::Utc;

use crate::timer::Timer;

static mut log_file: Option<File> = None;
static mut start_of_game: Option<Timer> = None;

pub fn create_log_file() {
    unsafe {
        start_of_game = Some(Timer::new())
    }

    let date = Utc::now();

    match std::fs::read_dir("logs") {
        Ok(entrys) => {
            for entry in entrys {
                if let Ok(entry) = entry {
                    if let Some((file_date, file_index)) = entry.file_name().to_str().unwrap().split_once(" ") {
                        if file_date == date.format("%d-%m-%Y").to_string() {
                            let file_index = file_index.get(0..1).unwrap();
                            let file_index: u32 = file_index.trim().parse().unwrap();
                            let file_index = file_index + 1;

                            unsafe {
                                log_file = Some(File::create(format!("logs/{} {file_index}.txt", date.format("%d-%m-%Y"))).unwrap())
                            }
                        }
                    }
                }
            }

            if unsafe {&log_file}.is_none() {
                unsafe {
                    log_file = Some(File::create(format!("logs/{}.txt", date.format("%d-%m-%Y 0"))).unwrap());
                }
            }
        }
        Err(_) => {
            std::fs::create_dir("logs");
            
            unsafe {
                log_file = Some(File::create(format!("logs/{}.txt", date.format("%d-%m-%Y 0"))).unwrap());
            }
        }
    }
}

/// this is gonna add the time that the program has been open for in milliseconds as a prefix
/// 
/// make sure it doesnt end with a \n
pub fn log(text: impl ToString) {
    let elapsed = unsafe {
        start_of_game.as_mut().unwrap().tick();
        start_of_game.as_ref().unwrap().elapsed_milliseconds()
    };

    let text = format!("[{elapsed}] {}\n", text.to_string());

    unsafe {
        loop {
            match log_file.as_mut().unwrap().write(text.as_bytes()) {
                Ok(num_bytes) => {
                    if num_bytes < text.as_bytes().len() {
                        continue;
                    } else {break;}
                }
                Err(err) => {
                    println!("{err}");
                    panic!();
                }
            }
        }
    }
}
