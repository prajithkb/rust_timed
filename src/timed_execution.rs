use std::fs::File;
use std::io::prelude::*;
use std::sync::Arc;
use std::sync::Mutex;
use std::{env, time::Instant};
type ThreadSafeFile = Arc<Mutex<Option<File>>>;

lazy_static! {
    static ref TIMING_FILE: ThreadSafeFile = Arc::new(Mutex::new(File::create("timing.txt").ok()));
}

/// logs the timing information into "timing.txt" file
fn write_to_file(message: String) {
    let handle =TIMING_FILE.clone();
    let mut thread_safe_file = handle.lock().unwrap();
    if let Some(file) = thread_safe_file.as_mut() {
        file.write_all(&String::into_bytes(message))
            .expect("Unable to log time");
    } else {
        println!("[timed]: file handle closed")
    }
}

/// used to explicitly close the timing file handle
pub fn _close() {
    let handle =TIMING_FILE.clone();
    let mut thread_safe_file = handle.lock().unwrap();
    if let Some(file) = thread_safe_file.as_mut() {
        drop(file);
        *thread_safe_file = None;
        println!("[timed]: Closed file");
    } else {
        println!("[timed]: file handle closed already");
    }
    
}

pub fn log_time<'a>(function: &str) -> LogTime {
    LogTime {
        function,
        start: Instant::now(),
        to_file: false
    }
}

pub fn log_time_to_file<'a>(function: & str) -> LogTime {
    LogTime {
        function,
        start: Instant::now(),
        to_file: true
    }
}

pub struct LogTime<'a> {
    function: &'a str,
    start: Instant,
    to_file: bool
}

impl<'a> Drop for LogTime<'a> {
    fn drop(&mut self) {
        if let Ok(_) = env::var("TIMED_ENABLED") {
            let us = self.start.elapsed().as_micros();
            let ms = self.start.elapsed().as_millis();
            let printable_value = format!("[timed]:[function:{}]:[{} ms, {} us]\n", self.function, ms, us);
            if self.to_file {
                write_to_file(printable_value);
            } else {
                println!("{}", printable_value);
            } 
        }
        
    }
}


