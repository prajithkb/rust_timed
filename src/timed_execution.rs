use std::fs::File;
use std::io::prelude::*;
use std::sync::Arc;
use std::sync::Mutex;
use std::time::Instant;
type ThreadSafeFile = Arc<Mutex<Option<File>>>;

lazy_static! {
    static ref TIMING_FILE: ThreadSafeFile = Arc::new(Mutex::new(File::create("timing.txt").ok()));
}

/// logs the timing information into "timing.txt" file
fn log(message: String) {
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

pub fn log_time<'a>(function: &'static str) -> LogTime {
    LogTime {
        function,
        start: Instant::now(),
    }
}

pub struct LogTime<'a> {
    function: &'a str,
    start: Instant,
}

impl<'a> Drop for LogTime<'a> {
    fn drop(&mut self) {
        let ns = self.start.elapsed().as_micros();
        let printable_value = format!("[timed]:[function:{}]:[{} ms, {} ns]\n", self.function, ns / 1000, ns);
        println!("{}", printable_value);
        log(printable_value);
    }
}


