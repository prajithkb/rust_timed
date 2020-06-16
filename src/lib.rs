//!
//! 
//! A simple library that provides a macro to measure the wall clock duration of a function.
//! 
//! ### By default timing is disabled, run your binary with `TIMED_ENABLED=1` to collect these measurements
//! ```ignore
//! fn some_function() {
//!     timed!("some_function")
//!     //... your logic
//! }
//! ```
//! This will print the duration in the following format to stdout
//! 
//! ```ignore
//! [timed_execution]:[function:some_function]:[134 ms, 134099 ns]
//! ```
//! if you want the output in a file use 
//! ```ignore
//! timed_to_file!("some_function")
//! ```
//! This will write the timings to timing.txt in the current folder
//! 
//! **Note:** Make sure to add the following to your `main.rs` or `lib.rs` 
//! ```ignore
//! #[macro_use]
//! extern crate timed;
//! ```
//! 
#[macro_use]
extern crate lazy_static;
pub mod timed_execution;


/// Used to log the time to stdout
#[macro_export]
macro_rules! timed {
    ($function_name:expr) => {
        let _log_time = $crate::timed_execution::log_time($function_name);
    };
}
/// Used to log time to "timing.txt" in the current folder
#[macro_export]
macro_rules! timed_to_file {
    ($function_name:expr) => {
        let _log_time = $crate::timed_execution::log_time_to_file($function_name);
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn measures_duration() {
        timed!("timed_test");
    }
}