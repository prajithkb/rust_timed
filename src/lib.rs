//!
//! 
//! A simple library that provides a macro to measure the wall clock duration of a function
//! ```ignore
//! fn some_function() {
//!     timed!("some_function")
//!     //... your logic
//! }
//! ```
//! This will print the duration in the following format
//! 
//! ```ignore
//! [timed_execution]:[function:some_function]:[134 ms, 134099 ns]
//! ```
//! The output is written to 'timing.txt' and to stdout.
//! 
//! Note: Make sure to add the following to your `main.rs` or `lib.rs` 
//! ```ignore
//! #[macro_use]
//! extern crate timed;
//! ```
//! 
#[macro_use]
extern crate lazy_static;
pub mod timed_execution;


#[macro_export]
macro_rules! timed {
    ($function_name:expr) => {
        let _log_time = $crate::timed_execution::log_time($function_name);
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn measures_duration() {
        timed!("timed_test");
    }
}