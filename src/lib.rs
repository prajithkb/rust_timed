//! Provides two macros to time your execution
//! 
//! Usage
//! ```ignore
//! 
//! extern crate timed;
//! use use timed::timed_block;
//! use timed::timed_fn;
//! 
//! fn main() {
//!     timed_block_fn();
//!     timed_attribute_fn_with_custom_name();
//!     timed_attribute_fn_with_no_name();
//! }
//! 
//! fn timed_block_fn() {
//!     timed_block!("timed_fn_name");
//!     println!("A timed block function");
//! }
//! 
//! #[timed_fn(name = "custom_name")]
//! fn timed_attribute_fn_with_custom_name() {
//!     println!("A timed attribute function with custom name");
//! }
//! 
//! #[timed_fn]
//! fn timed_attribute_fn_with_no_name() {
//!     println!("A timed attribute function no name");
//! }
//! ```
//! 
//! When the above code is run 
//! ```ignore
//! TIMED_ENABLED=1 cargo run;
//! 
//! //Outputs
//! 
//! A timed block function
//![timed]:[function:timed_fn_name]:[0 ms, 42 us]
//!
//!A timed attribute function with custom name
//![timed]:[function:timed_attribute_fn_with_custom_name]:[0 ms, 2 us]
//!
//!A timed attribute function no name
//![timed]:[function:timed_attribute_fn_with_no_name]:[0 ms, 1 us]
//! ```
//! 
//! Note: By default the **timed is disabled**, you need to run it with `TIMED_ENABLED=1`


pub mod timed_execution;
pub use timed_macro::timed_fn;

/// Used to log the time to stdout
/// ### Usage
/// ```ignore
/// {
///     timed_block!("a_timed_block")
///     // your code ...
/// }
/// ```
/// 
/// This will print `[timed]:[function:a_timed_block]:[0 ms, 15 us]` to std out
#[macro_export]
macro_rules! timed_block {
    ($function_name:expr) => {
        let _log_time = $crate::timed_execution::log_time($function_name);
    };
}
