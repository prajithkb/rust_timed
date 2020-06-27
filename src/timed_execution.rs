use std::{env, time::Instant};

pub fn log_time<'a>(function: &str) -> LogTime {
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
        if let Ok(_) = env::var("TIMED_ENABLED") {
            let us = self.start.elapsed().as_micros();
            let ms = self.start.elapsed().as_millis();
            let printable_value = format!(
                "[timed]:[function:{}]:[{} ms, {} us]",
                self.function, ms, us
            );
            println!("{}", printable_value);
        }
    }
}
